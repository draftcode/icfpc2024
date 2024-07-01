(define (string-take n s) (substring s 0 n))
(define (string-drop n s) (substring s n (string-length s)))

(define (rep i d) (if (= d 0) "" (string-append (string-take 1 (string-drop i "URDLURD")) (rep i (- d 1)))))

(define (recur d p) (
    if (< (* d p) 1) ""
    (string-append
        (rep p d)
        (recur (div d 2) 4)
        (rep (+ p 2) d)
        (recur d (- p 1))
    )
))

(define (res) (
    string-append
    "solve lambdaman19 "
    (recur 64 4)
))

(define (main args) (print (res)))
