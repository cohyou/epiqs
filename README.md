# epiqs

## 概要

- epiqsは、プログラミング言語である。主に、Lisp系言語に影響を受けている。
- Lispのコンスセルをそのまま書き下すような文法であり、この言語ではコンスセルをpiqと呼んでいる。
- piqではcarやcdrにあたるものをそれぞれpとqと呼び、<br>
またpiqはotagと呼ばれるいわば型情報を一つだけ持つことができる。
- otagはpiq作成時に埋め込まれた情報であり、変更不可である。
- otag付のpiqをepiq(=embedded tagged piq)と呼ぶ。

- piqで構成された式を、A式(A-Expression)と呼ぶ。

### 図解epiq
`(Otag p q)`が基本の形。
タグは大文字または記号から始め、シンボルは小文字から始める。
```
+------+-----+-----+
| Otag |  P  |  Q  |
+------+-----+-----+
```

## 文法

### g4 file(for ANTLR)

```
grammar epiqs;

efile: epiq* EOF;

elist: '[' epiqs ']';

etuple: '{' epiqs '}';

epiqs: (epiq WS+)*;

epiq : ('|' tag WS+ epiq WS+ epiq)
     | ('\'' tag WS+ epiq)
     | ('^' tag? WS+ epiq WS+ epiq)
     | '^{' epiq '}'
     | '^[' epiq ']'

tag : TAGHEAD ~(WS*)


literal : INT | STRING | NAME;

TAGHEAD :
        | ':'
        | '%' | '#' | '@' | '.'
        | '\' | '!' | '$' | '?'
        | '='
        | '/'
        | '&' | '~'
        | '+' | '-' | '*' | '/' | '`' | '<' | '>'
        | [A-Z]

INT : ('0'|[1-9][0-9]*);
STRING : '"' ( ~'"' | '\\' '"' )* '"' ;
NAME : [a-z]
     ('a'..'z' | 'A'..'Z' | '0'..'9'
     | '&' | '~'
     | '%' | '#' | '$' | '='
     | '+' | '-' | '*' | '/'
     | '<' | '>'])+

WS : [ \n\r\t] ;
```

#### antlr g4 注記

- ひとまず中置記法だけを書いている
- `{` ~ `}`と`[` ~ `]`関連に関しては、省略形だけを書いている
- `,`はまだ組み込んでいない


#### シンボル内に使える記号

- 1文字リテラルとディスパッチャに含まれる記号は使えない
- `:`は中置記法があるのでだめ
- 環境操作タグのうち、`%`と`#`は使える。<br>
  `@`もいいが前置では使えるので紛らわしいのでやめる<br>
  `.`は完全に中置記法で使う想定なのでだめ
- `!`は後置で使うのでだめ、`?`も三項演算子やるかもなので置いておく
- `$`と`=`はOK
- 余っている文字のうち、backquote 以外の`&` `~` `+` `-` `*` `/` `<` `>`は許可<br>
  `<` `>`は少し怖いがやってみる
- backquoteとbackslashはいけるはずだが怖いのでやめる


### 記号まとめ

種類|数|具体例
-|:-:|-
literal|1|`;`
parens|4|`[` `]` `{` `}`
dispatcher|5|`(` `)` `'` `^` `,`
tag|13|`:` `%` `#` `@` `.` `\` `!` `$` `?` `=` `/`
matching|2|`=` `_`
unused|7|`&` `~` `+` `-` `*` `/` `<` `>` backquote
合計|32|


### 各種キーワード

#### リテラル

正規表現|説明
:-:|-
数値|([1-9][0-9]*)&#x7C;0かな。
シンボル|[a-z&#x7C;A-z][a-z&#x7C;A-z&#x7C;0-9&#x7C;+-*\/]+ですね。
文字列|`"` ~ `"`
de bruijn index|`.[0-9]*` `.`の後に数値が続くと、de bruijn indexとみなす


#### リテラル(1文字)

記号|説明
:-:|-
`;`|Unit
`N`|nil
`T`|true
`F`|false


#### ディスパッチャ

記号|説明
:-:|-
`(` ~ `)`|piq(基本形)
&#x7C;|piq(p,qを指定)
`'`|piq(pのみを指定)
`^`|metadata 基本的には必ず2つ引数を取る
`,`|埋め込み 後に続くlistの[]を省略して書ける


#### タグ(基本)

記号|説明|単独
:-:|-|-
`:`|cons|中置記法でcons


#### タグ(リストリテラル)

記号|説明|単独
:-:|-|-
`{` ~ `}`|省略形 現在は`^*{` ~ `}`と同じ|
`^*{` ~ `}`|tuple|
`^+{` ~ `}`|enum|
`^#{` ~ `}`|hash|
`[` ~ `]`|省略形 現在は`^:[` ~ `]`と同じ|
`^:[` ~ `]`|list|
`^-[` ~ `]`|vector|


#### タグ(環境)

記号|説明|単独
:-:|-|-
`%`|environment|ナシ
`#`|bind|ナシ
`@`|resolve|前置記法でresolve
`.`|access|中置記法でもいける（間にWSは許されない


#### タグ(実行)

記号|説明|単独
:-:|-|-
`\`|block|ナシ
`>`|eval|ナシ
`!`|apply|中置記法でapply<br>（pとの間にWSは許されない
`$`|symbol|ナシ
`?`|condition|ナシ(中置記法は微妙)


#### タグ(マッチング)

記号|説明|単独
:-:|-|-
`=`|equal|色々な比較に使いたい 中置記法は迷ったがナシ
`^_(` ~ `)`|pattern|中に`_`がplaceholderで使われるので、<br>それだけでpatternと判別できればこれは不要


#### タグ(要仕様検討)

記号|説明|単独
:-:|-|-
`/`|path?|ファイルパス関連で使いたい


#### タグ(複数文字、確定)

記号|説明
:-:|-
&#x7C;&#x7C; ~ &#x7C;&#x7C;|comment(複数行)
`!?`|exception


#### タグ(複数文字、要仕様検討)

記号|説明
:-:|-
`''`|comment(単一行)
`!.`|yield （不要かも
`!<`|dispatch


#### タグ(一旦廃止、実行は|>でやる,quoteも必要なくなった)

記号|説明
:-:|-
`^{` ~ `}`|現在は`^!{` ~ `}`と同じ|ナシ
`^[` ~ `]`|現在は`^.[` ~ `]`と同じ|ナシ
`^!{` ~ `}`|中身を深さ優先で再帰的に評価|ナシ
`^.[` ~ `]`|quasiquote 実行を止める|ナシ


#### マクロかも(複数文字でのidiom)

記号|説明
:-:|-
`!&`|parallel
`.>`|print
`.>s`|format


#### その他未決定事項
- ASTをたどるQuery(XPathみたいになるよね)
- モジュールの具体的な仕様


#### tagとして余っている記号

```
Plus, // + plus
Hphn, // - hyphen-minus
Star, // * asterisk
Slsh, // / slash

Bkqt, // ` back quote
Less, // < less than
(使用済みになりました) Grtr, // > greater than
```


### マクロ

マクロは普通の関数と同様に作成する。
- 実行には`>>>>`タグか`^{` ~ `}`(これは`^!{` ~ `}`の略記)を使う。
- quasiquoteには`^[` ~ `]`(これは`^.[` ~ `]`の略記)を使う。

例として、`|!! symbol [args]`というタグがきたら、<br>
`|! @symbol [args]`に変更する関数を書いてみる。

```
|# .double-bang
|\ '% (piq) [
   |>>>> "実行用マクロを定義する"
   |? (= ^_(!! _)  .piq)
      |: ^[ |! (@ ^{ piq.p }) ^{ piq.q } ]
         piq
   ]

|Defn |Accs Unit double-bang
|Lmbd 'Envn 'Cons piq 'Cons
      |>>>> "実行用マクロを定義する"
      |Ifvl (Eql_ (!! _) |Accs Unit piq)
                  |Cons 'Quot |Appl (Rslv |Eval |Accs p piq) (Eval |Accs q piq)
            piq
```

次に、このマクロを適用したいコードブロックの環境内で、それらを紐付ける。<br>
あまりお得になっていない気がするが、まあしかたない。
```
|\ |% [a b], macro_function:@double-bang
   [ (!! plus [(!! plus [3 4]) (!! plus [5 6])]) ]
```

ちなみに、マクロ定義時に、引数で与えられたpiqから、<br>
.pや.qを使えば、子を辿ることは簡単だが、親や兄弟は辿れない。<br>
そのための何か文法は欲しい気はするが、親に影響を与えられてもいいんだろうか。


### コード例

```
'!?
  |:
    |# f @open! "myfile.txt"
    |# s @readline! f
    |# i @int! @s.strip!
  |:
    |: [ OSError   : (\ '.> '.>s ["OS error: {0}" .1])
         ValueError: (\ '.> "Could not convert data to an integer.")
         T         : |\ ['.> ["Unexpected error:" @sys.exc_info!.0] !?!] ]
       '.> [arg "has" @f.readlines!.size "lines"]
    @f.close!
```

```
'Html
  'Head
    'Title "onclick test"
    'Meta, http-equiv:"Content-Type", content:"text/html", charset:"utf-8"
    'Meta, name:"viewport", content:{width:divice-width initial-scale:1.0 minimum-scale:1.0 maximum-scale:1.0 user-scalable:no}
    ^!~ {
      (# leftpx 0)
      |# touch |\ '% ; [
        |# leftpx (+ .leftpx 10)
        (# (Selector#migi).style.left "{.leftpx}px")
      ]
      |# reset |\  '% ; [
        (# leftpx 0)
        (# (Selector#migi).style.left "{.leftpx}px")
      ]
    }
    'Style [
      ".yaji":{font-size:"20px"}
      ".btn":{font-size:"20px" width:"200px" height:"50px" text-align:"center" margin-top:"20px"}
    ]
  )
  |Body, bgcolor:"#FFFFFF", text:"#000000" [
    |Div#migi.yaji, style:{position:absolute left:'0px'} "■"
    'Div, style:{clear:both}
    |Div, style:{position:absolute top:'50px'} [
      |Button.btn, onclick:(! @touch) "onclick"
      |Button.btn, ontouchstart:(! @touch) "ontouchstart"
      |Button.btn, ontouchstart:(! @reset) "reset"
    ]
```

```
(FoldTagR +, 0, [1 2 3 4 5 6])
(FoldTagR :, ;, [1 2 3 4 5 6])

|# touch
   |\ '% ;
   [  |# leftpx (+ .leftpx 10)
      (# (Selector#migi).style.left "{.leftpx}px")]

|Defn touch:[],
  |# leftpx (+ .leftpx 10),
  (# (Selector#migi).style.left "{.leftpx}px")


|Defn sum:[a b], (+ .a .b)
|-> (+ .0 .1)
```
### epiq一覧(literalなど)

表記|対応する表現|説明
-|-|-
`Unit`|`;`|unit
`None`|`N`|nil null
`Tval`|`T`|true
`Fval`|`F`|false
`Int8(i64)`|`21` `745`|8byte integer
`Text(String)`|`"wowow"`|string
`Name(String)`|`map` `index`|symbol
`Plhd`|`_`|placeholder patternで使う

### epiq一覧(piq, 主に両方の引数を埋めるもの)

表記|対応する表現|説明
-|-|-
`Tpiq{_tag, pval, qval}`|`(_tag pval qval)`|tag assignable cons
`Lpiq{pval, qval}`|`(:a b)` '&#x7C;: a b' `': a` `a:b`|linked-list(normal cons cell)
`Bind{smbl, valu}`|`(# one 1)`|bind
`Accs{trgt, kynm}`|`(. piq q)` `obj.attr`|access
`Lmbd{envn, body}`|`'\ .0` `(\ '% [i] @incl!, .i)`|function piq block
`Same{val1, val2}`|`(= money happiness)`|equal
`Meta{mtag, trgt}`|`^{}` `^[]`|metadata
`Appl{func, args}`|`(! @p, "OMG")` `@p!, "Good"`|apply


### epiq一覧(piq, 主に2つ目の引数を埋めるもの)
表記|対応する表現|説明
-|-|-
`Envn{_, prms}`|`(% [i j] ^{})`|environment
`Rslv{_, smbl}`|`'@ sym` `@func`|resolve symbol
`Eval{_, qexp}`|`^{ @go-a-head! }`|exec eval


#### 廃止（tupleとenumはmetadataとしてのみ表現）

表記|対応する表現|説明
-|-|-
`Tupl{lpiq, rest}`|`(& a '&b)` `{a:1 b:2}`|tuple
`Enum{data, _}`|`(~ LIVE '~DIE) ^~{N E W S}`|enum


#### 廃止（defaultがQuoteになったので）

表記|対応する表現|説明
-|-|-
`Quot{qexp, _}`|`^[ a b c ]`|quote


## TODO

### その1 簡単そう
- Accs（とはいえ最初はpとqだけ）
- Same（これもまずは数値だけ）
- 文字列
- dispatcherの追加（'と&#x7C;だが、どちらがどちらかは迷っている）
- 中置記法(Lpiq, Accs, Rslv, Appl) 優先順位も決める必要がある
- N, T, F(ただ、すぐできるし、使い道が出た時で良いかも)
- プリミティブな数値の演算（加減乗除/ビット演算 shift, rotate）
- マクロをキックする（定義はできるようになったが、それをキックするタイミングを決めていない）


### その2 難しそう/大変そう
- パターンマッチ（これは難しそう）
- Tupl, Enum（仕様がよくわかっていない）
- Vector(Rustのarrayを使うことになる)
- エラー処理
- 配列というかイテレータ的な処理はなるべくタグで組み込みたいけれど、どうするのか
- 文字列関数
