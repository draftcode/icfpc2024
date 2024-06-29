(define (string-take n s) (substring s 0 n))
(define (string-drop n s) (substring s n (string-length s)))

(define (x4 x) (string-append (string-append x x) (string-append x x)))
; (define (x64 x) (x4 (x4 (x4 x))))

(define (res) (
    string-append
    "solve lambdaman6 "
    (x4 (x4 (x4 "RRRR")))
))

(define (main args) (print (res)))
