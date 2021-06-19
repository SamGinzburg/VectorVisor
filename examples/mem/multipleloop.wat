(module
  (memory (export "memory") 1)


 (func $loops (result i32)
    (local $l1 i32)
    i32.const 100
    local.set $l1
    i32.const 1337 ;; extra stack op
    loop ;; L0
      local.get $l1
      i32.const -1
      i32.add
      local.tee $l1
      br_if 0
    end
    i32.const 100
    local.set $l1
    i32.const 1337 ;; extra stack op 2
    loop ;; L0?
      local.get $l1
      i32.const -1
      i32.add
      local.tee $l1
      br_if 0
    end
    i32.add
  )

  (func $_start (result i32)
    call $loops
  )
  (export "_start" (func $_start))
)