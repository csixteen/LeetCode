#lang racket

(define/contract (transpose matrix)
  (-> (listof (listof exact-integer?)) (listof (listof exact-integer?)))
  (letrec ([f (lambda (col max-cols)
                (if (>= col max-cols)
                  '()
                  (cons (map (lambda (row) (list-ref row col)) matrix)
                        (f (+ col 1) max-cols))))])
    (f 0 (length (car matrix)))))

(require rackunit)

(define tests
  (test-suite
    "Transpose Matrix"

    (check-equal?
      (transpose '((1 2 3) (4 5 6) (7 8 9)))
      '((1 4 7) (2 5 8) (3 6 9))
      "Example 1")

    (check-equal?
      (transpose '((1 2 3) (4 5 6)))
      '((1 4) (2 5) (3 6))
      "Example 2")))

(require rackunit/text-ui)

(run-tests tests)
