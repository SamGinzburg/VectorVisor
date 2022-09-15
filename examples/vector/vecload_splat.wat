(module
  (memory (export "memory") 1)

  (func $_start (result v128)
    (local $l2 v128)
    (i32.const 0)
    ;;(v128.load32_splat)
    (v128.load64_splat)
  )
  (export "_start" (func $_start))
  (data $d0 (i32.const 0) "ABCDEFG")
)
