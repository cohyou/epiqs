# epiqs

## about

- affixはprefix+suffixのこと

## syntax

- `^$` anonymous parameter cons系より結合力が低いので貪欲にparseするように
- `[` ... `]` リスト
- `;` Unit
- `(` ... `)` 優先順位のカッコ
- `"` ... `"` 文字列
- `([1-9][0-9]*)|0` なぜかここだけ正規表現、数値です
- `_[0-9]*` `_`の後に数値が続くと、de bruijn indexとみなす
- `|^` 関数


### category of AST

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


### parse tree

- `Aexp`(A-Expression)
-   `Affx`


### example

`(^$;)|^_0`

`(` `^$` `;` `)` `|^` `_0`

確か、これは引数に何も設定されていないidentity関数を表したものだったはず。
`^$`が匿名パラメータだと書いてあるのは少し不思議ではある。

記述に少し混乱している様子が見られるので、改めて整理した方が良さそう。

- `;` Unit
- `N` `F` `T` それぞれnil false true
- `#.` self ref?


- `:` 基本のcons、listも作る
- `|\` functionを作るcons pが引数、qが本体
- `|$` symbolを作るcons pが名前、qが中身
- `|!` applyを作るcons
- `|*` tupleを作るcons
- `|?` conditionを作るcons pが条件、qが真偽のcons。本質的に三項演算子なので違和感はある


以下、Athletiq-Haskellからもらってきました

> - `@@()` "注釈の注釈"
> - `@()` "注釈" やはりアノテーションのイメージ

> - `^()` "メタデータ" Clojureより
> - `%()` "環境" 正直余っていたので、ただ消去法で
> - `$()` "文脈" シェルなどの変数のイメージから
> - `?()` "条件" これもいれますか。。。
> - `!()` "評価" 何か動作ぽい感じがよいので
> - `\()` "実装" ラムダ式を作るという視点から、またメタ文字として
> - `#()` "返却" コメントな感じもするけど

> - `[]` シーケンス
> - `{}` タプルやらレコードやら？
> `@aaa(\{:a "1"} {:b "2"})`
> - `|` はしっこまでカッコをつけたことにする
> - `;` []と同じ


> すべてのフォームはアノテーションとシーケンスから成り立つ
> シーケンスの「アノテーション」という扱いであり、
> アノテーションもまた中身はシーケンスである

> 実行を開始した時点で、まず`@@`が走る

> `@@[]` だと、エントリポイント

> パースして、ASTを作ります
> `@@`を走らせて、処理をします
> その中で`@`や`!`や`#`が呼ばれて結果的に処理が走ります

もちろんこれは、「外からの」構文案でした。epiqsは「consから始めよう」としています。
むしろ、これらAthletiqの「裏で」動くようなイメージかもしれません（確証はありません）。

基本のconsの種類が6種類だとして、実際にはそれだけでコードを書くことは困難だと思われます。
例えば、

`['x'|$;]|\_0`

と書くと、

`x => x` (JavaScript)

`\x -> x` (Haskell)

になりますが、全然記述が簡潔でない。。。
と言うか、中置記法がやりたくてこれらの構文を考えたわけですが、とてもではないが手書きする時は前置記法の方が楽である。
Elixirのようなマクロがあればいいのかなあ。
しかし、どう考えても`['x'|$;]|\_0`を`\x -> x`から作るのは難しそう。

更に踏み込んで、`'x'|$;`が単に`x`として表現できる、というのはどのように解釈すればいいんだろう。パーサとしては書けるけども（いきなり記号と数値以外のもので始まるのはシンボルだとすればよい）。しかし、普通マクロというのは、その前に必ず「マクロ名」が必要なわけです。いきなりは無理。`symbol x`とか書けるならいいけど、今回はそれはできない。つまり、渡ってきたトークン（そう、トークンレベルの話なのだ）がそういう風に始まっている、ということでしか定義できない。

特別な構文を作るとしたら、リードマクロである。仮に`@`で始まるものがそれで表せるとしよう。つまり、普通のリード処理が終わった後に、それらのマクロが走る。

`[` `'token'` `|$` `;` `]`

`|@`

`(`

　`(beginning_alpha` `|!` `_0)`

　`|?`

　`(`

　　`(_0` `|$` `;)`

　　`:`

　　`;`

　`)`

`)`

こんな感じになる。すでに地獄感ある。

`['token'|$;]|@((('beginning_alpha'|$(~))|!_0)|?((_0|$;):;))`

繋げて書くとこうだ。なんだかなあ。

なんだか、epiqsのコンセプトそのものがきついなあ。書きたいのにね、consを。
愛しているのにね、consを。というか、中値記法で、かつ優先順位がないので全部かっこをつけている。このせいで結局よいところがない（普通のリストのように、たくさん繋げる分には素晴らしかったけれど、二つだと決まっているものを繋げても悪夢だ）

`[ 'token' |$ ; ] |@ 'beginning_alpha' |$ ~ |! _0 |? _0 |$ ; : ;`

かといって、かっこを外せばいいというものでもない。特に最後の`; : ;`がつらいな。
```
[ 'token' |$ ; ]
|@
  'beginning_alpha' |$ ~ |! _0
  |?
  _0 |$ ; : ;
```

改行を入れてみたが、やはりオペレータが真ん中にいると、ネストもさせにくい。
```
|@
  [ |$
      'token'
      ; ]
  |?
    |!
      |$
        'beginning_alpha'
        ;
      _0
    :
      |$
        _0
        ;
      ;
```

まあ、前置記法にしてもめちゃめちゃわかりにくいけどね。でもまあ、ネストでましになった。
必ず引数が二つ、というのはよい感じだけど、それだけ。
それにしても、ASTの構造を見るという観点では、こちらの方がよい気がしてきた。

`(@ [($ 'token' ;)] (? (! ($ 'beginning_alpha' ;) _0) (: ($ _0 ;) ;)))`


`|@ [|$ 'token' ;] |? |! |$ 'beginning_alpha' ; _0 |: |$ _0 ; ;`

読みにくいが、結局こういうことですよね。`|`が思いがけず、前かっこの役割を果たしている。
そして、引数が二つと決まっているので、閉じかっこはなくなる。

```
|@ |: |$ 'token' ;
      ;
   |? |! |$ 'beginning_alpha' ; _0
      |: |$ _0 ;
         ;
```

そして、引数が2つでない場合は、かっこを使うようにしてはどうか。

```
|@ ': '$ "token"
   |? |! '$ "beginning_alpha" _0
      ': '$ _0
```

おお、かなりマシになった気がする。悪くない。

引数が1つの場合は、別の記法でもよいかも（）。

実際には、あれやろけど、シンボルはそのまま書きたいよね。

```
|@ '% 0
   |? 'beginning_alpha _0
      ': '$ _0
```

少し表記を変更してみる。

```
:@ .^ .$ 'token'
   :? :! .$ 'beginning_alpha' _0
      .^ .$ _0
```

```
:@ .% 0
   :? :! .beginning_alpha _0
      .^ .$ _0
```

```
|@ .% 0
   |? |! beginning_alpha _0
      .^ .$ 0
```

パイプの方が美しい感じはするなあ。。。


### 改めてまとめてみる
- `;` Unit
- `N` `F` `T` それぞれnil false true
- `#.` self ref?

- `|` 引数が2つ、というかconsだよという印
- `.` 引数が1つという印

- `:` 基本のcons、listも作る　中置記法も可能

- `\` functionを作るcons pが引数、qが本体
- `$` symbolを作るcons pが名前、qが中身
- `!` applyを作るcons pが関数 qが引数
- `?` conditionを作るcons pが条件、qが真偽のcons。本質的に三項演算子なので違和感はある
- `*` tupleを作るcons

```
|^ a |^ b |^ c .^ d
a : b : c : d:;
[a b c d]

|\ |$ 0 ; ;
.\ _0
```
