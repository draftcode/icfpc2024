(define (string-head s n) (substring s 0 n))
(define (string-tail s n) (substring s n (string-length s)))

(define (string-car s) (string-head s 1))

(define (string-cdr s) (string-tail s 1))

(define (power2repeat y x) (if (= y 0) x (string-append (power2repeat (- y 1) x) (power2repeat (- y 1) x))))

(define (string-map f l) (if (string=? l "") "" (string-append (f (string-car l)) (string-map f (string-cdr l)))))

(define (res) (
    power2repeat 7 (string-map (lambda (x) (power2repeat 7 x)) "DLUR")
))

(define (main args) (print (res)))
