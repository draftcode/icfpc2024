(define (string-take n s) (substring s 0 n))
(define (string-drop n s) (substring s n (string-length s)))

(define (x4 x) (string-append (string-append x x) (string-append x x)))
(define (res2) (x4 (x4 (x4 "RRRR"))))

(define (res) (
    string-append
    "solve lambdaman6 "
    (res2)
))

(define (main args) (print (res)))
