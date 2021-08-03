(module
  (memory (export "memory") 1)

  (func $_start (result i32)
    loop (result i32)
      br 0
      ;;i32.const 1
    end
  )
  (export "_start" (func $_start))
)
