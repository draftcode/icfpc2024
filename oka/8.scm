(define (string-car s) (substring s 0 1))

(define (string-cdr s) (substring s 1 (string-length s)))

(define (power2repeat y x) (if (= y 0) x (string-append (power2repeat (- y 1) x) (power2repeat (- y 1) x))))

(define (string-map f l) (if (string=? l "") "" (string-append (f (string-car l)) (string-map f (string-cdr l)))))

(define (res) (
    power2repeat 7 (string-map (lambda (x) (power2repeat 7 x)) "DLUR")
))

(define (main args) (print (res)))
