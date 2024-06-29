(define (string-take n s) (substring s 0 n))
(define (string-drop n s) (substring s n (string-length s)))

(define (string-car s) (string-take 1 s))

(define (string-cdr s) (string-drop 1 s))

(define (power2repeat y x) (if (= y 0) x (string-append (power2repeat (- y 1) x) (power2repeat (- y 1) x))))

(define (string-map f l) (if (string=? l "") "" (string-append (f (string-car l)) (string-map f (string-cdr l)))))

(define (res) (
    string-append
    "solve lambdaman8 "
    (power2repeat 7 (string-map (lambda (x) (power2repeat 7 x)) "DLUR"))
))

(define (main args) (print (res)))
