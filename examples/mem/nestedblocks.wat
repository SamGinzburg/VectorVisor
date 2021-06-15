(module
  (memory (export "memory") 1)


 (func $blocks (result i32)
    (local $l1 i32)
    i32.const 100
    local.set $l1
    i32.const 1 ;; i1
    i32.const 1 ;; i2
    block
      i32.const 1
      ;; save ctx (save everything)
      loop
        ;; restore ctx (restore only locals)
        local.get $l1 ;; i3
        i32.const 1 ;; i4
        i32.sub
        local.tee $l1
        i32.eqz
        ;; save ctx (save only locals)
        br_if 1
      end
      i32.const 1
      i32.add
      local.set $l1
    end
    i32.const 7 ;; i3?
    local.get $l1
    i32.add
    i32.add
    i32.add
  )

  (func $_start (result i32)
    call $blocks
  )
  (export "_start" (func $_start))
)