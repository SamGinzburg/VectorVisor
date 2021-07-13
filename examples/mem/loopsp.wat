(module
  (memory (export "memory") 1)

  (func $_start (result i32)
    (local i32)
    i32.const 100
    local.set 0
    i32.const 1
    block
      loop
        i32.const 0
        i32.const 1 ;; allocate *sp += 2
        block
          local.get 0
          i32.const 1
          i32.sub
          local.tee 0
          i32.eqz
          br_if 2 ;; on breaking / continuing from the loop any nested stack frames must be cleaned up
          br 1
        end
        i32.add
        drop
      end
    end
  )
  (export "_start" (func $_start))
)