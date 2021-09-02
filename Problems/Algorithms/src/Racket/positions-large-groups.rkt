#lang racket

; https://leetcode.com/problems/positions-of-large-groups/

(define/contract (large-group-positions s)
  (-> string? (listof (listof exact-integer?)))
  (letrec ([go (lambda (xs i acc)
                 (if (null? xs)
                   (reverse acc)
                   (let* ([head (car xs)]
                          [lst (takef xs (lambda (c) (char=? c head)))]
                          [len (length lst)])
                     (go (drop xs len)
                         (+ i len)
                         (if (>= len 3)
                           (cons (list i (+ i (- len 1))) acc)
                           acc)))))])
    (go (string->list s) 0 '()))
  )

(require rackunit)

(define tests
  (test-suite
    "Positions of Large Groups"

    (check-equal?
      (large-group-positions "abbxxxxzzy")
      '((3 6))
      "Example 1")

    (check-equal?
      (large-group-positions "abc")
      '()
      "Example 2")

    (check-equal?
      (large-group-positions "abcdddeeeeaabbbcd")
      '((3 5) (6 9) (12 14))
      "Example 3")))

(require rackunit/text-ui)

(run-tests tests)
