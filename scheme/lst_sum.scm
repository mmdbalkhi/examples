;; (define (list_sum lst)
;;   (if (null? lst)
;;       0 (+ (car lst)
;;            (list_sum (cdr lst)))))

;; (list_sum '(1 2 3 4 5 6)) ;; 21

(define (scale_list scale lst)
  (if (null? lst)
      '()
      (cons (* (car lst) scale)
            (scale_list scale (cdr lst)))))

(scale_list '(1 2 3 4))
