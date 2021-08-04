(module
  (func $_start (result i32)
    (local i32)
    i32.const 0
    local.set 0
    i32.const 0
    if
      i32.const 100
      local.set 0
    end
    local.get 0
    local.get 0
    i32.add
  )
  (export "_start" (func $_start))
)