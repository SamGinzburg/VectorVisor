(module
  (memory (export "memory") 1)

  (type (;0;) (func (param i32 i32) (result i32)))
  
  (func $_add2 (type 0) (param $p0 i32) (param $p1 i32) (result i32)
    (local.get $p0)
    (local.get $p1)
    (i32.add)
  )

  (func $_start (result i32)
    (local i32)
    block
      block
        i32.const 1337
        local.set 0
        ;; nothing saved
        loop
          i32.const 1
          drop
          ;; nothing saved
          br 1
        end
      end
      ;; local 0 hasn't been saved yet
      i32.const 555
      i32.const 555
      (i32.const 0)
      ;; save 0
      call_indirect (type 0)
      drop
    end
    local.get 0
  )
  (table $T0 5 5 funcref)
  (elem $e1 0 (i32.const 0) $_add2) ;; the table index can be implicitly 0
  (export "_start" (func $_start))
)
