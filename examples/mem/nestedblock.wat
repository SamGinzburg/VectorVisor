(module
  (memory (export "memory") 1)


 (func $nested (result i32)
    (local $l1 i32)
    i32.const 7
    i32.const 7
    block ;; track stack pointer, pop items off back to here
      block ;; 0
        i32.const 1
        br_table 1 0 ;; we need to pop the stack items off here
        i32.const 7
        i32.const 7
        i32.add
        local.set $l1
      end
    end
    i32.add
    local.get $l1
    i32.add
  )

  (func $_start (result i32)
    call $nested
  )
  (export "_start" (func $_start))
)