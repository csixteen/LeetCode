#lang racket

; https://leetcode.com/problems/assign-cookies/

(define/contract (find-content-children g s)
  (-> (listof exact-integer?) (listof exact-integer?) exact-integer?)
  (letrec ([go (lambda (xs ys acc)
                 (cond [(or (null? xs) (null? ys)) acc]
                       [(<= (car xs) (car ys))
                        (go (cdr xs) (cdr ys) (+ acc 1))]
                       [else (go xs (cdr ys) acc)]))])
    (go (sort g <) (sort s <) 0))
  )

(require rackunit)

(define tests
  (test-suite
    "Assign Cookies"

    (check-equal?
      (find-content-children '(1 2 3) '(1 1))
      1
      "Example 1")

    (check-equal?
      (find-content-children '(1 2) '(1 2 3))
      2
      "Example 2")))

(require rackunit/text-ui)

(run-tests tests)
