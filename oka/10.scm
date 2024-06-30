(define (string-take n s) (substring s 0 n))
(define (string-drop n s) (substring s n (string-length s)))

(define (repeat f v) (f (f (f (f v)))))

(define (drru) "DRRU")

(define (r10) "RRRRRRRRRR")
(define (l10) "LLLLLLLLLL")

(define (ulld) "ULLD")

(define (x4 x) (string-append x x x x))

(define (x16 x) (x4 (x4 x)))

(define (path1) (string-append (x16 (string-append 
    (r10) (drru) 
    (r10) (drru) 
    (r10) (drru) 
    (r10) (drru) 
    (r10)
    "D"
    (l10) (ulld)
    (l10) (ulld)
    (l10) (ulld)
    (l10) (ulld)
    (l10) (ulld)
    (l10)
)) "RDD"))

(define (res) (
    string-append
    "solve lambdaman10 "
    (x16 (path1))
))

(define (main args) (print (res)))
