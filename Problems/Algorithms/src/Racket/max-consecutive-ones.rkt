#lang racket

; https://leetcode.com/problems/max-consecutive-ones/

(define/contract (find-max-consecutive-ones nums)
  (-> (listof exact-integer?) exact-integer?)
  (letrec ([go (lambda (xs acc total)
                 (cond [(null? xs) (max acc total)]
                       [(zero? (car xs)) (go (cdr xs) 0 (max acc total))]
                       [else (go (cdr xs) (+ acc 1) total)]))])
    (go nums 0 0))
  )

(require rackunit)

(define tests
  (test-suite
    "Max Consecutive Ones"

    (check-equal?
      (find-max-consecutive-ones '(1 1 0 1 1 1))
      3
      "Example 1")

    (check-equal?
      (find-max-consecutive-ones '(1 0 1 1 0 1))
      2
      "Example 2")))

(require rackunit/text-ui)

(run-tests tests)
