(define (repeat f v) (f (f (f (f (f (f (f (f (f (f (f v))))))))))))

(define (left x) (repeat (lambda (v) (string-append v v)) "L"))
(define (right x) (repeat (lambda (v) (string-append v v)) "R"))
(define (up x) (repeat (lambda (v) (string-append v v)) "U"))
(define (down x) (repeat (lambda (v) (string-append v v)) "D"))

(define (turn x) (string-append (down x) (string-append (left x) (string-append (up x) (right x)))))

(define (res) (string-append "solve lambdaman8 " (turn "a")))

(define (main args) (print (res)))
