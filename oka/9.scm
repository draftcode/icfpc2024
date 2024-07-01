(define (string-take n s) (substring s 0 n))
(define (string-drop n s) (substring s n (string-length s)))

(define (d1 x) (string-append x x x))
(define (d2 x) (d1 (d1 (d1 x))))

(define (res) (
    string-append
    "solve lambdaman9 "
    (d2 (d2 (string-append (d2 "RR") "D" (d2 "LL"))))
))

(define (main args) (print (res)))
