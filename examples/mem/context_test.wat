;; cargo run --release -- -i context_test.wat --nvidia=false --disablefastcalls=true --partition=true --maxdup=0
(module
  (memory (export "memory") 1)

  (func $add (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.add
  )

  (func $_start (result i32)
    (local $l1 i32)
    block (result i32)
      i32.const 42
      local.set $l1
      i32.const 1
      i32.const 1
      call $add
      drop
      ;; now check to see if we still have the old result
      local.get $l1
    end
  )
  (export "_start" (func $_start))
)