#lang racket

; https://leetcode.com/problems/find-the-difference/

(define/contract (find-the-difference s t)
  (-> string? string? char?)
  (let ([xor (lambda (a b) (integer->char (bitwise-xor (char->integer a) (char->integer b))))]
        [z (integer->char 0)])
    (xor (foldl xor z (string->list s))
         (foldl xor z (string->list t))))
  )


(require rackunit)

(define tests
  (test-suite
    "Find the difference"

    (check-equal?
      (find-the-difference "abcd" "abcde")
      #\e
      "Example 1")

    (check-equal?
      (find-the-difference "" "y")
      #\y
      "Example 2")

    (check-equal?
      (find-the-difference "a" "aa")
      #\a
      "Example 3")

    (check-equal?
      (find-the-difference "ae" "aea")
      #\a
      "Example 4")))

(require rackunit/text-ui)

(run-tests tests)
