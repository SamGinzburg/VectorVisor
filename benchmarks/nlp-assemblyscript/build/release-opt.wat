(module
  (type (;0;) (func))
  (type (;1;) (func (param i32 i32)))
  (type (;2;) (func (param i32)))
  (type (;3;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;4;) (func (param i32 i32 i32)))
  (type (;5;) (func (param i32 i32) (result i32)))
  (import "wasi_snapshot_preview1" "fd_write" (func $fimport$0 (type 3)))
  (func $0 (type 0)
    (local i32 i32)
    i32.const 1296
    call $10
    i32.const 1104
    call $10
    i32.const 1504
    call $10
    global.get 4
    local.tee 1
    i32.load offset=4
    i32.const -4
    i32.and
    local.set 0
    loop  ;; label = @1
      local.get 0
      local.get 1
      i32.ne
      if  ;; label = @2
        local.get 0
        i32.load offset=4
        i32.const 3
        i32.and
        i32.const 3
        i32.ne
        if  ;; label = @3
          unreachable
        end
        local.get 0
        i32.const 20
        i32.add
        call $8
        local.get 0
        i32.load offset=4
        i32.const -4
        i32.and
        local.set 0
        br 1 (;@1;)
      end
    end)
  (func $1 (type 1) (param i32 i32)
    (local i32 i32 i32 i32)
    local.get 1
    i32.load
    local.tee 2
    i32.const 1
    i32.and
    i32.eqz
    if  ;; label = @1
      unreachable
    end
    local.get 2
    i32.const -4
    i32.and
    local.tee 2
    i32.const 12
    i32.lt_u
    if  ;; label = @1
      unreachable
    end
    local.get 2
    i32.const 256
    i32.lt_u
    if (result i32)  ;; label = @1
      local.get 2
      i32.const 4
      i32.shr_u
    else
      i32.const 31
      i32.const 1073741820
      local.get 2
      local.get 2
      i32.const 1073741820
      i32.ge_u
      select
      local.tee 2
      i32.clz
      i32.sub
      local.tee 3
      i32.const 7
      i32.sub
      local.set 4
      local.get 2
      local.get 3
      i32.const 4
      i32.sub
      i32.shr_u
      i32.const 16
      i32.xor
    end
    local.tee 3
    i32.const 16
    i32.lt_u
    local.get 4
    i32.const 23
    i32.lt_u
    i32.and
    i32.eqz
    if  ;; label = @1
      unreachable
    end
    local.get 1
    i32.load offset=8
    local.set 2
    local.get 1
    i32.load offset=4
    local.tee 5
    if  ;; label = @1
      local.get 5
      local.get 2
      i32.store offset=8
    end
    local.get 2
    if  ;; label = @1
      local.get 2
      local.get 5
      i32.store offset=4
    end
    local.get 1
    local.get 0
    local.get 4
    i32.const 4
    i32.shl
    local.get 3
    i32.add
    i32.const 2
    i32.shl
    i32.add
    i32.load offset=96
    i32.eq
    if  ;; label = @1
      local.get 0
      local.get 4
      i32.const 4
      i32.shl
      local.get 3
      i32.add
      i32.const 2
      i32.shl
      i32.add
      local.get 2
      i32.store offset=96
      local.get 2
      i32.eqz
      if  ;; label = @2
        local.get 0
        local.get 4
        i32.const 2
        i32.shl
        i32.add
        local.tee 2
        i32.load offset=4
        i32.const -2
        local.get 3
        i32.rotl
        i32.and
        local.set 1
        local.get 2
        local.get 1
        i32.store offset=4
        local.get 1
        i32.eqz
        if  ;; label = @3
          local.get 0
          local.get 0
          i32.load
          i32.const -2
          local.get 4
          i32.rotl
          i32.and
          i32.store
        end
      end
    end)
  (func $2 (type 1) (param i32 i32)
    (local i32 i32 i32 i32 i32)
    local.get 1
    i32.eqz
    if  ;; label = @1
      unreachable
    end
    local.get 1
    i32.load
    local.tee 2
    i32.const 1
    i32.and
    i32.eqz
    if  ;; label = @1
      unreachable
    end
    local.get 1
    i32.const 4
    i32.add
    local.get 1
    i32.load
    i32.const -4
    i32.and
    i32.add
    local.tee 3
    i32.load
    local.tee 5
    i32.const 1
    i32.and
    if  ;; label = @1
      local.get 0
      local.get 3
      call $1
      local.get 1
      local.get 2
      i32.const 4
      i32.add
      local.get 5
      i32.const -4
      i32.and
      i32.add
      local.tee 2
      i32.store
      local.get 1
      i32.const 4
      i32.add
      local.get 1
      i32.load
      i32.const -4
      i32.and
      i32.add
      local.tee 3
      i32.load
      local.set 5
    end
    local.get 2
    i32.const 2
    i32.and
    if  ;; label = @1
      local.get 1
      i32.const 4
      i32.sub
      i32.load
      local.tee 1
      i32.load
      local.tee 6
      i32.const 1
      i32.and
      i32.eqz
      if  ;; label = @2
        unreachable
      end
      local.get 0
      local.get 1
      call $1
      local.get 1
      local.get 6
      i32.const 4
      i32.add
      local.get 2
      i32.const -4
      i32.and
      i32.add
      local.tee 2
      i32.store
    end
    local.get 3
    local.get 5
    i32.const 2
    i32.or
    i32.store
    local.get 2
    i32.const -4
    i32.and
    local.tee 2
    i32.const 12
    i32.lt_u
    if  ;; label = @1
      unreachable
    end
    local.get 3
    local.get 1
    i32.const 4
    i32.add
    local.get 2
    i32.add
    i32.ne
    if  ;; label = @1
      unreachable
    end
    local.get 3
    i32.const 4
    i32.sub
    local.get 1
    i32.store
    local.get 2
    i32.const 256
    i32.lt_u
    if (result i32)  ;; label = @1
      local.get 2
      i32.const 4
      i32.shr_u
    else
      i32.const 31
      i32.const 1073741820
      local.get 2
      local.get 2
      i32.const 1073741820
      i32.ge_u
      select
      local.tee 2
      i32.clz
      i32.sub
      local.tee 3
      i32.const 7
      i32.sub
      local.set 4
      local.get 2
      local.get 3
      i32.const 4
      i32.sub
      i32.shr_u
      i32.const 16
      i32.xor
    end
    local.tee 2
    i32.const 16
    i32.lt_u
    local.get 4
    i32.const 23
    i32.lt_u
    i32.and
    i32.eqz
    if  ;; label = @1
      unreachable
    end
    local.get 0
    local.get 4
    i32.const 4
    i32.shl
    local.get 2
    i32.add
    i32.const 2
    i32.shl
    i32.add
    i32.load offset=96
    local.set 3
    local.get 1
    i32.const 0
    i32.store offset=4
    local.get 1
    local.get 3
    i32.store offset=8
    local.get 3
    if  ;; label = @1
      local.get 3
      local.get 1
      i32.store offset=4
    end
    local.get 0
    local.get 4
    i32.const 4
    i32.shl
    local.get 2
    i32.add
    i32.const 2
    i32.shl
    i32.add
    local.get 1
    i32.store offset=96
    local.get 0
    local.get 0
    i32.load
    i32.const 1
    local.get 4
    i32.shl
    i32.or
    i32.store
    local.get 0
    local.get 4
    i32.const 2
    i32.shl
    i32.add
    local.tee 0
    local.get 0
    i32.load offset=4
    i32.const 1
    local.get 2
    i32.shl
    i32.or
    i32.store offset=4)
  (func $3 (type 4) (param i32 i32 i32)
    (local i32 i32)
    local.get 1
    local.get 2
    i32.gt_u
    if  ;; label = @1
      unreachable
    end
    local.get 1
    i32.const 19
    i32.add
    i32.const -16
    i32.and
    i32.const 4
    i32.sub
    local.set 1
    local.get 0
    i32.load offset=1568
    local.tee 3
    if  ;; label = @1
      local.get 3
      i32.const 4
      i32.add
      local.get 1
      i32.gt_u
      if  ;; label = @2
        unreachable
      end
      local.get 1
      i32.const 16
      i32.sub
      local.get 3
      i32.eq
      if  ;; label = @2
        local.get 3
        i32.load
        local.set 4
        local.get 1
        i32.const 16
        i32.sub
        local.set 1
      end
    else
      local.get 0
      i32.const 1572
      i32.add
      local.get 1
      i32.gt_u
      if  ;; label = @2
        unreachable
      end
    end
    local.get 2
    i32.const -16
    i32.and
    local.get 1
    i32.sub
    local.tee 2
    i32.const 20
    i32.lt_u
    if  ;; label = @1
      return
    end
    local.get 1
    local.get 4
    i32.const 2
    i32.and
    local.get 2
    i32.const 8
    i32.sub
    local.tee 2
    i32.const 1
    i32.or
    i32.or
    i32.store
    local.get 1
    i32.const 0
    i32.store offset=4
    local.get 1
    i32.const 0
    i32.store offset=8
    local.get 1
    i32.const 4
    i32.add
    local.get 2
    i32.add
    local.tee 2
    i32.const 2
    i32.store
    local.get 0
    local.get 2
    i32.store offset=1568
    local.get 0
    local.get 1
    call $2)
  (func $4 (type 0)
    (local i32 i32)
    memory.size
    local.tee 0
    i32.const 0
    i32.le_s
    if (result i32)  ;; label = @1
      i32.const 1
      local.get 0
      i32.sub
      memory.grow
      i32.const 0
      i32.lt_s
    else
      i32.const 0
    end
    if  ;; label = @1
      unreachable
    end
    i32.const 34496
    i32.const 0
    i32.store
    i32.const 36064
    i32.const 0
    i32.store
    loop  ;; label = @1
      local.get 1
      i32.const 23
      i32.lt_u
      if  ;; label = @2
        local.get 1
        i32.const 2
        i32.shl
        i32.const 34496
        i32.add
        i32.const 0
        i32.store offset=4
        i32.const 0
        local.set 0
        loop  ;; label = @3
          local.get 0
          i32.const 16
          i32.lt_u
          if  ;; label = @4
            local.get 1
            i32.const 4
            i32.shl
            local.get 0
            i32.add
            i32.const 2
            i32.shl
            i32.const 34496
            i32.add
            i32.const 0
            i32.store offset=96
            local.get 0
            i32.const 1
            i32.add
            local.set 0
            br 1 (;@3;)
          end
        end
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        br 1 (;@1;)
      end
    end
    i32.const 34496
    i32.const 36068
    memory.size
    i32.const 16
    i32.shl
    call $3
    i32.const 34496
    global.set 9)
  (func $6 (type 5) (param i32 i32) (result i32)
    (local i32 i32)
    local.get 1
    i32.const 256
    i32.lt_u
    if (result i32)  ;; label = @1
      local.get 1
      i32.const 4
      i32.shr_u
    else
      i32.const 31
      local.get 1
      i32.const 1
      i32.const 27
      local.get 1
      i32.clz
      i32.sub
      i32.shl
      i32.add
      i32.const 1
      i32.sub
      local.get 1
      local.get 1
      i32.const 536870910
      i32.lt_u
      select
      local.tee 1
      i32.clz
      i32.sub
      local.tee 3
      i32.const 7
      i32.sub
      local.set 2
      local.get 1
      local.get 3
      i32.const 4
      i32.sub
      i32.shr_u
      i32.const 16
      i32.xor
    end
    local.tee 1
    i32.const 16
    i32.lt_u
    local.get 2
    i32.const 23
    i32.lt_u
    i32.and
    i32.eqz
    if  ;; label = @1
      unreachable
    end
    local.get 0
    local.get 2
    i32.const 2
    i32.shl
    i32.add
    i32.load offset=4
    i32.const -1
    local.get 1
    i32.shl
    i32.and
    local.tee 1
    if (result i32)  ;; label = @1
      local.get 0
      local.get 1
      i32.ctz
      local.get 2
      i32.const 4
      i32.shl
      i32.add
      i32.const 2
      i32.shl
      i32.add
      i32.load offset=96
    else
      local.get 0
      i32.load
      i32.const -1
      local.get 2
      i32.const 1
      i32.add
      i32.shl
      i32.and
      local.tee 1
      if (result i32)  ;; label = @2
        local.get 0
        local.get 1
        i32.ctz
        local.tee 1
        i32.const 2
        i32.shl
        i32.add
        i32.load offset=4
        local.tee 2
        i32.eqz
        if  ;; label = @3
          unreachable
        end
        local.get 0
        local.get 2
        i32.ctz
        local.get 1
        i32.const 4
        i32.shl
        i32.add
        i32.const 2
        i32.shl
        i32.add
        i32.load offset=96
      else
        i32.const 0
      end
    end)
  (func $8 (type 2) (param i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              i32.const 8
              i32.sub
              i32.load
              br_table 0 (;@5;) 1 (;@4;) 2 (;@3;) 3 (;@2;) 4 (;@1;)
            end
            return
          end
          return
        end
        return
      end
      local.get 0
      i32.load
      local.tee 0
      if  ;; label = @2
        local.get 0
        call $10
      end
      return
    end
    unreachable)
  (func $9 (type 0)
    (local i32 i32 i32 i32 i32 i32 i32)
    global.get 11
    i32.const 4
    i32.sub
    global.set 11
    global.get 11
    i32.const 1716
    i32.lt_s
    if  ;; label = @1
      unreachable
    end
    global.get 11
    local.tee 1
    i32.const 0
    i32.store
    memory.size
    i32.const 16
    i32.shl
    i32.const 34484
    i32.sub
    i32.const 1
    i32.shr_u
    global.set 1
    i32.const 1220
    i32.const 1216
    i32.store
    i32.const 1224
    i32.const 1216
    i32.store
    i32.const 1216
    global.set 4
    i32.const 1252
    i32.const 1248
    i32.store
    i32.const 1256
    i32.const 1248
    i32.store
    i32.const 1248
    global.set 6
    i32.const 1396
    i32.const 1392
    i32.store
    i32.const 1400
    i32.const 1392
    i32.store
    i32.const 1392
    global.set 8
    local.get 1
    i32.const 1056
    i32.store
    local.get 1
    i32.const 4
    i32.sub
    global.set 11
    global.get 11
    i32.const 1716
    i32.lt_s
    if  ;; label = @1
      unreachable
    end
    global.get 11
    local.tee 1
    i32.const 0
    i32.store
    local.get 1
    i32.const 4
    i32.sub
    global.set 11
    global.get 11
    i32.const 1716
    i32.lt_s
    if  ;; label = @1
      unreachable
    end
    global.get 11
    local.tee 1
    i32.const 0
    i32.store
    i32.const 1
    global.set 10
    local.get 1
    global.get 11
    i32.const 4
    i32.sub
    global.set 11
    global.get 11
    i32.const 1716
    i32.lt_s
    if  ;; label = @1
      unreachable
    end
    global.get 11
    i32.const 0
    i32.store
    i32.const 1056
    local.set 0
    i32.const 1052
    i32.load
    i32.const 1056
    i32.add
    local.set 2
    loop  ;; label = @1
      local.get 0
      local.get 2
      i32.lt_u
      if  ;; label = @2
        local.get 0
        i32.load16_u
        local.tee 1
        i32.const 128
        i32.lt_u
        if (result i32)  ;; label = @3
          local.get 5
          i32.const 1
          i32.add
        else
          local.get 1
          i32.const 2048
          i32.lt_u
          if (result i32)  ;; label = @4
            local.get 5
            i32.const 2
            i32.add
          else
            local.get 1
            i32.const 64512
            i32.and
            i32.const 55296
            i32.eq
            local.get 0
            i32.const 2
            i32.add
            local.get 2
            i32.lt_u
            i32.and
            if  ;; label = @5
              local.get 0
              i32.load16_u offset=2
              i32.const 64512
              i32.and
              i32.const 56320
              i32.eq
              if  ;; label = @6
                local.get 5
                i32.const 4
                i32.add
                local.set 5
                local.get 0
                i32.const 4
                i32.add
                local.set 0
                br 5 (;@1;)
              end
            end
            local.get 5
            i32.const 3
            i32.add
          end
        end
        local.set 5
        local.get 0
        i32.const 2
        i32.add
        local.set 0
        br 1 (;@1;)
      end
    end
    local.get 5
    i32.const 1073741804
    i32.ge_u
    if  ;; label = @1
      unreachable
    end
    global.get 11
    global.get 0
    global.get 1
    i32.ge_u
    if  ;; label = @1
      block  ;; label = @2
        i32.const 2048
        local.set 0
        loop  ;; label = @3
          local.get 0
          block (result i32)  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    global.get 2
                    br_table 0 (;@8;) 1 (;@7;) 2 (;@6;) 3 (;@5;)
                  end
                  i32.const 1
                  global.set 2
                  i32.const 0
                  global.set 3
                  call $0
                  global.get 6
                  global.set 5
                  global.get 3
                  br 3 (;@4;)
                end
                global.get 7
                i32.eqz
                local.set 0
                global.get 5
                i32.load offset=4
                i32.const -4
                i32.and
                local.set 2
                loop  ;; label = @7
                  local.get 2
                  global.get 6
                  i32.ne
                  if  ;; label = @8
                    local.get 2
                    global.set 5
                    local.get 0
                    local.get 2
                    i32.load offset=4
                    i32.const 3
                    i32.and
                    i32.ne
                    if  ;; label = @9
                      local.get 2
                      local.get 2
                      i32.load offset=4
                      i32.const -4
                      i32.and
                      local.get 0
                      i32.or
                      i32.store offset=4
                      i32.const 0
                      global.set 3
                      local.get 2
                      i32.const 20
                      i32.add
                      call $8
                      global.get 3
                      br 5 (;@4;)
                    end
                    local.get 2
                    i32.load offset=4
                    i32.const -4
                    i32.and
                    local.set 2
                    br 1 (;@7;)
                  end
                end
                i32.const 0
                global.set 3
                call $0
                global.get 6
                global.get 5
                i32.load offset=4
                i32.const -4
                i32.and
                i32.eq
                if  ;; label = @7
                  global.get 11
                  local.set 2
                  loop  ;; label = @8
                    local.get 2
                    i32.const 34484
                    i32.lt_u
                    if  ;; label = @9
                      local.get 2
                      i32.load
                      local.tee 1
                      if  ;; label = @10
                        local.get 1
                        call $10
                      end
                      local.get 2
                      i32.const 4
                      i32.add
                      local.set 2
                      br 1 (;@8;)
                    end
                  end
                  global.get 5
                  i32.load offset=4
                  i32.const -4
                  i32.and
                  local.set 2
                  loop  ;; label = @8
                    local.get 2
                    global.get 6
                    i32.ne
                    if  ;; label = @9
                      local.get 0
                      local.get 2
                      i32.load offset=4
                      i32.const 3
                      i32.and
                      i32.ne
                      if  ;; label = @10
                        local.get 2
                        local.get 2
                        i32.load offset=4
                        i32.const -4
                        i32.and
                        local.get 0
                        i32.or
                        i32.store offset=4
                        local.get 2
                        i32.const 20
                        i32.add
                        call $8
                      end
                      local.get 2
                      i32.load offset=4
                      i32.const -4
                      i32.and
                      local.set 2
                      br 1 (;@8;)
                    end
                  end
                  global.get 8
                  local.set 1
                  global.get 6
                  global.set 8
                  local.get 1
                  global.set 6
                  local.get 0
                  global.set 7
                  local.get 1
                  i32.load offset=4
                  i32.const -4
                  i32.and
                  global.set 5
                  i32.const 2
                  global.set 2
                end
                global.get 3
                br 2 (;@4;)
              end
              global.get 5
              local.tee 0
              global.get 6
              i32.ne
              if  ;; label = @6
                local.get 0
                i32.load offset=4
                local.tee 1
                i32.const -4
                i32.and
                global.set 5
                global.get 7
                i32.eqz
                local.get 1
                i32.const 3
                i32.and
                i32.ne
                if  ;; label = @7
                  unreachable
                end
                local.get 0
                i32.const 34484
                i32.lt_u
                if  ;; label = @7
                  local.get 0
                  i32.const 0
                  i32.store offset=4
                  local.get 0
                  i32.const 0
                  i32.store offset=8
                else
                  global.get 0
                  local.get 0
                  i32.load
                  i32.const -4
                  i32.and
                  i32.const 4
                  i32.add
                  i32.sub
                  global.set 0
                  local.get 0
                  i32.const 4
                  i32.add
                  local.tee 1
                  i32.const 34484
                  i32.ge_u
                  if  ;; label = @8
                    global.get 9
                    i32.eqz
                    if  ;; label = @9
                      call $4
                    end
                    local.get 1
                    i32.const 4
                    i32.sub
                    local.set 0
                    local.get 1
                    i32.const 15
                    i32.and
                    i32.const 1
                    local.get 1
                    select
                    if (result i32)  ;; label = @9
                      i32.const 1
                    else
                      local.get 0
                      i32.load
                      i32.const 1
                      i32.and
                    end
                    if  ;; label = @9
                      unreachable
                    end
                    local.get 0
                    local.get 0
                    i32.load
                    i32.const 1
                    i32.or
                    i32.store
                    global.get 9
                    local.get 0
                    call $2
                  end
                end
                i32.const 10
                br 2 (;@4;)
              end
              global.get 6
              local.tee 1
              local.get 1
              i32.store offset=4
              local.get 1
              local.get 1
              i32.store offset=8
              i32.const 0
              global.set 2
            end
            i32.const 0
          end
          i32.sub
          local.set 0
          global.get 2
          i32.eqz
          if  ;; label = @4
            global.get 0
            i64.extend_i32_u
            i64.const 200
            i64.mul
            i64.const 100
            i64.div_u
            i32.wrap_i64
            i32.const 1024
            i32.add
            global.set 1
            br 2 (;@2;)
          end
          local.get 0
          i32.const 0
          i32.gt_s
          br_if 0 (;@3;)
        end
        global.get 0
        local.tee 1
        global.get 1
        i32.sub
        i32.const 1024
        i32.lt_u
        i32.const 10
        i32.shl
        local.get 1
        i32.add
        global.set 1
      end
    end
    global.get 9
    i32.eqz
    if  ;; label = @1
      call $4
    end
    local.get 5
    i32.const 16
    i32.add
    local.tee 1
    i32.const 1073741820
    i32.gt_u
    if  ;; label = @1
      unreachable
    end
    global.get 9
    local.tee 4
    i32.const 12
    local.get 1
    i32.const 19
    i32.add
    i32.const -16
    i32.and
    i32.const 4
    i32.sub
    local.get 1
    i32.const 12
    i32.le_u
    select
    local.tee 3
    call $6
    local.tee 0
    i32.eqz
    if  ;; label = @1
      memory.size
      local.tee 0
      i32.const 4
      local.get 4
      i32.load offset=1568
      local.get 0
      i32.const 16
      i32.shl
      i32.const 4
      i32.sub
      i32.ne
      i32.shl
      local.get 3
      i32.const 1
      i32.const 27
      local.get 3
      i32.clz
      i32.sub
      i32.shl
      i32.const 1
      i32.sub
      i32.add
      local.get 3
      local.get 3
      i32.const 536870910
      i32.lt_u
      select
      i32.add
      i32.const 65535
      i32.add
      i32.const -65536
      i32.and
      i32.const 16
      i32.shr_u
      local.tee 1
      local.get 0
      local.get 1
      i32.gt_s
      select
      memory.grow
      i32.const 0
      i32.lt_s
      if  ;; label = @2
        local.get 1
        memory.grow
        i32.const 0
        i32.lt_s
        if  ;; label = @3
          unreachable
        end
      end
      local.get 4
      local.get 0
      i32.const 16
      i32.shl
      memory.size
      i32.const 16
      i32.shl
      call $3
      local.get 4
      local.get 3
      call $6
      local.tee 0
      i32.eqz
      if  ;; label = @2
        unreachable
      end
    end
    local.get 3
    local.get 0
    i32.load
    i32.const -4
    i32.and
    i32.gt_u
    if  ;; label = @1
      unreachable
    end
    local.get 4
    local.get 0
    call $1
    local.get 0
    i32.load
    local.set 1
    local.get 3
    i32.const 4
    i32.add
    i32.const 15
    i32.and
    if  ;; label = @1
      unreachable
    end
    local.get 1
    i32.const -4
    i32.and
    local.get 3
    i32.sub
    local.tee 2
    i32.const 16
    i32.ge_u
    if  ;; label = @1
      local.get 0
      local.get 3
      local.get 1
      i32.const 2
      i32.and
      i32.or
      i32.store
      local.get 0
      i32.const 4
      i32.add
      local.get 3
      i32.add
      local.tee 1
      local.get 2
      i32.const 4
      i32.sub
      i32.const 1
      i32.or
      i32.store
      local.get 4
      local.get 1
      call $2
    else
      local.get 0
      local.get 1
      i32.const -2
      i32.and
      i32.store
      local.get 0
      i32.const 4
      i32.add
      local.get 0
      i32.load
      i32.const -4
      i32.and
      i32.add
      local.tee 1
      local.get 1
      i32.load
      i32.const -3
      i32.and
      i32.store
    end
    local.get 0
    i32.const 1
    i32.store offset=12
    local.get 0
    local.get 5
    i32.store offset=16
    global.get 8
    local.tee 1
    i32.load offset=8
    local.set 2
    local.get 0
    local.get 1
    global.get 7
    i32.or
    i32.store offset=4
    local.get 0
    local.get 2
    i32.store offset=8
    local.get 2
    local.get 0
    local.get 2
    i32.load offset=4
    i32.const 3
    i32.and
    i32.or
    i32.store offset=4
    local.get 1
    local.get 0
    i32.store offset=8
    global.get 0
    local.get 0
    i32.load
    i32.const -4
    i32.and
    i32.const 4
    i32.add
    i32.add
    global.set 0
    local.get 0
    i32.const 20
    i32.add
    local.tee 0
    i32.const 0
    local.get 5
    memory.fill
    local.get 0
    local.set 1
    local.get 0
    i32.store
    i32.const 1056
    local.set 4
    i32.const 1052
    i32.load
    i32.const -2
    i32.and
    i32.const 1056
    i32.add
    local.set 6
    loop  ;; label = @1
      local.get 4
      local.get 6
      i32.lt_u
      if  ;; label = @2
        local.get 4
        i32.load16_u
        local.tee 3
        i32.const 128
        i32.lt_u
        if (result i32)  ;; label = @3
          local.get 0
          local.get 3
          i32.store8
          local.get 0
          i32.const 1
          i32.add
        else
          local.get 3
          i32.const 2048
          i32.lt_u
          if (result i32)  ;; label = @4
            local.get 0
            local.get 3
            i32.const 6
            i32.shr_u
            i32.const 192
            i32.or
            local.get 3
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.const 8
            i32.shl
            i32.or
            i32.store16
            local.get 0
            i32.const 2
            i32.add
          else
            local.get 3
            i32.const 56320
            i32.lt_u
            local.get 4
            i32.const 2
            i32.add
            local.get 6
            i32.lt_u
            i32.and
            local.get 3
            i32.const 63488
            i32.and
            i32.const 55296
            i32.eq
            i32.and
            if  ;; label = @5
              local.get 4
              i32.load16_u offset=2
              local.tee 2
              i32.const 64512
              i32.and
              i32.const 56320
              i32.eq
              if  ;; label = @6
                local.get 0
                local.get 3
                i32.const 1023
                i32.and
                i32.const 10
                i32.shl
                i32.const 65536
                i32.add
                local.get 2
                i32.const 1023
                i32.and
                i32.or
                local.tee 2
                i32.const 63
                i32.and
                i32.const 128
                i32.or
                i32.const 24
                i32.shl
                local.get 2
                i32.const 6
                i32.shr_u
                i32.const 63
                i32.and
                i32.const 128
                i32.or
                i32.const 16
                i32.shl
                i32.or
                local.get 2
                i32.const 12
                i32.shr_u
                i32.const 63
                i32.and
                i32.const 128
                i32.or
                i32.const 8
                i32.shl
                i32.or
                local.get 2
                i32.const 18
                i32.shr_u
                i32.const 240
                i32.or
                i32.or
                i32.store
                local.get 0
                i32.const 4
                i32.add
                local.set 0
                local.get 4
                i32.const 4
                i32.add
                local.set 4
                br 5 (;@1;)
              end
            end
            local.get 0
            local.get 3
            i32.const 12
            i32.shr_u
            i32.const 224
            i32.or
            local.get 3
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.const 8
            i32.shl
            i32.or
            i32.store16
            local.get 0
            local.get 3
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=2
            local.get 0
            i32.const 3
            i32.add
          end
        end
        local.set 0
        local.get 4
        i32.const 2
        i32.add
        local.set 4
        br 1 (;@1;)
      end
    end
    global.get 11
    i32.const 4
    i32.add
    global.set 11
    local.get 1
    local.tee 0
    i32.store
    local.get 0
    i32.const 20
    i32.sub
    i32.load offset=16
    local.set 1
    i32.const 1600
    local.get 0
    i32.store
    i32.const 1604
    local.get 1
    i32.store
    i32.const 1632
    i32.const 10
    i32.store8
    i32.const 1608
    i32.const 1632
    i32.store
    i32.const 1612
    i32.const 1
    i32.store
    i32.const 1
    i32.const 1600
    i32.const 2
    i32.const 1648
    call $fimport$0
    drop
    global.get 11
    i32.const 4
    i32.add
    global.set 11
    global.get 11
    i32.const 4
    i32.add
    global.set 11
    global.get 11
    i32.const 4
    i32.add
    global.set 11)
  (func $10 (type 2) (param i32)
    (local i32 i32 i32)
    global.get 7
    local.get 0
    i32.const 20
    i32.sub
    local.tee 0
    i32.load offset=4
    i32.const 3
    i32.and
    i32.eq
    if  ;; label = @1
      local.get 0
      global.get 5
      i32.eq
      if  ;; label = @2
        local.get 0
        i32.load offset=8
        local.tee 1
        i32.eqz
        if  ;; label = @3
          unreachable
        end
        local.get 1
        global.set 5
      end
      block  ;; label = @2
        local.get 0
        i32.load offset=4
        i32.const -4
        i32.and
        local.tee 2
        i32.eqz
        if  ;; label = @3
          local.get 0
          i32.load offset=8
          i32.eqz
          local.get 0
          i32.const 34484
          i32.lt_u
          i32.and
          i32.eqz
          if  ;; label = @4
            unreachable
          end
          br 1 (;@2;)
        end
        local.get 0
        i32.load offset=8
        local.tee 1
        i32.eqz
        if  ;; label = @3
          unreachable
        end
        local.get 2
        local.get 1
        i32.store offset=8
        local.get 1
        local.get 2
        local.get 1
        i32.load offset=4
        i32.const 3
        i32.and
        i32.or
        i32.store offset=4
      end
      global.get 6
      local.tee 2
      i32.load offset=8
      local.set 1
      local.get 0
      global.get 7
      i32.eqz
      i32.const 2
      local.get 0
      i32.load offset=12
      local.tee 3
      i32.const 2
      i32.le_u
      if (result i32)  ;; label = @2
        i32.const 1
      else
        local.get 3
        i32.const 1696
        i32.load
        i32.gt_u
        if  ;; label = @3
          unreachable
        end
        local.get 3
        i32.const 2
        i32.shl
        i32.const 1700
        i32.add
        i32.load
        i32.const 32
        i32.and
      end
      select
      local.get 2
      i32.or
      i32.store offset=4
      local.get 0
      local.get 1
      i32.store offset=8
      local.get 1
      local.get 0
      local.get 1
      i32.load offset=4
      i32.const 3
      i32.and
      i32.or
      i32.store offset=4
      local.get 2
      local.get 0
      i32.store offset=8
      global.get 3
      i32.const 1
      i32.add
      global.set 3
    end)
  (memory (;0;) 1)
  (global (;0;) (mut i32) (i32.const 0))
  (global (;1;) (mut i32) (i32.const 0))
  (global (;2;) (mut i32) (i32.const 0))
  (global (;3;) (mut i32) (i32.const 0))
  (global (;4;) (mut i32) (i32.const 0))
  (global (;5;) (mut i32) (i32.const 0))
  (global (;6;) (mut i32) (i32.const 0))
  (global (;7;) (mut i32) (i32.const 0))
  (global (;8;) (mut i32) (i32.const 0))
  (global (;9;) (mut i32) (i32.const 0))
  (global (;10;) (mut i32) (i32.const 0))
  (global (;11;) (mut i32) (i32.const 34484))
  (export "memory" (memory 0))
  (start $9)
  (data (;0;) (i32.const 1036) ",")
  (data (;1;) (i32.const 1048) "\02\00\00\00\18\00\00\00H\00e\00l\00l\00o\00 \00W\00o\00r\00l\00d\00!")
  (data (;2;) (i32.const 1084) "<")
  (data (;3;) (i32.const 1096) "\02\00\00\00(\00\00\00A\00l\00l\00o\00c\00a\00t\00i\00o\00n\00 \00t\00o\00o\00 \00l\00a\00r\00g\00e")
  (data (;4;) (i32.const 1148) "<")
  (data (;5;) (i32.const 1160) "\02\00\00\00 \00\00\00~\00l\00i\00b\00/\00r\00t\00/\00i\00t\00c\00m\00s\00.\00t\00s")
  (data (;6;) (i32.const 1276) "<")
  (data (;7;) (i32.const 1288) "\02\00\00\00$\00\00\00I\00n\00d\00e\00x\00 \00o\00u\00t\00 \00o\00f\00 \00r\00a\00n\00g\00e")
  (data (;8;) (i32.const 1340) ",")
  (data (;9;) (i32.const 1352) "\02\00\00\00\14\00\00\00~\00l\00i\00b\00/\00r\00t\00.\00t\00s")
  (data (;10;) (i32.const 1420) "<")
  (data (;11;) (i32.const 1432) "\02\00\00\00\1e\00\00\00~\00l\00i\00b\00/\00r\00t\00/\00t\00l\00s\00f\00.\00t\00s")
  (data (;12;) (i32.const 1484) "<")
  (data (;13;) (i32.const 1496) "\02\00\00\00$\00\00\00U\00n\00p\00a\00i\00r\00e\00d\00 \00s\00u\00r\00r\00o\00g\00a\00t\00e")
  (data (;14;) (i32.const 1548) ",")
  (data (;15;) (i32.const 1560) "\02\00\00\00\1c\00\00\00~\00l\00i\00b\00/\00s\00t\00r\00i\00n\00g\00.\00t\00s")
  (data (;16;) (i32.const 1696) "\04\00\00\00 \00\00\00 \00\00\00 "))
