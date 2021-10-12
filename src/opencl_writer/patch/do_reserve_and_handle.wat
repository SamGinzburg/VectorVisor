(module
 (memory (export "memory") 1)
  (func $__rust_realloc (param i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32)
    block (result i32)  ;; label = @1
      local.get 2
      i32.const 8
      i32.le_u
      i32.const 0
      local.get 2
      local.get 3
      i32.le_u
      select
      i32.eqz
      if  ;; label = @2
        i32.const 0
        local.get 2
        local.get 3
        call $aligned_alloc
        local.tee 2
        i32.eqz
        br_if 1 (;@1;)
        drop
        local.get 2
        local.get 0
        local.get 3
        local.get 1
        local.get 1
        local.get 3
        i32.gt_u
        select
        call $memcpy
        local.get 0
        call $dlfree
        br 1 (;@1;)
      end
      block (result i32)  ;; label = @2
        local.get 0
        i32.eqz
        if  ;; label = @3
          local.get 3
          call $dlmalloc
          br 1 (;@2;)
        end
        local.get 3
        i32.const -64
        i32.ge_u
        if  ;; label = @3
          i32.const 1099708
          i32.const 48
          i32.store
          i32.const 0
          br 1 (;@2;)
        end
        local.get 3
        i32.const 11
        i32.lt_u
        local.set 2
        local.get 3
        i32.const 19
        i32.add
        i32.const -16
        i32.and
        local.set 4
        local.get 0
        i32.const 8
        i32.sub
        local.set 6
        local.get 0
        i32.const 4
        i32.sub
        local.tee 7
        i32.load
        local.tee 8
        i32.const 3
        i32.and
        local.set 5
        i32.const 1099228
        i32.load
        local.set 9
        block  ;; label = @3
          local.get 8
          i32.const -8
          i32.and
          local.tee 1
          i32.const 1
          i32.lt_s
          br_if 0 (;@3;)
          local.get 6
          local.get 9
          i32.lt_u
          br_if 0 (;@3;)
        end
        i32.const 16
        local.get 4
        local.get 2
        select
        local.set 4
        block  ;; label = @3
          block  ;; label = @4
            local.get 5
            i32.eqz
            if  ;; label = @5
              local.get 4
              i32.const 256
              i32.lt_u
              br_if 1 (;@4;)
              local.get 1
              local.get 4
              i32.const 4
              i32.or
              i32.lt_u
              br_if 1 (;@4;)
              local.get 1
              local.get 4
              i32.sub
              i32.const 1099692
              i32.load
              i32.const 1
              i32.shl
              i32.le_u
              br_if 2 (;@3;)
              br 1 (;@4;)
            end
            local.get 1
            local.get 6
            i32.add
            local.set 5
            local.get 1
            local.get 4
            i32.ge_u
            if  ;; label = @5
              local.get 1
              local.get 4
              i32.sub
              local.tee 1
              i32.const 16
              i32.lt_u
              br_if 2 (;@3;)
              local.get 7
              local.get 4
              local.get 8
              i32.const 1
              i32.and
              i32.or
              i32.const 2
              i32.or
              i32.store
              local.get 4
              local.get 6
              i32.add
              local.tee 2
              local.get 1
              i32.const 3
              i32.or
              i32.store offset=4
              local.get 5
              local.get 5
              i32.load offset=4
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 2
              local.get 1
              call $dispose_chunk
              local.get 0
              br 3 (;@2;)
            end
            local.get 5
            i32.const 1099236
            i32.load
            i32.eq
            if  ;; label = @5
              i32.const 1099224
              i32.load
              local.get 1
              i32.add
              local.tee 1
              local.get 4
              i32.le_u
              br_if 1 (;@4;)
              local.get 7
              local.get 4
              local.get 8
              i32.const 1
              i32.and
              i32.or
              i32.const 2
              i32.or
              i32.store
              i32.const 1099236
              local.get 4
              local.get 6
              i32.add
              local.tee 2
              i32.store
              i32.const 1099224
              local.get 1
              local.get 4
              i32.sub
              local.tee 1
              i32.store
              local.get 2
              local.get 1
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 0
              br 3 (;@2;)
            end
            local.get 5
            i32.const 1099232
            i32.load
            i32.eq
            if  ;; label = @5
              i32.const 1099220
              i32.load
              local.get 1
              i32.add
              local.tee 1
              local.get 4
              i32.lt_u
              br_if 1 (;@4;)
              block  ;; label = @6
                local.get 1
                local.get 4
                i32.sub
                local.tee 2
                i32.const 16
                i32.ge_u
                if  ;; label = @7
                  local.get 7
                  local.get 4
                  local.get 8
                  i32.const 1
                  i32.and
                  i32.or
                  i32.const 2
                  i32.or
                  i32.store
                  local.get 4
                  local.get 6
                  i32.add
                  local.tee 3
                  local.get 2
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 1
                  local.get 6
                  i32.add
                  local.tee 1
                  local.get 2
                  i32.store
                  local.get 1
                  local.get 1
                  i32.load offset=4
                  i32.const -2
                  i32.and
                  i32.store offset=4
                  br 1 (;@6;)
                end
                local.get 7
                local.get 8
                i32.const 1
                i32.and
                local.get 1
                i32.or
                i32.const 2
                i32.or
                i32.store
                local.get 1
                local.get 6
                i32.add
                local.tee 1
                local.get 1
                i32.load offset=4
                i32.const 1
                i32.or
                i32.store offset=4
                i32.const 0
                local.set 2
                i32.const 0
                local.set 3
              end
              i32.const 1099232
              local.get 3
              i32.store
              i32.const 1099220
              local.get 2
              i32.store
              local.get 0
              br 3 (;@2;)
            end
            local.get 5
            i32.load offset=4
            local.tee 2
            i32.const 2
            i32.and
            br_if 0 (;@4;)
            local.get 2
            i32.const -8
            i32.and
            local.get 1
            i32.add
            local.tee 10
            local.get 4
            i32.lt_u
            br_if 0 (;@4;)
            local.get 10
            local.get 4
            i32.sub
            local.set 12
            block  ;; label = @5
              local.get 2
              i32.const 255
              i32.le_u
              if  ;; label = @6
                local.get 5
                i32.load offset=8
                local.tee 1
                local.get 2
                i32.const 3
                i32.shr_u
                local.tee 3
                i32.const 3
                i32.shl
                i32.const 1099252
                i32.add
                i32.ne
                drop
                local.get 1
                local.get 5
                i32.load offset=12
                local.tee 2
                i32.eq
                if  ;; label = @7
                  i32.const 1099212
                  i32.const 1099212
                  i32.load
                  i32.const -2
                  local.get 3
                  i32.rotl
                  i32.and
                  i32.store
                  br 2 (;@5;)
                end
                local.get 2
                local.get 1
                i32.store offset=8
                local.get 1
                local.get 2
                i32.store offset=12
                br 1 (;@5;)
              end
              local.get 5
              i32.load offset=24
              local.set 11
              block  ;; label = @6
                local.get 5
                local.get 5
                i32.load offset=12
                local.tee 1
                i32.ne
                if  ;; label = @7
                  local.get 9
                  local.get 5
                  i32.load offset=8
                  local.tee 2
                  i32.le_u
                  if  ;; label = @8
                    local.get 2
                    i32.load offset=12
                    drop
                  end
                  local.get 1
                  local.get 2
                  i32.store offset=8
                  local.get 2
                  local.get 1
                  i32.store offset=12
                  br 1 (;@6;)
                end
                block  ;; label = @7
                  local.get 5
                  i32.const 20
                  i32.add
                  local.tee 2
                  i32.load
                  local.tee 3
                  br_if 0 (;@7;)
                  local.get 5
                  i32.const 16
                  i32.add
                  local.tee 2
                  i32.load
                  local.tee 3
                  br_if 0 (;@7;)
                  i32.const 0
                  local.set 1
                  br 1 (;@6;)
                end
                loop  ;; label = @7
                  local.get 2
                  local.set 9
                  local.get 3
                  local.tee 1
                  i32.const 20
                  i32.add
                  local.tee 2
                  i32.load
                  local.tee 3
                  br_if 0 (;@7;)
                  local.get 1
                  i32.const 16
                  i32.add
                  local.set 2
                  local.get 1
                  i32.load offset=16
                  local.tee 3
                  br_if 0 (;@7;)
                end
                local.get 9
                i32.const 0
                i32.store
              end
              local.get 11
              i32.eqz
              br_if 0 (;@5;)
              block  ;; label = @6
                local.get 5
                local.get 5
                i32.load offset=28
                local.tee 2
                i32.const 2
                i32.shl
                i32.const 1099516
                i32.add
                local.tee 3
                i32.load
                i32.eq
                if  ;; label = @7
                  local.get 3
                  local.get 1
                  i32.store
                  local.get 1
                  br_if 1 (;@6;)
                  i32.const 1099216
                  i32.const 1099216
                  i32.load
                  i32.const -2
                  local.get 2
                  i32.rotl
                  i32.and
                  i32.store
                  br 2 (;@5;)
                end
                local.get 11
                i32.const 16
                i32.const 20
                local.get 11
                i32.load offset=16
                local.get 5
                i32.eq
                select
                i32.add
                local.get 1
                i32.store
                local.get 1
                i32.eqz
                br_if 1 (;@5;)
              end
              local.get 1
              local.get 11
              i32.store offset=24
              local.get 5
              i32.load offset=16
              local.tee 2
              if  ;; label = @6
                local.get 1
                local.get 2
                i32.store offset=16
                local.get 2
                local.get 1
                i32.store offset=24
              end
              local.get 5
              i32.load offset=20
              local.tee 2
              i32.eqz
              br_if 0 (;@5;)
              local.get 1
              i32.const 20
              i32.add
              local.get 2
              i32.store
              local.get 2
              local.get 1
              i32.store offset=24
            end
            local.get 12
            i32.const 15
            i32.le_u
            if  ;; label = @5
              local.get 7
              local.get 8
              i32.const 1
              i32.and
              local.get 10
              i32.or
              i32.const 2
              i32.or
              i32.store
              local.get 6
              local.get 10
              i32.add
              local.tee 1
              local.get 1
              i32.load offset=4
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 0
              br 3 (;@2;)
            end
            local.get 7
            local.get 4
            local.get 8
            i32.const 1
            i32.and
            i32.or
            i32.const 2
            i32.or
            i32.store
            local.get 4
            local.get 6
            i32.add
            local.tee 1
            local.get 12
            i32.const 3
            i32.or
            i32.store offset=4
            local.get 6
            local.get 10
            i32.add
            local.tee 2
            local.get 2
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 1
            local.get 12
            call $dispose_chunk
            local.get 0
            br 2 (;@2;)
          end
          i32.const 0
          local.get 3
          call $dlmalloc
          local.tee 1
          i32.eqz
          br_if 1 (;@2;)
          drop
          local.get 1
          local.get 0
          i32.const -4
          i32.const -8
          local.get 7
          i32.load
          local.tee 1
          i32.const 3
          i32.and
          select
          local.get 1
          i32.const -8
          i32.and
          i32.add
          local.tee 1
          local.get 3
          local.get 1
          local.get 3
          i32.lt_u
          select
          call $memcpy
          local.get 0
          call $dlfree
          local.set 0
        end
        local.get 0
      end
    end)

  (func $__rust_alloc (param i32 i32) (result i32)
    block (result i32)  ;; label = @1
      local.get 1
      i32.const 8
      i32.le_u
      i32.const 0
      local.get 0
      local.get 1
      i32.ge_u
      select
      i32.eqz
      if  ;; label = @2
        local.get 1
        local.get 0
        call $aligned_alloc
        br 1 (;@1;)
      end
      local.get 0
      call $dlmalloc
    end)

  (func $alloc::raw_vec::finish_grow::custom (param i32 i32 i32 i32)
    (local i32 i32)
    block  ;; label = @1
      block (result i32)  ;; label = @2
        block  ;; label = @3
          block (result i32)  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 2
                  if  ;; label = @8
                    i32.const 1
                    local.set 5
                    local.get 1
                    i32.const 0
                    i32.lt_s
                    br_if 7 (;@1;)
                    local.get 3
                    i32.load
                    local.tee 4
                    i32.eqz
                    br_if 2 (;@6;)
                    local.get 3
                    i32.load offset=4
                    local.tee 3
                    br_if 1 (;@7;)
                    local.get 1
                    br_if 3 (;@5;)
                    br 5 (;@3;)
                  end
                  local.get 0
                  local.get 1
                  i32.store offset=4
                  i32.const 1
                  local.set 5
                  br 6 (;@1;)
                end
                local.get 4
                local.get 3
                local.get 2
                local.get 1
                call $__rust_realloc
                br 2 (;@4;)
              end
              local.get 1
              i32.eqz
              br_if 2 (;@3;)
            end
            local.get 1
            local.get 2
            call $__rust_alloc
          end
          local.set 3
          local.get 1
          br 1 (;@2;)
        end
        local.get 2
        local.set 3
        i32.const 0
      end
      local.set 4
      local.get 3
      if  ;; label = @2
        local.get 0
        local.get 3
        i32.store offset=4
        i32.const 0
        local.set 5
        br 1 (;@1;)
      end
      local.get 0
      local.get 1
      i32.store offset=4
      local.get 2
      local.set 4
    end
    local.get 0
    local.get 5
    i32.store
    local.get 0
    i32.const 8
    i32.add
    local.get 4
    i32.store)

  (func $alloc::raw_vec::RawVec<T_A>::reserve::do_reserve_and_handle::three (param i32 i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 3
    global.set 0
    block  ;; label = @1
      local.get 1
      local.get 1
      local.get 2
      i32.add
      local.tee 2
      i32.gt_u
      br_if 0 (;@1;)
      local.get 0
      i32.const 4
      i32.add
      i32.load
      local.tee 1
      i32.const 1
      i32.shl
      local.tee 4
      local.get 2
      local.get 2
      local.get 4
      i32.lt_u
      select
      local.tee 2
      i32.const 8
      local.get 2
      i32.const 8
      i32.gt_u
      select
      local.set 2
      block  ;; label = @2
        local.get 1
        if  ;; label = @3
          local.get 3
          i32.const 24
          i32.add
          i32.const 1
          i32.store
          local.get 3
          local.get 1
          i32.store offset=20
          local.get 3
          local.get 0
          i32.load
          i32.store offset=16
          br 1 (;@2;)
        end
        local.get 3
        i32.const 0
        i32.store offset=16
      end
      local.get 3
      local.get 2
      i32.const 1
      local.get 3
      i32.const 16
      i32.add
      call $alloc::raw_vec::finish_grow::custom
      local.get 3
      i32.load
      i32.const 1
      i32.eq
      if  ;; label = @2
        local.get 3
        i32.const 8
        i32.add
        i32.load
        local.tee 0
        i32.eqz
        br_if 1 (;@1;)
        unreachable
      end
      local.get 0
      local.get 3
      i64.load offset=4 align=4
      i64.store align=4
      local.get 3
      i32.const 32
      i32.add
      global.set 0
      return
    end
    unreachable)
 
  (func $alloc::raw_vec::RawVec<T_A>::reserve::do_reserve_and_handle::two (param i32 i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    block  ;; label = @1
      local.get 1
      local.get 1
      i32.const 1
      i32.add
      local.tee 3
      i32.gt_u
      br_if 0 (;@1;)
      local.get 0
      i32.const 4
      i32.add
      i32.load
      local.tee 1
      i32.const 1
      i32.shl
      local.tee 4
      local.get 3
      local.get 3
      local.get 4
      i32.lt_u
      select
      local.tee 3
      i32.const 4
      local.get 3
      i32.const 4
      i32.gt_u
      select
      local.tee 3
      i32.const 1073741823
      i32.and
      local.get 3
      i32.eq
      i32.const 1
      i32.shl
      local.set 4
      local.get 3
      i32.const 2
      i32.shl
      local.set 3
      block  ;; label = @2
        local.get 1
        if  ;; label = @3
          local.get 2
          i32.const 24
          i32.add
          i32.const 2
          i32.store
          local.get 2
          local.get 1
          i32.const 2
          i32.shl
          i32.store offset=20
          local.get 2
          local.get 0
          i32.load
          i32.store offset=16
          br 1 (;@2;)
        end
        local.get 2
        i32.const 0
        i32.store offset=16
      end
      local.get 2
      local.get 3
      local.get 4
      local.get 2
      i32.const 16
      i32.add
      call $alloc::raw_vec::finish_grow::custom
      local.get 2
      i32.load
      i32.const 1
      i32.eq
      if  ;; label = @2
        local.get 2
        i32.const 8
        i32.add
        i32.load
        local.tee 0
        i32.eqz
        br_if 1 (;@1;)
        unreachable
      end
      local.get 2
      i32.load offset=4
      local.set 1
      local.get 0
      i32.const 4
      i32.add
      local.get 2
      i32.const 8
      i32.add
      i32.load
      i32.const 2
      i32.shr_u
      i32.store
      local.get 0
      local.get 1
      i32.store
      local.get 2
      i32.const 32
      i32.add
      global.set 0
      return
    end
    unreachable)
    (global (;0;) (mut i32) (i32.const 1048576))
) 
