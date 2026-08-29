(define (deriv expr val)
  (cond
   ((number? expr) 0)
   ((symbol? expr) (if (eq? expr val) 1 0))
   ((eq? (car expr) '+)
    (let ((a (cadr expr))
          (b (caddr expr)))
      (list '+
            (deriv a val)
            (deriv b val))))
   ((eq? (car expr) '*)
    (let ((a (cadr expr))(b (caddr expr)))
      (list '+
            (list '* a (deriv b val))
            (list '* b (deriv a val)))))
   (else (error "unknown expression type"))))

(deriv 'x 'x)              ;; Should return: 1
(deriv 5 'x)               ;; Should return: 0
(deriv '(+ x 3) 'x)        ;; Should return: (+ 1 0)
(deriv '(* x y) 'x)        ;; Should return: (+ (* 1 y) (* x 0))
