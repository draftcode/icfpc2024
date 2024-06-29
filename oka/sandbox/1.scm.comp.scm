(define (string-head s n) (substring s 0 n))
(define (string-tail s n) (substring s n (string-length s)))
(print ((lambda (plus) ((((plus 2) 3) 4) 5)) (lambda (x) (lambda (y) (lambda (z) (lambda (w) (+ x (+ y (+ z w)))))))))
