(define (string-take n s) (substring s 0 n))
(define (string-drop n s) (substring s n (string-length s)))

(define (rep i d) (if (= d 1) (string-take 1 (string-drop i "URDLUR")) (string-append (rep i (- d 1)) (rep i (- d 1)))))

(define (recur d p) (
    if (or (= d 0) (< p 0)) ""
    (string-append
        (rep p d)
        (recur (- d 1) 3)
        (rep (+ p 2) d)
        (recur d (- p 1))
    )
))

(define (res) (
    string-append
    "solve lambdaman19 "
    (recur 7 3)
))

(define (main args) (print (res)))
