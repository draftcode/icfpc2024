(define (string-take n s) (substring s 0 n))
(define (string-drop n s) (substring s n (string-length s)))

; comment

(define (d1 x) (string-append x x x))
(define (d2 x) (d1 (d1 (d1 (d1 x)))))

(define (res) (
    string-append
    "solve lambdaman8 "
    (d2 (string-append (d2 "DLDLD") (d2 "URURU")))
))

(define (main args) (print (res)))
