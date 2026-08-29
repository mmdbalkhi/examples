(define (lst_reverser lst)
  (if (null? lst)
      '()
      (let ((first_element (car lst))
            (rest_rev (lst_reverser(cdr lst))))
        (if (pair? first_element)
            (append rest_rev
                    (list (lst_reverser first_element)))
            (append rest_rev
                    (list first_element))))))

(lst_reverser '(1 2 (7 8 9) 4 (5 4 (9(2 4 33 21) 10))))
;; (((10 (21 33 4 2) 9) 4 5) 4 (9 8 7) 2 1)
