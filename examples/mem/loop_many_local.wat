(module
  (type (;0;) (func (param i32 i32) (result i32)))
  
  (func $_add2 (type 0) (param $p0 i32) (param $p1 i32) (result i32)
    (local.get $p0)
    (local.get $p1)
    (i32.add)
  )
  (func $_start (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    i32.const 0
    local.set 0
    i32.const 0
    local.set 1
    i32.const 1337
    local.set 2
    ;; save 0,1,2
    block
    loop
      ;; load l1
      ;; save l1
      loop
        ;; load l0
        local.get 0
        (i32.const 1)
        ;;(i32.const 0) ;; the indirect call table index
        ;; Save 0 here
        ;;call_indirect (type 0)
        i32.add
        ;; load 0
        local.tee 0
        i32.const 10000
        i32.ne
        br_if 0
      end
      ;; load 1
      br 1
      ;; only intermediates loaded
      local.get 1
      (i32.const 1)
      (i32.const 0) ;; the indirect call table index
      ;; save 1
      call_indirect (type 0)
      ;; load 1
      local.tee 1
      i32.const 5
      i32.eq
      br_if 0 ;; write l1
    end
    end
    ;; load 0
    local.get 0
  )
  (table $T0 5 5 funcref)
  (elem $e1 0 (i32.const 0) $_add2) ;; the table index can be implicitly 0
  ;; multiple elements can overwrite values
  (export "_start" (func $_start))
)
