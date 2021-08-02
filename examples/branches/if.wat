(module
  (func $_start (result i32)
    i32.const 100
    i32.const 0
    ;; pop i32 for branch, store if statement type
    if (result i32) ;; alloc result register
      i32.const 1
    else ;; pop most recent val in result register
      i32.const 2
    end ;; pop most recent val in result register
    i32.add
  )
  (export "_start" (func $_start))
)