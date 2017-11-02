# epiqs

## 概要

- epiqsは、プログラミング言語である。主に、Lisp系言語に影響を受けている。
- Lispのコンスセルをそのまま書き下すような文法であり、この言語ではコンスセルをpiqと呼んでいる。
- piqではcarやcdrにあたるものをそれぞれpとqと呼び、<br>
またpiqはtagと呼ばれるいわば型情報を一つだけ持つことができる。
- tagはpiq作成時に埋め込まれた情報であり、変更不可である。
- tag付のpiqをepiq(=embedded tagged piq)と呼ぶ。

- piqで構成された式を、Q式(Q-Expression)と呼ぶ。

### 図解epiq
`(tag p q)`が基本の形。

```
+-----+-----+-----+
| tag |  P  |  Q  |
+-----+-----+-----+
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

tag : | ':'
      | '&' | '~'
      | '%' | '#' | '@' | '.'
      | '\' | '!' | '$' | '?'
      | '='
      | '/'
      | '+' | '-' | '*' | '/' | '`' | '<' | '>'

literal : INT | STRING | NAME;

INT : ('0'|[1-9][0-9]*);
STRING : '"' ( ~'"' | '\\' '"' )* '"' ;
NAME : [a-z|A-z]
     ('a' .. 'z' | 'A' .. 'Z' | '0' .. '9'
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
- リストリテラルで使う`&` `~`はOK
- 環境操作タグのうち、`%`と`#`は使える。<br>
  `@`もいいが前置では使えるので紛らわしいのでやめる<br>
  `.`は完全に中置記法で使う想定なのでだめ
- `!`は後置で使うのでだめ、`?`も三項演算子やるかもなので置いておく
- `$`と`=`はOK
- 余っている文字のうち、` ` ` 以外の`+` `-` `*` `/` `<` `>`は許可<br>
  `<` `>`は少し怖いがやってみる
- `\``と'\'はいけるはずだが怖いのでやめる


### 記号まとめ

種類|数|具体例
-|:-:|-
literal|1|`;`
parens|4|`[` `]` `{` `}`
dispatcher|5|`(` `)` `'` `^` `,`
tag|13|`:` `&` `~` `%` `#` `@` `.` `\` `!` `$` `?` `=` `/`
matching|2|`=` `_`
unused|7|`+` `-` `*` `/` `<` `>` backquote
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
`&`|tuple|ナシ
`{` ~ `}`|省略形 現在は`^&{` ~ `}`と同じ|
`^&{` ~ `}`|tuple|
`^~{` ~ `}`|enum|
`^#{` ~ `}`|hash|
`~`|enum|ナシ
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
`!`|apply|中置記法でapply<br>（pとの間にWSは許されない
`$`|symbol|ナシ
`?`|condition|ナシ(中置記法は微妙)
`^{` ~ `}`|現在は`^!{` ~ `}`と同じ|ナシ
`^[` ~ `]`|現在は`^.[` ~ `]`と同じ|ナシ
`^!{` ~ `}`|中身を深さ優先で再帰的に評価|ナシ
`^.[` ~ `]`|quasiquote 実行を止める|ナシ


#### タグ(マッチング)

記号|説明|単独
:-:|-|-
`=`|equal|色々な比較に使いたい 中置記法も
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
Grtr, // > greater than
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
   |? (= ^_(!! _)  @piq)
      |: ^[ |! (@ ^{ piq.p }) ^{ piq.q } ]
         piq
   ]
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


### epiq一覧

- `Unit`
- `Int8(i64)`
- `Text(String)`

- `Lpiq { p: usize, q: usize }` // (linked) list piq
- `Vpiq { p: usize, q: usize }`, // vector piq
- `Fpiq { p: usize, q: usize }` // function piq
- `Aexp { a: usize, e: usize }` // A-Expression
- `Prmt(usize)` // anonymous parameter
- `Pprn(usize)` // priority parentheses
- `Dbri(usize)` // de bruijn index
