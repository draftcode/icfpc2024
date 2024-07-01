(define (string-take n s) (substring s 0 n))
(define (string-drop n s) (substring s n (string-length s)))

(define (rep s d) (if (= d 0) s (string-append (rep s (- d 1)) (rep s (- d 1)))))

(define (get d) (string-take 1 (string-drop d "URDL")))

(define (add d x) (modulo (+ d x) 4))

(define (recur one dep dir) (
    if (= dep 0) ""
    (string-append
        (one dep (add dir 3))
        (one dep (add dir 0))
        (one dep (add dir 1))
        (get dir)
    )
))

(define (one dep dir) (
    if (< dep 1) (string-append (get (add dir 2)) (get (add dir 1)))
    (string-append
        (rep (get dir) (- dep 1))
        (one (- dep 2) (add dir 3))
        (one (- dep 2) (add dir 1))
        (rep (get dir) (- dep 1))

        (recur one (- dep 1) dir)
 
        (rep (get (add dir 2)) dep)
        (get (add dir 2))
    )
))

(define (res) (
    string-append
    "solve lambdaman20 "
    (one 6 0)
    (one 6 1)
    (one 6 2)
    (one 6 3)
))

(define (main args) (print (res)))
