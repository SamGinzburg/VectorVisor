(module
 (func $_start (result i32) (local $x i32)
  ;; x = 0
  (i32.const 100)
  (set_local $x)
  (block $B0
    (loop $L0
      (get_local $x)
      (i32.const 1)
      (i32.sub)
      (set_local $x)
      (get_local $x)
      (i32.const 0)
      (i32.eq)
      (br_if $B0)
      (br $L0)))
    local.get $x
    return)
  (export "_start" (func $_start))
)