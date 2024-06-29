(define (string-head s n) (substring s 0 n))
(define (string-tail s n) (substring s n (string-length s)))
(print ((lambda (Z) ((lambda (fact) (fact 5)) (lambda (n) ((Z (lambda (fact) (lambda (n) (if (= n 0) 1 (* n (fact (- n 1))))))) n)))) (lambda (f) ((lambda (x) (f (lambda (y) ((x x) y)))) (lambda (x) (f (lambda (y) ((x x) y))))))))
