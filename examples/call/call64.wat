(module

  (func $_add2 (param $p0 i64) (param $p1 i64) (result i64)
    (local.get $p0)
    (local.get $p1)
    (i64.add)
  )
  (func $_main (param $p0 i64) (result i64)
    (local $l2 i64)
    (i64.const 11)
    (i64.const 5)
    (call $_add2)
    (local.set $l2)
    (local.get $l2)
    (i64.const 100)
    (call $_add2)
  )
  (export "main" (func $_main))
)