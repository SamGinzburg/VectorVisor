(module
  (memory (export "memory") 1)


 (func $add (param $p0 i32) (param $p1 i32) (result i32)
    (local.get $p0)
    (local.get $p1)
    (i32.add)
  )

  (func $_start (result i32)
    (local $l1 i32)
    i32.const 100
    local.set $l1
    i32.const 1 ;; i1
    i32.const 1 ;; i2
    ;; save ctx (save everything)
    loop
      ;; restore ctx (restore only locals)
      local.get $l1 ;; i3
      i32.const 1 ;; i4
      i32.sub
      local.tee $l1
      i32.eqz
      ;; save ctx (save only locals)
      br_if 0
    end
    i32.const 7 ;; i3?
    i32.const 8 ;; i4?
    i32.add
    i32.add
    i32.add
  )
  (export "_start" (func $_start))
)