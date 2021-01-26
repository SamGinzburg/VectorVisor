

(module
  ;; this function taken directly from a compiled rust program
  (func $_factorial (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i64 i64 i64 i64 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    local.set 1
    i32.const 16
    local.set 2
    local.get 1
    local.get 2
    i32.sub
    local.set 3
    local.get 3
    global.set 0
    local.get 3
    local.get 0
    i32.store offset=12
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              i32.eqz
              br_if 0 (;@5;)
              i32.const -1
              local.set 4
              local.get 0
              local.get 4
              i32.add
              local.set 5
              local.get 5
              local.get 0
              i32.gt_u
              local.set 6
              i32.const 1
              local.set 7
              local.get 6
              local.get 7
              i32.and
              local.set 8
              local.get 8
              br_if 2 (;@3;)
              br 1 (;@4;)
            end
            i32.const 1
            local.set 9
            local.get 3
            local.get 9
            i32.store offset=8
            br 3 (;@1;)
          end
          local.get 5
          call $_factorial
          local.set 10
          local.get 10
          i64.extend_i32_u
          local.set 11
          local.get 0
          i64.extend_i32_u
          local.set 12
          local.get 12
          local.get 11
          i64.mul
          local.set 13
          i64.const 32
          local.set 14
          local.get 13
          local.get 14
          i64.shr_u
          local.set 15
          local.get 15
          i32.wrap_i64
          local.set 16
          i32.const 0
          local.set 17
          local.get 16
          local.get 17
          i32.ne
          local.set 18
          local.get 13
          i32.wrap_i64
          local.set 19
          i32.const 1
          local.set 20
          local.get 18
          local.get 20
          i32.and
          local.set 21
          local.get 21
          br_if 1 (;@2;)
          local.get 3
          local.get 19
          i32.store offset=8
          br 2 (;@1;)
        end
        i32.const 1048912
        local.set 22
        local.get 22
        local.set 23
        i32.const 33
        local.set 24
        i32.const 1048892
        local.set 25
        local.get 25
        local.set 26
        local.get 23
        local.get 24
        local.get 26
        unreachable
      end
      i32.const 1048976
      local.set 27
      local.get 27
      local.set 28
      i32.const 33
      local.set 29
      i32.const 1048948
      local.set 30
      local.get 30
      local.set 31
      local.get 28
      local.get 29
      local.get 31
      unreachable
    end
    local.get 3
    i32.load offset=8
    local.set 32
    i32.const 16
    local.set 33
    local.get 3
    local.get 33
    i32.add
    local.set 34
    local.get 34
    global.set 0
    local.get 32
    return)

  (func $_start (result i32)
    (local $l2 i64)
    (i32.const 10)
    (call $_factorial)
  )
  (memory (;0;) 17)
  (global (;0;) (mut i32) (i32.const 1048576))
  (export "_start" (func $_start))
)