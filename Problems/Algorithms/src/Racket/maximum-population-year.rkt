#lang racket

(define/contract (count-range rs acc)
  (-> (listof exact-integer?) hash? hash?)
  (letrec ([inc (lambda (x) (+ x 1))]
           [aux (lambda (lo hi _acc)
                  (if (= lo hi)
                    _acc
                    (let ([new-acc (hash-update _acc lo inc 0)])
                      (aux (inc lo) hi new-acc))))])
    (aux (car rs) (cadr rs) acc))
  )

(define/contract (count-ranges ranges)
  (-> (listof (listof exact-integer?)) hash?)
  (letrec ([aux (lambda (rs acc)
                  (if (null? rs)
                    acc
                    (let ([new-acc (count-range (car rs) acc)])
                      (aux (cdr rs) new-acc))))])
    (aux ranges (make-immutable-hash)))
  )

(define/contract (maximum-population logs)
  (-> (listof (listof exact-integer?)) exact-integer?)
  (let* ([c (count-ranges logs)]
         [sorted (sort (hash->list c)
                      (lambda (x y)
                        (or (> (cdr x) (cdr y))
                            (and (= (cdr x) (cdr y))
                                 (< (car x) (car y))))))])
    (caar sorted))
  )

(require rackunit)

(define tests
  (test-suite
    "Maximum Population Year"

    (check-equal?
      (maximum-population '((1993 1999) (2000 2010)))
      1993
      "Example 1")

    (check-equal?
      (maximum-population '((1950 1961) (1960 1971) (1970 1981)))
      1960
      "Example 2")

    (check-equal?
      (maximum-population '((2008 2026)
                            (2004 2008)
                            (2034 2035)
                            (1999 2050)
                            (2049 2050)
                            (2011 2035)
                            (1966 2033)
                            (2044 2049)))
      2011
      "Failing input")))


(require rackunit/text-ui)

(run-tests tests)
