(define/contract (prefix-count words pref)
  (-> (listof string?) string? exact-integer?)
  (define (starts-with? word prefix)
    (equal? (substring word 0 (min (string-length word) (string-length prefix)))
            prefix))
  (length (filter (lambda (word) (starts-with? word pref)) words)))

