#lang racket

; https://leetcode.com/problems/unique-binary-search-trees/

(define (make-memoize f)
  (let ([lookup (make-hash)])
    (lambda (n)
      (if (hash-has-key? lookup n)
        (hash-ref lookup n)
        (let ([new-val (f n)])
          (hash-set! lookup n new-val)
          new-val)))))

(define (go n)
  (cond [(< n 2) 1]
        [else (foldl +
                     0
                     (map (lambda (i) (* (go (- i 1)) (go (- n i))))
                          (range 1 (+ n 1))))]))

(define/contract (num-trees n)
  (-> exact-integer? exact-integer?)
  (begin
    (set! go (make-memoize go))
    (go n))
  )

(require rackunit)

(define tests
  (test-suite
    "Unique Binary Search Trees"

    (check-equal?
      (num-trees 3)
      5
      "Example 1")

    (check-equal?
      (num-trees 1)
      1
      "Example 2")

    (check-equal?
      (num-trees 2)
      2
      "Example 3")))

(require rackunit/text-ui)

(run-tests tests)
