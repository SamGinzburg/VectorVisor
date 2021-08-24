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
    i32.const 0
    local.set 0
    i32.const 1
    i32.const 2
    i32.const 3
    i32.const 4
    block (result i32)
    	loop (result i32)
        local.get 0
        i32.const 1
        (i32.const 0) ;; the indirect call table index
        call_indirect (type 0)
        local.tee 0
        local.get 0
        i32.const 777
        i32.eq
        br_if 1
        local.get 0
    	  i32.const 1000
        i32.ne
        br_if 0
      end
    end
    i32.add
    i32.add
    i32.add
    i32.add
  )
  (table $T0 5 5 funcref)
  (elem $e1 0 (i32.const 0) $_add2) ;; the table index can be implicitly 0
  (export "_start" (func $_start))
)
