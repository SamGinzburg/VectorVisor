(module
  (memory (export "memory") 1)


 (func $add (param $p0 i32) (param $p1 i32) (result i32)
    (local.get $p0)
    (local.get $p1)
    (i32.add)
  )

  (func $_start (result i32)
    (local $l1 i32)
    block ;; track stack pointer, pop items off back to here
      i32.const 1
      i32.const 1
      br 0
    end
    block ;; track stack pointer, pop items off back to here
      i32.const 7
      i32.const 7
      i32.add
      local.set $l1
    end
    local.get $l1
  )
  (export "_start" (func $_start))
)