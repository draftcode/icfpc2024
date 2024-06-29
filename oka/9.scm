(define (string-take n s) (substring s 0 n))
(define (string-drop n s) (substring s n (string-length s)))

(define (double x) (string-append x x))
(define (x64 x) (double (double (double (double (double (double x)))))))

(define (res) (
    string-append
    "solve lambdaman9 "
    (x64 (string-append (string-append (x64 "R") "D") (x64 "L")))
))

(define (main args) (print (res)))
