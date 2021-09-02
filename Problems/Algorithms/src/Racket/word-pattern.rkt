#lang racket

; https://leetcode.com/problems/word-pattern/

(define/contract (word-pattern pattern s)
  (-> string? string? boolean?)
  (letrec ([go (lambda (pattern-word word-pattern ps xs)
                 (cond [(and (null? ps) (null? xs)) #t]
                       [(or (null? ps) (null? xs)) #f]
                       [else (let* ([pat  (car ps)]
                                    [word (car xs)]
                                    [w    (hash-ref pattern-word pat #f)]
                                    [p    (hash-ref word-pattern word #f)])
                               (cond [(and w (string=? w word))
                                      (go pattern-word word-pattern (cdr ps) (cdr xs))]
                                     [(and (false? w) (false? p))
                                      (go (hash-set pattern-word pat word)
                                          (hash-set word-pattern word pat)
                                          (cdr ps)
                                          (cdr xs))]
                                     [else #f]))]))])
    (go (make-immutable-hash)
        (make-immutable-hash)
        (string->list pattern)
        (string-split s)))
  )

(require rackunit)

(define tests
  (test-suite
    "Word Pattern"

    (check-true
      (word-pattern "abba" "dog cat cat dog")
      "Example 1")

    (check-false
      (word-pattern "abba" "dog cat cat fish")
      "Example 2")

    (check-false
      (word-pattern "aaaa" "dog cat cat dog")
      "Example 3")

    (check-false
      (word-pattern "abba" "dog dog dog dog")
      "Example 4")

    (check-true
      (word-pattern "aaaa" "dog dog dog dog")
      "Example 5")))

(require rackunit/text-ui)

(run-tests tests)
