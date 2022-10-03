;; cargo run --release -- -i context_test.wat --nvidia=false --disablefastcalls=true --partition=true --maxdup=0
(module
  (memory (export "memory") 1)

  (func $add (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.add
  )

  (func $_start (result i32)
    (local i32 i32)
    block (result i32)
    loop (result i32)
    i32.const 1
    i32.const 1
    i32.eq
    if (result i32)
        i32.const 2
        i32.const 2
        i32.add
        i32.const 1
        br_if 0
        drop
        i32.const 1
    else
        i32.const 42
    end
    br 1
    end
    end
  )
  (export "_start" (func $_start))
)
