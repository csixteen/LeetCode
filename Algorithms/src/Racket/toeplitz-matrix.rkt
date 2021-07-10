#lang racket

(define/contract (is-toeplitz-matrix matrix)
  (-> (listof (listof exact-integer?)) boolean?)
  (letrec ([go (lambda (cs)
                 (if (null? cs)
                   #t
                   (let ([r (caar cs)]
                         [c (cadar cs)])
                     (cond [(or (zero? r) (zero? c))
                            (go (cdr cs))]
                           [(not (= (list-ref (list-ref matrix r) c)
                                    (list-ref (list-ref matrix (- r 1)) (- c 1))))
                            #f]
                           [else (go (cdr cs))]))))])
    (go (cartesian-product (range (length matrix))
                           (range (length (car matrix))))))
  )

(require rackunit)

(define tests
  (test-suite
    "Toeplitz Matrix"

    (check-true
      (is-toeplitz-matrix '((1 2 3 4)
                            (5 1 2 3)
                            (9 5 1 2)))
      "Example 1")

    (check-false
      (is-toeplitz-matrix '((1 2) (2 2)))
      "Example 2")

    (check-true
      (is-toeplitz-matrix '((1)
                            (5)
                            (9)))
      "Example 3")))

(require rackunit/text-ui)

(run-tests tests)
