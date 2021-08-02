#lang racket

; https://leetcode.com/problems/teemo-attacking/

(define/contract (find-poisoned-duration timeSeries duration)
  (-> (listof exact-integer?) exact-integer? exact-integer?)
  (letrec ([go (lambda (xs acc)
                 (cond [(null? xs) acc]
                       [(null? (cdr xs)) (+ acc duration)]
                       [else (go (cdr xs)
                                 (+ acc (min (- (cadr xs) (car xs)) duration)))]))])
    (go timeSeries 0))
  )

(require rackunit)

(define tests
  (test-suite
    "Teemo Attacking"

    (check-equal?
      (find-poisoned-duration '(1 4) 2)
      4
      "Example 1")

    (check-equal?
      (find-poisoned-duration '(1 2) 2)
      3
      "Example 2")))

(require rackunit/text-ui)

(run-tests tests)
