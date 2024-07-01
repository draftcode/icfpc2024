(define (string-take n s) (substring s 0 n))
(define (string-drop n s) (substring s n (string-length s)))

(define (rep s d) (if (= d 1) s (string-append (rep s (- d 1)) (rep s (- d 1)))))

(define (get d) (string-take 1 (string-drop d "URDL")))

(define (recur2))

(define (recur dep from_dir) (
    if (= d 0) ""
        (string-append
            (rep "U" d)
            "R"
            (recur (- d 1))
            "L"
            (rep "D" d)
        )
))

(define (res) (
    string-append
    "solve lambdaman20 "
    (recur 7 0)
))

(define (main args) (print (res)))
