Scheme program を ICFP にコンパイルする

# 使い方

oka/8.scm みたいな Scheme プログラムを書いて、

`cargo run -r --bin scmcomp submit < oka/8.scm`

を実行すると、プログラムをコンパイルして送ります。

# 制約条件

(サンプル oka/8.scm を参照)

top level の expression はすべて `(define ...)` 形式である必要がある。
また、`(define (res) ...)` が存在する必要があり、res の値に評価されるような ICFP を出力する。

`(define (res) ...)` 以降の行は無視される。

`string-take` と `string-drop` の実装は書いても書かなくてもよい（無視される）。

`(define (res) ...)` の代わりに、`(define (solve-lambdaman4) ...)` などと、`solve-` で始まる
変数を定義してもよい。この場合、`-` を空白に変え、空白を足した、`"solve lambdaman4 "` を追加するコードが自動的に
追加される。res, solve-... が両方ある場合、後にあるものが優先され、それより前にあるものはコードに組み入れられてしまうので注意。

## 組み込み関数

以下の関数は定義なしで使える。

`string-append` は特別に 2 以上の任意個の引数をとれる。

```
# Unary
(- A)               -> U- A'
(not A)             -> U! A'
(string-to-int A)   -> U# A'
(int-to-string A)   -> U$ A'

# Binary
(+ A B)             -> B+ A' B'
(- A B)             -> B- A' B'
(* A B)             -> B* A' B'
(/ A B)             -> B/ A' B'
(% A B)             -> B% A' B'
(< A B)             -> B< A' B'
(> A B)             -> B> A' B'
(= A B)             -> B= A' B'
(| A B)             -> B| A' B'
(& A B)             -> B& A' B'
(string-append A B) -> B. A' B'
(string-take A B)   -> BT A' B'
(string-drop A B)   -> BD A' B'
(A B)               -> B$ A' B'

(string=? A B)      -> B= A' B'
(modulo A B)        -> B% A' B'
(div A B)           -> B/ A' B'

# If
(if A B C)            -> ? A' B' C'

# Lambda
(lambda (x) A)      -> L# A'   // A' 内の自由変数 x の出現を v# で置き換える
```

# 実装メモ

ICFP のスペック https://icfpcontest2024.github.io/icfp.html
Scheme のスペック https://groups.csail.mit.edu/mac/ftpdir/scheme-7.4/doc-html/scheme_7.html

## 2 引数以上の define

(define (f x1 x2) A)

は

(define (f x1) ((lambda (x2) A)))

とおなじ

## 1 引数の define

(define (f x) A) B

は

((lambda (f) B) (lambda (x) A))

とおなじ

## 0 引数の define

(define (f) A) B

は

((lambda (f) B') A)

と同じ。B' では B に出現する (f) を f に置き換える。

## 再帰関数の変換

(define (f x) A) B

は
Z コンビネータを pre-defined 関数として、

(define (F f) (lambda (x) A))
(define (f x) ((Z F) x))
B

(define (f x) ((Z (lambda (f) ((lambda (x) A)))) x))
B

に書き換えられる

## Let の書き換え (未実装)

(let () A)

は

A

と同じ。

(let (... (x y)) A)

は

(let (...) ((lambda (x) A) y))

と同じ。
