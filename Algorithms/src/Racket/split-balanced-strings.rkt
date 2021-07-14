#lang racket

; https://leetcode.com/problems/split-a-string-in-balanced-strings/

(define/contract (balanced-string-split s)
  (-> string? exact-integer?)
  (letrec ([go (lambda (cs counter acc)
                 (if (null? cs)
                   acc
                   (let* ([new-counter (if (char=? (car cs) #\L) (+ counter 1) (- counter 1))]
                          [increment (if (zero? new-counter) 1 0)])
                       (go (cdr cs) new-counter (+ acc increment)))))])
    (go (string->list s) 0 0))
  )

(require rackunit)

(define tests
  (test-suite
    "Split a String in Balanced Strings"

    (check-equal?
      (balanced-string-split "RLRRLLRLRL")
      4
      "Example 1")

    (check-equal?
      (balanced-string-split "RLLLLRRRLR")
      3
      "Example 2")

    (check-equal?
      (balanced-string-split "LLLLRRRR")
      1
      "Example 3")

    (check-equal?
      (balanced-string-split "RLRRRLLRLL")
      2
      "Example 4")))

(require rackunit/text-ui)

(run-tests tests)
