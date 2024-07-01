(define (string-take n s) (substring s 0 n))
(define (string-drop n s) (substring s n (string-length s)))

(define (x3 x) (string-append x x x))
; (define (x64 x) (x4 (x4 (x4 x))))

(define (res) (
    string-append
    "solve lambdaman6 "
    (x3 (x3 (x3 "RRRRRRRR")))
))

(define (main args) (print (res)))
