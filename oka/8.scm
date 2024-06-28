(define (repeat2power x y) (if (= y 0) x (string-append (repeat2power x (- y 1)) (repeat2power x (- y 1)))))

(define (map f l) (if (null? l) '() (cons (f (car l)) (map f (cdr l)))))

(define (concat-all l) (if (null? l) "" (string-append (car l) (concat-all (cdr l)))))

(define (concat x y) (string-append x y))

(define (res) (repeat2power () 4))

(define (main args) (print (repeat2power "x" 4)))
