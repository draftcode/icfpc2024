(define (string-take n s) (substring s 0 n))
(define (string-drop n s) (substring s n (string-length s)))

(define (double x) (string-append x x))

(define (solve rng iter) (if (= iter 0) "" (
  string-append
  (solve 
    (modulo (* rng 48271) 2147483647) (- iter 1))
  (double (string-take 1 (string-drop (div rng 536870912) "DLUR")))
)))

(define (res) 
  (string-append "solve lambdaman11 " (
    solve 1 50000
  ))
)

(define (main args) (print (res)))
