(module
 (func $_main (local $x i32)
  ;; x = 0
  (i32.const 0)
  (set_local $x)
  (block $B0
    (loop $L0
      (get_local $x)
      (i32.const 1)
      (i32.add)
      (set_local $x)
      (get_local $x)
      (i32.const 10)
      (i32.eq)
      (br_if $B0)
      (br $L0))))
  (export "_main" (func $_main))
)