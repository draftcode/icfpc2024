(define (string-take n s) (substring s 0 n))
(define (string-drop n s) (substring s n (string-length s)))

; comment

(define (double x) (string-append x x))
(define (x64 x) (double (double (double (double (double (double (double x))))))))

(define (res) (
    string-append
    "solve lambdaman8 "
    (x64 (string-append (string-append (string-append (x64 "D") (x64 "L")) (x64 "U")) (x64 "R")))
))

(define (main args) (print (res)))
