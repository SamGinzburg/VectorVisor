(module
  (func $_add2 (param $p0 i32) (param $p1 i32) (result i32)
    (local.get $p0)
    (local.get $p1)
    (i32.add)
    (local.set $p1)
    (local.get $p1)
  )
  (func $_start (result i32)
    (local $l2 i32)
    (i32.const 11)
    (i32.const 5)
    (call $_add2)
    (local.set $l2)
    (local.get $l2)
    (i32.const 100)
    (call $_add2)
    block
    i32.const 1
    end
  )
  (export "_start" (func $_start))
)