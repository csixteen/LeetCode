#lang racket

; https://leetcode.com/problems/lemonade-change/

(define/contract (lemonade-change bills)
  (-> (listof exact-integer?) boolean?)
  (letrec ([go (lambda (bs five ten)
                 (if (null? bs)
                   #t
                   (let ([head (car bs)])
                     (cond [(= head 5) (go (cdr bs) (+ five 1) ten)]
                           [(= head 10) (if (> five 0)
                                          (go (cdr bs) (- five 1) (+ ten 1))
                                          #f)]
                           [(and (> ten 0) (> five 0))
                            (go (cdr bs) (- five 1) (- ten 1))]
                           [(>= five 3) (go (cdr bs) (- five 3) ten)]
                           [else #f]))))])
    (go bills 0 0))
  )

(require rackunit)

(define tests
  (test-suite
    "Lemonade Change"

    (check-true
      (lemonade-change '(5 5 5 10 20))
      "Example 1")

    (check-true
      (lemonade-change '(5 5 10))
      "Example 2")

    (check-false
      (lemonade-change '(5 5 10 10 20))
      "Example 3")))

(require rackunit/text-ui)

(run-tests tests)
