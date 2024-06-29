(define (Z f) (
    (lambda (x) (f (lambda (y) ((x x) y))))
    (lambda (x) (f (lambda (y) ((x x) y))))
))

(define (Y f) ((lambda (x) (f (x x))) (lambda (x) (f (x x)))))

(define (fact n) (if (= n 0) 1 (* n (fact (- n 1)))))

(print (fact 5))

(define (Fact fact) (lambda (n) (if (= n 0) 1 (* n (fact (- n 1))))))

(define (fact n) ((Z Fact) n ))

(print (fact 5))

(define (fact n) ((Z
  (lambda (f) 
    (lambda (n) (if (= n 0) 1 (* n (fact (- n 1)))))
  )
) n))

(define (res) (fact 5))

(define (main args) (print (res)))
