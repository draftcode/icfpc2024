(define (string-take n s) (substring s 0 n))
(define (string-drop n s) (substring s n (string-length s)))

(define (repeat f v) (f (f (f (f v)))))

(define (urrrd) "URRRD")

(define (r10) "RRRRRRRRRR")
(define (l10) "LLLLLLLLLL")

(define (ullld) "ULLLD")

(define (x4 x) (string-append x x x x)

(define (path) )

(define (res) (
    string-append
    "solve lambdaman10 "
    (x64 (string-append (string-append (string-append (x64 "D") (x64 "L")) (x64 "U")) (x64 "R")))
))

(define (main args) (print (res)))
