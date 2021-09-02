#lang racket

(define/contract (largest-altitude gain)
  (-> (listof exact-integer?) exact-integer?)
  (letrec ([go (lambda (xs prev max-so-far)
                 (if (null? xs)
                   (max max-so-far prev)
                   (go (cdr xs) (+ (car xs) prev) (max prev max-so-far))))])
    (go gain 0 0))
  )

(require rackunit)

(define tests
  (test-suite
    "Find highest altitude"

    (check-equal?
      (largest-altitude '(-5 1 5 0 -7))
      1
      "Example 1")

    (check-equal?
      (largest-altitude '(-4 -3 -2 -1 4 3 2))
      0
      "Example 2")))

(require rackunit/text-ui)

(run-tests tests)
