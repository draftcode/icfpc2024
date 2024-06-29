Scheme program を ICFP にコンパイルする。(WIP)

# 制約条件

(サンプル oka/8.scm を参照)

top level の expression はすべて (define ...) 形式である必要がある。
また、`(define (res) ...)` が存在する必要があり、res の値に評価されるような ICFP を出力する。
それ以外で 0 引数の define があってはならない。

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

## 組み込み関数

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
(string-head A B)   -> BT A' B'
(string-tail A B)   -> BD A' B'
(A B)               -> B$ A' B'

(string=? A B)      -> B= A' B'

# If
(if A B)            -> ? A' B'

# Lambda
(lambda (x) A)      -> L# A'   // A' 内の自由変数 x の出現を v# で置き換える
```

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
