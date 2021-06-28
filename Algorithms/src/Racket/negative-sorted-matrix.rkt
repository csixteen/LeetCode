#lang racket

(define/contract (count-negatives grid)
  (-> (listof (listof exact-integer?)) exact-integer?)
  (foldl +
         0
         (map (lambda (row) (count negative? row))
              grid)))

(require rackunit)

(define tests
  (test-suite
    "Count Negative Numbers in a Sorted Matrix"

    ; Example 1
    (check-equal?
      (count-negatives '())
      0
      "Example 0")

    (check-equal?
      (count-negatives '((4 3 2 -1)
                         (3 2 1 -1)
                         (1 1 -2 -1)
                         (-1 -1 -2 -3)))
      8
      "Example 1")

    (check-equal?
      (count-negatives '((3 2)
                         (1 0)))
      0
      "Example 2")

    (check-equal?
      (count-negatives '((-1)))
      1
      "Example 3")))

(require rackunit/text-ui)

(run-tests tests)
