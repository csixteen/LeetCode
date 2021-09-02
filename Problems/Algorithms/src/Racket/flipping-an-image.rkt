#lang racket

; https://leetcode.com/problems/flipping-an-image/

(define/contract (flip-and-invert-image image)
  (-> (listof (listof exact-integer?)) (listof (listof exact-integer?)))
  (map (lambda (row)
         (map (lambda (d) (bitwise-and (bitwise-not d) 1)) (reverse row)))
       image)
  )

(require rackunit)

(define tests
  (test-suite
    "Flipping an Image"

    (check-equal?
      (rev-transform '(1 1 0))
      '(1 0 0)
      "Reverse and transform 1")

    (check-equal?
      (rev-transform '(0 0 0 0))
      '(1 1 1 1)
      "Reverse and transform 2")

    (check-equal?
      (flip-and-invert-image '((1 1 0)
                               (1 0 1)
                               (0 0 0)))
      '((1 0 0)
        (0 1 0)
        (1 1 1))
      "Example 1")

    (check-equal?
      (flip-and-invert-image '((1 1 0 0)
                               (1 0 0 1)
                               (0 1 1 1)
                               (1 0 1 0)))
      '((1 1 0 0)
        (0 1 1 0)
        (0 0 0 1)
        (1 0 1 0))
      "Example 2")))

(require rackunit/text-ui)

(run-tests tests)
