#lang racket

; https://leetcode.com/problems/uncommon-words-from-two-sentences/

(define/contract (uncommon-from-sentences s1 s2)
  (-> string? string? (listof string?))
  (letrec ([occur (lambda (xs acc)
                    (if (null? xs)
                      acc
                      (let ([n (hash-ref acc (car xs) 0)])
                        (occur (cdr xs)
                               (hash-set acc (car xs) (+ n 1))))))]
           [xs (append (string-split s1) (string-split s2))]
           [count (occur xs (make-immutable-hash))]
           [hash-filter (lambda (hash pos)
                          (if (false? pos)
                            '()
                            (let-values ([(k v) (hash-iterate-key+value hash pos)])
                              (if (> v 1)
                                (hash-filter hash (hash-iterate-next hash pos))
                                (cons k (hash-filter hash (hash-iterate-next hash pos)))))))])
    (hash-filter count (hash-iterate-first count)))
  )

(require rackunit)

(define tests
  (test-suite
    "Uncommon Words from Two Sentences"

    (check-equal?
      (sort
        (uncommon-from-sentences "this apple is sweet" "this apple is sour")
        string<?)
      '("sour" "sweet")
      "Example 1")

    (check-equal?
      (uncommon-from-sentences "apple apple" "banana")
      '("banana")
      "Example 2")))

(require rackunit/text-ui)

(run-tests tests)
