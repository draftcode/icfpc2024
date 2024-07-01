(define (string-take n s) (substring s 0 n))
(define (string-drop n s) (substring s n (string-length s)))

(define (rep i d) (if (= d 1) (string-take 1 (string-drop i "URDLURD")) (string-append (rep i (- d 1)) (rep i (- d 1)))))

(define (recur d p) (
    if (< (* d p) 1) ""
    (string-append
        (rep p d)
        (recur (- d 1) 4)
        (rep (+ p 2) d)
        (recur d (- p 1))
    )
))

(define (res) (
    string-append
    "solve lambdaman19 "
    (recur 7 4)
))

(define (main args) (print (res)))
