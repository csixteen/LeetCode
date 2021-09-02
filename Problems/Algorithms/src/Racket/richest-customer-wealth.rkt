#lang racket

; https://leetcode.com/problems/richest-customer-wealth/

(define/contract (maximum-wealth accounts)
  (-> (listof (listof exact-integer?)) exact-integer?)
  (foldl max 0 (map (lambda (xs) (foldl + 0 xs)) accounts)))

(require rackunit)

(define tests
  (test-suite
    "Richest Customer Wealth"

    (check-equal?
      (maximum-wealth '((1 2 3) (3 2 1)))
      6
      "Example 1")

    (check-equal?
      (maximum-wealth '((1 5) (7 3) (3 5)))
      10
      "Example 2")

    (check-equal?
      (maximum-wealth '((2 8 7) (7 1 3) (1 9 5)))
      17
      "Example 3")))

(require rackunit/text-ui)

(run-tests tests)
