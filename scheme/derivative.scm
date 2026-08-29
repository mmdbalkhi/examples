(define (derivative f dx)
  (lambda (x)
    (/(- (f (+ x dx))
         (f x))
      dx)))

(define (square x) (* x x))
(define dsquare (derivative square 0.001))
(dsquare 5)
