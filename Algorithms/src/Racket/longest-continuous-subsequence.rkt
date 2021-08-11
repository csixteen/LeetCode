#lang racket

; https://leetcode.com/problems/longest-continuous-increasing-subsequence/

(define/contract (find-length-of-lcis nums)
  (-> (listof exact-integer?) exact-integer?)
  (letrec ([go (lambda (xs prev acc max-so-far)
                 (cond [(null? xs) (max acc max-so-far)]
                       [(<= (car xs) prev) (go (cdr xs) (car xs) 1 (max acc max-so-far))]
                       [else (go (cdr xs) (car xs) (+ acc 1) max-so-far)]))])
    (go (cdr nums) (car nums) 1 1))
  )

(require rackunit)

(define tests
  (test-suite
    "Longest Continuous Increasing Subsequence"

    (check-equal?
      (find-length-of-lcis '(1 3 5 4 7))
      3
      "Example 1")

    (check-equal?
      (find-length-of-lcis '(2 2 2 2 2 2))
      1
      "Example 2")))

(require rackunit/text-ui)

(run-tests tests)
