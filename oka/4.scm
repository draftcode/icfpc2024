(define (string-take n s) (substring s 0 n))
(define (string-drop n s) (substring s n (string-length s)))

(define (solve rng iter) (if (= iter 0) "" (
  string-append
  (solve 
    (modulo (* rng 48271) 2147483647) (- iter 1))
  (string-take 1 (string-drop (div rng 536870912) "DLUR"))
)))

(define (solve-lambdaman4) (
    solve 1 40000
))

(define (main args) (print (solve-lambdaman4)))
