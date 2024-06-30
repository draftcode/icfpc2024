(define (string-take n s) (substring s 0 n))
(define (string-drop n s) (substring s n (string-length s)))

; https://en.wikipedia.org/wiki/Lehmer_random_number_generator
(define (next-rng rng)
    (modulo (* rng 48271) 2147483647)
)

; Use most significant 2 bits
(define (get rng) 
    (string-take 1 (string-drop (div rng 536870912) "DLUR"))
)

(define (solve rng iter) (if (= iter 0) "" (
  (lambda (rng) 
    (string-append (solve rng (- iter 1)) (get rng))
  ) (next-rng rng)
)
))

(define (solve-lambdaman4) (
    solve 1 100
))

(define (main args) (print (res)))
