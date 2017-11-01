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
```
+-----+-----+-----+
| tag |  P  |  Q  |
+-----+-----+-----+
```

## 文法

### g4 file(for ANTLR)

```
grammar epiqs;

efile: form* EOF;

form: not_cons_form | pair | cons;

not_cons_form: literal | elist | vector | etuple;

forms: (form WS+)*;

cons: ('|' annotation WS+ form WS+ form) | ('.' annotation WS+ form);

pair: not_cons_form ':' (not_cons_form | pair);

annotation: '!' | '$' | '?' | '%' | '*' | '\\';

elist: '(' forms ')';

vector: '[' forms ']';

etuple: '{' forms '}';

literal: INT | STRING;

INT: ('0'|[1-9][0-9]*);
STRING : '"' ( ~'"' | '\\' '"' )* '"' ;

WS : [ \n\r\t] ;
```

### 各種キーワード

#### リテラル

正規表現|説明
:-:|-
数値|([1-9][0-9]*)&#x7C;0かな。
シンボル|[a-z&#x7C;A-z][a-z&#x7C;A-z&#x7C;0-9]+ですね。
文字列|`'` ... `'` or `"` ... `"`
de bruijn index|`_[0-9]*` `_`の後に数値が続くと、とみなす


#### 確定(1文字目=tag dispatcher or literal)

記号|説明
:-:|-
`(` ~ `)`|piq(基本形)
&#x7C;|piq(p,qを指定)
`.`|piq(pのみを指定)
`;`|Unit
`N`|nil
`T`|true
`F`|false

#### 要仕様検討(1文字目=tag dispatcher)

記号|説明
:-:|-
`,`|埋め込み

#### 確定(2文字目=tag)

記号|説明|単独
:-:|-|-
`:`|cons|中置記法でcons
`\`|block|ナシ
`%`|environment|ナシ
`!`|apply|ナシ
`$`|symbol|ナシ
`@`|deref|前置記法でderef
`?`|condition|ナシ(中置記法やってもいいけどややこしい)
`#`|access|後置記法でaccess
`=`|equal|中置記法でもいける

#### 要仕様検討(2文字目=tag)

記号|説明|単独
:-:|-|-
`&`|tuple|ナシ
`~`|enum|ナシ
`^`|metadata|ナシ
`/`|module?|ナシ

#### 確定(2文字でのidiom)

記号|説明
:-:|-|-
`..`|comment(単一行)
&#x7C;&#x7C;|comment(複数行)
`.{` ~ `}`|実行部分
`.[` ~ `]`|quote
`!?`|exception

#### 要仕様検討(2文字でのidiom)

記号|説明
:-:|-
`%+`|define
`!#`|yield （不要かも
`!<`|dispatch
`#.`|self ref?

#### マクロかも(複数文字でのidiom)

記号|説明
:-:|-
`!&`|parallel
`#>`|print
`#>"`|format

#### その他未決定事項
- ASTをたどるQuery(XPathみたいになるよね)
- モジュールの具体的な仕様


#### tagとして使いそうな記号

```
Crrt, // ^ carret
Comm, // , comma
```


#### tagとして余っている記号

```
Plus, // + plus
Hphn, // - hyphen-minus
Star, // * asterisk
Slsh, // / slash

Sgqt, // ' single quotation
Bkqt, // ` back quote
Less, // < less than
Grtr, // > greater than
Udsc, // _ underscore
```

### マクロ

特定のキーワードに反応するのではなく、AST全体を見回して、クエリで書き換える部分を指定する。
この方が、まあ書き換えフィルタっぽい。両方あってもいい気もするけど。
それに、書き換えるタイミングとか、重ねがけ、も指定できる。
その代わり、本気でやるならXPath的なやつも必要になるよね。

まあ、まずは、各ASTのパース時に毎回走る関数を設定する、ということで。
それって特殊なイベントハンドラでは？

`|!! symbol [args]`というタグがきたら、`|! @symbol [args]`に変更する

```
|~ .% [ast]
   |>>>> "実行用マクロを定義する"
   |? (= |! @ast#^ !!) .. ^ はそのpiqのタグを表す
      |: .[ |! (@ .{ast#p}) .{ast#q} ]
         ast
```


### epiq一覧

- `Unit`
- `Int8(i64)`
- `Text(String)`

- `Lpiq { p: usize, q: usize }` // (linked) list piq
- // Vpiq { p: usize, q: usize }, // vector piq
- `Fpiq { p: usize, q: usize }` // function piq
- `Aexp { a: usize, e: usize }` // A-Expression
- `Prmt(usize)` // anonymous parameter
- `Pprn(usize)` // priority parentheses
- `Dbri(usize)` // de bruijn index
