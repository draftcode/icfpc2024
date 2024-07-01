(define (string-take n s) (substring s 0 n))
(define (string-drop n s) (substring s n (string-length s)))

(define (rep s d) (if (= d 1) s (string-append (rep s (- d 1)) (rep s (- d 1)))))

(define (get p) (string-take 1 (string-drop p "URDLURDL")))

(define (recur d p) (
    if (or (= d 0) (< p 0)) ""
    (string-append
        (rep (get p) d)
        (recur (- d 1) 3)
        (rep (get (+ p 2)) d)
        (recur d (- p 1))
    )
))

(define (res) (
    string-append
    "solve lambdaman19 "
    (recur 7 3)
))

(define (main args) (print (res)))
