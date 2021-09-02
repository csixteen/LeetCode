#lang racket

; https://leetcode.com/problems/decode-ways/

(define (make-memoize f)
  (let ([lookup (make-hash)])
    (lambda (xs)
      (if (hash-has-key? lookup xs)
        (hash-ref lookup xs)
        (let ([new-val (f xs)])
          (hash-set! lookup xs new-val)
          new-val)))))

(define (go s)
  (cond [(zero? (string-length s)) 1]
        [(char=? #\0 (string-ref s 0)) 0]
        [(= 1 (string-length s)) 1]
        [else (let ([ans1 (go (substring s 1))]
                    [ans2 (if (<= (string->number (substring s 0 2)) 26)
                            (go (substring s 2))
                            0)])
                (+ ans1 ans2))]))

(define/contract (num-decodings s)
  (-> string? exact-integer?)
  (begin
    (set! go (make-memoize go))
    (go s))
  )

(require rackunit)

(define tests
  (test-suite
    "Decode Ways"

    (check-equal?
      (num-decodings "12")
      2
      "Example 1")

    (check-equal?
      (num-decodings "226")
      3
      "Example 2")

    (check-equal?
      (num-decodings "0")
      0
      "Example 3")

    (check-equal?
      (num-decodings "06")
      0
      "Example 4")

    (check-equal?
      (num-decodings "60")
      0
      "Example 5")))

(require rackunit/text-ui)

(run-tests tests)
