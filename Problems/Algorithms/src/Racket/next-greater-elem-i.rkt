#lang racket

; https://leetcode.com/problems/next-greater-element-i/

(define/contract (next-greater-element nums1 nums2)
  (-> (listof exact-integer?) (listof exact-integer?) (listof exact-integer?))
  (letrec ([unload-stack (lambda (acc stack)
                           (cond [(null? stack) acc]
                                 [else (unload-stack (hash-set acc (car stack) -1)
                                                     (cdr stack))]))]
           [insert (lambda (elem acc stack)
                     (cond [(or (null? stack) (< elem (car stack)))
                            (values acc (cons elem stack))]
                           [else (insert elem
                                         (hash-set acc (car stack) elem)
                                         (cdr stack))]))]
           [go (lambda (xs acc stack)
                 (cond [(null? xs) (unload-stack acc stack)]
                       [else (let-values ([(new-acc new-stack)
                                           (insert (car xs) acc stack)])
                               (go (cdr xs) new-acc new-stack))]))]
           [mapping (go nums2 (make-immutable-hash) '())])
    (map (lambda (x) (hash-ref mapping x)) nums1))
  )

(require rackunit)

(define tests
  (test-suite
    "Next Greater Element I"

    (check-equal?
      (next-greater-element '(4 1 2) '(1 3 4 2))
      '(-1 3 -1)
      "Example 1")

    (check-equal?
      (next-greater-element '(2 4) '(1 2 3 4))
      '(3 -1)
      "Example 2")

    (check-equal?
      (next-greater-element '(1 3 5 2 4) '(6 5 4 3 2 1 7))
      '(7 7 7 7 7)
      "Example 3")))

(require rackunit/text-ui)

(run-tests tests)
