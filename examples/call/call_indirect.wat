(module
  (type $t0 (func (param i32 i32) (result i32)))
  (func $_add2 (param $p0 i32) (param $p1 i32) (result i32)
    (local.get $p0)
    (local.get $p1)
    (i32.add)
  )
  (func $_start (param $p0 i32) (result i32)
    (local $l2 i32)
    (i32.const 11)
    (i32.const 5)
    (i32.const 0) ;; the indirect call table index
    (call_indirect (type $t0) $T0)
    (local.set $l2)
    (local.get $l2)
    (i32.const 100)
    (i32.const 0) ;; the indirect call table index
    (call_indirect (type $t0) $T0)
  )
  (table $T0 5 5 funcref)
  (elem $e0 0 (i32.const 0) $_add2) ;; the first param is the table index
  (elem $e1 0 (i32.const 0) $_add2) ;; the table index can be implicitly 0
  ;; multiple elements can overwrite values
  (export "_start" (func $_start))
)