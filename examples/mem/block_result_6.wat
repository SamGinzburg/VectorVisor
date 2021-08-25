(module
  (memory (export "memory") 1)

  (type (;0;) (func (param i32 i32) (result i32)))
  
  (func $_add2 (type 0) (param $p0 i32) (param $p1 i32) (result i32)
    (local.get $p0)
    (local.get $p1)
    (i32.add)
  )

  (func $_start (result i32)
    (local i32 i32)
    ;; save 0
    block
      i32.const 1337
      local.set 0
      ;; local 0 saved
    end
    i32.const 1
    i32.const 1
    ;; possible bug: we don't load ints
    (i32.const 0)
    ;; what gets saved here?
    call_indirect (type 0)
    ;; we need to load any locals used in this loop before starting
    loop (result i32)
      local.get 0
    end
    i32.add
  )
  (table $T0 5 5 funcref)
  (elem $e1 0 (i32.const 0) $_add2) ;; the table index can be implicitly 0
  (export "_start" (func $_start))
)
