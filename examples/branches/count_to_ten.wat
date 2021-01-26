(module
  (func $_start (result i32)
    (i32.const 100)
    block $B0
        (i32.const 0)
        (br_if $B0)
    end
    return)
  (export "_start" (func $_start))
)