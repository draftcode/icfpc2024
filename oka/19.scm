(define (string-take n s) (substring s 0 n))
(define (string-drop n s) (substring s n (string-length s)))

(define (rep s d) (if (= d 1) s (string-append (rep s (- d 1)) (rep s (- d 1)))))

(define (recur d) (
    if (= d 0) "" 
        (string-append
        (string-append
        (string-append
        (string-append
        (string-append
        (string-append
        (string-append
        (string-append
        (string-append
            (rep "D" d)
            (recur (- d 1)))
            (rep "U" (+ d 1)))
            (recur (- d 1)))
            (rep "D" d))
            (rep "L" d))
            (recur (- d 1)))
            (rep "R" (+ d 1)))
            (recur (- d 1)))
            (rep "L" d))
))

(define (res) (
    string-append
    "solve lambdaman19 "
    (recur 7)
))

(define (main args) (print (res)))
