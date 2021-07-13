(module
  (memory (export "memory") 1)

  (func $_start (result i32)
    (local i32)
    i32.const 100
    local.set 0
    i32.const 1
    block
      loop
        block
          local.get 0
          i32.const 1
          i32.sub
          local.tee 0
          i32.eqz
          br_if 2
          br 1
        end
      end
    end
  )
  (export "_start" (func $_start))
)