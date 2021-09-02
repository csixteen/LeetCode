#lang racket

; https://leetcode.com/problems/two-sum-iv-input-is-a-bst/

; val : integer?
; left : (or/c tree-node? #f)
; right : (or/c tree-node? #f)
(struct tree-node
  (val left right) #:mutable #:transparent)

; constructor
(define (make-tree-node [val 0])
  (tree-node val #f #f))

(define/contract (find-target root k)
  (-> (or/c tree-node? #f) exact-integer? boolean?)
  (letrec ([go (lambda (node target visited)
                 (cond [(false? node) #f]
                       [(set-member? visited (- target (tree-node-val node))) #t]
                       [else (begin
                               (set-add! visited (tree-node-val node))
                               (or (go (tree-node-left node) target visited)
                                   (go (tree-node-right node) target visited)))]))])
    (go root k (mutable-set)))
  )

(require rackunit)

(define tests
  (test-suite
    "Two Sum IV - Input is a BST"

    (check-true
      (find-target
        (tree-node 5
                   (tree-node 3 (make-tree-node 2) (make-tree-node 4))
                   (tree-node 6 #f (make-tree-node 7)))
        9)
      "Example 1")

    (check-false
      (find-target
        (tree-node 5
                   (tree-node 3 (make-tree-node 2) (make-tree-node 4))
                   (tree-node 6 #f (make-tree-node 7)))
        28)
      "Example 2")

    (check-true
      (find-target
        (tree-node 2 (make-tree-node 1) (make-tree-node 3))
        4)
      "Example 3")

    (check-false
      (find-target
        (tree-node 2 (make-tree-node 1) (make-tree-node 3))
        1)
      "Example 4")

    (check-true
      (find-target
        (tree-node 2 (make-tree-node 1) (make-tree-node 3))
        3)
      "Example 5")))

(require rackunit/text-ui)

(run-tests tests)
