(module
  (type $t0 (func))
  (type $t1 (func (param i32)))
  (type $t2 (func (param i32 i32) (result i32)))
  (type $t3 (func (param i32 i32)))
  (type $t4 (func (param i32 i32 i32)))
  (type $t5 (func (param i32) (result i32)))
  (type $t6 (func (param i32 i32 i32) (result i32)))
  (type $t7 (func (result i32)))
  (type $t8 (func (param i32 i32 i32 i32 i32)))
  (type $t9 (func (param i32 i32 i32 i32) (result i32)))
  (type $t10 (func (param i32) (result i64)))
  (type $t11 (func (param i32 i32 i32 i32)))
  (type $t12 (func (param i32 i32 i32 i32 i32 i32) (result i32)))
  (type $t13 (func (param i64 i32 i32) (result i32)))
  (func $_ZN4core4hint9black_box17h50a3ebb8161ea701E (type $t0)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32)
    global.get $g0
    local.set $l0
    i32.const 16
    local.set $l1
    local.get $l0
    local.get $l1
    i32.sub
    local.set $l2
    i32.const 8
    local.set $l3
    local.get $l2
    local.get $l3
    i32.add
    local.set $l4
    local.get $l4
    local.set $l5
    return)
  (func $_ZN3std2rt10lang_start17h85d4dc5477f92666E (type $t6) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32) (local $l10 i32) (local $l11 i32)
    global.get $g0
    local.set $l3
    i32.const 16
    local.set $l4
    local.get $l3
    local.get $l4
    i32.sub
    local.set $l5
    local.get $l5
    global.set $g0
    i32.const 1048576
    local.set $l6
    local.get $l6
    local.set $l7
    local.get $l5
    local.set $l8
    local.get $l5
    local.get $p0
    i32.store offset=4
    local.get $l5
    local.get $p1
    i32.store offset=8
    local.get $l5
    local.get $p2
    i32.store offset=12
    local.get $l5
    local.get $p0
    i32.store
    local.get $l8
    local.get $l7
    local.get $p1
    local.get $p2
    call $_ZN3std2rt19lang_start_internal17h49e742537b17034cE
    local.set $l9
    i32.const 16
    local.set $l10
    local.get $l5
    local.get $l10
    i32.add
    local.set $l11
    local.get $l11
    global.set $g0
    local.get $l9
    return)
  (func $_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h99b8727af158158aE (type $t5) (param $p0 i32) (result i32)
    (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32)
    global.get $g0
    local.set $l1
    i32.const 16
    local.set $l2
    local.get $l1
    local.get $l2
    i32.sub
    local.set $l3
    local.get $l3
    global.set $g0
    local.get $l3
    local.get $p0
    i32.store offset=12
    local.get $p0
    i32.load
    local.set $l4
    local.get $l4
    call $_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hc2bc411c2142d8d5E
    call $_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h5876dae78213c9f6E
    local.set $l5
    i32.const 16
    local.set $l6
    local.get $l3
    local.get $l6
    i32.add
    local.set $l7
    local.get $l7
    global.set $g0
    local.get $l5
    return)
  (func $_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h7cb1f56a4edd3851E (type $t5) (param $p0 i32) (result i32)
    (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32)
    global.get $g0
    local.set $l1
    i32.const 16
    local.set $l2
    local.get $l1
    local.get $l2
    i32.sub
    local.set $l3
    local.get $l3
    global.set $g0
    local.get $l3
    local.get $p0
    i32.store offset=12
    local.get $p0
    i32.load
    local.set $l4
    local.get $l4
    call $_ZN4core3ops8function6FnOnce9call_once17h60c3519d11a9cc20E
    local.set $l5
    i32.const 16
    local.set $l6
    local.get $l3
    local.get $l6
    i32.add
    local.set $l7
    local.get $l7
    global.set $g0
    local.get $l5
    return)
  (func $_ZN4core3ops8function6FnOnce9call_once17h60c3519d11a9cc20E (type $t5) (param $p0 i32) (result i32)
    (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32)
    global.get $g0
    local.set $l1
    i32.const 16
    local.set $l2
    local.get $l1
    local.get $l2
    i32.sub
    local.set $l3
    local.get $l3
    global.set $g0
    i32.const 4
    local.set $l4
    local.get $l3
    local.get $l4
    i32.add
    local.set $l5
    local.get $l5
    local.set $l6
    local.get $l3
    local.get $p0
    i32.store offset=4
    local.get $l6
    call $_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h99b8727af158158aE
    local.set $l7
    i32.const 16
    local.set $l8
    local.get $l3
    local.get $l8
    i32.add
    local.set $l9
    local.get $l9
    global.set $g0
    local.get $l7
    return)
  (func $_ZN4core3ops8function6FnOnce9call_once17hf99cac6972f040cdE (type $t1) (param $p0 i32)
    (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32)
    global.get $g0
    local.set $l1
    i32.const 16
    local.set $l2
    local.get $l1
    local.get $l2
    i32.sub
    local.set $l3
    local.get $l3
    global.set $g0
    local.get $l3
    local.get $p0
    i32.store offset=12
    local.get $p0
    call_indirect (type $t0) $T0
    i32.const 16
    local.set $l4
    local.get $l3
    local.get $l4
    i32.add
    local.set $l5
    local.get $l5
    global.set $g0
    return)
  (func $_ZN4core3ptr13drop_in_place17hf76295713a90097aE (type $t1) (param $p0 i32)
    (local $l1 i32) (local $l2 i32) (local $l3 i32)
    global.get $g0
    local.set $l1
    i32.const 16
    local.set $l2
    local.get $l1
    local.get $l2
    i32.sub
    local.set $l3
    local.get $l3
    local.get $p0
    i32.store offset=12
    return)
  (func $_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h5876dae78213c9f6E (type $t7) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32)
    global.get $g0
    local.set $l0
    i32.const 16
    local.set $l1
    local.get $l0
    local.get $l1
    i32.sub
    local.set $l2
    local.get $l2
    global.set $g0
    i32.const 0
    local.set $l3
    i32.const 1
    local.set $l4
    local.get $l3
    local.get $l4
    i32.and
    local.set $l5
    local.get $l5
    call $_ZN68_$LT$std..process..ExitCode$u20$as$u20$std..process..Termination$GT$6report17h73c5b067124e6ee3E
    local.set $l6
    i32.const 16
    local.set $l7
    local.get $l2
    local.get $l7
    i32.add
    local.set $l8
    local.get $l8
    global.set $g0
    local.get $l6
    return)
  (func $_ZN68_$LT$std..process..ExitCode$u20$as$u20$std..process..Termination$GT$6report17h73c5b067124e6ee3E (type $t5) (param $p0 i32) (result i32)
    (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32) (local $l10 i32)
    global.get $g0
    local.set $l1
    i32.const 16
    local.set $l2
    local.get $l1
    local.get $l2
    i32.sub
    local.set $l3
    local.get $l3
    global.set $g0
    i32.const 15
    local.set $l4
    local.get $l3
    local.get $l4
    i32.add
    local.set $l5
    local.get $l5
    local.set $l6
    local.get $p0
    local.set $l7
    local.get $l3
    local.get $l7
    i32.store8 offset=15
    local.get $l6
    call $_ZN3std3sys4wasm7process8ExitCode6as_i3217h08794ec4286789ffE
    local.set $l8
    i32.const 16
    local.set $l9
    local.get $l3
    local.get $l9
    i32.add
    local.set $l10
    local.get $l10
    global.set $g0
    local.get $l8
    return)
  (func $_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hc2bc411c2142d8d5E (type $t1) (param $p0 i32)
    (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32)
    global.get $g0
    local.set $l1
    i32.const 32
    local.set $l2
    local.get $l1
    local.get $l2
    i32.sub
    local.set $l3
    local.get $l3
    global.set $g0
    local.get $l3
    local.get $p0
    i32.store offset=28
    local.get $p0
    call $_ZN4core3ops8function6FnOnce9call_once17hf99cac6972f040cdE
    call $_ZN4core4hint9black_box17h50a3ebb8161ea701E
    i32.const 32
    local.set $l4
    local.get $l3
    local.get $l4
    i32.add
    local.set $l5
    local.get $l5
    global.set $g0
    return)
  (func $_ZN4core3fmt9Arguments6new_v117h7b44d5d7fb9527b0E (type $t8) (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32) (param $p4 i32)
    (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32) (local $l10 i32)
    global.get $g0
    local.set $l5
    i32.const 32
    local.set $l6
    local.get $l5
    local.get $l6
    i32.sub
    local.set $l7
    i32.const 0
    local.set $l8
    local.get $l7
    local.get $p1
    i32.store offset=16
    local.get $l7
    local.get $p2
    i32.store offset=20
    local.get $l7
    local.get $p3
    i32.store offset=24
    local.get $l7
    local.get $p4
    i32.store offset=28
    local.get $l7
    local.get $l8
    i32.store offset=8
    local.get $p0
    local.get $p1
    i32.store
    local.get $p0
    local.get $p2
    i32.store offset=4
    local.get $l7
    i32.load offset=8
    local.set $l9
    local.get $l7
    i32.load offset=12
    local.set $l10
    local.get $p0
    local.get $l9
    i32.store offset=8
    local.get $p0
    local.get $l10
    i32.store offset=12
    local.get $p0
    local.get $p3
    i32.store offset=16
    local.get $p0
    local.get $p4
    i32.store offset=20
    return)
  (func $_ZN9rust_test4main17h5541390f7c4dfc2eE (type $t0)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32) (local $l10 i32) (local $l11 i32) (local $l12 i32) (local $l13 i32) (local $l14 i32) (local $l15 i32) (local $l16 i32)
    global.get $g0
    local.set $l0
    i32.const 32
    local.set $l1
    local.get $l0
    local.get $l1
    i32.sub
    local.set $l2
    local.get $l2
    global.set $g0
    i32.const 8
    local.set $l3
    local.get $l2
    local.get $l3
    i32.add
    local.set $l4
    local.get $l4
    local.set $l5
    i32.const 1
    local.set $l6
    i32.const 0
    local.set $l7
    i32.const 0
    local.set $l8
    local.get $l8
    i32.load offset=1048616
    local.set $l9
    i32.const 0
    local.set $l10
    local.get $l10
    i32.load offset=1048620
    local.set $l11
    local.get $l5
    local.get $l9
    local.get $l6
    local.get $l11
    local.get $l7
    call $_ZN4core3fmt9Arguments6new_v117h7b44d5d7fb9527b0E
    i32.const 8
    local.set $l12
    local.get $l2
    local.get $l12
    i32.add
    local.set $l13
    local.get $l13
    local.set $l14
    local.get $l14
    call $_ZN3std2io5stdio6_print17hf23893e4604d52b7E
    i32.const 32
    local.set $l15
    local.get $l2
    local.get $l15
    i32.add
    local.set $l16
    local.get $l16
    global.set $g0
    return)
  (func $main (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32) (local $l3 i32)
    i32.const 4
    local.set $l2
    local.get $l2
    local.get $p0
    local.get $p1
    call $_ZN3std2rt10lang_start17h85d4dc5477f92666E
    local.set $l3
    local.get $l3
    return)
  (func $__rust_alloc (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32)
    local.get $p0
    local.get $p1
    call $__rdl_alloc
    local.set $l2
    local.get $l2
    return)
  (func $__rust_dealloc (type $t4) (param $p0 i32) (param $p1 i32) (param $p2 i32)
    local.get $p0
    local.get $p1
    local.get $p2
    call $__rdl_dealloc
    return)
  (func $__rust_realloc (type $t9) (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32) (result i32)
    (local $l4 i32)
    local.get $p0
    local.get $p1
    local.get $p2
    local.get $p3
    call $__rdl_realloc
    local.set $l4
    local.get $l4
    return)
  (func $_ZN109_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..SpecExtend$LT$$RF$T$C$core..slice..Iter$LT$T$GT$$GT$$GT$11spec_extend17h43cd9ab83136c1f8E (type $t4) (param $p0 i32) (param $p1 i32) (param $p2 i32)
    (local $l3 i32) (local $l4 i32) (local $l5 i32)
    block $B0
      block $B1
        local.get $p0
        i32.const 4
        i32.add
        i32.load
        local.tee $l3
        local.get $p0
        i32.const 8
        i32.add
        i32.load
        local.tee $l4
        i32.sub
        local.get $p2
        local.get $p1
        i32.sub
        local.tee $p2
        i32.lt_u
        br_if $B1
        local.get $p0
        i32.load
        local.set $l3
        br $B0
      end
      block $B2
        block $B3
          block $B4
            local.get $l4
            local.get $p2
            i32.add
            local.tee $l5
            local.get $l4
            i32.lt_u
            br_if $B4
            local.get $l3
            i32.const 1
            i32.shl
            local.tee $l4
            local.get $l5
            local.get $l4
            local.get $l5
            i32.gt_u
            select
            local.tee $l4
            i32.const 8
            local.get $l4
            i32.const 8
            i32.gt_u
            select
            local.set $l4
            block $B5
              local.get $l3
              i32.eqz
              br_if $B5
              local.get $l4
              i32.const 0
              i32.lt_s
              br_if $B4
              local.get $p0
              i32.load
              local.tee $l5
              i32.eqz
              br_if $B3
              local.get $l5
              local.get $l3
              i32.const 1
              local.get $l4
              call $__rust_realloc
              local.set $l3
              br $B2
            end
            local.get $l4
            i32.const 0
            i32.ge_s
            br_if $B3
          end
          call $_ZN5alloc7raw_vec17capacity_overflow17heb1d9eef88f15a21E
          unreachable
        end
        local.get $l4
        i32.const 1
        call $__rust_alloc
        local.set $l3
      end
      block $B6
        local.get $l3
        i32.eqz
        br_if $B6
        local.get $p0
        local.get $l3
        i32.store
        local.get $p0
        i32.const 4
        i32.add
        local.get $l4
        i32.store
        local.get $p0
        i32.const 8
        i32.add
        i32.load
        local.set $l4
        br $B0
      end
      local.get $l4
      i32.const 1
      call $_ZN5alloc5alloc18handle_alloc_error17h02613b455f15ef67E
      unreachable
    end
    local.get $l3
    local.get $l4
    i32.add
    local.get $p1
    local.get $p2
    call $memcpy
    drop
    local.get $p0
    i32.const 8
    i32.add
    local.tee $p0
    local.get $p0
    i32.load
    local.get $p2
    i32.add
    i32.store)
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h3ad6b7dc2f5cec71E (type $t10) (param $p0 i32) (result i64)
    i64.const 5966890128770411197)
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hac9d42c1070e325fE (type $t10) (param $p0 i32) (result i64)
    i64.const 9147559743429524724)
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hc4ed596920ed0f82E (type $t10) (param $p0 i32) (result i64)
    i64.const 6731278161113390719)
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h3525f3f9a0adc297E (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    i32.load
    local.set $p0
    block $B0
      local.get $p1
      call $_ZN4core3fmt9Formatter15debug_lower_hex17he16ae5aeaad8d5abE
      br_if $B0
      block $B1
        local.get $p1
        call $_ZN4core3fmt9Formatter15debug_upper_hex17h8b72eec9a9ee7d24E
        br_if $B1
        local.get $p0
        local.get $p1
        call $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17hf76888becbde89b4E
        return
      end
      local.get $p0
      local.get $p1
      call $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17h2c02422bfe9eb594E
      return
    end
    local.get $p0
    local.get $p1
    call $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17h7dfebd7501684a06E)
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h6b90d67ef72b6162E (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    i32.load
    local.set $p0
    block $B0
      local.get $p1
      call $_ZN4core3fmt9Formatter15debug_lower_hex17he16ae5aeaad8d5abE
      br_if $B0
      block $B1
        local.get $p1
        call $_ZN4core3fmt9Formatter15debug_upper_hex17h8b72eec9a9ee7d24E
        br_if $B1
        local.get $p0
        local.get $p1
        call $_ZN4core3fmt3num3imp51_$LT$impl$u20$core..fmt..Display$u20$for$u20$u8$GT$3fmt17h98c236a29d0072e5E
        return
      end
      local.get $p0
      local.get $p1
      call $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i8$GT$3fmt17haa011cd9b81643deE
      return
    end
    local.get $p0
    local.get $p1
    call $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i8$GT$3fmt17h74ea3e673a2ac4f8E)
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17haddd63b3d1b18b68E (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32) (local $l3 i32)
    global.get $g0
    i32.const 16
    i32.sub
    local.tee $l2
    global.set $g0
    local.get $p0
    i32.load
    local.tee $p0
    i32.load offset=8
    local.set $l3
    local.get $p0
    i32.load
    local.set $p0
    local.get $l2
    local.get $p1
    call $_ZN4core3fmt9Formatter10debug_list17h4df433c222cafce6E
    block $B0
      local.get $l3
      i32.eqz
      br_if $B0
      loop $L1
        local.get $l2
        local.get $p0
        i32.store offset=12
        local.get $l2
        local.get $l2
        i32.const 12
        i32.add
        i32.const 1048752
        call $_ZN4core3fmt8builders8DebugSet5entry17h4da9ac0fd443c627E
        drop
        local.get $p0
        i32.const 1
        i32.add
        local.set $p0
        local.get $l3
        i32.const -1
        i32.add
        local.tee $l3
        br_if $L1
      end
    end
    local.get $l2
    call $_ZN4core3fmt8builders9DebugList6finish17h15497983fc988cedE
    local.set $p0
    local.get $l2
    i32.const 16
    i32.add
    global.set $g0
    local.get $p0)
  (func $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h5d7b0744a6e3b82fE (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    i32.load
    local.get $p0
    i32.load offset=4
    local.get $p1
    call $_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hd7770bbf948948ffE)
  (func $_ZN4core3fmt5Write10write_char17h0246824b0281d4ecE (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32) (local $l3 i32) (local $l4 i64) (local $l5 i32)
    global.get $g0
    i32.const 16
    i32.sub
    local.tee $l2
    global.set $g0
    local.get $l2
    i32.const 0
    i32.store offset=4
    block $B0
      block $B1
        block $B2
          block $B3
            local.get $p1
            i32.const 128
            i32.lt_u
            br_if $B3
            local.get $p1
            i32.const 2048
            i32.lt_u
            br_if $B2
            local.get $l2
            i32.const 4
            i32.add
            local.set $l3
            local.get $p1
            i32.const 65536
            i32.ge_u
            br_if $B1
            local.get $l2
            local.get $p1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=6
            local.get $l2
            local.get $p1
            i32.const 12
            i32.shr_u
            i32.const 224
            i32.or
            i32.store8 offset=4
            local.get $l2
            local.get $p1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=5
            i32.const 3
            local.set $p1
            br $B0
          end
          local.get $l2
          local.get $p1
          i32.store8 offset=4
          local.get $l2
          i32.const 4
          i32.add
          local.set $l3
          i32.const 1
          local.set $p1
          br $B0
        end
        local.get $l2
        local.get $p1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=5
        local.get $l2
        local.get $p1
        i32.const 6
        i32.shr_u
        i32.const 192
        i32.or
        i32.store8 offset=4
        local.get $l2
        i32.const 4
        i32.add
        local.set $l3
        i32.const 2
        local.set $p1
        br $B0
      end
      local.get $l2
      local.get $p1
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=7
      local.get $l2
      local.get $p1
      i32.const 18
      i32.shr_u
      i32.const 240
      i32.or
      i32.store8 offset=4
      local.get $l2
      local.get $p1
      i32.const 6
      i32.shr_u
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=6
      local.get $l2
      local.get $p1
      i32.const 12
      i32.shr_u
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=5
      i32.const 4
      local.set $p1
    end
    local.get $l2
    i32.const 8
    i32.add
    local.get $p0
    i32.load
    local.get $l3
    local.get $p1
    call $_ZN61_$LT$std..io..stdio..StdoutLock$u20$as$u20$std..io..Write$GT$9write_all17hd421315622c6dec2E
    i32.const 0
    local.set $p1
    block $B4
      local.get $l2
      i32.load8_u offset=8
      i32.const 3
      i32.eq
      br_if $B4
      local.get $l2
      i64.load offset=8
      local.set $l4
      block $B5
        block $B6
          i32.const 0
          br_if $B6
          local.get $p0
          i32.load8_u offset=4
          i32.const 2
          i32.ne
          br_if $B5
        end
        local.get $p0
        i32.const 8
        i32.add
        i32.load
        local.tee $p1
        i32.load
        local.get $p1
        i32.load offset=4
        i32.load
        call_indirect (type $t1) $T0
        block $B7
          local.get $p1
          i32.load offset=4
          local.tee $l3
          i32.load offset=4
          local.tee $l5
          i32.eqz
          br_if $B7
          local.get $p1
          i32.load
          local.get $l5
          local.get $l3
          i32.load offset=8
          call $__rust_dealloc
        end
        local.get $p0
        i32.load offset=8
        i32.const 12
        i32.const 4
        call $__rust_dealloc
      end
      local.get $p0
      local.get $l4
      i64.store offset=4 align=4
      i32.const 1
      local.set $p1
    end
    local.get $l2
    i32.const 16
    i32.add
    global.set $g0
    local.get $p1)
  (func $_ZN61_$LT$std..io..stdio..StdoutLock$u20$as$u20$std..io..Write$GT$9write_all17hd421315622c6dec2E (type $t11) (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32)
    (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32)
    global.get $g0
    i32.const 16
    i32.sub
    local.tee $l4
    global.set $g0
    block $B0
      block $B1
        block $B2
          block $B3
            block $B4
              block $B5
                block $B6
                  block $B7
                    local.get $p1
                    i32.load
                    local.tee $p1
                    i32.load
                    br_if $B7
                    local.get $p1
                    i32.const -1
                    i32.store
                    local.get $l4
                    i32.const 10
                    local.get $p2
                    local.get $p3
                    call $_ZN4core5slice6memchr7memrchr17hf1e7486ad49e8b64E
                    local.get $p1
                    i32.const 4
                    i32.add
                    local.set $l5
                    block $B8
                      block $B9
                        block $B10
                          block $B11
                            block $B12
                              block $B13
                                local.get $l4
                                i32.load
                                i32.eqz
                                br_if $B13
                                local.get $l4
                                i32.load offset=4
                                i32.const 1
                                i32.add
                                local.tee $l6
                                local.get $p3
                                i32.gt_u
                                br_if $B6
                                local.get $p2
                                local.get $l6
                                i32.add
                                local.set $l7
                                local.get $p1
                                i32.const 12
                                i32.add
                                i32.load
                                local.tee $l8
                                i32.eqz
                                br_if $B12
                                block $B14
                                  local.get $l8
                                  local.get $l6
                                  i32.add
                                  local.get $p1
                                  i32.const 8
                                  i32.add
                                  i32.load
                                  local.tee $l9
                                  i32.le_u
                                  br_if $B14
                                  local.get $p1
                                  i32.const 16
                                  i32.add
                                  i32.load8_u
                                  i32.const 1
                                  i32.ne
                                  br_if $B2
                                  i32.const 0
                                  local.set $l8
                                  local.get $p1
                                  i32.const 0
                                  i32.store8 offset=17
                                  local.get $p1
                                  i32.const 12
                                  i32.add
                                  i32.const 0
                                  i32.store
                                end
                                local.get $l9
                                local.get $l6
                                i32.gt_u
                                br_if $B11
                                local.get $p1
                                i32.const 1
                                i32.store8 offset=17
                                local.get $p1
                                i32.const 16
                                i32.add
                                i32.load8_u
                                i32.const 1
                                i32.ne
                                br_if $B1
                                local.get $l5
                                i32.const 0
                                i32.store8 offset=13
                                br $B10
                              end
                              block $B15
                                block $B16
                                  local.get $p1
                                  i32.const 12
                                  i32.add
                                  i32.load
                                  local.tee $l6
                                  br_if $B16
                                  i32.const 0
                                  local.set $l6
                                  br $B15
                                end
                                local.get $l6
                                local.get $l5
                                i32.load
                                i32.add
                                i32.const -1
                                i32.add
                                i32.load8_u
                                i32.const 10
                                i32.ne
                                br_if $B15
                                local.get $p1
                                i32.const 16
                                i32.add
                                i32.load8_u
                                i32.const 1
                                i32.ne
                                br_if $B5
                                i32.const 0
                                local.set $l6
                                local.get $p1
                                i32.const 0
                                i32.store8 offset=17
                                local.get $p1
                                i32.const 12
                                i32.add
                                i32.const 0
                                i32.store
                              end
                              block $B17
                                local.get $l6
                                local.get $p3
                                i32.add
                                local.get $p1
                                i32.const 8
                                i32.add
                                i32.load
                                local.tee $l8
                                i32.le_u
                                br_if $B17
                                local.get $p1
                                i32.const 16
                                i32.add
                                i32.load8_u
                                i32.const 1
                                i32.ne
                                br_if $B4
                                local.get $l6
                                i32.eqz
                                br_if $B17
                                local.get $p1
                                i32.const 0
                                i32.store8 offset=17
                                local.get $p1
                                i32.const 12
                                i32.add
                                i32.const 0
                                i32.store
                              end
                              block $B18
                                local.get $l8
                                local.get $p3
                                i32.le_u
                                br_if $B18
                                local.get $l5
                                local.get $p2
                                local.get $p2
                                local.get $p3
                                i32.add
                                call $_ZN109_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..SpecExtend$LT$$RF$T$C$core..slice..Iter$LT$T$GT$$GT$$GT$11spec_extend17h43cd9ab83136c1f8E
                                local.get $p0
                                i32.const 3
                                i32.store8
                                br $B8
                              end
                              local.get $p1
                              i32.const 1
                              i32.store8 offset=17
                              local.get $p1
                              i32.const 16
                              i32.add
                              i32.load8_u
                              i32.const 1
                              i32.ne
                              br_if $B3
                              local.get $p0
                              i64.const 3
                              i64.store align=4
                              local.get $l5
                              i32.const 0
                              i32.store8 offset=13
                              br $B8
                            end
                            local.get $p1
                            i32.const 16
                            i32.add
                            i32.load8_u
                            i32.const 1
                            i32.eq
                            br_if $B9
                            i32.const 1048960
                            i32.const 43
                            i32.const 1049604
                            call $_ZN4core9panicking5panic17hc886a4cb4479b06eE
                            unreachable
                          end
                          local.get $l5
                          local.get $p2
                          local.get $l7
                          call $_ZN109_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..SpecExtend$LT$$RF$T$C$core..slice..Iter$LT$T$GT$$GT$$GT$11spec_extend17h43cd9ab83136c1f8E
                          local.get $p1
                          i32.const 16
                          i32.add
                          i32.load8_u
                          i32.const 1
                          i32.ne
                          br_if $B0
                          local.get $p1
                          i32.const 12
                          i32.add
                          i32.load
                          local.set $l8
                        end
                        local.get $l8
                        i32.eqz
                        br_if $B9
                        local.get $p1
                        i32.const 0
                        i32.store8 offset=17
                        local.get $p1
                        i32.const 12
                        i32.add
                        i32.const 0
                        i32.store
                      end
                      block $B19
                        local.get $p1
                        i32.const 8
                        i32.add
                        i32.load
                        local.get $p3
                        local.get $l6
                        i32.sub
                        i32.le_u
                        br_if $B19
                        local.get $l5
                        local.get $l7
                        local.get $p2
                        local.get $p3
                        i32.add
                        call $_ZN109_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..SpecExtend$LT$$RF$T$C$core..slice..Iter$LT$T$GT$$GT$$GT$11spec_extend17h43cd9ab83136c1f8E
                        local.get $p0
                        i32.const 3
                        i32.store8
                        br $B8
                      end
                      local.get $p0
                      i64.const 3
                      i64.store align=4
                      local.get $l5
                      i32.const 0
                      i32.store8 offset=13
                    end
                    local.get $p1
                    local.get $p1
                    i32.load
                    i32.const 1
                    i32.add
                    i32.store
                    local.get $l4
                    i32.const 16
                    i32.add
                    global.set $g0
                    return
                  end
                  i32.const 1048768
                  i32.const 16
                  local.get $l4
                  i32.const 8
                  i32.add
                  i32.const 1049004
                  i32.const 1050044
                  call $_ZN4core6option18expect_none_failed17h6878b94853f7ecf6E
                  unreachable
                end
                i32.const 1048908
                i32.const 35
                i32.const 1048944
                call $_ZN4core9panicking5panic17hc886a4cb4479b06eE
                unreachable
              end
              i32.const 1048960
              i32.const 43
              i32.const 1049588
              call $_ZN4core9panicking5panic17hc886a4cb4479b06eE
              unreachable
            end
            i32.const 1048960
            i32.const 43
            i32.const 1049588
            call $_ZN4core9panicking5panic17hc886a4cb4479b06eE
            unreachable
          end
          i32.const 1048960
          i32.const 43
          i32.const 1049604
          call $_ZN4core9panicking5panic17hc886a4cb4479b06eE
          unreachable
        end
        i32.const 1048960
        i32.const 43
        i32.const 1049588
        call $_ZN4core9panicking5panic17hc886a4cb4479b06eE
        unreachable
      end
      i32.const 1048960
      i32.const 43
      i32.const 1049604
      call $_ZN4core9panicking5panic17hc886a4cb4479b06eE
      unreachable
    end
    i32.const 1048960
    i32.const 43
    i32.const 1049588
    call $_ZN4core9panicking5panic17hc886a4cb4479b06eE
    unreachable)
  (func $_ZN4core3fmt5Write9write_fmt17h2f805ac767637cb6E (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32)
    global.get $g0
    i32.const 32
    i32.sub
    local.tee $l2
    global.set $g0
    local.get $l2
    local.get $p0
    i32.store offset=4
    local.get $l2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get $p1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get $l2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get $p1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get $l2
    local.get $p1
    i64.load align=4
    i64.store offset=8
    local.get $l2
    i32.const 4
    i32.add
    i32.const 1048704
    local.get $l2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17hb395f946a5ce2cabE
    local.set $p1
    local.get $l2
    i32.const 32
    i32.add
    global.set $g0
    local.get $p1)
  (func $_ZN3std9panicking11begin_panic17h90326787ac4041daE (type $t4) (param $p0 i32) (param $p1 i32) (param $p2 i32)
    (local $l3 i32)
    global.get $g0
    i32.const 16
    i32.sub
    local.tee $l3
    global.set $g0
    local.get $l3
    local.get $p2
    call $_ZN4core5panic8Location6caller17hbeb99f2804420dffE
    i32.store offset=8
    local.get $l3
    local.get $p1
    i32.store offset=4
    local.get $l3
    local.get $p0
    i32.store
    local.get $l3
    call $_ZN3std10sys_common9backtrace26__rust_end_short_backtrace17hdeeb31f429323cb3E
    unreachable)
  (func $_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h1bfafad7a2b0fca0E (type $t1) (param $p0 i32)
    (local $l1 i32)
    block $B0
      local.get $p0
      i32.load
      local.tee $p0
      i32.load8_u offset=4
      br_if $B0
      local.get $p0
      i32.const 0
      i32.store8 offset=4
      local.get $p0
      i32.load
      local.set $l1
      local.get $p0
      i32.const 1
      i32.store
      local.get $l1
      i32.load
      local.tee $p0
      local.get $p0
      i32.load
      local.tee $p0
      i32.const -1
      i32.add
      i32.store
      block $B1
        local.get $p0
        i32.const 1
        i32.ne
        br_if $B1
        local.get $l1
        call $_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17h497a468b6f1c6d8eE
      end
      local.get $l1
      i32.const 4
      i32.const 4
      call $__rust_dealloc
      return
    end
    i32.const 1051108
    i32.const 32
    i32.const 1051188
    call $_ZN3std9panicking11begin_panic17h90326787ac4041daE
    unreachable)
  (func $_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17h497a468b6f1c6d8eE (type $t1) (param $p0 i32)
    (local $l1 i32) (local $l2 i32)
    block $B0
      block $B1
        local.get $p0
        i32.load
        local.tee $l1
        i32.load8_u offset=24
        local.tee $l2
        i32.eqz
        br_if $B1
        local.get $l1
        i32.const 25
        i32.add
        i32.load8_u
        i32.const 255
        i32.and
        br_if $B1
        local.get $l2
        i32.const 1
        i32.ne
        br_if $B0
        local.get $l1
        i32.const 20
        i32.add
        local.tee $l2
        i32.load
        i32.eqz
        br_if $B1
        local.get $l1
        i32.const 0
        i32.store8 offset=25
        local.get $l2
        i32.const 0
        i32.store
      end
      block $B2
        local.get $l1
        i32.load offset=12
        local.tee $l2
        i32.eqz
        br_if $B2
        local.get $l1
        i32.const 16
        i32.add
        i32.load
        local.tee $l1
        i32.eqz
        br_if $B2
        local.get $l2
        local.get $l1
        i32.const 1
        call $__rust_dealloc
      end
      block $B3
        local.get $p0
        i32.load
        local.tee $l1
        i32.const -1
        i32.eq
        br_if $B3
        local.get $l1
        local.get $l1
        i32.load offset=4
        local.tee $p0
        i32.const -1
        i32.add
        i32.store offset=4
        local.get $p0
        i32.const 1
        i32.ne
        br_if $B3
        local.get $l1
        i32.const 28
        i32.const 4
        call $__rust_dealloc
      end
      return
    end
    i32.const 1048960
    i32.const 43
    i32.const 1049588
    call $_ZN4core9panicking5panic17hc886a4cb4479b06eE
    unreachable)
  (func $_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h9e4f16104979e5cbE (type $t3) (param $p0 i32) (param $p1 i32)
    (local $l2 i32)
    global.get $g0
    i32.const 16
    i32.sub
    local.tee $l2
    global.set $g0
    local.get $l2
    local.get $p0
    i32.load
    i32.store offset=12
    local.get $l2
    i32.const 12
    i32.add
    local.get $p1
    call $_ZN3std4sync4once4Once9call_once28_$u7b$$u7b$closure$u7d$$u7d$17h34d413a2f6944e79E
    local.get $l2
    i32.const 16
    i32.add
    global.set $g0)
  (func $_ZN3std4sync4once4Once9call_once28_$u7b$$u7b$closure$u7d$$u7d$17h34d413a2f6944e79E (type $t3) (param $p0 i32) (param $p1 i32)
    (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32) (local $l10 i32)
    local.get $p0
    i32.load
    local.tee $p0
    i32.load8_u
    local.set $l2
    local.get $p0
    i32.const 0
    i32.store8
    block $B0
      block $B1
        local.get $l2
        i32.const 1
        i32.and
        i32.eqz
        br_if $B1
        i32.const 1
        local.set $l3
        loop $L2
          i32.const 0
          i32.load8_u offset=1055297
          local.set $p0
          block $B3
            block $B4
              block $B5
                local.get $l3
                i32.const 9
                i32.gt_u
                br_if $B5
                local.get $p0
                i32.const 1
                i32.and
                i32.eqz
                br_if $B4
                br $B0
              end
              i32.const 1
              local.set $l4
              local.get $p0
              i32.const 1
              i32.and
              br_if $B0
              i32.const 10
              local.set $l3
              br $B3
            end
            local.get $l3
            i32.const 1
            i32.add
            local.set $l3
            i32.const 0
            local.set $l4
          end
          i32.const 0
          i32.load offset=1054776
          local.set $l5
          i32.const 0
          local.get $l4
          i32.store offset=1054776
          i32.const 0
          i32.const 0
          i32.store8 offset=1055297
          block $B6
            block $B7
              block $B8
                local.get $l5
                br_table $B6 $B8 $B7
              end
              i32.const 1050532
              i32.const 31
              i32.const 1050604
              call $_ZN3std9panicking11begin_panic17h90326787ac4041daE
              unreachable
            end
            local.get $l5
            i32.load
            local.tee $l6
            local.get $l5
            i32.load offset=8
            local.tee $l2
            i32.const 3
            i32.shl
            i32.add
            local.set $l7
            local.get $l5
            i32.load offset=4
            local.set $l8
            local.get $l6
            local.set $p0
            block $B9
              block $B10
                local.get $l2
                i32.eqz
                br_if $B10
                local.get $l6
                local.set $p0
                loop $L11
                  block $B12
                    local.get $p0
                    i32.load
                    local.tee $l2
                    br_if $B12
                    local.get $p0
                    i32.const 8
                    i32.add
                    local.set $p0
                    br $B10
                  end
                  local.get $l2
                  local.get $p0
                  i32.const 4
                  i32.add
                  i32.load
                  local.tee $l9
                  i32.load offset=12
                  call_indirect (type $t1) $T0
                  block $B13
                    local.get $l9
                    i32.load offset=4
                    local.tee $l10
                    i32.eqz
                    br_if $B13
                    local.get $l2
                    local.get $l10
                    local.get $l9
                    i32.load offset=8
                    call $__rust_dealloc
                  end
                  local.get $p0
                  i32.const 8
                  i32.add
                  local.tee $p0
                  local.get $l7
                  i32.ne
                  br_if $L11
                  br $B9
                end
              end
              local.get $l7
              local.get $p0
              i32.eq
              br_if $B9
              loop $L14
                local.get $p0
                i32.load
                local.get $p0
                i32.const 4
                i32.add
                local.tee $l2
                i32.load
                i32.load
                call_indirect (type $t1) $T0
                block $B15
                  local.get $l2
                  i32.load
                  local.tee $l2
                  i32.load offset=4
                  local.tee $l9
                  i32.eqz
                  br_if $B15
                  local.get $p0
                  i32.load
                  local.get $l9
                  local.get $l2
                  i32.load offset=8
                  call $__rust_dealloc
                end
                local.get $p0
                i32.const 8
                i32.add
                local.tee $p0
                local.get $l7
                i32.ne
                br_if $L14
              end
            end
            block $B16
              local.get $l8
              i32.eqz
              br_if $B16
              local.get $l8
              i32.const 3
              i32.shl
              local.tee $p0
              i32.eqz
              br_if $B16
              local.get $l6
              local.get $p0
              i32.const 4
              call $__rust_dealloc
            end
            local.get $l5
            i32.const 12
            i32.const 4
            call $__rust_dealloc
          end
          local.get $l3
          i32.const 11
          i32.lt_u
          local.get $l4
          i32.const 1
          i32.xor
          i32.and
          br_if $L2
        end
        return
      end
      i32.const 1048960
      i32.const 43
      i32.const 1050348
      call $_ZN4core9panicking5panic17hc886a4cb4479b06eE
      unreachable
    end
    i32.const 1051108
    i32.const 32
    i32.const 1051188
    call $_ZN3std9panicking11begin_panic17h90326787ac4041daE
    unreachable)
  (func $_ZN4core3ptr13drop_in_place17h068db193d06fd1fbE (type $t1) (param $p0 i32))
  (func $_ZN4core3ptr13drop_in_place17h07bb3761885668acE (type $t1) (param $p0 i32)
    (local $l1 i32)
    block $B0
      local.get $p0
      i32.load8_u offset=4
      br_if $B0
      i32.const 0
      i32.load offset=1054800
      i32.eqz
      br_if $B0
      local.get $p0
      i32.load
      local.set $l1
      call $_ZN3std9panicking11panic_count17is_zero_slow_path17hf6ba0dc1fa3c5010E
      br_if $B0
      local.get $l1
      i32.const 1
      i32.store8 offset=4
    end
    local.get $p0
    i32.load
    i32.load
    i32.const 0
    i32.store8)
  (func $_ZN3std9panicking11panic_count17is_zero_slow_path17hf6ba0dc1fa3c5010E (type $t7) (result i32)
    block $B0
      i32.const 0
      i32.load offset=1055288
      i32.const 1
      i32.ne
      br_if $B0
      i32.const 0
      i32.load offset=1055292
      i32.eqz
      return
    end
    i32.const 0
    i64.const 1
    i64.store offset=1055288
    i32.const 1)
  (func $_ZN4core3ptr13drop_in_place17h198d7b3663eb2020E (type $t1) (param $p0 i32)
    (local $l1 i32)
    block $B0
      local.get $p0
      i32.load
      local.tee $l1
      i32.eqz
      br_if $B0
      local.get $p0
      i32.const 4
      i32.add
      i32.load
      local.tee $p0
      i32.eqz
      br_if $B0
      local.get $l1
      local.get $p0
      i32.const 1
      call $__rust_dealloc
    end)
  (func $_ZN4core3ptr13drop_in_place17h33cb88e18f1743e1E (type $t1) (param $p0 i32)
    (local $l1 i32) (local $l2 i32) (local $l3 i32)
    block $B0
      block $B1
        i32.const 0
        br_if $B1
        local.get $p0
        i32.load8_u offset=4
        i32.const 2
        i32.ne
        br_if $B0
      end
      local.get $p0
      i32.const 8
      i32.add
      i32.load
      local.tee $l1
      i32.load
      local.get $l1
      i32.load offset=4
      i32.load
      call_indirect (type $t1) $T0
      block $B2
        local.get $l1
        i32.load offset=4
        local.tee $l2
        i32.load offset=4
        local.tee $l3
        i32.eqz
        br_if $B2
        local.get $l1
        i32.load
        local.get $l3
        local.get $l2
        i32.load offset=8
        call $__rust_dealloc
      end
      local.get $p0
      i32.load offset=8
      i32.const 12
      i32.const 4
      call $__rust_dealloc
    end)
  (func $_ZN4core3ptr13drop_in_place17h8fa49d8f224e9668E (type $t1) (param $p0 i32)
    (local $l1 i32)
    block $B0
      local.get $p0
      i32.load offset=4
      local.tee $l1
      i32.eqz
      br_if $B0
      local.get $p0
      i32.const 8
      i32.add
      i32.load
      local.tee $p0
      i32.eqz
      br_if $B0
      local.get $l1
      local.get $p0
      i32.const 1
      call $__rust_dealloc
    end)
  (func $_ZN4core3ptr13drop_in_place17hf58877e2af32fc2aE (type $t1) (param $p0 i32)
    (local $l1 i32)
    block $B0
      local.get $p0
      i32.load offset=4
      local.tee $l1
      i32.eqz
      br_if $B0
      local.get $p0
      i32.const 8
      i32.add
      i32.load
      local.tee $p0
      i32.eqz
      br_if $B0
      local.get $l1
      local.get $p0
      i32.const 1
      call $__rust_dealloc
    end)
  (func $_ZN4core6option15Option$LT$T$GT$6unwrap17h6c4ab1581d7c7209E (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    block $B0
      local.get $p0
      br_if $B0
      i32.const 1048960
      i32.const 43
      local.get $p1
      call $_ZN4core9panicking5panic17hc886a4cb4479b06eE
      unreachable
    end
    local.get $p0)
  (func $_ZN4core6option15Option$LT$T$GT$6unwrap17hb5ee9bfe6199eb07E (type $t5) (param $p0 i32) (result i32)
    block $B0
      local.get $p0
      br_if $B0
      i32.const 1048960
      i32.const 43
      i32.const 1050884
      call $_ZN4core9panicking5panic17hc886a4cb4479b06eE
      unreachable
    end
    local.get $p0)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h2566d3d077c1c374E (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    i32.load
    local.get $p1
    call $_ZN4core3fmt5Write10write_char17h0246824b0281d4ecE)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h37953ac5dd826edfE (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32)
    global.get $g0
    i32.const 16
    i32.sub
    local.tee $l2
    global.set $g0
    local.get $p0
    i32.load
    local.set $p0
    block $B0
      block $B1
        block $B2
          block $B3
            block $B4
              block $B5
                block $B6
                  local.get $p1
                  i32.const 128
                  i32.lt_u
                  br_if $B6
                  local.get $l2
                  i32.const 0
                  i32.store offset=12
                  local.get $p1
                  i32.const 2048
                  i32.lt_u
                  br_if $B5
                  local.get $l2
                  i32.const 12
                  i32.add
                  local.set $l3
                  block $B7
                    local.get $p1
                    i32.const 65536
                    i32.ge_u
                    br_if $B7
                    local.get $l2
                    local.get $p1
                    i32.const 63
                    i32.and
                    i32.const 128
                    i32.or
                    i32.store8 offset=14
                    local.get $l2
                    local.get $p1
                    i32.const 12
                    i32.shr_u
                    i32.const 224
                    i32.or
                    i32.store8 offset=12
                    local.get $l2
                    local.get $p1
                    i32.const 6
                    i32.shr_u
                    i32.const 63
                    i32.and
                    i32.const 128
                    i32.or
                    i32.store8 offset=13
                    i32.const 3
                    local.set $p1
                    br $B1
                  end
                  local.get $l2
                  local.get $p1
                  i32.const 63
                  i32.and
                  i32.const 128
                  i32.or
                  i32.store8 offset=15
                  local.get $l2
                  local.get $p1
                  i32.const 18
                  i32.shr_u
                  i32.const 240
                  i32.or
                  i32.store8 offset=12
                  local.get $l2
                  local.get $p1
                  i32.const 6
                  i32.shr_u
                  i32.const 63
                  i32.and
                  i32.const 128
                  i32.or
                  i32.store8 offset=14
                  local.get $l2
                  local.get $p1
                  i32.const 12
                  i32.shr_u
                  i32.const 63
                  i32.and
                  i32.const 128
                  i32.or
                  i32.store8 offset=13
                  i32.const 4
                  local.set $p1
                  br $B1
                end
                block $B8
                  local.get $p0
                  i32.load offset=8
                  local.tee $l3
                  local.get $p0
                  i32.const 4
                  i32.add
                  i32.load
                  i32.eq
                  br_if $B8
                  local.get $p0
                  i32.load
                  local.set $l4
                  br $B2
                end
                block $B9
                  local.get $l3
                  i32.const 1
                  i32.add
                  local.tee $l4
                  local.get $l3
                  i32.lt_u
                  br_if $B9
                  local.get $l3
                  i32.const 1
                  i32.shl
                  local.tee $l5
                  local.get $l4
                  local.get $l5
                  local.get $l4
                  i32.gt_u
                  select
                  local.tee $l4
                  i32.const 8
                  local.get $l4
                  i32.const 8
                  i32.gt_u
                  select
                  local.set $l5
                  block $B10
                    local.get $l3
                    i32.eqz
                    br_if $B10
                    local.get $l5
                    i32.const 0
                    i32.lt_s
                    br_if $B9
                    local.get $p0
                    i32.load
                    local.tee $l4
                    i32.eqz
                    br_if $B4
                    local.get $l4
                    local.get $l3
                    i32.const 1
                    local.get $l5
                    call $__rust_realloc
                    local.set $l4
                    br $B3
                  end
                  local.get $l5
                  i32.const 0
                  i32.ge_s
                  br_if $B4
                end
                call $_ZN5alloc7raw_vec17capacity_overflow17heb1d9eef88f15a21E
                unreachable
              end
              local.get $l2
              local.get $p1
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=13
              local.get $l2
              local.get $p1
              i32.const 6
              i32.shr_u
              i32.const 192
              i32.or
              i32.store8 offset=12
              local.get $l2
              i32.const 12
              i32.add
              local.set $l3
              i32.const 2
              local.set $p1
              br $B1
            end
            local.get $l5
            i32.const 1
            call $__rust_alloc
            local.set $l4
          end
          block $B11
            local.get $l4
            i32.eqz
            br_if $B11
            local.get $p0
            local.get $l4
            i32.store
            local.get $p0
            i32.const 4
            i32.add
            local.get $l5
            i32.store
            local.get $p0
            i32.load offset=8
            local.set $l3
            br $B2
          end
          local.get $l5
          i32.const 1
          call $_ZN5alloc5alloc18handle_alloc_error17h02613b455f15ef67E
          unreachable
        end
        local.get $l4
        local.get $l3
        i32.add
        local.get $p1
        i32.store8
        local.get $p0
        local.get $p0
        i32.load offset=8
        i32.const 1
        i32.add
        i32.store offset=8
        br $B0
      end
      local.get $p0
      local.get $l3
      local.get $l3
      local.get $p1
      i32.add
      call $_ZN109_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..SpecExtend$LT$$RF$T$C$core..slice..Iter$LT$T$GT$$GT$$GT$11spec_extend17h43cd9ab83136c1f8E
    end
    local.get $l2
    i32.const 16
    i32.add
    global.set $g0
    i32.const 0)
  (func $_ZN3std2io5error5Error3new17hd5a96441cd7fe084E (type $t11) (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32)
    (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32)
    global.get $g0
    i32.const 16
    i32.sub
    local.tee $l4
    global.set $g0
    block $B0
      block $B1
        block $B2
          block $B3
            block $B4
              block $B5
                block $B6
                  block $B7
                    local.get $p3
                    i32.const -1
                    i32.le_s
                    br_if $B7
                    block $B8
                      block $B9
                        local.get $p3
                        br_if $B9
                        i32.const 0
                        local.set $l5
                        i32.const 1
                        local.set $l6
                        br $B8
                      end
                      local.get $p3
                      local.set $l5
                      local.get $p3
                      i32.const 1
                      call $__rust_alloc
                      local.tee $l6
                      i32.eqz
                      br_if $B6
                    end
                    local.get $l5
                    local.get $p3
                    i32.ge_u
                    br_if $B3
                    local.get $l5
                    i32.const 1
                    i32.shl
                    local.tee $l7
                    local.get $p3
                    local.get $l7
                    local.get $p3
                    i32.gt_u
                    select
                    local.tee $l7
                    i32.const 8
                    local.get $l7
                    i32.const 8
                    i32.gt_u
                    select
                    local.set $l7
                    block $B10
                      local.get $l5
                      i32.eqz
                      br_if $B10
                      local.get $l7
                      i32.const 0
                      i32.lt_s
                      br_if $B7
                      local.get $l6
                      i32.eqz
                      br_if $B5
                      local.get $l6
                      local.get $l5
                      i32.const 1
                      local.get $l7
                      call $__rust_realloc
                      local.tee $l6
                      br_if $B4
                      br $B2
                    end
                    local.get $l7
                    i32.const 0
                    i32.ge_s
                    br_if $B5
                  end
                  call $_ZN5alloc7raw_vec17capacity_overflow17heb1d9eef88f15a21E
                  unreachable
                end
                local.get $p3
                i32.const 1
                call $_ZN5alloc5alloc18handle_alloc_error17h02613b455f15ef67E
                unreachable
              end
              local.get $l7
              i32.const 1
              call $__rust_alloc
              local.tee $l6
              i32.eqz
              br_if $B2
            end
            local.get $l7
            local.set $l5
          end
          local.get $l6
          local.get $p2
          local.get $p3
          call $memcpy
          local.set $p2
          i32.const 12
          i32.const 4
          call $__rust_alloc
          local.tee $l6
          i32.eqz
          br_if $B1
          local.get $l6
          local.get $p3
          i32.store offset=8
          local.get $l6
          local.get $l5
          i32.store offset=4
          local.get $l6
          local.get $p2
          i32.store
          i32.const 12
          i32.const 4
          call $__rust_alloc
          local.tee $p3
          i32.eqz
          br_if $B0
          local.get $p3
          local.get $p1
          i32.store8 offset=8
          local.get $p3
          i32.const 1049516
          i32.store offset=4
          local.get $p3
          local.get $l6
          i32.store
          local.get $p3
          local.get $l4
          i32.load16_u offset=13 align=1
          i32.store16 offset=9 align=1
          local.get $p3
          i32.const 11
          i32.add
          local.get $l4
          i32.const 13
          i32.add
          i32.const 2
          i32.add
          i32.load8_u
          i32.store8
          local.get $p0
          i32.const 2
          i32.store8
          local.get $p0
          local.get $l4
          i32.load16_u offset=10 align=1
          i32.store16 offset=1 align=1
          local.get $p0
          i32.const 3
          i32.add
          local.get $l4
          i32.const 10
          i32.add
          i32.const 2
          i32.add
          i32.load8_u
          i32.store8
          local.get $p0
          i32.const 4
          i32.add
          local.get $p3
          i32.store
          local.get $l4
          i32.const 16
          i32.add
          global.set $g0
          return
        end
        local.get $l7
        i32.const 1
        call $_ZN5alloc5alloc18handle_alloc_error17h02613b455f15ef67E
        unreachable
      end
      i32.const 12
      i32.const 4
      call $_ZN5alloc5alloc18handle_alloc_error17h02613b455f15ef67E
      unreachable
    end
    i32.const 12
    i32.const 4
    call $_ZN5alloc5alloc18handle_alloc_error17h02613b455f15ef67E
    unreachable)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17hdec56c84cea7811cE (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32)
    global.get $g0
    i32.const 32
    i32.sub
    local.tee $l2
    global.set $g0
    local.get $l2
    local.get $p0
    i32.load
    i32.store offset=4
    local.get $l2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get $p1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get $l2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get $p1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get $l2
    local.get $p1
    i64.load align=4
    i64.store offset=8
    local.get $l2
    i32.const 4
    i32.add
    i32.const 1048704
    local.get $l2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17hb395f946a5ce2cabE
    local.set $p1
    local.get $l2
    i32.const 32
    i32.add
    global.set $g0
    local.get $p1)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17hdfc78445ae553426E (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32)
    global.get $g0
    i32.const 32
    i32.sub
    local.tee $l2
    global.set $g0
    local.get $l2
    local.get $p0
    i32.load
    i32.store offset=4
    local.get $l2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get $p1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get $l2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get $p1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get $l2
    local.get $p1
    i64.load align=4
    i64.store offset=8
    local.get $l2
    i32.const 4
    i32.add
    i32.const 1048728
    local.get $l2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17hb395f946a5ce2cabE
    local.set $p1
    local.get $l2
    i32.const 32
    i32.add
    global.set $g0
    local.get $p1)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h39f86d728dd50852E (type $t6) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    local.get $p0
    i32.load
    local.get $p1
    local.get $p1
    local.get $p2
    i32.add
    call $_ZN109_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..SpecExtend$LT$$RF$T$C$core..slice..Iter$LT$T$GT$$GT$$GT$11spec_extend17h43cd9ab83136c1f8E
    i32.const 0)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17hea8a3f0a0a103a61E (type $t6) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    (local $l3 i32) (local $l4 i64) (local $l5 i32)
    global.get $g0
    i32.const 16
    i32.sub
    local.tee $l3
    global.set $g0
    local.get $l3
    i32.const 8
    i32.add
    local.get $p0
    i32.load
    local.tee $p0
    i32.load
    local.get $p1
    local.get $p2
    call $_ZN61_$LT$std..io..stdio..StdoutLock$u20$as$u20$std..io..Write$GT$9write_all17hd421315622c6dec2E
    i32.const 0
    local.set $p1
    block $B0
      local.get $l3
      i32.load8_u offset=8
      i32.const 3
      i32.eq
      br_if $B0
      local.get $l3
      i64.load offset=8
      local.set $l4
      block $B1
        block $B2
          i32.const 0
          br_if $B2
          local.get $p0
          i32.load8_u offset=4
          i32.const 2
          i32.ne
          br_if $B1
        end
        local.get $p0
        i32.const 8
        i32.add
        i32.load
        local.tee $p1
        i32.load
        local.get $p1
        i32.load offset=4
        i32.load
        call_indirect (type $t1) $T0
        block $B3
          local.get $p1
          i32.load offset=4
          local.tee $p2
          i32.load offset=4
          local.tee $l5
          i32.eqz
          br_if $B3
          local.get $p1
          i32.load
          local.get $l5
          local.get $p2
          i32.load offset=8
          call $__rust_dealloc
        end
        local.get $p0
        i32.load offset=8
        i32.const 12
        i32.const 4
        call $__rust_dealloc
      end
      local.get $p0
      local.get $l4
      i64.store offset=4 align=4
      i32.const 1
      local.set $p1
    end
    local.get $l3
    i32.const 16
    i32.add
    global.set $g0
    local.get $p1)
  (func $_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17h01bd736b3b3e1565E (type $t1) (param $p0 i32)
    (local $l1 i32) (local $l2 i32)
    block $B0
      local.get $p0
      i32.load
      local.tee $l1
      i32.load offset=16
      local.tee $l2
      i32.eqz
      br_if $B0
      local.get $l2
      i32.const 0
      i32.store8
      local.get $l1
      i32.load offset=20
      local.tee $l2
      i32.eqz
      br_if $B0
      local.get $l1
      i32.load offset=16
      local.get $l2
      i32.const 1
      call $__rust_dealloc
    end
    local.get $l1
    i32.load offset=28
    i32.const 1
    i32.const 1
    call $__rust_dealloc
    block $B1
      local.get $p0
      i32.load
      local.tee $l1
      i32.const -1
      i32.eq
      br_if $B1
      local.get $l1
      local.get $l1
      i32.load offset=4
      local.tee $p0
      i32.const -1
      i32.add
      i32.store offset=4
      local.get $p0
      i32.const 1
      i32.ne
      br_if $B1
      local.get $l1
      i32.const 48
      i32.const 8
      call $__rust_dealloc
    end)
  (func $_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h43e9c0fc7264942bE (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    i32.load
    local.get $p0
    i32.load offset=8
    local.get $p1
    call $_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hd7770bbf948948ffE)
  (func $_ZN3std10sys_common11thread_info10ThreadInfo4with28_$u7b$$u7b$closure$u7d$$u7d$17h1d9aa9bea4f8c181E (type $t5) (param $p0 i32) (result i32)
    (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32)
    global.get $g0
    i32.const 32
    i32.sub
    local.tee $l1
    global.set $g0
    block $B0
      block $B1
        block $B2
          block $B3
            local.get $p0
            i32.load
            local.tee $l2
            i32.const 1
            i32.add
            i32.const 0
            i32.le_s
            br_if $B3
            local.get $p0
            local.get $l2
            i32.store
            block $B4
              local.get $p0
              i32.load offset=4
              local.tee $l3
              br_if $B4
              local.get $l1
              i32.const 0
              i32.store offset=8
              local.get $l1
              i32.const 8
              i32.add
              call $_ZN3std6thread6Thread3new17h0ad4b740297a0352E
              local.set $l3
              local.get $p0
              i32.load
              br_if $B2
              local.get $p0
              i32.const -1
              i32.store
              block $B5
                local.get $p0
                i32.load offset=4
                local.tee $l2
                i32.eqz
                br_if $B5
                local.get $l2
                local.get $l2
                i32.load
                local.tee $l4
                i32.const -1
                i32.add
                i32.store
                local.get $l4
                i32.const 1
                i32.ne
                br_if $B5
                local.get $p0
                i32.const 4
                i32.add
                call $_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17h01bd736b3b3e1565E
              end
              local.get $p0
              local.get $l3
              i32.store offset=4
              local.get $p0
              local.get $p0
              i32.load
              i32.const 1
              i32.add
              local.tee $l2
              i32.store
            end
            local.get $l2
            br_if $B1
            local.get $p0
            i32.const -1
            i32.store
            local.get $l3
            local.get $l3
            i32.load
            local.tee $l2
            i32.const 1
            i32.add
            i32.store
            local.get $l2
            i32.const -1
            i32.le_s
            br_if $B0
            local.get $p0
            local.get $p0
            i32.load
            i32.const 1
            i32.add
            i32.store
            local.get $l1
            i32.const 32
            i32.add
            global.set $g0
            local.get $l3
            return
          end
          i32.const 1048784
          i32.const 24
          local.get $l1
          i32.const 24
          i32.add
          i32.const 1049020
          i32.const 1050688
          call $_ZN4core6option18expect_none_failed17h6878b94853f7ecf6E
          unreachable
        end
        i32.const 1048768
        i32.const 16
        local.get $l1
        i32.const 24
        i32.add
        i32.const 1049004
        i32.const 1050704
        call $_ZN4core6option18expect_none_failed17h6878b94853f7ecf6E
        unreachable
      end
      i32.const 1048768
      i32.const 16
      local.get $l1
      i32.const 24
      i32.add
      i32.const 1049004
      i32.const 1050720
      call $_ZN4core6option18expect_none_failed17h6878b94853f7ecf6E
      unreachable
    end
    unreachable
    unreachable)
  (func $_ZN3std6thread4park17h93960e6e940434aaE (type $t0)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32)
    global.get $g0
    i32.const 96
    i32.sub
    local.tee $l0
    global.set $g0
    block $B0
      i32.const 0
      i32.load offset=1054820
      i32.const 1
      i32.eq
      br_if $B0
      i32.const 0
      i64.const 1
      i64.store offset=1054820 align=4
      i32.const 0
      i32.const 0
      i32.store offset=1054828
    end
    i32.const 1054824
    call $_ZN3std10sys_common11thread_info10ThreadInfo4with28_$u7b$$u7b$closure$u7d$$u7d$17h1d9aa9bea4f8c181E
    local.tee $l1
    i32.const 0
    local.get $l1
    i32.load offset=24
    local.tee $l2
    local.get $l2
    i32.const 2
    i32.eq
    local.tee $l2
    select
    i32.store offset=24
    local.get $l0
    local.get $l1
    i32.store offset=8
    block $B1
      local.get $l2
      br_if $B1
      block $B2
        block $B3
          block $B4
            block $B5
              local.get $l0
              i32.load offset=8
              local.tee $l1
              i32.const 28
              i32.add
              local.tee $l3
              i32.load
              local.tee $l2
              i32.load8_u
              br_if $B5
              local.get $l2
              i32.const 1
              i32.store8
              i32.const 0
              local.set $l4
              block $B6
                i32.const 0
                i32.load offset=1054800
                i32.eqz
                br_if $B6
                call $_ZN3std9panicking11panic_count17is_zero_slow_path17hf6ba0dc1fa3c5010E
                i32.const 1
                i32.xor
                local.set $l4
              end
              local.get $l1
              i32.load8_u offset=32
              br_if $B4
              local.get $l1
              local.get $l1
              i32.load offset=24
              local.tee $l2
              i32.const 1
              local.get $l2
              select
              i32.store offset=24
              block $B7
                local.get $l2
                br_if $B7
                local.get $l0
                i32.load offset=8
                i32.const 36
                i32.add
                local.get $l3
                i32.load
                call $_ZN3std4sync7condvar7Condvar6verify17hb30be69d05ff35b2E
                call $_ZN3std10sys_common7condvar7Condvar4wait17hafaeb3c89ca4927fE
                unreachable
              end
              local.get $l2
              i32.const 2
              i32.ne
              br_if $B3
              local.get $l0
              i32.load offset=8
              local.tee $l5
              i32.load offset=24
              local.set $l2
              local.get $l5
              i32.const 0
              i32.store offset=24
              local.get $l0
              local.get $l2
              i32.store offset=12
              local.get $l2
              i32.const 2
              i32.ne
              br_if $B2
              block $B8
                local.get $l4
                br_if $B8
                i32.const 0
                i32.load offset=1054800
                i32.eqz
                br_if $B8
                call $_ZN3std9panicking11panic_count17is_zero_slow_path17hf6ba0dc1fa3c5010E
                br_if $B8
                local.get $l1
                i32.const 1
                i32.store8 offset=32
              end
              local.get $l3
              i32.load
              i32.const 0
              i32.store8
              br $B1
            end
            i32.const 1051108
            i32.const 32
            i32.const 1051188
            call $_ZN3std9panicking11begin_panic17h90326787ac4041daE
            unreachable
          end
          local.get $l0
          local.get $l4
          i32.store8 offset=76
          local.get $l0
          local.get $l3
          i32.store offset=72
          i32.const 1049052
          i32.const 43
          local.get $l0
          i32.const 72
          i32.add
          i32.const 1049096
          i32.const 1049160
          call $_ZN4core6option18expect_none_failed17h6878b94853f7ecf6E
          unreachable
        end
        i32.const 1049176
        i32.const 23
        i32.const 1049200
        call $_ZN3std9panicking11begin_panic17h90326787ac4041daE
        unreachable
      end
      local.get $l0
      i32.const 40
      i32.add
      i32.const 20
      i32.add
      i32.const 5
      i32.store
      local.get $l0
      i32.const 52
      i32.add
      i32.const 6
      i32.store
      local.get $l0
      i32.const 16
      i32.add
      i32.const 20
      i32.add
      i32.const 3
      i32.store
      local.get $l0
      local.get $l0
      i32.const 12
      i32.add
      i32.store offset=64
      local.get $l0
      i32.const 1049216
      i32.store offset=68
      local.get $l0
      i32.const 72
      i32.add
      i32.const 20
      i32.add
      i32.const 0
      i32.store
      local.get $l0
      i64.const 3
      i64.store offset=20 align=4
      local.get $l0
      i32.const 1049224
      i32.store offset=16
      local.get $l0
      i32.const 6
      i32.store offset=44
      local.get $l0
      i32.const 1048892
      i32.store offset=88
      local.get $l0
      i64.const 1
      i64.store offset=76 align=4
      local.get $l0
      i32.const 1049280
      i32.store offset=72
      local.get $l0
      local.get $l0
      i32.const 40
      i32.add
      i32.store offset=32
      local.get $l0
      local.get $l0
      i32.const 72
      i32.add
      i32.store offset=56
      local.get $l0
      local.get $l0
      i32.const 68
      i32.add
      i32.store offset=48
      local.get $l0
      local.get $l0
      i32.const 64
      i32.add
      i32.store offset=40
      local.get $l0
      i32.const 16
      i32.add
      i32.const 1049288
      call $_ZN3std9panicking15begin_panic_fmt17h81d6d5fcf87a41f9E
      unreachable
    end
    local.get $l0
    i32.load offset=8
    local.tee $l1
    local.get $l1
    i32.load
    local.tee $l1
    i32.const -1
    i32.add
    i32.store
    block $B9
      local.get $l1
      i32.const 1
      i32.ne
      br_if $B9
      local.get $l0
      i32.const 8
      i32.add
      call $_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17h01bd736b3b3e1565E
    end
    local.get $l0
    i32.const 96
    i32.add
    global.set $g0)
  (func $_ZN3std4sync7condvar7Condvar6verify17hb30be69d05ff35b2E (type $t3) (param $p0 i32) (param $p1 i32)
    (local $l2 i32)
    local.get $p0
    local.get $p0
    i32.load offset=4
    local.tee $l2
    local.get $p1
    local.get $l2
    select
    i32.store offset=4
    block $B0
      local.get $l2
      i32.eqz
      br_if $B0
      local.get $l2
      local.get $p1
      i32.eq
      br_if $B0
      i32.const 1050196
      i32.const 54
      i32.const 1050284
      call $_ZN3std9panicking11begin_panic17h90326787ac4041daE
      unreachable
    end)
  (func $_ZN3std10sys_common7condvar7Condvar4wait17hafaeb3c89ca4927fE (type $t0)
    (local $l0 i32)
    local.get $l0
    local.get $l0
    call $_ZN3std3sys4wasm7condvar7Condvar4wait17h610ae0be866fd8c5E
    unreachable)
  (func $_ZN3std9panicking15begin_panic_fmt17h81d6d5fcf87a41f9E (type $t3) (param $p0 i32) (param $p1 i32)
    (local $l2 i32)
    global.get $g0
    i32.const 16
    i32.sub
    local.tee $l2
    global.set $g0
    local.get $l2
    local.get $p1
    call $_ZN4core5panic8Location6caller17hbeb99f2804420dffE
    i32.store offset=12
    local.get $l2
    local.get $p0
    i32.store offset=8
    local.get $l2
    i32.const 1048892
    i32.store offset=4
    local.get $l2
    i32.const 1048892
    i32.store
    local.get $l2
    call $rust_begin_unwind
    unreachable)
  (func $_ZN3std6thread6Thread3new17h0ad4b740297a0352E (type $t5) (param $p0 i32) (result i32)
    (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i64)
    global.get $g0
    i32.const 48
    i32.sub
    local.tee $l1
    global.set $g0
    block $B0
      block $B1
        block $B2
          block $B3
            block $B4
              block $B5
                local.get $p0
                i32.load
                local.tee $l2
                br_if $B5
                i32.const 0
                local.set $l3
                br $B4
              end
              local.get $l1
              local.get $p0
              i64.load offset=4 align=4
              i64.store offset=36 align=4
              local.get $l1
              local.get $l2
              i32.store offset=32
              local.get $l1
              i32.const 16
              i32.add
              local.get $l1
              i32.const 32
              i32.add
              call $_ZN5alloc6string104_$LT$impl$u20$core..convert..From$LT$alloc..string..String$GT$$u20$for$u20$alloc..vec..Vec$LT$u8$GT$$GT$4from17hc7c291bec8ce730aE
              local.get $l1
              i32.const 8
              i32.add
              i32.const 0
              local.get $l1
              i32.load offset=16
              local.tee $p0
              local.get $l1
              i32.load offset=24
              call $_ZN4core5slice6memchr6memchr17h0f2bc0ed161f00a2E
              local.get $l1
              i32.load offset=8
              br_if $B3
              local.get $l1
              i32.const 32
              i32.add
              i32.const 8
              i32.add
              local.get $l1
              i32.const 16
              i32.add
              i32.const 8
              i32.add
              i32.load
              i32.store
              local.get $l1
              local.get $l1
              i64.load offset=16
              i64.store offset=32
              local.get $l1
              local.get $l1
              i32.const 32
              i32.add
              call $_ZN3std3ffi5c_str7CString18from_vec_unchecked17hfd66f75c7b92a678E
              local.get $l1
              i32.load offset=4
              local.set $l4
              local.get $l1
              i32.load
              local.set $l3
            end
            i32.const 0
            i32.load8_u offset=1055296
            br_if $B2
            i32.const 0
            i32.const 1
            i32.store8 offset=1055296
            block $B6
              block $B7
                i32.const 0
                i64.load offset=1054760
                local.tee $l5
                i64.const -1
                i64.eq
                br_if $B7
                i32.const 0
                local.get $l5
                i64.const 1
                i64.add
                i64.store offset=1054760
                local.get $l5
                i64.const 0
                i64.ne
                br_if $B6
                i32.const 1048960
                i32.const 43
                i32.const 1049376
                call $_ZN4core9panicking5panic17hc886a4cb4479b06eE
                unreachable
              end
              i32.const 1049304
              i32.const 55
              i32.const 1049360
              call $_ZN3std9panicking11begin_panic17h90326787ac4041daE
              unreachable
            end
            i32.const 0
            i32.const 0
            i32.store8 offset=1055296
            i32.const 1
            i32.const 1
            call $__rust_alloc
            local.tee $l2
            i32.eqz
            br_if $B1
            local.get $l2
            i32.const 0
            i32.store8
            i32.const 48
            i32.const 8
            call $__rust_alloc
            local.tee $p0
            i32.eqz
            br_if $B0
            local.get $p0
            i64.const 1
            i64.store offset=36 align=4
            local.get $p0
            i32.const 0
            i32.store offset=24
            local.get $p0
            local.get $l4
            i32.store offset=20
            local.get $p0
            local.get $l3
            i32.store offset=16
            local.get $p0
            local.get $l5
            i64.store offset=8
            local.get $p0
            i64.const 4294967297
            i64.store
            local.get $p0
            local.get $l2
            i64.extend_i32_u
            i64.store offset=28 align=4
            local.get $l1
            i32.const 48
            i32.add
            global.set $g0
            local.get $p0
            return
          end
          local.get $l1
          i32.load offset=12
          local.set $l2
          local.get $l1
          i32.const 40
          i32.add
          local.get $l1
          i64.load offset=20 align=4
          i64.store
          local.get $l1
          local.get $p0
          i32.store offset=36
          local.get $l1
          local.get $l2
          i32.store offset=32
          i32.const 1049392
          i32.const 47
          local.get $l1
          i32.const 32
          i32.add
          i32.const 1049036
          i32.const 1049440
          call $_ZN4core6option18expect_none_failed17h6878b94853f7ecf6E
          unreachable
        end
        i32.const 1051108
        i32.const 32
        i32.const 1051188
        call $_ZN3std9panicking11begin_panic17h90326787ac4041daE
        unreachable
      end
      i32.const 1
      i32.const 1
      call $_ZN5alloc5alloc18handle_alloc_error17h02613b455f15ef67E
      unreachable
    end
    i32.const 48
    i32.const 8
    call $_ZN5alloc5alloc18handle_alloc_error17h02613b455f15ef67E
    unreachable)
  (func $_ZN3std3ffi5c_str7CString18from_vec_unchecked17hfd66f75c7b92a678E (type $t3) (param $p0 i32) (param $p1 i32)
    (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32)
    block $B0
      block $B1
        block $B2
          block $B3
            block $B4
              local.get $p1
              i32.const 4
              i32.add
              i32.load
              local.tee $l2
              local.get $p1
              i32.load offset=8
              local.tee $l3
              i32.ne
              br_if $B4
              local.get $l3
              i32.const 1
              i32.add
              local.tee $l2
              local.get $l3
              i32.lt_u
              br_if $B1
              block $B5
                block $B6
                  block $B7
                    local.get $l3
                    i32.eqz
                    br_if $B7
                    local.get $l2
                    i32.const 0
                    i32.lt_s
                    br_if $B1
                    local.get $p1
                    i32.load
                    local.tee $l4
                    i32.eqz
                    br_if $B6
                    local.get $l4
                    local.get $l3
                    i32.const 1
                    local.get $l2
                    call $__rust_realloc
                    local.set $l4
                    br $B5
                  end
                  local.get $l2
                  i32.const 0
                  i32.lt_s
                  br_if $B1
                end
                local.get $l2
                i32.const 1
                call $__rust_alloc
                local.set $l4
              end
              local.get $l4
              i32.eqz
              br_if $B3
              local.get $p1
              local.get $l4
              i32.store
              local.get $p1
              i32.const 4
              i32.add
              local.get $l2
              i32.store
            end
            local.get $l3
            local.get $l2
            i32.eq
            br_if $B2
            local.get $l3
            i32.const 1
            i32.add
            local.set $l2
            local.get $p1
            i32.load
            local.set $l4
            br $B0
          end
          local.get $l2
          i32.const 1
          call $_ZN5alloc5alloc18handle_alloc_error17h02613b455f15ef67E
          unreachable
        end
        local.get $l3
        i32.const 1
        i32.add
        local.tee $l2
        local.get $l3
        i32.lt_u
        br_if $B1
        local.get $l3
        i32.const 1
        i32.shl
        local.tee $l4
        local.get $l2
        local.get $l4
        local.get $l2
        i32.gt_u
        select
        local.tee $l4
        i32.const 8
        local.get $l4
        i32.const 8
        i32.gt_u
        select
        local.set $l5
        block $B8
          block $B9
            block $B10
              local.get $l3
              i32.eqz
              br_if $B10
              local.get $l5
              i32.const 0
              i32.lt_s
              br_if $B1
              local.get $p1
              i32.load
              local.tee $l4
              i32.eqz
              br_if $B9
              local.get $l4
              local.get $l3
              i32.const 1
              local.get $l5
              call $__rust_realloc
              local.set $l4
              br $B8
            end
            local.get $l5
            i32.const 0
            i32.lt_s
            br_if $B1
          end
          local.get $l5
          i32.const 1
          call $__rust_alloc
          local.set $l4
        end
        block $B11
          local.get $l4
          i32.eqz
          br_if $B11
          local.get $p1
          local.get $l4
          i32.store
          local.get $p1
          i32.const 4
          i32.add
          local.get $l5
          i32.store
          br $B0
        end
        local.get $l5
        i32.const 1
        call $_ZN5alloc5alloc18handle_alloc_error17h02613b455f15ef67E
        unreachable
      end
      call $_ZN5alloc7raw_vec17capacity_overflow17heb1d9eef88f15a21E
      unreachable
    end
    local.get $l4
    local.get $l3
    i32.add
    i32.const 0
    i32.store8
    local.get $p1
    local.get $l2
    i32.store offset=8
    block $B12
      block $B13
        local.get $p1
        i32.const 4
        i32.add
        i32.load
        local.tee $l3
        local.get $l2
        i32.gt_u
        br_if $B13
        local.get $l4
        local.set $p1
        br $B12
      end
      block $B14
        local.get $l2
        br_if $B14
        i32.const 1
        local.set $p1
        local.get $l4
        local.get $l3
        i32.const 1
        call $__rust_dealloc
        br $B12
      end
      local.get $l4
      local.get $l3
      i32.const 1
      local.get $l2
      call $__rust_realloc
      local.tee $p1
      br_if $B12
      local.get $l2
      i32.const 1
      call $_ZN5alloc5alloc18handle_alloc_error17h02613b455f15ef67E
      unreachable
    end
    local.get $p0
    local.get $l2
    i32.store offset=4
    local.get $p0
    local.get $p1
    i32.store)
  (func $_ZN3std6thread6Thread6unpark17hd2c776a4f99f2b06E (type $t1) (param $p0 i32)
    (local $l1 i32) (local $l2 i32) (local $l3 i32)
    global.get $g0
    i32.const 16
    i32.sub
    local.tee $l1
    global.set $g0
    local.get $p0
    i32.load
    local.tee $l2
    i32.load offset=24
    local.set $l3
    local.get $l2
    i32.const 2
    i32.store offset=24
    block $B0
      block $B1
        block $B2
          block $B3
            block $B4
              local.get $l3
              br_table $B2 $B3 $B2 $B4
            end
            i32.const 1049456
            i32.const 28
            i32.const 1049484
            call $_ZN3std9panicking11begin_panic17h90326787ac4041daE
            unreachable
          end
          local.get $p0
          i32.load
          local.tee $p0
          i32.const 28
          i32.add
          local.tee $l2
          i32.load
          local.tee $l3
          i32.load8_u
          br_if $B1
          local.get $l3
          i32.const 1
          i32.store8
          i32.const 0
          local.set $l3
          block $B5
            block $B6
              block $B7
                block $B8
                  i32.const 0
                  i32.load offset=1054800
                  i32.eqz
                  br_if $B8
                  call $_ZN3std9panicking11panic_count17is_zero_slow_path17hf6ba0dc1fa3c5010E
                  local.set $l3
                  local.get $p0
                  i32.load8_u offset=32
                  i32.eqz
                  br_if $B7
                  local.get $l3
                  i32.const 1
                  i32.xor
                  local.set $l3
                  br $B0
                end
                local.get $p0
                i32.load8_u offset=32
                br_if $B0
                local.get $p0
                i32.const 32
                i32.add
                local.set $p0
                br $B6
              end
              local.get $l3
              i32.eqz
              br_if $B5
              local.get $p0
              i32.const 32
              i32.add
              local.set $p0
            end
            i32.const 0
            i32.load offset=1054800
            i32.eqz
            br_if $B5
            call $_ZN3std9panicking11panic_count17is_zero_slow_path17hf6ba0dc1fa3c5010E
            br_if $B5
            local.get $p0
            i32.const 1
            i32.store8
          end
          local.get $l2
          i32.load
          i32.const 0
          i32.store8
        end
        local.get $l1
        i32.const 16
        i32.add
        global.set $g0
        return
      end
      i32.const 1051108
      i32.const 32
      i32.const 1051188
      call $_ZN3std9panicking11begin_panic17h90326787ac4041daE
      unreachable
    end
    local.get $l1
    local.get $l3
    i32.store8 offset=12
    local.get $l1
    local.get $l2
    i32.store offset=8
    i32.const 1049052
    i32.const 43
    local.get $l1
    i32.const 8
    i32.add
    i32.const 1049096
    i32.const 1049500
    call $_ZN4core6option18expect_none_failed17h6878b94853f7ecf6E
    unreachable)
  (func $_ZN60_$LT$std..io..error..Error$u20$as$u20$core..fmt..Display$GT$3fmt17h63dc9f43fba9e471E (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32) (local $l3 i32) (local $l4 i32)
    global.get $g0
    i32.const 64
    i32.sub
    local.tee $l2
    global.set $g0
    block $B0
      block $B1
        block $B2
          block $B3
            block $B4
              local.get $p0
              i32.load8_u
              br_table $B3 $B4 $B2 $B3
            end
            i32.const 1049620
            local.set $l3
            i32.const 22
            local.set $l4
            block $B5
              block $B6
                block $B7
                  block $B8
                    block $B9
                      block $B10
                        block $B11
                          block $B12
                            block $B13
                              block $B14
                                block $B15
                                  block $B16
                                    block $B17
                                      block $B18
                                        block $B19
                                          block $B20
                                            block $B21
                                              block $B22
                                                block $B23
                                                  local.get $p0
                                                  i32.load8_u offset=1
                                                  br_table $B23 $B22 $B21 $B20 $B19 $B18 $B17 $B16 $B15 $B14 $B13 $B12 $B11 $B10 $B9 $B8 $B7 $B5 $B23
                                                end
                                                i32.const 1049901
                                                local.set $l3
                                                i32.const 16
                                                local.set $l4
                                                br $B5
                                              end
                                              i32.const 1049884
                                              local.set $l3
                                              i32.const 17
                                              local.set $l4
                                              br $B5
                                            end
                                            i32.const 1049866
                                            local.set $l3
                                            i32.const 18
                                            local.set $l4
                                            br $B5
                                          end
                                          i32.const 1049850
                                          local.set $l3
                                          i32.const 16
                                          local.set $l4
                                          br $B5
                                        end
                                        i32.const 1049832
                                        local.set $l3
                                        i32.const 18
                                        local.set $l4
                                        br $B5
                                      end
                                      i32.const 1049819
                                      local.set $l3
                                      i32.const 13
                                      local.set $l4
                                      br $B5
                                    end
                                    i32.const 1049805
                                    local.set $l3
                                    br $B6
                                  end
                                  i32.const 1049784
                                  local.set $l3
                                  i32.const 21
                                  local.set $l4
                                  br $B5
                                end
                                i32.const 1049773
                                local.set $l3
                                i32.const 11
                                local.set $l4
                                br $B5
                              end
                              i32.const 1049752
                              local.set $l3
                              i32.const 21
                              local.set $l4
                              br $B5
                            end
                            i32.const 1049731
                            local.set $l3
                            i32.const 21
                            local.set $l4
                            br $B5
                          end
                          i32.const 1049708
                          local.set $l3
                          i32.const 23
                          local.set $l4
                          br $B5
                        end
                        i32.const 1049696
                        local.set $l3
                        i32.const 12
                        local.set $l4
                        br $B5
                      end
                      i32.const 1049687
                      local.set $l3
                      i32.const 9
                      local.set $l4
                      br $B5
                    end
                    i32.const 1049677
                    local.set $l3
                    i32.const 10
                    local.set $l4
                    br $B5
                  end
                  i32.const 1049656
                  local.set $l3
                  i32.const 21
                  local.set $l4
                  br $B5
                end
                i32.const 1049642
                local.set $l3
              end
              i32.const 14
              local.set $l4
            end
            local.get $l2
            i32.const 60
            i32.add
            i32.const 1
            i32.store
            local.get $l2
            local.get $l4
            i32.store offset=28
            local.get $l2
            local.get $l3
            i32.store offset=24
            local.get $l2
            i32.const 7
            i32.store offset=12
            local.get $l2
            i64.const 1
            i64.store offset=44 align=4
            local.get $l2
            i32.const 1049920
            i32.store offset=40
            local.get $l2
            local.get $l2
            i32.const 24
            i32.add
            i32.store offset=8
            local.get $l2
            local.get $l2
            i32.const 8
            i32.add
            i32.store offset=56
            local.get $p1
            local.get $l2
            i32.const 40
            i32.add
            call $_ZN4core3fmt9Formatter9write_fmt17hc1fb6c199a6a4c9dE
            local.set $p0
            br $B1
          end
          local.get $l2
          local.get $p0
          i32.const 4
          i32.add
          i32.load
          i32.store offset=4
          i32.const 20
          i32.const 1
          call $__rust_alloc
          local.tee $p0
          i32.eqz
          br_if $B0
          local.get $p0
          i32.const 16
          i32.add
          i32.const 0
          i32.load offset=1051012 align=1
          i32.store align=1
          local.get $p0
          i32.const 8
          i32.add
          i32.const 0
          i64.load offset=1051004 align=1
          i64.store align=1
          local.get $p0
          i32.const 0
          i64.load offset=1050996 align=1
          i64.store align=1
          local.get $l2
          i64.const 85899345940
          i64.store offset=12 align=4
          local.get $l2
          local.get $p0
          i32.store offset=8
          local.get $l2
          i32.const 40
          i32.add
          i32.const 20
          i32.add
          i32.const 2
          i32.store
          local.get $l2
          i32.const 36
          i32.add
          i32.const 8
          i32.store
          local.get $l2
          i64.const 3
          i64.store offset=44 align=4
          local.get $l2
          i32.const 1049940
          i32.store offset=40
          local.get $l2
          i32.const 9
          i32.store offset=28
          local.get $l2
          local.get $l2
          i32.const 24
          i32.add
          i32.store offset=56
          local.get $l2
          local.get $l2
          i32.const 4
          i32.add
          i32.store offset=32
          local.get $l2
          local.get $l2
          i32.const 8
          i32.add
          i32.store offset=24
          local.get $p1
          local.get $l2
          i32.const 40
          i32.add
          call $_ZN4core3fmt9Formatter9write_fmt17hc1fb6c199a6a4c9dE
          local.set $p0
          local.get $l2
          i32.load offset=8
          local.tee $p1
          i32.eqz
          br_if $B1
          local.get $l2
          i32.load offset=12
          local.tee $l3
          i32.eqz
          br_if $B1
          local.get $p1
          local.get $l3
          i32.const 1
          call $__rust_dealloc
          br $B1
        end
        local.get $p0
        i32.const 4
        i32.add
        i32.load
        local.tee $p0
        i32.load
        local.get $p1
        local.get $p0
        i32.load offset=4
        i32.load offset=32
        call_indirect (type $t2) $T0
        local.set $p0
      end
      local.get $l2
      i32.const 64
      i32.add
      global.set $g0
      local.get $p0
      return
    end
    i32.const 20
    i32.const 1
    call $_ZN5alloc5alloc18handle_alloc_error17h02613b455f15ef67E
    unreachable)
  (func $_ZN3std5error5Error7type_id17hd6272b78ea2ed3abE (type $t10) (param $p0 i32) (result i64)
    i64.const 8340024501247481077)
  (func $_ZN3std5error5Error9backtrace17hd42982a9d518845cE (type $t5) (param $p0 i32) (result i32)
    i32.const 0)
  (func $_ZN3std5error5Error5cause17h2b069ed21176965fE (type $t3) (param $p0 i32) (param $p1 i32)
    local.get $p0
    i32.const 0
    i32.store)
  (func $_ZN243_$LT$std..error..$LT$impl$u20$core..convert..From$LT$alloc..string..String$GT$$u20$for$u20$alloc..boxed..Box$LT$dyn$u20$std..error..Error$u2b$core..marker..Sync$u2b$core..marker..Send$GT$$GT$..from..StringError$u20$as$u20$std..error..Error$GT$11description17hbc74e6da51e8498dE (type $t3) (param $p0 i32) (param $p1 i32)
    local.get $p0
    local.get $p1
    i32.load offset=8
    i32.store offset=4
    local.get $p0
    local.get $p1
    i32.load
    i32.store)
  (func $_ZN244_$LT$std..error..$LT$impl$u20$core..convert..From$LT$alloc..string..String$GT$$u20$for$u20$alloc..boxed..Box$LT$dyn$u20$std..error..Error$u2b$core..marker..Sync$u2b$core..marker..Send$GT$$GT$..from..StringError$u20$as$u20$core..fmt..Display$GT$3fmt17hd22e6bf5f24cc832E (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    i32.load
    local.get $p0
    i32.load offset=8
    local.get $p1
    call $_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hd7770bbf948948ffE)
  (func $_ZN242_$LT$std..error..$LT$impl$u20$core..convert..From$LT$alloc..string..String$GT$$u20$for$u20$alloc..boxed..Box$LT$dyn$u20$std..error..Error$u2b$core..marker..Sync$u2b$core..marker..Send$GT$$GT$..from..StringError$u20$as$u20$core..fmt..Debug$GT$3fmt17hc54eb153061f3528E (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    i32.load
    local.get $p0
    i32.load offset=8
    local.get $p1
    call $_ZN40_$LT$str$u20$as$u20$core..fmt..Debug$GT$3fmt17h4e37a0e4f747f286E)
  (func $_ZN3std10sys_common11at_exit_imp4push17hd8699242b7fbe3b3E (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32)
    block $B0
      block $B1
        block $B2
          i32.const 0
          i32.load8_u offset=1055297
          br_if $B2
          i32.const 0
          i32.const 1
          i32.store8 offset=1055297
          block $B3
            block $B4
              block $B5
                i32.const 0
                i32.load offset=1054776
                local.tee $l2
                br_table $B5 $B3 $B4
              end
              i32.const 12
              i32.const 4
              call $__rust_alloc
              local.tee $l2
              i32.eqz
              br_if $B1
              local.get $l2
              i32.const 0
              i32.store offset=8
              local.get $l2
              i64.const 4
              i64.store align=4
              i32.const 0
              local.get $l2
              i32.store offset=1054776
            end
            block $B6
              local.get $l2
              i32.load offset=8
              local.tee $l3
              local.get $l2
              i32.const 4
              i32.add
              i32.load
              i32.eq
              br_if $B6
              local.get $l2
              i32.load
              local.set $l4
              br $B0
            end
            block $B7
              block $B8
                local.get $l3
                i32.const 1
                i32.add
                local.tee $l4
                local.get $l3
                i32.lt_u
                br_if $B8
                local.get $l3
                i32.const 1
                i32.shl
                local.tee $l5
                local.get $l4
                local.get $l5
                local.get $l4
                i32.gt_u
                select
                local.tee $l4
                i32.const 4
                local.get $l4
                i32.const 4
                i32.gt_u
                select
                local.tee $l4
                i32.const 536870911
                i32.and
                local.tee $l6
                local.get $l4
                i32.ne
                br_if $B8
                local.get $l4
                i32.const 3
                i32.shl
                local.tee $l5
                i32.const 0
                i32.lt_s
                br_if $B8
                local.get $l6
                local.get $l4
                i32.eq
                i32.const 2
                i32.shl
                local.set $l4
                block $B9
                  block $B10
                    local.get $l2
                    i32.load
                    i32.const 0
                    local.get $l3
                    select
                    local.tee $l6
                    br_if $B10
                    local.get $l5
                    i32.eqz
                    br_if $B9
                    local.get $l5
                    local.get $l4
                    call $__rust_alloc
                    local.set $l4
                    br $B9
                  end
                  block $B11
                    local.get $l3
                    i32.const 3
                    i32.shl
                    local.tee $l3
                    br_if $B11
                    local.get $l5
                    i32.eqz
                    br_if $B9
                    local.get $l5
                    local.get $l4
                    call $__rust_alloc
                    local.set $l4
                    br $B9
                  end
                  local.get $l6
                  local.get $l3
                  local.get $l4
                  local.get $l5
                  call $__rust_realloc
                  local.set $l4
                end
                local.get $l4
                i32.eqz
                br_if $B7
                local.get $l2
                local.get $l4
                i32.store
                local.get $l2
                i32.const 4
                i32.add
                local.get $l5
                i32.const 3
                i32.shr_u
                i32.store
                local.get $l2
                i32.load offset=8
                local.set $l3
                br $B0
              end
              call $_ZN5alloc7raw_vec17capacity_overflow17heb1d9eef88f15a21E
              unreachable
            end
            local.get $l5
            i32.const 4
            call $_ZN5alloc5alloc18handle_alloc_error17h02613b455f15ef67E
            unreachable
          end
          i32.const 0
          i32.const 0
          i32.store8 offset=1055297
          local.get $p0
          local.get $p1
          i32.load
          call_indirect (type $t1) $T0
          block $B12
            local.get $p1
            i32.load offset=4
            local.tee $l2
            i32.eqz
            br_if $B12
            local.get $p0
            local.get $l2
            local.get $p1
            i32.load offset=8
            call $__rust_dealloc
          end
          i32.const 0
          return
        end
        i32.const 1051108
        i32.const 32
        i32.const 1051188
        call $_ZN3std9panicking11begin_panic17h90326787ac4041daE
        unreachable
      end
      i32.const 12
      i32.const 4
      call $_ZN5alloc5alloc18handle_alloc_error17h02613b455f15ef67E
      unreachable
    end
    local.get $l4
    local.get $l3
    i32.const 3
    i32.shl
    i32.add
    local.tee $l3
    local.get $p1
    i32.store offset=4
    local.get $l3
    local.get $p0
    i32.store
    local.get $l2
    local.get $l2
    i32.load offset=8
    i32.const 1
    i32.add
    i32.store offset=8
    i32.const 0
    i32.const 0
    i32.store8 offset=1055297
    i32.const 1)
  (func $_ZN3std2io5stdio6stdout17h412a99e1c61b9f7cE (type $t7) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32)
    block $B0
      block $B1
        block $B2
          block $B3
            block $B4
              block $B5
                block $B6
                  i32.const 0
                  i32.load8_u offset=1054772
                  br_if $B6
                  i32.const 0
                  i32.const 1
                  i32.store8 offset=1054772
                  block $B7
                    block $B8
                      block $B9
                        block $B10
                          i32.const 0
                          i32.load offset=1054768
                          local.tee $l0
                          br_table $B9 $B10 $B8
                        end
                        i32.const 0
                        i32.const 0
                        i32.store8 offset=1054772
                        br $B0
                      end
                      i32.const 4
                      i32.const 4
                      call $__rust_alloc
                      local.tee $l0
                      i32.eqz
                      br_if $B5
                      local.get $l0
                      i32.const 1054768
                      i32.store
                      local.get $l0
                      i32.const 1050824
                      call $_ZN3std10sys_common11at_exit_imp4push17hd8699242b7fbe3b3E
                      local.set $l1
                      i32.const 1024
                      i32.const 1
                      call $__rust_alloc
                      local.tee $l2
                      i32.eqz
                      br_if $B4
                      i32.const 28
                      i32.const 4
                      call $__rust_alloc
                      local.tee $l0
                      i32.eqz
                      br_if $B3
                      local.get $l0
                      i32.const 1
                      i32.store16 offset=24
                      local.get $l0
                      i64.const 1024
                      i64.store offset=16 align=4
                      local.get $l0
                      local.get $l2
                      i32.store offset=12
                      local.get $l0
                      i32.const 0
                      i32.store offset=8
                      local.get $l0
                      i64.const 4294967297
                      i64.store align=4
                      local.get $l1
                      i32.eqz
                      br_if $B7
                      local.get $l0
                      local.get $l0
                      i32.load
                      local.tee $l1
                      i32.const 1
                      i32.add
                      i32.store
                      local.get $l1
                      i32.const -1
                      i32.le_s
                      br_if $B2
                      i32.const 4
                      i32.const 4
                      call $__rust_alloc
                      local.tee $l1
                      i32.eqz
                      br_if $B1
                      i32.const 0
                      local.get $l1
                      i32.store offset=1054768
                      local.get $l1
                      local.get $l0
                      i32.store
                      br $B7
                    end
                    local.get $l0
                    i32.load
                    local.tee $l0
                    local.get $l0
                    i32.load
                    local.tee $l1
                    i32.const 1
                    i32.add
                    i32.store
                    local.get $l1
                    i32.const -1
                    i32.le_s
                    br_if $B2
                  end
                  i32.const 0
                  i32.const 0
                  i32.store8 offset=1054772
                  local.get $l0
                  i32.eqz
                  br_if $B0
                  local.get $l0
                  return
                end
                i32.const 1051108
                i32.const 32
                i32.const 1051188
                call $_ZN3std9panicking11begin_panic17h90326787ac4041daE
                unreachable
              end
              i32.const 4
              i32.const 4
              call $_ZN5alloc5alloc18handle_alloc_error17h02613b455f15ef67E
              unreachable
            end
            i32.const 1024
            i32.const 1
            call $_ZN5alloc5alloc18handle_alloc_error17h02613b455f15ef67E
            unreachable
          end
          i32.const 28
          i32.const 4
          call $_ZN5alloc5alloc18handle_alloc_error17h02613b455f15ef67E
          unreachable
        end
        unreachable
        unreachable
      end
      i32.const 4
      i32.const 4
      call $_ZN5alloc5alloc18handle_alloc_error17h02613b455f15ef67E
      unreachable
    end
    i32.const 1049991
    i32.const 36
    i32.const 1050028
    call $_ZN4core6option13expect_failed17hafe643dc99f2fb33E
    unreachable)
  (func $_ZN57_$LT$std..io..stdio..Stdout$u20$as$u20$std..io..Write$GT$9write_fmt17h216a282b76b418a8E (type $t4) (param $p0 i32) (param $p1 i32) (param $p2 i32)
    (local $l3 i32)
    global.get $g0
    i32.const 48
    i32.sub
    local.tee $l3
    global.set $g0
    local.get $l3
    local.get $p1
    i32.load
    i32.const 8
    i32.add
    i32.store offset=4
    local.get $l3
    i32.const 3
    i32.store8 offset=12
    local.get $l3
    local.get $l3
    i32.const 4
    i32.add
    i32.store offset=8
    local.get $l3
    i32.const 24
    i32.add
    i32.const 16
    i32.add
    local.get $p2
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get $l3
    i32.const 24
    i32.add
    i32.const 8
    i32.add
    local.get $p2
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get $l3
    local.get $p2
    i64.load align=4
    i64.store offset=24
    block $B0
      block $B1
        block $B2
          local.get $l3
          i32.const 8
          i32.add
          i32.const 1050172
          local.get $l3
          i32.const 24
          i32.add
          call $_ZN4core3fmt5write17hb395f946a5ce2cabE
          i32.eqz
          br_if $B2
          block $B3
            local.get $l3
            i32.load8_u offset=12
            i32.const 3
            i32.ne
            br_if $B3
            local.get $l3
            i32.const 24
            i32.add
            i32.const 16
            i32.const 1050154
            i32.const 15
            call $_ZN3std2io5error5Error3new17hd5a96441cd7fe084E
            local.get $p0
            local.get $l3
            i64.load offset=24
            i64.store align=4
            br $B1
          end
          local.get $p0
          local.get $l3
          i64.load offset=12 align=4
          i64.store align=4
          br $B0
        end
        local.get $p0
        i32.const 3
        i32.store8
      end
      block $B4
        i32.const 0
        br_if $B4
        local.get $l3
        i32.load8_u offset=12
        i32.const 2
        i32.ne
        br_if $B0
      end
      local.get $l3
      i32.const 16
      i32.add
      i32.load
      local.tee $p2
      i32.load
      local.get $p2
      i32.load offset=4
      i32.load
      call_indirect (type $t1) $T0
      block $B5
        local.get $p2
        i32.load offset=4
        local.tee $p0
        i32.load offset=4
        local.tee $p1
        i32.eqz
        br_if $B5
        local.get $p2
        i32.load
        local.get $p1
        local.get $p0
        i32.load offset=8
        call $__rust_dealloc
      end
      local.get $l3
      i32.load offset=16
      i32.const 12
      i32.const 4
      call $__rust_dealloc
    end
    local.get $l3
    i32.const 48
    i32.add
    global.set $g0)
  (func $_ZN3std4sync4once4Once10call_inner17h154489a5744708e1E (type $t11) (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32)
    (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32)
    global.get $g0
    i32.const 32
    i32.sub
    local.tee $l4
    global.set $g0
    local.get $l4
    i32.const 16
    i32.add
    i32.const 2
    i32.or
    local.set $l5
    local.get $p0
    i32.load
    local.set $l6
    block $B0
      block $B1
        block $B2
          block $B3
            block $B4
              block $B5
                local.get $p1
                br_if $B5
                br $B4
              end
              loop $L6
                block $B7
                  block $B8
                    local.get $l6
                    local.tee $p1
                    br_table $B8 $B8 $B7 $B1 $B7
                  end
                  local.get $p0
                  i32.const 2
                  local.get $p0
                  i32.load
                  local.tee $l6
                  local.get $l6
                  local.get $p1
                  i32.eq
                  local.tee $l7
                  select
                  i32.store
                  local.get $l7
                  i32.eqz
                  br_if $L6
                  br $B2
                end
                local.get $p1
                i32.const 3
                i32.and
                i32.const 2
                i32.ne
                br_if $B3
                block $B9
                  loop $L10
                    local.get $p1
                    local.set $l6
                    block $B11
                      i32.const 0
                      i32.load offset=1054820
                      i32.const 1
                      i32.eq
                      br_if $B11
                      i32.const 0
                      i64.const 1
                      i64.store offset=1054820 align=4
                      i32.const 0
                      i32.const 0
                      i32.store offset=1054828
                    end
                    i32.const 1054824
                    call $_ZN3std10sys_common11thread_info10ThreadInfo4with28_$u7b$$u7b$closure$u7d$$u7d$17h1d9aa9bea4f8c181E
                    local.set $l7
                    local.get $p0
                    local.get $l5
                    local.get $p0
                    i32.load
                    local.tee $p1
                    local.get $p1
                    local.get $l6
                    i32.eq
                    local.tee $l8
                    select
                    i32.store
                    local.get $l4
                    i32.const 0
                    i32.store8 offset=24
                    local.get $l4
                    local.get $l7
                    i32.store offset=16
                    local.get $l4
                    local.get $l6
                    i32.const -4
                    i32.and
                    i32.store offset=20
                    block $B12
                      local.get $l8
                      br_if $B12
                      block $B13
                        local.get $l4
                        i32.load offset=16
                        local.tee $l6
                        i32.eqz
                        br_if $B13
                        local.get $l6
                        local.get $l6
                        i32.load
                        local.tee $l7
                        i32.const -1
                        i32.add
                        i32.store
                        local.get $l7
                        i32.const 1
                        i32.ne
                        br_if $B13
                        local.get $l4
                        i32.const 16
                        i32.add
                        call $_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17h01bd736b3b3e1565E
                      end
                      local.get $p1
                      i32.const 3
                      i32.and
                      i32.const 2
                      i32.eq
                      br_if $L10
                      br $B9
                    end
                  end
                  block $B14
                    local.get $l4
                    i32.load8_u offset=24
                    br_if $B14
                    loop $L15
                      call $_ZN3std6thread4park17h93960e6e940434aaE
                      local.get $l4
                      i32.load8_u offset=24
                      i32.eqz
                      br_if $L15
                    end
                  end
                  local.get $l4
                  i32.load offset=16
                  local.tee $p1
                  i32.eqz
                  br_if $B9
                  local.get $p1
                  local.get $p1
                  i32.load
                  local.tee $l6
                  i32.const -1
                  i32.add
                  i32.store
                  local.get $l6
                  i32.const 1
                  i32.ne
                  br_if $B9
                  local.get $l4
                  i32.const 16
                  i32.add
                  call $_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17h01bd736b3b3e1565E
                end
                local.get $p0
                i32.load
                local.set $l6
                br $L6
              end
            end
            loop $L16
              block $B17
                block $B18
                  local.get $l6
                  br_table $B18 $B0 $B17 $B1 $B17
                end
                local.get $p0
                local.get $p0
                i32.load
                local.tee $l6
                i32.const 2
                local.get $l6
                select
                i32.store
                local.get $l6
                br_if $L16
                i32.const 0
                local.set $p1
                br $B2
              end
              local.get $l6
              i32.const 3
              i32.and
              i32.const 2
              i32.ne
              br_if $B3
              block $B19
                block $B20
                  loop $L21
                    local.get $l6
                    local.set $p1
                    block $B22
                      i32.const 0
                      i32.load offset=1054820
                      i32.const 1
                      i32.eq
                      br_if $B22
                      i32.const 0
                      i64.const 1
                      i64.store offset=1054820 align=4
                      i32.const 0
                      i32.const 0
                      i32.store offset=1054828
                    end
                    i32.const 1054824
                    call $_ZN3std10sys_common11thread_info10ThreadInfo4with28_$u7b$$u7b$closure$u7d$$u7d$17h1d9aa9bea4f8c181E
                    local.set $l7
                    local.get $p0
                    local.get $l5
                    local.get $p0
                    i32.load
                    local.tee $l6
                    local.get $l6
                    local.get $p1
                    i32.eq
                    select
                    i32.store
                    local.get $l4
                    i32.const 0
                    i32.store8 offset=24
                    local.get $l4
                    local.get $l7
                    i32.store offset=16
                    local.get $l4
                    local.get $p1
                    i32.const -4
                    i32.and
                    i32.store offset=20
                    block $B23
                      block $B24
                        local.get $l6
                        local.get $p1
                        i32.ne
                        br_if $B24
                        local.get $l4
                        i32.load8_u offset=24
                        i32.eqz
                        br_if $B23
                        br $B20
                      end
                      block $B25
                        local.get $l4
                        i32.load offset=16
                        local.tee $p1
                        i32.eqz
                        br_if $B25
                        local.get $p1
                        local.get $p1
                        i32.load
                        local.tee $l7
                        i32.const -1
                        i32.add
                        i32.store
                        local.get $l7
                        i32.const 1
                        i32.ne
                        br_if $B25
                        local.get $l4
                        i32.const 16
                        i32.add
                        call $_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17h01bd736b3b3e1565E
                      end
                      local.get $l6
                      i32.const 3
                      i32.and
                      i32.const 2
                      i32.eq
                      br_if $L21
                      br $B19
                    end
                  end
                  loop $L26
                    call $_ZN3std6thread4park17h93960e6e940434aaE
                    local.get $l4
                    i32.load8_u offset=24
                    i32.eqz
                    br_if $L26
                  end
                end
                local.get $l4
                i32.load offset=16
                local.tee $p1
                i32.eqz
                br_if $B19
                local.get $p1
                local.get $p1
                i32.load
                local.tee $l6
                i32.const -1
                i32.add
                i32.store
                local.get $l6
                i32.const 1
                i32.ne
                br_if $B19
                local.get $l4
                i32.const 16
                i32.add
                call $_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17h01bd736b3b3e1565E
              end
              local.get $p0
              i32.load
              local.set $l6
              br $L16
            end
          end
          i32.const 1050364
          i32.const 57
          i32.const 1050424
          call $_ZN3std9panicking11begin_panic17h90326787ac4041daE
          unreachable
        end
        local.get $l4
        local.get $p0
        i32.store offset=8
        local.get $l4
        i32.const 3
        i32.store offset=16
        local.get $l4
        local.get $p1
        i32.const 1
        i32.eq
        i32.store8 offset=20
        local.get $p2
        local.get $l4
        i32.const 16
        i32.add
        local.get $p3
        i32.load offset=12
        call_indirect (type $t3) $T0
        local.get $l4
        local.get $l4
        i32.load offset=16
        i32.store offset=12
        local.get $l4
        i32.const 8
        i32.add
        call $_ZN70_$LT$std..sync..once..WaiterQueue$u20$as$u20$core..ops..drop..Drop$GT$4drop17hba24087bb7983537E
      end
      local.get $l4
      i32.const 32
      i32.add
      global.set $g0
      return
    end
    i32.const 1050440
    i32.const 42
    i32.const 1050484
    call $_ZN3std9panicking11begin_panic17h90326787ac4041daE
    unreachable)
  (func $_ZN3std2io5stdio6_print17hf23893e4604d52b7E (type $t1) (param $p0 i32)
    (local $l1 i32) (local $l2 i32) (local $l3 i64) (local $l4 i32) (local $l5 i32)
    global.get $g0
    i32.const 96
    i32.sub
    local.tee $l1
    global.set $g0
    local.get $l1
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get $p0
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get $l1
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get $p0
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get $l1
    local.get $p0
    i64.load align=4
    i64.store offset=8
    local.get $l1
    i32.const 6
    i32.store offset=36
    local.get $l1
    i32.const 1050148
    i32.store offset=32
    block $B0
      block $B1
        block $B2
          block $B3
            block $B4
              block $B5
                block $B6
                  i32.const 0
                  i32.load offset=1054804
                  i32.const 1
                  i32.eq
                  br_if $B6
                  i32.const 0
                  i64.const 1
                  i64.store offset=1054804 align=4
                  i32.const 0
                  i32.const 0
                  i32.store offset=1054812
                  br $B5
                end
                i32.const 0
                i32.load offset=1054808
                br_if $B2
                i32.const 0
                i32.const 0
                i32.store offset=1054808
                i32.const 0
                i32.load offset=1054812
                local.set $p0
                i32.const 0
                i32.const 0
                i32.store offset=1054812
                local.get $p0
                br_if $B4
              end
              local.get $l1
              call $_ZN3std2io5stdio6stdout17h412a99e1c61b9f7cE
              local.tee $p0
              i32.store offset=48
              local.get $l1
              i32.const 72
              i32.add
              i32.const 16
              i32.add
              local.get $l1
              i32.const 8
              i32.add
              i32.const 16
              i32.add
              i64.load
              i64.store
              local.get $l1
              i32.const 72
              i32.add
              i32.const 8
              i32.add
              local.get $l1
              i32.const 8
              i32.add
              i32.const 8
              i32.add
              i64.load
              i64.store
              local.get $l1
              local.get $l1
              i64.load offset=8
              i64.store offset=72
              local.get $l1
              i32.const 64
              i32.add
              local.get $l1
              i32.const 48
              i32.add
              local.get $l1
              i32.const 72
              i32.add
              call $_ZN57_$LT$std..io..stdio..Stdout$u20$as$u20$std..io..Write$GT$9write_fmt17h216a282b76b418a8E
              local.get $p0
              local.get $p0
              i32.load
              local.tee $l2
              i32.const -1
              i32.add
              i32.store
              block $B7
                local.get $l2
                i32.const 1
                i32.ne
                br_if $B7
                local.get $l1
                i32.const 48
                i32.add
                call $_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17h497a468b6f1c6d8eE
              end
              local.get $l1
              i64.load offset=64
              local.set $l3
              br $B3
            end
            i32.const 0
            i32.load offset=1054816
            local.set $l2
            local.get $l1
            i32.const 72
            i32.add
            i32.const 16
            i32.add
            local.get $l1
            i32.const 8
            i32.add
            i32.const 16
            i32.add
            i64.load
            i64.store
            local.get $l1
            i32.const 72
            i32.add
            i32.const 8
            i32.add
            local.get $l1
            i32.const 8
            i32.add
            i32.const 8
            i32.add
            i64.load
            i64.store
            local.get $l1
            local.get $l1
            i64.load offset=8
            i64.store offset=72
            local.get $l1
            i32.const 48
            i32.add
            local.get $p0
            local.get $l1
            i32.const 72
            i32.add
            local.get $l2
            i32.load offset=36
            call_indirect (type $t4) $T0
            i32.const 0
            i32.load offset=1054808
            br_if $B1
            i32.const 0
            i32.const -1
            i32.store offset=1054808
            block $B8
              i32.const 0
              i32.load offset=1054812
              local.tee $l4
              i32.eqz
              br_if $B8
              local.get $l4
              i32.const 0
              i32.load offset=1054816
              i32.load
              call_indirect (type $t1) $T0
              i32.const 0
              i32.load offset=1054816
              local.tee $l4
              i32.load offset=4
              local.tee $l5
              i32.eqz
              br_if $B8
              i32.const 0
              i32.load offset=1054812
              local.get $l5
              local.get $l4
              i32.load offset=8
              call $__rust_dealloc
            end
            i32.const 0
            local.get $l2
            i32.store offset=1054816
            i32.const 0
            local.get $p0
            i32.store offset=1054812
            i32.const 0
            i32.const 0
            i32.load offset=1054808
            i32.const 1
            i32.add
            i32.store offset=1054808
            local.get $l1
            local.get $l1
            i64.load offset=48
            local.tee $l3
            i64.store offset=64
          end
          block $B9
            block $B10
              local.get $l3
              i32.wrap_i64
              local.tee $p0
              i32.const 255
              i32.and
              i32.const 4
              i32.ne
              br_if $B10
              local.get $l1
              call $_ZN3std2io5stdio6stdout17h412a99e1c61b9f7cE
              local.tee $p0
              i32.store offset=48
              local.get $l1
              i32.const 72
              i32.add
              i32.const 16
              i32.add
              local.get $l1
              i32.const 8
              i32.add
              i32.const 16
              i32.add
              i64.load
              i64.store
              local.get $l1
              i32.const 72
              i32.add
              i32.const 8
              i32.add
              local.get $l1
              i32.const 8
              i32.add
              i32.const 8
              i32.add
              i64.load
              i64.store
              local.get $l1
              local.get $l1
              i64.load offset=8
              i64.store offset=72
              local.get $l1
              i32.const 40
              i32.add
              local.get $l1
              i32.const 48
              i32.add
              local.get $l1
              i32.const 72
              i32.add
              call $_ZN57_$LT$std..io..stdio..Stdout$u20$as$u20$std..io..Write$GT$9write_fmt17h216a282b76b418a8E
              local.get $p0
              local.get $p0
              i32.load
              local.tee $l2
              i32.const -1
              i32.add
              i32.store
              block $B11
                local.get $l2
                i32.const 1
                i32.ne
                br_if $B11
                local.get $l1
                i32.const 48
                i32.add
                call $_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17h497a468b6f1c6d8eE
              end
              local.get $l1
              i32.load8_u offset=40
              local.tee $l2
              local.set $p0
              br $B9
            end
            local.get $l1
            local.get $l3
            i64.store offset=40
            local.get $l3
            i32.wrap_i64
            local.set $l2
          end
          local.get $p0
          i32.const 255
          i32.and
          i32.const 3
          i32.ne
          br_if $B0
          block $B12
            block $B13
              i32.const 0
              br_if $B13
              local.get $l2
              i32.const 3
              i32.and
              i32.const 2
              i32.ne
              br_if $B12
            end
            local.get $l1
            i32.load offset=44
            local.tee $p0
            i32.load
            local.get $p0
            i32.load offset=4
            i32.load
            call_indirect (type $t1) $T0
            block $B14
              local.get $p0
              i32.load offset=4
              local.tee $l2
              i32.load offset=4
              local.tee $l4
              i32.eqz
              br_if $B14
              local.get $p0
              i32.load
              local.get $l4
              local.get $l2
              i32.load offset=8
              call $__rust_dealloc
            end
            local.get $p0
            i32.const 12
            i32.const 4
            call $__rust_dealloc
          end
          local.get $l1
          i32.const 96
          i32.add
          global.set $g0
          return
        end
        i32.const 1048768
        i32.const 16
        local.get $l1
        i32.const 72
        i32.add
        i32.const 1049004
        i32.const 1050116
        call $_ZN4core6option18expect_none_failed17h6878b94853f7ecf6E
        unreachable
      end
      i32.const 1048768
      i32.const 16
      local.get $l1
      i32.const 72
      i32.add
      i32.const 1049004
      i32.const 1050132
      call $_ZN4core6option18expect_none_failed17h6878b94853f7ecf6E
      unreachable
    end
    local.get $l1
    local.get $l1
    i64.load offset=40
    i64.store offset=64
    local.get $l1
    i32.const 92
    i32.add
    i32.const 2
    i32.store
    local.get $l1
    i32.const 60
    i32.add
    i32.const 10
    i32.store
    local.get $l1
    i64.const 2
    i64.store offset=76 align=4
    local.get $l1
    i32.const 1050084
    i32.store offset=72
    local.get $l1
    i32.const 7
    i32.store offset=52
    local.get $l1
    local.get $l1
    i32.const 48
    i32.add
    i32.store offset=88
    local.get $l1
    local.get $l1
    i32.const 64
    i32.add
    i32.store offset=56
    local.get $l1
    local.get $l1
    i32.const 32
    i32.add
    i32.store offset=48
    local.get $l1
    i32.const 72
    i32.add
    i32.const 1050100
    call $_ZN3std9panicking15begin_panic_fmt17h81d6d5fcf87a41f9E
    unreachable)
  (func $_ZN80_$LT$std..io..Write..write_fmt..Adaptor$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h4a28ff28c6c8fdc2E (type $t6) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    (local $l3 i32) (local $l4 i64) (local $l5 i32)
    global.get $g0
    i32.const 16
    i32.sub
    local.tee $l3
    global.set $g0
    local.get $l3
    i32.const 8
    i32.add
    local.get $p0
    i32.load
    local.get $p1
    local.get $p2
    call $_ZN61_$LT$std..io..stdio..StdoutLock$u20$as$u20$std..io..Write$GT$9write_all17hd421315622c6dec2E
    i32.const 0
    local.set $p1
    block $B0
      local.get $l3
      i32.load8_u offset=8
      i32.const 3
      i32.eq
      br_if $B0
      local.get $l3
      i64.load offset=8
      local.set $l4
      block $B1
        block $B2
          i32.const 0
          br_if $B2
          local.get $p0
          i32.load8_u offset=4
          i32.const 2
          i32.ne
          br_if $B1
        end
        local.get $p0
        i32.const 8
        i32.add
        i32.load
        local.tee $p1
        i32.load
        local.get $p1
        i32.load offset=4
        i32.load
        call_indirect (type $t1) $T0
        block $B3
          local.get $p1
          i32.load offset=4
          local.tee $p2
          i32.load offset=4
          local.tee $l5
          i32.eqz
          br_if $B3
          local.get $p1
          i32.load
          local.get $l5
          local.get $p2
          i32.load offset=8
          call $__rust_dealloc
        end
        local.get $p0
        i32.load offset=8
        i32.const 12
        i32.const 4
        call $__rust_dealloc
      end
      local.get $p0
      local.get $l4
      i64.store offset=4 align=4
      i32.const 1
      local.set $p1
    end
    local.get $l3
    i32.const 16
    i32.add
    global.set $g0
    local.get $p1)
  (func $_ZN70_$LT$std..sync..once..WaiterQueue$u20$as$u20$core..ops..drop..Drop$GT$4drop17hba24087bb7983537E (type $t1) (param $p0 i32)
    (local $l1 i32) (local $l2 i32) (local $l3 i32)
    global.get $g0
    i32.const 64
    i32.sub
    local.tee $l1
    global.set $g0
    local.get $p0
    i32.load
    local.tee $l2
    i32.load
    local.set $l3
    local.get $l2
    local.get $p0
    i32.load offset=4
    i32.store
    local.get $l1
    local.get $l3
    i32.const 3
    i32.and
    local.tee $p0
    i32.store offset=12
    block $B0
      local.get $p0
      i32.const 2
      i32.ne
      br_if $B0
      block $B1
        block $B2
          local.get $l3
          i32.const -4
          i32.and
          local.tee $p0
          i32.eqz
          br_if $B2
          loop $L3
            local.get $p0
            i32.load offset=4
            local.set $l3
            local.get $p0
            i32.load
            local.set $l2
            local.get $p0
            i32.const 0
            i32.store
            local.get $l2
            i32.eqz
            br_if $B1
            local.get $p0
            i32.const 1
            i32.store8 offset=8
            local.get $l1
            local.get $l2
            i32.store offset=16
            local.get $l1
            i32.const 16
            i32.add
            call $_ZN3std6thread6Thread6unpark17hd2c776a4f99f2b06E
            local.get $l1
            i32.load offset=16
            local.tee $p0
            local.get $p0
            i32.load
            local.tee $p0
            i32.const -1
            i32.add
            i32.store
            block $B4
              local.get $p0
              i32.const 1
              i32.ne
              br_if $B4
              local.get $l1
              i32.const 16
              i32.add
              call $_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17h01bd736b3b3e1565E
            end
            local.get $l3
            local.set $p0
            local.get $l3
            br_if $L3
          end
        end
        local.get $l1
        i32.const 64
        i32.add
        global.set $g0
        return
      end
      i32.const 1048960
      i32.const 43
      i32.const 1050516
      call $_ZN4core9panicking5panic17hc886a4cb4479b06eE
      unreachable
    end
    local.get $l1
    i32.const 52
    i32.add
    i32.const 6
    i32.store
    local.get $l1
    i32.const 36
    i32.add
    i32.const 2
    i32.store
    local.get $l1
    i64.const 3
    i64.store offset=20 align=4
    local.get $l1
    i32.const 1048868
    i32.store offset=16
    local.get $l1
    i32.const 6
    i32.store offset=44
    local.get $l1
    local.get $l1
    i32.const 12
    i32.add
    i32.store offset=56
    local.get $l1
    i32.const 1049216
    i32.store offset=60
    local.get $l1
    local.get $l1
    i32.const 40
    i32.add
    i32.store offset=32
    local.get $l1
    local.get $l1
    i32.const 60
    i32.add
    i32.store offset=48
    local.get $l1
    local.get $l1
    i32.const 56
    i32.add
    i32.store offset=40
    local.get $l1
    i32.const 16
    i32.add
    i32.const 1050500
    call $_ZN3std9panicking15begin_panic_fmt17h81d6d5fcf87a41f9E
    unreachable)
  (func $_ZN3std10sys_common9backtrace26__rust_end_short_backtrace17h4433fb1f839a5629E (type $t1) (param $p0 i32)
    (local $l1 i32)
    global.get $g0
    i32.const 16
    i32.sub
    local.tee $l1
    global.set $g0
    local.get $l1
    i32.const 8
    i32.add
    local.get $p0
    i32.const 8
    i32.add
    i32.load
    i32.store
    local.get $l1
    local.get $p0
    i64.load align=4
    i64.store
    local.get $l1
    call $_ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17he77c6dd87d797d7bE
    unreachable)
  (func $_ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17he77c6dd87d797d7bE (type $t1) (param $p0 i32)
    (local $l1 i32)
    global.get $g0
    i32.const 16
    i32.sub
    local.tee $l1
    global.set $g0
    local.get $l1
    i32.const 0
    i32.store offset=4
    local.get $l1
    local.get $p0
    i32.load
    i32.store
    local.get $l1
    i32.const 1050900
    local.get $p0
    i32.load offset=4
    call $_ZN4core5panic9PanicInfo7message17h1ce7bd5bc7e6939cE
    local.get $p0
    i32.load offset=8
    call $_ZN3std9panicking20rust_panic_with_hook17hd5d2d56648f30906E
    unreachable)
  (func $_ZN3std10sys_common9backtrace26__rust_end_short_backtrace17hdeeb31f429323cb3E (type $t1) (param $p0 i32)
    (local $l1 i32)
    global.get $g0
    i32.const 16
    i32.sub
    local.tee $l1
    global.set $g0
    local.get $l1
    i32.const 8
    i32.add
    local.get $p0
    i32.const 8
    i32.add
    i32.load
    i32.store
    local.get $l1
    local.get $p0
    i64.load align=4
    i64.store
    local.get $l1
    call $_ZN3std9panicking11begin_panic28_$u7b$$u7b$closure$u7d$$u7d$17h149036b37029a3d4E
    unreachable)
  (func $_ZN3std9panicking11begin_panic28_$u7b$$u7b$closure$u7d$$u7d$17h149036b37029a3d4E (type $t1) (param $p0 i32)
    (local $l1 i32)
    global.get $g0
    i32.const 16
    i32.sub
    local.tee $l1
    global.set $g0
    local.get $l1
    local.get $p0
    i64.load align=4
    i64.store offset=8
    local.get $l1
    i32.const 8
    i32.add
    i32.const 1050936
    i32.const 0
    local.get $p0
    i32.load offset=8
    call $_ZN3std9panicking20rust_panic_with_hook17hd5d2d56648f30906E
    unreachable)
  (func $_ZN3std3sys4wasm7condvar7Condvar4wait17h610ae0be866fd8c5E (type $t3) (param $p0 i32) (param $p1 i32)
    i32.const 1051016
    i32.const 26
    i32.const 1051092
    call $_ZN3std9panicking11begin_panic17h90326787ac4041daE
    unreachable)
  (func $_ZN82_$LT$std..sys_common..poison..PoisonError$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h063158582f775d68E (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    i32.const 1050620
    i32.const 25
    local.get $p1
    call $_ZN40_$LT$str$u20$as$u20$core..fmt..Debug$GT$3fmt17h4e37a0e4f747f286E)
  (func $_ZN3std5alloc24default_alloc_error_hook17hc03eb1d26ecad9f0E (type $t3) (param $p0 i32) (param $p1 i32))
  (func $rust_oom (type $t3) (param $p0 i32) (param $p1 i32)
    (local $l2 i32)
    local.get $p0
    local.get $p1
    i32.const 0
    i32.load offset=1054784
    local.tee $l2
    i32.const 11
    local.get $l2
    select
    call_indirect (type $t3) $T0
    unreachable
    unreachable)
  (func $__rdl_alloc (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    block $B0
      i32.const 1054832
      call $_ZN8dlmalloc8dlmalloc8Dlmalloc16malloc_alignment17hc214b6a181f683cdE
      local.get $p1
      i32.ge_u
      br_if $B0
      i32.const 1054832
      local.get $p1
      local.get $p0
      call $_ZN8dlmalloc8dlmalloc8Dlmalloc8memalign17hf8d339f9992b4eadE
      return
    end
    i32.const 1054832
    local.get $p0
    call $_ZN8dlmalloc8dlmalloc8Dlmalloc6malloc17hb0329e71e24f7e2fE)
  (func $__rdl_dealloc (type $t4) (param $p0 i32) (param $p1 i32) (param $p2 i32)
    i32.const 1054832
    local.get $p0
    call $_ZN8dlmalloc8dlmalloc8Dlmalloc4free17h7ab57ecacfa2b1c3E)
  (func $__rdl_realloc (type $t9) (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32) (result i32)
    block $B0
      block $B1
        i32.const 1054832
        call $_ZN8dlmalloc8dlmalloc8Dlmalloc16malloc_alignment17hc214b6a181f683cdE
        local.get $p2
        i32.ge_u
        br_if $B1
        block $B2
          block $B3
            i32.const 1054832
            call $_ZN8dlmalloc8dlmalloc8Dlmalloc16malloc_alignment17hc214b6a181f683cdE
            local.get $p2
            i32.ge_u
            br_if $B3
            i32.const 1054832
            local.get $p2
            local.get $p3
            call $_ZN8dlmalloc8dlmalloc8Dlmalloc8memalign17hf8d339f9992b4eadE
            local.set $p2
            br $B2
          end
          i32.const 1054832
          local.get $p3
          call $_ZN8dlmalloc8dlmalloc8Dlmalloc6malloc17hb0329e71e24f7e2fE
          local.set $p2
        end
        local.get $p2
        br_if $B0
        i32.const 0
        return
      end
      i32.const 1054832
      local.get $p0
      local.get $p3
      call $_ZN8dlmalloc8dlmalloc8Dlmalloc7realloc17h1e42fbdcdf2a4cf4E
      return
    end
    local.get $p2
    local.get $p0
    local.get $p3
    local.get $p1
    local.get $p1
    local.get $p3
    i32.gt_u
    select
    call $memcpy
    local.set $p2
    i32.const 1054832
    local.get $p0
    call $_ZN8dlmalloc8dlmalloc8Dlmalloc4free17h7ab57ecacfa2b1c3E
    local.get $p2)
  (func $rust_begin_unwind (type $t1) (param $p0 i32)
    (local $l1 i32) (local $l2 i32) (local $l3 i32)
    global.get $g0
    i32.const 16
    i32.sub
    local.tee $l1
    global.set $g0
    local.get $p0
    call $_ZN4core5panic9PanicInfo8location17h96ba60a01800530cE
    i32.const 1050868
    call $_ZN4core6option15Option$LT$T$GT$6unwrap17h6c4ab1581d7c7209E
    local.set $l2
    local.get $p0
    call $_ZN4core5panic9PanicInfo7message17h1ce7bd5bc7e6939cE
    call $_ZN4core6option15Option$LT$T$GT$6unwrap17hb5ee9bfe6199eb07E
    local.set $l3
    local.get $l1
    local.get $l2
    i32.store offset=8
    local.get $l1
    local.get $p0
    i32.store offset=4
    local.get $l1
    local.get $l3
    i32.store
    local.get $l1
    call $_ZN3std10sys_common9backtrace26__rust_end_short_backtrace17h4433fb1f839a5629E
    unreachable)
  (func $_ZN3std9panicking20rust_panic_with_hook17hd5d2d56648f30906E (type $t11) (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32)
    (local $l4 i32) (local $l5 i32)
    global.get $g0
    i32.const 32
    i32.sub
    local.tee $l4
    global.set $g0
    i32.const 1
    local.set $l5
    i32.const 0
    i32.const 0
    i32.load offset=1054800
    i32.const 1
    i32.add
    i32.store offset=1054800
    block $B0
      block $B1
        block $B2
          block $B3
            i32.const 0
            i32.load offset=1055288
            i32.const 1
            i32.eq
            br_if $B3
            i32.const 0
            i64.const 4294967297
            i64.store offset=1055288
            br $B2
          end
          i32.const 0
          i32.const 0
          i32.load offset=1055292
          i32.const 1
          i32.add
          local.tee $l5
          i32.store offset=1055292
          local.get $l5
          i32.const 2
          i32.gt_u
          br_if $B1
        end
        local.get $l4
        local.get $p3
        i32.store offset=28
        local.get $l4
        local.get $p2
        i32.store offset=24
        local.get $l4
        i32.const 1048892
        i32.store offset=20
        local.get $l4
        i32.const 1048892
        i32.store offset=16
        i32.const 0
        i32.load offset=1054788
        local.tee $p2
        i32.const -1
        i32.le_s
        br_if $B1
        i32.const 0
        local.get $p2
        i32.const 1
        i32.add
        local.tee $p2
        i32.store offset=1054788
        block $B4
          i32.const 0
          i32.load offset=1054796
          local.tee $p3
          i32.eqz
          br_if $B4
          i32.const 0
          i32.load offset=1054792
          local.set $p2
          local.get $l4
          i32.const 8
          i32.add
          local.get $p0
          local.get $p1
          i32.load offset=16
          call_indirect (type $t3) $T0
          local.get $l4
          local.get $l4
          i64.load offset=8
          i64.store offset=16
          local.get $p2
          local.get $l4
          i32.const 16
          i32.add
          local.get $p3
          i32.load offset=12
          call_indirect (type $t3) $T0
          i32.const 0
          i32.load offset=1054788
          local.set $p2
        end
        i32.const 0
        local.get $p2
        i32.const -1
        i32.add
        i32.store offset=1054788
        local.get $l5
        i32.const 1
        i32.le_u
        br_if $B0
      end
      unreachable
      unreachable
    end
    local.get $p0
    local.get $p1
    call $rust_panic
    unreachable)
  (func $_ZN90_$LT$std..panicking..begin_panic_handler..PanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$8take_box17h327c8a118334ec23E (type $t3) (param $p0 i32) (param $p1 i32)
    (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32)
    global.get $g0
    i32.const 64
    i32.sub
    local.tee $l2
    global.set $g0
    block $B0
      local.get $p1
      i32.load offset=4
      local.tee $l3
      br_if $B0
      local.get $p1
      i32.const 4
      i32.add
      local.set $l3
      local.get $p1
      i32.load
      local.set $l4
      local.get $l2
      i32.const 0
      i32.store offset=32
      local.get $l2
      i64.const 1
      i64.store offset=24
      local.get $l2
      local.get $l2
      i32.const 24
      i32.add
      i32.store offset=36
      local.get $l2
      i32.const 40
      i32.add
      i32.const 16
      i32.add
      local.get $l4
      i32.const 16
      i32.add
      i64.load align=4
      i64.store
      local.get $l2
      i32.const 40
      i32.add
      i32.const 8
      i32.add
      local.get $l4
      i32.const 8
      i32.add
      i64.load align=4
      i64.store
      local.get $l2
      local.get $l4
      i64.load align=4
      i64.store offset=40
      local.get $l2
      i32.const 36
      i32.add
      i32.const 1048728
      local.get $l2
      i32.const 40
      i32.add
      call $_ZN4core3fmt5write17hb395f946a5ce2cabE
      drop
      local.get $l2
      i32.const 8
      i32.add
      i32.const 8
      i32.add
      local.tee $l4
      local.get $l2
      i32.load offset=32
      i32.store
      local.get $l2
      local.get $l2
      i64.load offset=24
      i64.store offset=8
      block $B1
        local.get $p1
        i32.load offset=4
        local.tee $l5
        i32.eqz
        br_if $B1
        local.get $p1
        i32.const 8
        i32.add
        i32.load
        local.tee $l6
        i32.eqz
        br_if $B1
        local.get $l5
        local.get $l6
        i32.const 1
        call $__rust_dealloc
      end
      local.get $l3
      local.get $l2
      i64.load offset=8
      i64.store align=4
      local.get $l3
      i32.const 8
      i32.add
      local.get $l4
      i32.load
      i32.store
      local.get $l3
      i32.load
      local.set $l3
    end
    local.get $p1
    i32.const 1
    i32.store offset=4
    local.get $p1
    i32.const 12
    i32.add
    i32.load
    local.set $l4
    local.get $p1
    i32.const 8
    i32.add
    local.tee $p1
    i32.load
    local.set $l5
    local.get $p1
    i64.const 0
    i64.store align=4
    block $B2
      i32.const 12
      i32.const 4
      call $__rust_alloc
      local.tee $p1
      br_if $B2
      i32.const 12
      i32.const 4
      call $_ZN5alloc5alloc18handle_alloc_error17h02613b455f15ef67E
      unreachable
    end
    local.get $p1
    local.get $l4
    i32.store offset=8
    local.get $p1
    local.get $l5
    i32.store offset=4
    local.get $p1
    local.get $l3
    i32.store
    local.get $p0
    i32.const 1050920
    i32.store offset=4
    local.get $p0
    local.get $p1
    i32.store
    local.get $l2
    i32.const 64
    i32.add
    global.set $g0)
  (func $_ZN90_$LT$std..panicking..begin_panic_handler..PanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$3get17he829a193e7bd084bE (type $t3) (param $p0 i32) (param $p1 i32)
    (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32)
    global.get $g0
    i32.const 64
    i32.sub
    local.tee $l2
    global.set $g0
    local.get $p1
    i32.const 4
    i32.add
    local.set $l3
    block $B0
      local.get $p1
      i32.load offset=4
      br_if $B0
      local.get $p1
      i32.load
      local.set $l4
      local.get $l2
      i32.const 0
      i32.store offset=32
      local.get $l2
      i64.const 1
      i64.store offset=24
      local.get $l2
      local.get $l2
      i32.const 24
      i32.add
      i32.store offset=36
      local.get $l2
      i32.const 40
      i32.add
      i32.const 16
      i32.add
      local.get $l4
      i32.const 16
      i32.add
      i64.load align=4
      i64.store
      local.get $l2
      i32.const 40
      i32.add
      i32.const 8
      i32.add
      local.get $l4
      i32.const 8
      i32.add
      i64.load align=4
      i64.store
      local.get $l2
      local.get $l4
      i64.load align=4
      i64.store offset=40
      local.get $l2
      i32.const 36
      i32.add
      i32.const 1048728
      local.get $l2
      i32.const 40
      i32.add
      call $_ZN4core3fmt5write17hb395f946a5ce2cabE
      drop
      local.get $l2
      i32.const 8
      i32.add
      i32.const 8
      i32.add
      local.tee $l4
      local.get $l2
      i32.load offset=32
      i32.store
      local.get $l2
      local.get $l2
      i64.load offset=24
      i64.store offset=8
      block $B1
        local.get $p1
        i32.load offset=4
        local.tee $l5
        i32.eqz
        br_if $B1
        local.get $p1
        i32.const 8
        i32.add
        i32.load
        local.tee $p1
        i32.eqz
        br_if $B1
        local.get $l5
        local.get $p1
        i32.const 1
        call $__rust_dealloc
      end
      local.get $l3
      local.get $l2
      i64.load offset=8
      i64.store align=4
      local.get $l3
      i32.const 8
      i32.add
      local.get $l4
      i32.load
      i32.store
    end
    local.get $p0
    i32.const 1050920
    i32.store offset=4
    local.get $p0
    local.get $l3
    i32.store
    local.get $l2
    i32.const 64
    i32.add
    global.set $g0)
  (func $_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$8take_box17h3148b39bb58acd57E (type $t3) (param $p0 i32) (param $p1 i32)
    (local $l2 i32) (local $l3 i32)
    local.get $p1
    i32.load
    local.set $l2
    local.get $p1
    i32.const 0
    i32.store
    block $B0
      block $B1
        local.get $l2
        i32.eqz
        br_if $B1
        local.get $p1
        i32.load offset=4
        local.set $l3
        i32.const 8
        i32.const 4
        call $__rust_alloc
        local.tee $p1
        i32.eqz
        br_if $B0
        local.get $p1
        local.get $l3
        i32.store offset=4
        local.get $p1
        local.get $l2
        i32.store
        local.get $p0
        i32.const 1050956
        i32.store offset=4
        local.get $p0
        local.get $p1
        i32.store
        return
      end
      unreachable
      unreachable
    end
    i32.const 8
    i32.const 4
    call $_ZN5alloc5alloc18handle_alloc_error17h02613b455f15ef67E
    unreachable)
  (func $_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$3get17hdc67d47d21f85001E (type $t3) (param $p0 i32) (param $p1 i32)
    block $B0
      local.get $p1
      i32.load
      br_if $B0
      unreachable
      unreachable
    end
    local.get $p0
    i32.const 1050956
    i32.store offset=4
    local.get $p0
    local.get $p1
    i32.store)
  (func $rust_panic (type $t3) (param $p0 i32) (param $p1 i32)
    (local $l2 i32)
    global.get $g0
    i32.const 16
    i32.sub
    local.tee $l2
    global.set $g0
    local.get $l2
    local.get $p1
    i32.store offset=12
    local.get $l2
    local.get $p0
    i32.store offset=8
    local.get $l2
    i32.const 8
    i32.add
    call $__rust_start_panic
    drop
    unreachable
    unreachable)
  (func $_ZN3std2rt19lang_start_internal17h49e742537b17034cE (type $t9) (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32) (result i32)
    (local $l4 i32) (local $l5 i32) (local $l6 i32)
    global.get $g0
    i32.const 16
    i32.sub
    local.tee $l4
    global.set $g0
    block $B0
      block $B1
        block $B2
          block $B3
            i32.const 4
            i32.const 1
            call $__rust_alloc
            local.tee $l5
            i32.eqz
            br_if $B3
            local.get $l5
            i32.const 1852399981
            i32.store align=1
            local.get $l4
            i64.const 17179869188
            i64.store offset=4 align=4
            local.get $l4
            local.get $l5
            i32.store
            local.get $l4
            call $_ZN3std6thread6Thread3new17h0ad4b740297a0352E
            local.set $l5
            block $B4
              block $B5
                i32.const 0
                i32.load offset=1054820
                i32.const 1
                i32.eq
                br_if $B5
                i32.const 0
                i64.const 1
                i64.store offset=1054820 align=4
                i32.const 0
                i32.const 0
                i32.store offset=1054828
                br $B4
              end
              i32.const 0
              i32.load offset=1054824
              local.tee $l6
              i32.const 1
              i32.add
              i32.const 0
              i32.le_s
              br_if $B2
              i32.const 0
              i32.load offset=1054828
              br_if $B1
              local.get $l6
              br_if $B0
            end
            i32.const 0
            local.get $l5
            i32.store offset=1054828
            i32.const 0
            i32.const 0
            i32.store offset=1054824
            local.get $p0
            local.get $p1
            i32.load offset=12
            call_indirect (type $t5) $T0
            local.set $l5
            block $B6
              i32.const 0
              i32.load offset=1054780
              i32.const 3
              i32.eq
              br_if $B6
              local.get $l4
              i32.const 1
              i32.store8 offset=15
              local.get $l4
              local.get $l4
              i32.const 15
              i32.add
              i32.store
              i32.const 1054780
              i32.const 0
              local.get $l4
              i32.const 1050300
              call $_ZN3std4sync4once4Once10call_inner17h154489a5744708e1E
            end
            local.get $l4
            i32.const 16
            i32.add
            global.set $g0
            local.get $l5
            return
          end
          i32.const 4
          i32.const 1
          call $_ZN5alloc5alloc18handle_alloc_error17h02613b455f15ef67E
          unreachable
        end
        i32.const 1048784
        i32.const 24
        local.get $l4
        i32.const 1049020
        i32.const 1050736
        call $_ZN4core6option18expect_none_failed17h6878b94853f7ecf6E
        unreachable
      end
      i32.const 1050752
      i32.const 38
      i32.const 1050792
      call $_ZN3std9panicking11begin_panic17h90326787ac4041daE
      unreachable
    end
    i32.const 1048768
    i32.const 16
    local.get $l4
    i32.const 1049004
    i32.const 1050808
    call $_ZN4core6option18expect_none_failed17h6878b94853f7ecf6E
    unreachable)
  (func $_ZN62_$LT$std..ffi..c_str..NulError$u20$as$u20$core..fmt..Debug$GT$3fmt17hdbe9560da47ba080E (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32)
    global.get $g0
    i32.const 16
    i32.sub
    local.tee $l2
    global.set $g0
    local.get $l2
    local.get $p1
    i32.const 1050972
    i32.const 8
    call $_ZN4core3fmt9Formatter11debug_tuple17h242798767252cce4E
    local.get $l2
    local.get $p0
    i32.store offset=12
    local.get $l2
    local.get $l2
    i32.const 12
    i32.add
    i32.const 1049112
    call $_ZN4core3fmt8builders10DebugTuple5field17h6c7d284ba7c32ea1E
    drop
    local.get $l2
    local.get $p0
    i32.const 4
    i32.add
    i32.store offset=12
    local.get $l2
    local.get $l2
    i32.const 12
    i32.add
    i32.const 1050980
    call $_ZN4core3fmt8builders10DebugTuple5field17h6c7d284ba7c32ea1E
    drop
    local.get $l2
    call $_ZN4core3fmt8builders10DebugTuple6finish17h6ed5b55943d7a61eE
    local.set $p0
    local.get $l2
    i32.const 16
    i32.add
    global.set $g0
    local.get $p0)
  (func $_ZN3std3sys4wasm7process8ExitCode6as_i3217h08794ec4286789ffE (type $t5) (param $p0 i32) (result i32)
    local.get $p0
    i32.load8_u)
  (func $__rust_start_panic (type $t5) (param $p0 i32) (result i32)
    unreachable
    unreachable)
  (func $_ZN8dlmalloc8dlmalloc8Dlmalloc16malloc_alignment17hc214b6a181f683cdE (type $t5) (param $p0 i32) (result i32)
    i32.const 8)
  (func $_ZN8dlmalloc8dlmalloc8Dlmalloc6malloc17hb0329e71e24f7e2fE (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32) (local $l10 i64)
    block $B0
      block $B1
        block $B2
          block $B3
            block $B4
              local.get $p1
              i32.const 245
              i32.lt_u
              br_if $B4
              i32.const 0
              local.set $l2
              local.get $p1
              i32.const -65587
              i32.ge_u
              br_if $B0
              local.get $p1
              i32.const 11
              i32.add
              local.tee $p1
              i32.const -8
              i32.and
              local.set $l3
              local.get $p0
              i32.const 4
              i32.add
              i32.load
              local.tee $l4
              i32.eqz
              br_if $B3
              i32.const 0
              local.set $l5
              block $B5
                local.get $p1
                i32.const 8
                i32.shr_u
                local.tee $p1
                i32.eqz
                br_if $B5
                i32.const 31
                local.set $l5
                local.get $l3
                i32.const 16777215
                i32.gt_u
                br_if $B5
                local.get $l3
                i32.const 6
                local.get $p1
                i32.clz
                local.tee $p1
                i32.sub
                i32.const 31
                i32.and
                i32.shr_u
                i32.const 1
                i32.and
                local.get $p1
                i32.const 1
                i32.shl
                i32.sub
                i32.const 62
                i32.add
                local.set $l5
              end
              i32.const 0
              local.get $l3
              i32.sub
              local.set $l2
              block $B6
                block $B7
                  block $B8
                    local.get $p0
                    local.get $l5
                    i32.const 2
                    i32.shl
                    i32.add
                    i32.const 272
                    i32.add
                    i32.load
                    local.tee $p1
                    i32.eqz
                    br_if $B8
                    i32.const 0
                    local.set $l6
                    local.get $l3
                    i32.const 0
                    i32.const 25
                    local.get $l5
                    i32.const 1
                    i32.shr_u
                    i32.sub
                    i32.const 31
                    i32.and
                    local.get $l5
                    i32.const 31
                    i32.eq
                    select
                    i32.shl
                    local.set $l7
                    i32.const 0
                    local.set $l8
                    loop $L9
                      block $B10
                        local.get $p1
                        i32.const 4
                        i32.add
                        i32.load
                        i32.const -8
                        i32.and
                        local.tee $l9
                        local.get $l3
                        i32.lt_u
                        br_if $B10
                        local.get $l9
                        local.get $l3
                        i32.sub
                        local.tee $l9
                        local.get $l2
                        i32.ge_u
                        br_if $B10
                        local.get $l9
                        local.set $l2
                        local.get $p1
                        local.set $l8
                        local.get $l9
                        br_if $B10
                        i32.const 0
                        local.set $l2
                        local.get $p1
                        local.set $l8
                        br $B7
                      end
                      local.get $p1
                      i32.const 20
                      i32.add
                      i32.load
                      local.tee $l9
                      local.get $l6
                      local.get $l9
                      local.get $p1
                      local.get $l7
                      i32.const 29
                      i32.shr_u
                      i32.const 4
                      i32.and
                      i32.add
                      i32.const 16
                      i32.add
                      i32.load
                      local.tee $p1
                      i32.ne
                      select
                      local.get $l6
                      local.get $l9
                      select
                      local.set $l6
                      local.get $l7
                      i32.const 1
                      i32.shl
                      local.set $l7
                      local.get $p1
                      br_if $L9
                    end
                    block $B11
                      local.get $l6
                      i32.eqz
                      br_if $B11
                      local.get $l6
                      local.set $p1
                      br $B7
                    end
                    local.get $l8
                    br_if $B6
                  end
                  i32.const 0
                  local.set $l8
                  i32.const 2
                  local.get $l5
                  i32.const 31
                  i32.and
                  i32.shl
                  local.tee $p1
                  i32.const 0
                  local.get $p1
                  i32.sub
                  i32.or
                  local.get $l4
                  i32.and
                  local.tee $p1
                  i32.eqz
                  br_if $B3
                  local.get $p0
                  local.get $p1
                  i32.const 0
                  local.get $p1
                  i32.sub
                  i32.and
                  i32.ctz
                  i32.const 2
                  i32.shl
                  i32.add
                  i32.const 272
                  i32.add
                  i32.load
                  local.tee $p1
                  i32.eqz
                  br_if $B3
                end
                loop $L12
                  local.get $p1
                  i32.const 4
                  i32.add
                  i32.load
                  i32.const -8
                  i32.and
                  local.tee $l6
                  local.get $l3
                  i32.ge_u
                  local.get $l6
                  local.get $l3
                  i32.sub
                  local.tee $l9
                  local.get $l2
                  i32.lt_u
                  i32.and
                  local.set $l7
                  block $B13
                    local.get $p1
                    i32.load offset=16
                    local.tee $l6
                    br_if $B13
                    local.get $p1
                    i32.const 20
                    i32.add
                    i32.load
                    local.set $l6
                  end
                  local.get $p1
                  local.get $l8
                  local.get $l7
                  select
                  local.set $l8
                  local.get $l9
                  local.get $l2
                  local.get $l7
                  select
                  local.set $l2
                  local.get $l6
                  local.set $p1
                  local.get $l6
                  br_if $L12
                end
                local.get $l8
                i32.eqz
                br_if $B3
              end
              block $B14
                local.get $p0
                i32.load offset=400
                local.tee $p1
                local.get $l3
                i32.lt_u
                br_if $B14
                local.get $l2
                local.get $p1
                local.get $l3
                i32.sub
                i32.ge_u
                br_if $B3
              end
              local.get $l8
              i32.load offset=24
              local.set $l5
              block $B15
                block $B16
                  block $B17
                    local.get $l8
                    i32.load offset=12
                    local.tee $l6
                    local.get $l8
                    i32.ne
                    br_if $B17
                    local.get $l8
                    i32.const 20
                    i32.const 16
                    local.get $l8
                    i32.const 20
                    i32.add
                    local.tee $l6
                    i32.load
                    local.tee $l7
                    select
                    i32.add
                    i32.load
                    local.tee $p1
                    br_if $B16
                    i32.const 0
                    local.set $l6
                    br $B15
                  end
                  local.get $l8
                  i32.load offset=8
                  local.tee $p1
                  local.get $l6
                  i32.store offset=12
                  local.get $l6
                  local.get $p1
                  i32.store offset=8
                  br $B15
                end
                local.get $l6
                local.get $l8
                i32.const 16
                i32.add
                local.get $l7
                select
                local.set $l7
                loop $L18
                  local.get $l7
                  local.set $l9
                  block $B19
                    local.get $p1
                    local.tee $l6
                    i32.const 20
                    i32.add
                    local.tee $l7
                    i32.load
                    local.tee $p1
                    br_if $B19
                    local.get $l6
                    i32.const 16
                    i32.add
                    local.set $l7
                    local.get $l6
                    i32.load offset=16
                    local.set $p1
                  end
                  local.get $p1
                  br_if $L18
                end
                local.get $l9
                i32.const 0
                i32.store
              end
              block $B20
                local.get $l5
                i32.eqz
                br_if $B20
                block $B21
                  block $B22
                    local.get $p0
                    local.get $l8
                    i32.load offset=28
                    i32.const 2
                    i32.shl
                    i32.add
                    i32.const 272
                    i32.add
                    local.tee $p1
                    i32.load
                    local.get $l8
                    i32.eq
                    br_if $B22
                    local.get $l5
                    i32.const 16
                    i32.const 20
                    local.get $l5
                    i32.load offset=16
                    local.get $l8
                    i32.eq
                    select
                    i32.add
                    local.get $l6
                    i32.store
                    local.get $l6
                    i32.eqz
                    br_if $B20
                    br $B21
                  end
                  local.get $p1
                  local.get $l6
                  i32.store
                  local.get $l6
                  br_if $B21
                  local.get $p0
                  i32.const 4
                  i32.add
                  local.tee $p1
                  local.get $p1
                  i32.load
                  i32.const -2
                  local.get $l8
                  i32.load offset=28
                  i32.rotl
                  i32.and
                  i32.store
                  br $B20
                end
                local.get $l6
                local.get $l5
                i32.store offset=24
                block $B23
                  local.get $l8
                  i32.load offset=16
                  local.tee $p1
                  i32.eqz
                  br_if $B23
                  local.get $l6
                  local.get $p1
                  i32.store offset=16
                  local.get $p1
                  local.get $l6
                  i32.store offset=24
                end
                local.get $l8
                i32.const 20
                i32.add
                i32.load
                local.tee $p1
                i32.eqz
                br_if $B20
                local.get $l6
                i32.const 20
                i32.add
                local.get $p1
                i32.store
                local.get $p1
                local.get $l6
                i32.store offset=24
              end
              block $B24
                block $B25
                  local.get $l2
                  i32.const 16
                  i32.lt_u
                  br_if $B25
                  local.get $l8
                  local.get $l3
                  i32.const 3
                  i32.or
                  i32.store offset=4
                  local.get $l8
                  local.get $l3
                  i32.add
                  local.tee $l3
                  local.get $l2
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get $l3
                  local.get $l2
                  i32.add
                  local.get $l2
                  i32.store
                  block $B26
                    local.get $l2
                    i32.const 256
                    i32.lt_u
                    br_if $B26
                    block $B27
                      block $B28
                        local.get $l2
                        i32.const 8
                        i32.shr_u
                        local.tee $l6
                        br_if $B28
                        i32.const 0
                        local.set $p1
                        br $B27
                      end
                      i32.const 31
                      local.set $p1
                      local.get $l2
                      i32.const 16777215
                      i32.gt_u
                      br_if $B27
                      local.get $l2
                      i32.const 6
                      local.get $l6
                      i32.clz
                      local.tee $p1
                      i32.sub
                      i32.const 31
                      i32.and
                      i32.shr_u
                      i32.const 1
                      i32.and
                      local.get $p1
                      i32.const 1
                      i32.shl
                      i32.sub
                      i32.const 62
                      i32.add
                      local.set $p1
                    end
                    local.get $l3
                    i64.const 0
                    i64.store offset=16 align=4
                    local.get $l3
                    local.get $p1
                    i32.store offset=28
                    local.get $p0
                    local.get $p1
                    i32.const 2
                    i32.shl
                    i32.add
                    i32.const 272
                    i32.add
                    local.set $l6
                    block $B29
                      block $B30
                        block $B31
                          block $B32
                            block $B33
                              local.get $p0
                              i32.const 4
                              i32.add
                              local.tee $l7
                              i32.load
                              local.tee $l9
                              i32.const 1
                              local.get $p1
                              i32.const 31
                              i32.and
                              i32.shl
                              local.tee $p0
                              i32.and
                              i32.eqz
                              br_if $B33
                              local.get $l6
                              i32.load
                              local.tee $l7
                              i32.const 4
                              i32.add
                              i32.load
                              i32.const -8
                              i32.and
                              local.get $l2
                              i32.ne
                              br_if $B32
                              local.get $l7
                              local.set $p1
                              br $B31
                            end
                            local.get $l7
                            local.get $l9
                            local.get $p0
                            i32.or
                            i32.store
                            local.get $l6
                            local.get $l3
                            i32.store
                            local.get $l3
                            local.get $l6
                            i32.store offset=24
                            br $B29
                          end
                          local.get $l2
                          i32.const 0
                          i32.const 25
                          local.get $p1
                          i32.const 1
                          i32.shr_u
                          i32.sub
                          i32.const 31
                          i32.and
                          local.get $p1
                          i32.const 31
                          i32.eq
                          select
                          i32.shl
                          local.set $l6
                          loop $L34
                            local.get $l7
                            local.get $l6
                            i32.const 29
                            i32.shr_u
                            i32.const 4
                            i32.and
                            i32.add
                            i32.const 16
                            i32.add
                            local.tee $l9
                            i32.load
                            local.tee $p1
                            i32.eqz
                            br_if $B30
                            local.get $l6
                            i32.const 1
                            i32.shl
                            local.set $l6
                            local.get $p1
                            local.set $l7
                            local.get $p1
                            i32.const 4
                            i32.add
                            i32.load
                            i32.const -8
                            i32.and
                            local.get $l2
                            i32.ne
                            br_if $L34
                          end
                        end
                        local.get $p1
                        i32.load offset=8
                        local.tee $l2
                        local.get $l3
                        i32.store offset=12
                        local.get $p1
                        local.get $l3
                        i32.store offset=8
                        local.get $l3
                        i32.const 0
                        i32.store offset=24
                        local.get $l3
                        local.get $p1
                        i32.store offset=12
                        local.get $l3
                        local.get $l2
                        i32.store offset=8
                        br $B24
                      end
                      local.get $l9
                      local.get $l3
                      i32.store
                      local.get $l3
                      local.get $l7
                      i32.store offset=24
                    end
                    local.get $l3
                    local.get $l3
                    i32.store offset=12
                    local.get $l3
                    local.get $l3
                    i32.store offset=8
                    br $B24
                  end
                  local.get $p0
                  local.get $l2
                  i32.const 3
                  i32.shr_u
                  local.tee $l2
                  i32.const 3
                  i32.shl
                  i32.add
                  i32.const 8
                  i32.add
                  local.set $p1
                  block $B35
                    block $B36
                      local.get $p0
                      i32.load
                      local.tee $l6
                      i32.const 1
                      local.get $l2
                      i32.shl
                      local.tee $l2
                      i32.and
                      i32.eqz
                      br_if $B36
                      local.get $p1
                      i32.load offset=8
                      local.set $l2
                      br $B35
                    end
                    local.get $p0
                    local.get $l6
                    local.get $l2
                    i32.or
                    i32.store
                    local.get $p1
                    local.set $l2
                  end
                  local.get $p1
                  local.get $l3
                  i32.store offset=8
                  local.get $l2
                  local.get $l3
                  i32.store offset=12
                  local.get $l3
                  local.get $p1
                  i32.store offset=12
                  local.get $l3
                  local.get $l2
                  i32.store offset=8
                  br $B24
                end
                local.get $l8
                local.get $l2
                local.get $l3
                i32.add
                local.tee $p1
                i32.const 3
                i32.or
                i32.store offset=4
                local.get $l8
                local.get $p1
                i32.add
                local.tee $p1
                local.get $p1
                i32.load offset=4
                i32.const 1
                i32.or
                i32.store offset=4
              end
              local.get $l8
              i32.const 8
              i32.add
              return
            end
            block $B37
              block $B38
                block $B39
                  local.get $p0
                  i32.load
                  local.tee $l8
                  i32.const 16
                  local.get $p1
                  i32.const 11
                  i32.add
                  i32.const -8
                  i32.and
                  local.get $p1
                  i32.const 11
                  i32.lt_u
                  select
                  local.tee $l3
                  i32.const 3
                  i32.shr_u
                  local.tee $l2
                  i32.shr_u
                  local.tee $p1
                  i32.const 3
                  i32.and
                  br_if $B39
                  local.get $l3
                  local.get $p0
                  i32.load offset=400
                  i32.le_u
                  br_if $B3
                  local.get $p1
                  br_if $B38
                  local.get $p0
                  i32.load offset=4
                  local.tee $p1
                  i32.eqz
                  br_if $B3
                  local.get $p0
                  local.get $p1
                  i32.const 0
                  local.get $p1
                  i32.sub
                  i32.and
                  i32.ctz
                  i32.const 2
                  i32.shl
                  i32.add
                  i32.const 272
                  i32.add
                  i32.load
                  local.tee $l6
                  i32.const 4
                  i32.add
                  i32.load
                  i32.const -8
                  i32.and
                  local.get $l3
                  i32.sub
                  local.set $l2
                  local.get $l6
                  local.set $l7
                  loop $L40
                    block $B41
                      local.get $l6
                      i32.load offset=16
                      local.tee $p1
                      br_if $B41
                      local.get $l6
                      i32.const 20
                      i32.add
                      i32.load
                      local.tee $p1
                      i32.eqz
                      br_if $B37
                    end
                    local.get $p1
                    i32.const 4
                    i32.add
                    i32.load
                    i32.const -8
                    i32.and
                    local.get $l3
                    i32.sub
                    local.tee $l6
                    local.get $l2
                    local.get $l6
                    local.get $l2
                    i32.lt_u
                    local.tee $l6
                    select
                    local.set $l2
                    local.get $p1
                    local.get $l7
                    local.get $l6
                    select
                    local.set $l7
                    local.get $p1
                    local.set $l6
                    br $L40
                  end
                end
                block $B42
                  block $B43
                    local.get $p0
                    local.get $p1
                    i32.const -1
                    i32.xor
                    i32.const 1
                    i32.and
                    local.get $l2
                    i32.add
                    local.tee $l3
                    i32.const 3
                    i32.shl
                    i32.add
                    local.tee $l7
                    i32.const 16
                    i32.add
                    i32.load
                    local.tee $p1
                    i32.const 8
                    i32.add
                    local.tee $l2
                    i32.load
                    local.tee $l6
                    local.get $l7
                    i32.const 8
                    i32.add
                    local.tee $l7
                    i32.eq
                    br_if $B43
                    local.get $l6
                    local.get $l7
                    i32.store offset=12
                    local.get $l7
                    local.get $l6
                    i32.store offset=8
                    br $B42
                  end
                  local.get $p0
                  local.get $l8
                  i32.const -2
                  local.get $l3
                  i32.rotl
                  i32.and
                  i32.store
                end
                local.get $p1
                local.get $l3
                i32.const 3
                i32.shl
                local.tee $l3
                i32.const 3
                i32.or
                i32.store offset=4
                local.get $p1
                local.get $l3
                i32.add
                local.tee $p1
                local.get $p1
                i32.load offset=4
                i32.const 1
                i32.or
                i32.store offset=4
                br $B0
              end
              block $B44
                block $B45
                  local.get $p0
                  local.get $p1
                  local.get $l2
                  i32.shl
                  i32.const 2
                  local.get $l2
                  i32.shl
                  local.tee $p1
                  i32.const 0
                  local.get $p1
                  i32.sub
                  i32.or
                  i32.and
                  local.tee $p1
                  i32.const 0
                  local.get $p1
                  i32.sub
                  i32.and
                  i32.ctz
                  local.tee $l2
                  i32.const 3
                  i32.shl
                  i32.add
                  local.tee $l7
                  i32.const 16
                  i32.add
                  i32.load
                  local.tee $p1
                  i32.const 8
                  i32.add
                  local.tee $l9
                  i32.load
                  local.tee $l6
                  local.get $l7
                  i32.const 8
                  i32.add
                  local.tee $l7
                  i32.eq
                  br_if $B45
                  local.get $l6
                  local.get $l7
                  i32.store offset=12
                  local.get $l7
                  local.get $l6
                  i32.store offset=8
                  br $B44
                end
                local.get $p0
                local.get $l8
                i32.const -2
                local.get $l2
                i32.rotl
                i32.and
                i32.store
              end
              local.get $p1
              local.get $l3
              i32.const 3
              i32.or
              i32.store offset=4
              local.get $p1
              local.get $l3
              i32.add
              local.tee $l6
              local.get $l2
              i32.const 3
              i32.shl
              local.tee $l2
              local.get $l3
              i32.sub
              local.tee $l3
              i32.const 1
              i32.or
              i32.store offset=4
              local.get $p1
              local.get $l2
              i32.add
              local.get $l3
              i32.store
              block $B46
                local.get $p0
                i32.load offset=400
                local.tee $p1
                i32.eqz
                br_if $B46
                local.get $p0
                local.get $p1
                i32.const 3
                i32.shr_u
                local.tee $l7
                i32.const 3
                i32.shl
                i32.add
                i32.const 8
                i32.add
                local.set $l2
                local.get $p0
                i32.load offset=408
                local.set $p1
                block $B47
                  block $B48
                    local.get $p0
                    i32.load
                    local.tee $l8
                    i32.const 1
                    local.get $l7
                    i32.const 31
                    i32.and
                    i32.shl
                    local.tee $l7
                    i32.and
                    i32.eqz
                    br_if $B48
                    local.get $l2
                    i32.load offset=8
                    local.set $l7
                    br $B47
                  end
                  local.get $p0
                  local.get $l8
                  local.get $l7
                  i32.or
                  i32.store
                  local.get $l2
                  local.set $l7
                end
                local.get $l2
                local.get $p1
                i32.store offset=8
                local.get $l7
                local.get $p1
                i32.store offset=12
                local.get $p1
                local.get $l2
                i32.store offset=12
                local.get $p1
                local.get $l7
                i32.store offset=8
              end
              local.get $p0
              local.get $l6
              i32.store offset=408
              local.get $p0
              local.get $l3
              i32.store offset=400
              local.get $l9
              return
            end
            local.get $l7
            i32.load offset=24
            local.set $l5
            block $B49
              block $B50
                block $B51
                  local.get $l7
                  i32.load offset=12
                  local.tee $l6
                  local.get $l7
                  i32.ne
                  br_if $B51
                  local.get $l7
                  i32.const 20
                  i32.const 16
                  local.get $l7
                  i32.const 20
                  i32.add
                  local.tee $l6
                  i32.load
                  local.tee $l8
                  select
                  i32.add
                  i32.load
                  local.tee $p1
                  br_if $B50
                  i32.const 0
                  local.set $l6
                  br $B49
                end
                local.get $l7
                i32.load offset=8
                local.tee $p1
                local.get $l6
                i32.store offset=12
                local.get $l6
                local.get $p1
                i32.store offset=8
                br $B49
              end
              local.get $l6
              local.get $l7
              i32.const 16
              i32.add
              local.get $l8
              select
              local.set $l8
              loop $L52
                local.get $l8
                local.set $l9
                block $B53
                  local.get $p1
                  local.tee $l6
                  i32.const 20
                  i32.add
                  local.tee $l8
                  i32.load
                  local.tee $p1
                  br_if $B53
                  local.get $l6
                  i32.const 16
                  i32.add
                  local.set $l8
                  local.get $l6
                  i32.load offset=16
                  local.set $p1
                end
                local.get $p1
                br_if $L52
              end
              local.get $l9
              i32.const 0
              i32.store
            end
            local.get $l5
            i32.eqz
            br_if $B1
            block $B54
              local.get $p0
              local.get $l7
              i32.load offset=28
              i32.const 2
              i32.shl
              i32.add
              i32.const 272
              i32.add
              local.tee $p1
              i32.load
              local.get $l7
              i32.eq
              br_if $B54
              local.get $l5
              i32.const 16
              i32.const 20
              local.get $l5
              i32.load offset=16
              local.get $l7
              i32.eq
              select
              i32.add
              local.get $l6
              i32.store
              local.get $l6
              i32.eqz
              br_if $B1
              br $B2
            end
            local.get $p1
            local.get $l6
            i32.store
            local.get $l6
            br_if $B2
            local.get $p0
            local.get $p0
            i32.load offset=4
            i32.const -2
            local.get $l7
            i32.load offset=28
            i32.rotl
            i32.and
            i32.store offset=4
            br $B1
          end
          block $B55
            block $B56
              block $B57
                block $B58
                  block $B59
                    block $B60
                      block $B61
                        local.get $p0
                        i32.load offset=400
                        local.tee $p1
                        local.get $l3
                        i32.ge_u
                        br_if $B61
                        local.get $p0
                        i32.load offset=404
                        local.tee $p1
                        local.get $l3
                        i32.gt_u
                        br_if $B58
                        i32.const 0
                        local.set $l2
                        local.get $l3
                        i32.const 65583
                        i32.add
                        local.tee $l6
                        i32.const 16
                        i32.shr_u
                        memory.grow
                        local.tee $p1
                        i32.const -1
                        i32.eq
                        br_if $B0
                        local.get $p1
                        i32.const 16
                        i32.shl
                        local.tee $l8
                        i32.eqz
                        br_if $B0
                        local.get $p0
                        local.get $p0
                        i32.load offset=416
                        local.get $l6
                        i32.const -65536
                        i32.and
                        local.tee $l5
                        i32.add
                        local.tee $p1
                        i32.store offset=416
                        local.get $p0
                        local.get $p0
                        i32.load offset=420
                        local.tee $l6
                        local.get $p1
                        local.get $l6
                        local.get $p1
                        i32.gt_u
                        select
                        i32.store offset=420
                        local.get $p0
                        i32.load offset=412
                        local.tee $l6
                        i32.eqz
                        br_if $B60
                        local.get $p0
                        i32.const 424
                        i32.add
                        local.tee $l4
                        local.set $p1
                        loop $L62
                          local.get $p1
                          i32.load
                          local.tee $l7
                          local.get $p1
                          i32.load offset=4
                          local.tee $l9
                          i32.add
                          local.get $l8
                          i32.eq
                          br_if $B59
                          local.get $p1
                          i32.load offset=8
                          local.tee $p1
                          br_if $L62
                          br $B56
                        end
                      end
                      local.get $p0
                      i32.load offset=408
                      local.set $l2
                      block $B63
                        block $B64
                          local.get $p1
                          local.get $l3
                          i32.sub
                          local.tee $l6
                          i32.const 15
                          i32.gt_u
                          br_if $B64
                          local.get $p0
                          i32.const 0
                          i32.store offset=408
                          local.get $p0
                          i32.const 0
                          i32.store offset=400
                          local.get $l2
                          local.get $p1
                          i32.const 3
                          i32.or
                          i32.store offset=4
                          local.get $l2
                          local.get $p1
                          i32.add
                          local.tee $l3
                          i32.const 4
                          i32.add
                          local.set $p1
                          local.get $l3
                          i32.load offset=4
                          i32.const 1
                          i32.or
                          local.set $l3
                          br $B63
                        end
                        local.get $p0
                        local.get $l6
                        i32.store offset=400
                        local.get $p0
                        local.get $l2
                        local.get $l3
                        i32.add
                        local.tee $l7
                        i32.store offset=408
                        local.get $l7
                        local.get $l6
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        local.get $l2
                        local.get $p1
                        i32.add
                        local.get $l6
                        i32.store
                        local.get $l3
                        i32.const 3
                        i32.or
                        local.set $l3
                        local.get $l2
                        i32.const 4
                        i32.add
                        local.set $p1
                      end
                      local.get $p1
                      local.get $l3
                      i32.store
                      local.get $l2
                      i32.const 8
                      i32.add
                      return
                    end
                    block $B65
                      block $B66
                        local.get $p0
                        i32.load offset=444
                        local.tee $p1
                        i32.eqz
                        br_if $B66
                        local.get $p1
                        local.get $l8
                        i32.le_u
                        br_if $B65
                      end
                      local.get $p0
                      local.get $l8
                      i32.store offset=444
                    end
                    local.get $p0
                    i32.const 4095
                    i32.store offset=448
                    local.get $p0
                    local.get $l8
                    i32.store offset=424
                    local.get $p0
                    i32.const 436
                    i32.add
                    i32.const 0
                    i32.store
                    local.get $p0
                    i32.const 428
                    i32.add
                    local.get $l5
                    i32.store
                    local.get $p0
                    i32.const 20
                    i32.add
                    local.get $p0
                    i32.const 8
                    i32.add
                    local.tee $l6
                    i32.store
                    local.get $p0
                    i32.const 28
                    i32.add
                    local.get $p0
                    i32.const 16
                    i32.add
                    local.tee $p1
                    i32.store
                    local.get $p1
                    local.get $l6
                    i32.store
                    local.get $p0
                    i32.const 36
                    i32.add
                    local.get $p0
                    i32.const 24
                    i32.add
                    local.tee $l6
                    i32.store
                    local.get $l6
                    local.get $p1
                    i32.store
                    local.get $p0
                    i32.const 44
                    i32.add
                    local.get $p0
                    i32.const 32
                    i32.add
                    local.tee $p1
                    i32.store
                    local.get $p1
                    local.get $l6
                    i32.store
                    local.get $p0
                    i32.const 52
                    i32.add
                    local.get $p0
                    i32.const 40
                    i32.add
                    local.tee $l6
                    i32.store
                    local.get $l6
                    local.get $p1
                    i32.store
                    local.get $p0
                    i32.const 60
                    i32.add
                    local.get $p0
                    i32.const 48
                    i32.add
                    local.tee $p1
                    i32.store
                    local.get $p1
                    local.get $l6
                    i32.store
                    local.get $p0
                    i32.const 68
                    i32.add
                    local.get $p0
                    i32.const 56
                    i32.add
                    local.tee $l6
                    i32.store
                    local.get $l6
                    local.get $p1
                    i32.store
                    local.get $p0
                    i32.const 76
                    i32.add
                    local.get $p0
                    i32.const 64
                    i32.add
                    local.tee $p1
                    i32.store
                    local.get $p1
                    local.get $l6
                    i32.store
                    local.get $p0
                    i32.const 84
                    i32.add
                    local.get $p0
                    i32.const 72
                    i32.add
                    local.tee $l6
                    i32.store
                    local.get $l6
                    local.get $p1
                    i32.store
                    local.get $p0
                    i32.const 80
                    i32.add
                    local.tee $p1
                    local.get $l6
                    i32.store
                    local.get $p0
                    i32.const 92
                    i32.add
                    local.get $p1
                    i32.store
                    local.get $p0
                    i32.const 88
                    i32.add
                    local.tee $l6
                    local.get $p1
                    i32.store
                    local.get $p0
                    i32.const 100
                    i32.add
                    local.get $l6
                    i32.store
                    local.get $p0
                    i32.const 96
                    i32.add
                    local.tee $p1
                    local.get $l6
                    i32.store
                    local.get $p0
                    i32.const 108
                    i32.add
                    local.get $p1
                    i32.store
                    local.get $p0
                    i32.const 104
                    i32.add
                    local.tee $l6
                    local.get $p1
                    i32.store
                    local.get $p0
                    i32.const 116
                    i32.add
                    local.get $l6
                    i32.store
                    local.get $p0
                    i32.const 112
                    i32.add
                    local.tee $p1
                    local.get $l6
                    i32.store
                    local.get $p0
                    i32.const 124
                    i32.add
                    local.get $p1
                    i32.store
                    local.get $p0
                    i32.const 120
                    i32.add
                    local.tee $l6
                    local.get $p1
                    i32.store
                    local.get $p0
                    i32.const 132
                    i32.add
                    local.get $l6
                    i32.store
                    local.get $p0
                    i32.const 128
                    i32.add
                    local.tee $p1
                    local.get $l6
                    i32.store
                    local.get $p0
                    i32.const 140
                    i32.add
                    local.get $p1
                    i32.store
                    local.get $p0
                    i32.const 136
                    i32.add
                    local.tee $l6
                    local.get $p1
                    i32.store
                    local.get $p0
                    i32.const 148
                    i32.add
                    local.get $l6
                    i32.store
                    local.get $p0
                    i32.const 156
                    i32.add
                    local.get $p0
                    i32.const 144
                    i32.add
                    local.tee $p1
                    i32.store
                    local.get $p1
                    local.get $l6
                    i32.store
                    local.get $p0
                    i32.const 164
                    i32.add
                    local.get $p0
                    i32.const 152
                    i32.add
                    local.tee $l6
                    i32.store
                    local.get $l6
                    local.get $p1
                    i32.store
                    local.get $p0
                    i32.const 172
                    i32.add
                    local.get $p0
                    i32.const 160
                    i32.add
                    local.tee $p1
                    i32.store
                    local.get $p1
                    local.get $l6
                    i32.store
                    local.get $p0
                    i32.const 180
                    i32.add
                    local.get $p0
                    i32.const 168
                    i32.add
                    local.tee $l6
                    i32.store
                    local.get $l6
                    local.get $p1
                    i32.store
                    local.get $p0
                    i32.const 188
                    i32.add
                    local.get $p0
                    i32.const 176
                    i32.add
                    local.tee $p1
                    i32.store
                    local.get $p1
                    local.get $l6
                    i32.store
                    local.get $p0
                    i32.const 196
                    i32.add
                    local.get $p0
                    i32.const 184
                    i32.add
                    local.tee $l6
                    i32.store
                    local.get $l6
                    local.get $p1
                    i32.store
                    local.get $p0
                    i32.const 204
                    i32.add
                    local.get $p0
                    i32.const 192
                    i32.add
                    local.tee $p1
                    i32.store
                    local.get $p1
                    local.get $l6
                    i32.store
                    local.get $p0
                    i32.const 212
                    i32.add
                    local.get $p0
                    i32.const 200
                    i32.add
                    local.tee $l6
                    i32.store
                    local.get $l6
                    local.get $p1
                    i32.store
                    local.get $p0
                    i32.const 220
                    i32.add
                    local.get $p0
                    i32.const 208
                    i32.add
                    local.tee $p1
                    i32.store
                    local.get $p1
                    local.get $l6
                    i32.store
                    local.get $p0
                    i32.const 228
                    i32.add
                    local.get $p0
                    i32.const 216
                    i32.add
                    local.tee $l6
                    i32.store
                    local.get $l6
                    local.get $p1
                    i32.store
                    local.get $p0
                    i32.const 236
                    i32.add
                    local.get $p0
                    i32.const 224
                    i32.add
                    local.tee $p1
                    i32.store
                    local.get $p1
                    local.get $l6
                    i32.store
                    local.get $p0
                    i32.const 244
                    i32.add
                    local.get $p0
                    i32.const 232
                    i32.add
                    local.tee $l6
                    i32.store
                    local.get $l6
                    local.get $p1
                    i32.store
                    local.get $p0
                    i32.const 252
                    i32.add
                    local.get $p0
                    i32.const 240
                    i32.add
                    local.tee $p1
                    i32.store
                    local.get $p1
                    local.get $l6
                    i32.store
                    local.get $p0
                    i32.const 260
                    i32.add
                    local.get $p0
                    i32.const 248
                    i32.add
                    local.tee $l6
                    i32.store
                    local.get $l6
                    local.get $p1
                    i32.store
                    local.get $p0
                    i32.const 268
                    i32.add
                    local.get $p0
                    i32.const 256
                    i32.add
                    local.tee $p1
                    i32.store
                    local.get $p1
                    local.get $l6
                    i32.store
                    local.get $p0
                    local.get $l8
                    i32.store offset=412
                    local.get $p0
                    i32.const 264
                    i32.add
                    local.get $p1
                    i32.store
                    local.get $p0
                    local.get $l5
                    i32.const -40
                    i32.add
                    local.tee $p1
                    i32.store offset=404
                    local.get $l8
                    local.get $p1
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    local.get $l8
                    local.get $p1
                    i32.add
                    i32.const 40
                    i32.store offset=4
                    local.get $p0
                    i32.const 2097152
                    i32.store offset=440
                    br $B55
                  end
                  local.get $p1
                  i32.const 12
                  i32.add
                  i32.load
                  i32.eqz
                  br_if $B57
                  br $B56
                end
                local.get $p0
                local.get $p1
                local.get $l3
                i32.sub
                local.tee $l2
                i32.store offset=404
                local.get $p0
                local.get $p0
                i32.load offset=412
                local.tee $p1
                local.get $l3
                i32.add
                local.tee $l6
                i32.store offset=412
                local.get $l6
                local.get $l2
                i32.const 1
                i32.or
                i32.store offset=4
                local.get $p1
                local.get $l3
                i32.const 3
                i32.or
                i32.store offset=4
                local.get $p1
                i32.const 8
                i32.add
                return
              end
              local.get $l8
              local.get $l6
              i32.le_u
              br_if $B56
              local.get $l7
              local.get $l6
              i32.gt_u
              br_if $B56
              local.get $p1
              local.get $l9
              local.get $l5
              i32.add
              i32.store offset=4
              local.get $p0
              local.get $p0
              i32.load offset=412
              local.tee $p1
              i32.const 15
              i32.add
              i32.const -8
              i32.and
              local.tee $l6
              i32.const -8
              i32.add
              i32.store offset=412
              local.get $p0
              local.get $p1
              local.get $l6
              i32.sub
              local.get $p0
              i32.load offset=404
              local.get $l5
              i32.add
              local.tee $l7
              i32.add
              i32.const 8
              i32.add
              local.tee $l8
              i32.store offset=404
              local.get $l6
              i32.const -4
              i32.add
              local.get $l8
              i32.const 1
              i32.or
              i32.store
              local.get $p1
              local.get $l7
              i32.add
              i32.const 40
              i32.store offset=4
              local.get $p0
              i32.const 2097152
              i32.store offset=440
              br $B55
            end
            local.get $p0
            local.get $p0
            i32.load offset=444
            local.tee $p1
            local.get $l8
            local.get $p1
            local.get $l8
            i32.lt_u
            select
            i32.store offset=444
            local.get $l8
            local.get $l5
            i32.add
            local.set $l7
            local.get $l4
            local.set $p1
            block $B67
              block $B68
                block $B69
                  loop $L70
                    local.get $p1
                    i32.load
                    local.get $l7
                    i32.eq
                    br_if $B69
                    local.get $p1
                    i32.load offset=8
                    local.tee $p1
                    br_if $L70
                    br $B68
                  end
                end
                local.get $p1
                i32.const 12
                i32.add
                i32.load
                i32.eqz
                br_if $B67
              end
              local.get $l4
              local.set $p1
              block $B71
                loop $L72
                  block $B73
                    local.get $p1
                    i32.load
                    local.tee $l7
                    local.get $l6
                    i32.gt_u
                    br_if $B73
                    local.get $l7
                    local.get $p1
                    i32.load offset=4
                    i32.add
                    local.tee $l7
                    local.get $l6
                    i32.gt_u
                    br_if $B71
                  end
                  local.get $p1
                  i32.load offset=8
                  local.set $p1
                  br $L72
                end
              end
              local.get $p0
              local.get $l8
              i32.store offset=412
              local.get $p0
              local.get $l5
              i32.const -40
              i32.add
              local.tee $p1
              i32.store offset=404
              local.get $l8
              local.get $p1
              i32.const 1
              i32.or
              i32.store offset=4
              local.get $l8
              local.get $p1
              i32.add
              i32.const 40
              i32.store offset=4
              local.get $p0
              i32.const 2097152
              i32.store offset=440
              local.get $l6
              local.get $l7
              i32.const -32
              i32.add
              i32.const -8
              i32.and
              i32.const -8
              i32.add
              local.tee $p1
              local.get $p1
              local.get $l6
              i32.const 16
              i32.add
              i32.lt_u
              select
              local.tee $l9
              i32.const 27
              i32.store offset=4
              local.get $l4
              i64.load align=4
              local.set $l10
              local.get $l9
              i32.const 16
              i32.add
              local.get $l4
              i32.const 8
              i32.add
              i64.load align=4
              i64.store align=4
              local.get $l9
              local.get $l10
              i64.store offset=8 align=4
              local.get $p0
              i32.const 436
              i32.add
              i32.const 0
              i32.store
              local.get $p0
              i32.const 428
              i32.add
              local.get $l5
              i32.store
              local.get $p0
              local.get $l8
              i32.store offset=424
              local.get $p0
              i32.const 432
              i32.add
              local.get $l9
              i32.const 8
              i32.add
              i32.store
              local.get $l9
              i32.const 28
              i32.add
              local.set $p1
              loop $L74
                local.get $p1
                i32.const 7
                i32.store
                local.get $l7
                local.get $p1
                i32.const 4
                i32.add
                local.tee $p1
                i32.gt_u
                br_if $L74
              end
              local.get $l9
              local.get $l6
              i32.eq
              br_if $B55
              local.get $l9
              local.get $l9
              i32.load offset=4
              i32.const -2
              i32.and
              i32.store offset=4
              local.get $l6
              local.get $l9
              local.get $l6
              i32.sub
              local.tee $l8
              i32.const 1
              i32.or
              i32.store offset=4
              local.get $l9
              local.get $l8
              i32.store
              block $B75
                local.get $l8
                i32.const 256
                i32.lt_u
                br_if $B75
                block $B76
                  block $B77
                    local.get $l8
                    i32.const 8
                    i32.shr_u
                    local.tee $l7
                    br_if $B77
                    i32.const 0
                    local.set $p1
                    br $B76
                  end
                  i32.const 31
                  local.set $p1
                  local.get $l8
                  i32.const 16777215
                  i32.gt_u
                  br_if $B76
                  local.get $l8
                  i32.const 6
                  local.get $l7
                  i32.clz
                  local.tee $p1
                  i32.sub
                  i32.const 31
                  i32.and
                  i32.shr_u
                  i32.const 1
                  i32.and
                  local.get $p1
                  i32.const 1
                  i32.shl
                  i32.sub
                  i32.const 62
                  i32.add
                  local.set $p1
                end
                local.get $l6
                i64.const 0
                i64.store offset=16 align=4
                local.get $l6
                i32.const 28
                i32.add
                local.get $p1
                i32.store
                local.get $p0
                local.get $p1
                i32.const 2
                i32.shl
                i32.add
                i32.const 272
                i32.add
                local.set $l7
                block $B78
                  block $B79
                    block $B80
                      block $B81
                        block $B82
                          local.get $p0
                          i32.const 4
                          i32.add
                          local.tee $l9
                          i32.load
                          local.tee $l5
                          i32.const 1
                          local.get $p1
                          i32.const 31
                          i32.and
                          i32.shl
                          local.tee $l4
                          i32.and
                          i32.eqz
                          br_if $B82
                          local.get $l7
                          i32.load
                          local.tee $l9
                          i32.const 4
                          i32.add
                          i32.load
                          i32.const -8
                          i32.and
                          local.get $l8
                          i32.ne
                          br_if $B81
                          local.get $l9
                          local.set $p1
                          br $B80
                        end
                        local.get $l9
                        local.get $l5
                        local.get $l4
                        i32.or
                        i32.store
                        local.get $l7
                        local.get $l6
                        i32.store
                        local.get $l6
                        i32.const 24
                        i32.add
                        local.get $l7
                        i32.store
                        br $B78
                      end
                      local.get $l8
                      i32.const 0
                      i32.const 25
                      local.get $p1
                      i32.const 1
                      i32.shr_u
                      i32.sub
                      i32.const 31
                      i32.and
                      local.get $p1
                      i32.const 31
                      i32.eq
                      select
                      i32.shl
                      local.set $l7
                      loop $L83
                        local.get $l9
                        local.get $l7
                        i32.const 29
                        i32.shr_u
                        i32.const 4
                        i32.and
                        i32.add
                        i32.const 16
                        i32.add
                        local.tee $l5
                        i32.load
                        local.tee $p1
                        i32.eqz
                        br_if $B79
                        local.get $l7
                        i32.const 1
                        i32.shl
                        local.set $l7
                        local.get $p1
                        local.set $l9
                        local.get $p1
                        i32.const 4
                        i32.add
                        i32.load
                        i32.const -8
                        i32.and
                        local.get $l8
                        i32.ne
                        br_if $L83
                      end
                    end
                    local.get $p1
                    i32.load offset=8
                    local.tee $l7
                    local.get $l6
                    i32.store offset=12
                    local.get $p1
                    local.get $l6
                    i32.store offset=8
                    local.get $l6
                    i32.const 24
                    i32.add
                    i32.const 0
                    i32.store
                    local.get $l6
                    local.get $p1
                    i32.store offset=12
                    local.get $l6
                    local.get $l7
                    i32.store offset=8
                    br $B55
                  end
                  local.get $l5
                  local.get $l6
                  i32.store
                  local.get $l6
                  i32.const 24
                  i32.add
                  local.get $l9
                  i32.store
                end
                local.get $l6
                local.get $l6
                i32.store offset=12
                local.get $l6
                local.get $l6
                i32.store offset=8
                br $B55
              end
              local.get $p0
              local.get $l8
              i32.const 3
              i32.shr_u
              local.tee $l7
              i32.const 3
              i32.shl
              i32.add
              i32.const 8
              i32.add
              local.set $p1
              block $B84
                block $B85
                  local.get $p0
                  i32.load
                  local.tee $l8
                  i32.const 1
                  local.get $l7
                  i32.shl
                  local.tee $l7
                  i32.and
                  i32.eqz
                  br_if $B85
                  local.get $p1
                  i32.load offset=8
                  local.set $l7
                  br $B84
                end
                local.get $p0
                local.get $l8
                local.get $l7
                i32.or
                i32.store
                local.get $p1
                local.set $l7
              end
              local.get $p1
              local.get $l6
              i32.store offset=8
              local.get $l7
              local.get $l6
              i32.store offset=12
              local.get $l6
              local.get $p1
              i32.store offset=12
              local.get $l6
              local.get $l7
              i32.store offset=8
              br $B55
            end
            local.get $p1
            local.get $l8
            i32.store
            local.get $p1
            local.get $p1
            i32.load offset=4
            local.get $l5
            i32.add
            i32.store offset=4
            local.get $l8
            local.get $l3
            i32.const 3
            i32.or
            i32.store offset=4
            local.get $l8
            local.get $l3
            i32.add
            local.set $p1
            local.get $l7
            local.get $l8
            i32.sub
            local.get $l3
            i32.sub
            local.set $l3
            block $B86
              block $B87
                block $B88
                  local.get $p0
                  i32.load offset=412
                  local.get $l7
                  i32.eq
                  br_if $B88
                  local.get $p0
                  i32.load offset=408
                  local.get $l7
                  i32.eq
                  br_if $B87
                  block $B89
                    local.get $l7
                    i32.const 4
                    i32.add
                    i32.load
                    local.tee $l2
                    i32.const 3
                    i32.and
                    i32.const 1
                    i32.ne
                    br_if $B89
                    local.get $p0
                    local.get $l7
                    local.get $l2
                    i32.const -8
                    i32.and
                    local.tee $l2
                    call $_ZN8dlmalloc8dlmalloc8Dlmalloc12unlink_chunk17hfe7c47108d1938c5E
                    local.get $l2
                    local.get $l3
                    i32.add
                    local.set $l3
                    local.get $l7
                    local.get $l2
                    i32.add
                    local.set $l7
                  end
                  local.get $l7
                  local.get $l7
                  i32.load offset=4
                  i32.const -2
                  i32.and
                  i32.store offset=4
                  local.get $p1
                  local.get $l3
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get $p1
                  local.get $l3
                  i32.add
                  local.get $l3
                  i32.store
                  block $B90
                    local.get $l3
                    i32.const 256
                    i32.lt_u
                    br_if $B90
                    block $B91
                      block $B92
                        local.get $l3
                        i32.const 8
                        i32.shr_u
                        local.tee $l6
                        br_if $B92
                        i32.const 0
                        local.set $l2
                        br $B91
                      end
                      i32.const 31
                      local.set $l2
                      local.get $l3
                      i32.const 16777215
                      i32.gt_u
                      br_if $B91
                      local.get $l3
                      i32.const 6
                      local.get $l6
                      i32.clz
                      local.tee $l2
                      i32.sub
                      i32.const 31
                      i32.and
                      i32.shr_u
                      i32.const 1
                      i32.and
                      local.get $l2
                      i32.const 1
                      i32.shl
                      i32.sub
                      i32.const 62
                      i32.add
                      local.set $l2
                    end
                    local.get $p1
                    i64.const 0
                    i64.store offset=16 align=4
                    local.get $p1
                    local.get $l2
                    i32.store offset=28
                    local.get $p0
                    local.get $l2
                    i32.const 2
                    i32.shl
                    i32.add
                    i32.const 272
                    i32.add
                    local.set $l6
                    block $B93
                      block $B94
                        block $B95
                          block $B96
                            block $B97
                              local.get $p0
                              i32.const 4
                              i32.add
                              local.tee $l7
                              i32.load
                              local.tee $l9
                              i32.const 1
                              local.get $l2
                              i32.const 31
                              i32.and
                              i32.shl
                              local.tee $p0
                              i32.and
                              i32.eqz
                              br_if $B97
                              local.get $l6
                              i32.load
                              local.tee $l7
                              i32.const 4
                              i32.add
                              i32.load
                              i32.const -8
                              i32.and
                              local.get $l3
                              i32.ne
                              br_if $B96
                              local.get $l7
                              local.set $l2
                              br $B95
                            end
                            local.get $l7
                            local.get $l9
                            local.get $p0
                            i32.or
                            i32.store
                            local.get $l6
                            local.get $p1
                            i32.store
                            local.get $p1
                            local.get $l6
                            i32.store offset=24
                            br $B93
                          end
                          local.get $l3
                          i32.const 0
                          i32.const 25
                          local.get $l2
                          i32.const 1
                          i32.shr_u
                          i32.sub
                          i32.const 31
                          i32.and
                          local.get $l2
                          i32.const 31
                          i32.eq
                          select
                          i32.shl
                          local.set $l6
                          loop $L98
                            local.get $l7
                            local.get $l6
                            i32.const 29
                            i32.shr_u
                            i32.const 4
                            i32.and
                            i32.add
                            i32.const 16
                            i32.add
                            local.tee $l9
                            i32.load
                            local.tee $l2
                            i32.eqz
                            br_if $B94
                            local.get $l6
                            i32.const 1
                            i32.shl
                            local.set $l6
                            local.get $l2
                            local.set $l7
                            local.get $l2
                            i32.const 4
                            i32.add
                            i32.load
                            i32.const -8
                            i32.and
                            local.get $l3
                            i32.ne
                            br_if $L98
                          end
                        end
                        local.get $l2
                        i32.load offset=8
                        local.tee $l3
                        local.get $p1
                        i32.store offset=12
                        local.get $l2
                        local.get $p1
                        i32.store offset=8
                        local.get $p1
                        i32.const 0
                        i32.store offset=24
                        local.get $p1
                        local.get $l2
                        i32.store offset=12
                        local.get $p1
                        local.get $l3
                        i32.store offset=8
                        br $B86
                      end
                      local.get $l9
                      local.get $p1
                      i32.store
                      local.get $p1
                      local.get $l7
                      i32.store offset=24
                    end
                    local.get $p1
                    local.get $p1
                    i32.store offset=12
                    local.get $p1
                    local.get $p1
                    i32.store offset=8
                    br $B86
                  end
                  local.get $p0
                  local.get $l3
                  i32.const 3
                  i32.shr_u
                  local.tee $l2
                  i32.const 3
                  i32.shl
                  i32.add
                  i32.const 8
                  i32.add
                  local.set $l3
                  block $B99
                    block $B100
                      local.get $p0
                      i32.load
                      local.tee $l6
                      i32.const 1
                      local.get $l2
                      i32.shl
                      local.tee $l2
                      i32.and
                      i32.eqz
                      br_if $B100
                      local.get $l3
                      i32.load offset=8
                      local.set $l2
                      br $B99
                    end
                    local.get $p0
                    local.get $l6
                    local.get $l2
                    i32.or
                    i32.store
                    local.get $l3
                    local.set $l2
                  end
                  local.get $l3
                  local.get $p1
                  i32.store offset=8
                  local.get $l2
                  local.get $p1
                  i32.store offset=12
                  local.get $p1
                  local.get $l3
                  i32.store offset=12
                  local.get $p1
                  local.get $l2
                  i32.store offset=8
                  br $B86
                end
                local.get $p0
                local.get $p1
                i32.store offset=412
                local.get $p0
                local.get $p0
                i32.load offset=404
                local.get $l3
                i32.add
                local.tee $l3
                i32.store offset=404
                local.get $p1
                local.get $l3
                i32.const 1
                i32.or
                i32.store offset=4
                br $B86
              end
              local.get $p0
              local.get $p1
              i32.store offset=408
              local.get $p0
              local.get $p0
              i32.load offset=400
              local.get $l3
              i32.add
              local.tee $l3
              i32.store offset=400
              local.get $p1
              local.get $l3
              i32.const 1
              i32.or
              i32.store offset=4
              local.get $p1
              local.get $l3
              i32.add
              local.get $l3
              i32.store
            end
            local.get $l8
            i32.const 8
            i32.add
            return
          end
          local.get $p0
          i32.load offset=404
          local.tee $p1
          local.get $l3
          i32.le_u
          br_if $B0
          local.get $p0
          local.get $p1
          local.get $l3
          i32.sub
          local.tee $l2
          i32.store offset=404
          local.get $p0
          local.get $p0
          i32.load offset=412
          local.tee $p1
          local.get $l3
          i32.add
          local.tee $l6
          i32.store offset=412
          local.get $l6
          local.get $l2
          i32.const 1
          i32.or
          i32.store offset=4
          local.get $p1
          local.get $l3
          i32.const 3
          i32.or
          i32.store offset=4
          local.get $p1
          i32.const 8
          i32.add
          return
        end
        local.get $l6
        local.get $l5
        i32.store offset=24
        block $B101
          local.get $l7
          i32.load offset=16
          local.tee $p1
          i32.eqz
          br_if $B101
          local.get $l6
          local.get $p1
          i32.store offset=16
          local.get $p1
          local.get $l6
          i32.store offset=24
        end
        local.get $l7
        i32.const 20
        i32.add
        i32.load
        local.tee $p1
        i32.eqz
        br_if $B1
        local.get $l6
        i32.const 20
        i32.add
        local.get $p1
        i32.store
        local.get $p1
        local.get $l6
        i32.store offset=24
      end
      block $B102
        block $B103
          local.get $l2
          i32.const 16
          i32.lt_u
          br_if $B103
          local.get $l7
          local.get $l3
          i32.const 3
          i32.or
          i32.store offset=4
          local.get $l7
          local.get $l3
          i32.add
          local.tee $l3
          local.get $l2
          i32.const 1
          i32.or
          i32.store offset=4
          local.get $l3
          local.get $l2
          i32.add
          local.get $l2
          i32.store
          block $B104
            local.get $p0
            i32.load offset=400
            local.tee $p1
            i32.eqz
            br_if $B104
            local.get $p0
            local.get $p1
            i32.const 3
            i32.shr_u
            local.tee $l8
            i32.const 3
            i32.shl
            i32.add
            i32.const 8
            i32.add
            local.set $l6
            local.get $p0
            i32.load offset=408
            local.set $p1
            block $B105
              block $B106
                local.get $p0
                i32.load
                local.tee $l9
                i32.const 1
                local.get $l8
                i32.const 31
                i32.and
                i32.shl
                local.tee $l8
                i32.and
                i32.eqz
                br_if $B106
                local.get $l6
                i32.load offset=8
                local.set $l8
                br $B105
              end
              local.get $p0
              local.get $l9
              local.get $l8
              i32.or
              i32.store
              local.get $l6
              local.set $l8
            end
            local.get $l6
            local.get $p1
            i32.store offset=8
            local.get $l8
            local.get $p1
            i32.store offset=12
            local.get $p1
            local.get $l6
            i32.store offset=12
            local.get $p1
            local.get $l8
            i32.store offset=8
          end
          local.get $p0
          local.get $l3
          i32.store offset=408
          local.get $p0
          local.get $l2
          i32.store offset=400
          br $B102
        end
        local.get $l7
        local.get $l2
        local.get $l3
        i32.add
        local.tee $p1
        i32.const 3
        i32.or
        i32.store offset=4
        local.get $l7
        local.get $p1
        i32.add
        local.tee $p1
        local.get $p1
        i32.load offset=4
        i32.const 1
        i32.or
        i32.store offset=4
      end
      local.get $l7
      i32.const 8
      i32.add
      return
    end
    local.get $l2)
  (func $_ZN8dlmalloc8dlmalloc8Dlmalloc12unlink_chunk17hfe7c47108d1938c5E (type $t4) (param $p0 i32) (param $p1 i32) (param $p2 i32)
    (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32)
    block $B0
      block $B1
        block $B2
          local.get $p2
          i32.const 256
          i32.lt_u
          br_if $B2
          local.get $p1
          i32.const 24
          i32.add
          i32.load
          local.set $l3
          block $B3
            block $B4
              block $B5
                local.get $p1
                i32.load offset=12
                local.tee $l4
                local.get $p1
                i32.ne
                br_if $B5
                local.get $p1
                i32.const 20
                i32.const 16
                local.get $p1
                i32.const 20
                i32.add
                local.tee $l4
                i32.load
                local.tee $l5
                select
                i32.add
                i32.load
                local.tee $p2
                br_if $B4
                i32.const 0
                local.set $l4
                br $B3
              end
              local.get $p1
              i32.load offset=8
              local.tee $p2
              local.get $l4
              i32.store offset=12
              local.get $l4
              local.get $p2
              i32.store offset=8
              br $B3
            end
            local.get $l4
            local.get $p1
            i32.const 16
            i32.add
            local.get $l5
            select
            local.set $l5
            loop $L6
              local.get $l5
              local.set $l6
              block $B7
                local.get $p2
                local.tee $l4
                i32.const 20
                i32.add
                local.tee $l5
                i32.load
                local.tee $p2
                br_if $B7
                local.get $l4
                i32.const 16
                i32.add
                local.set $l5
                local.get $l4
                i32.load offset=16
                local.set $p2
              end
              local.get $p2
              br_if $L6
            end
            local.get $l6
            i32.const 0
            i32.store
          end
          local.get $l3
          i32.eqz
          br_if $B0
          block $B8
            local.get $p0
            local.get $p1
            i32.const 28
            i32.add
            i32.load
            i32.const 2
            i32.shl
            i32.add
            i32.const 272
            i32.add
            local.tee $p2
            i32.load
            local.get $p1
            i32.eq
            br_if $B8
            local.get $l3
            i32.const 16
            i32.const 20
            local.get $l3
            i32.load offset=16
            local.get $p1
            i32.eq
            select
            i32.add
            local.get $l4
            i32.store
            local.get $l4
            i32.eqz
            br_if $B0
            br $B1
          end
          local.get $p2
          local.get $l4
          i32.store
          local.get $l4
          br_if $B1
          local.get $p0
          local.get $p0
          i32.load offset=4
          i32.const -2
          local.get $p1
          i32.load offset=28
          i32.rotl
          i32.and
          i32.store offset=4
          return
        end
        block $B9
          local.get $p1
          i32.const 12
          i32.add
          i32.load
          local.tee $l4
          local.get $p1
          i32.const 8
          i32.add
          i32.load
          local.tee $l5
          i32.eq
          br_if $B9
          local.get $l5
          local.get $l4
          i32.store offset=12
          local.get $l4
          local.get $l5
          i32.store offset=8
          return
        end
        local.get $p0
        local.get $p0
        i32.load
        i32.const -2
        local.get $p2
        i32.const 3
        i32.shr_u
        i32.rotl
        i32.and
        i32.store
        br $B0
      end
      local.get $l4
      local.get $l3
      i32.store offset=24
      block $B10
        local.get $p1
        i32.load offset=16
        local.tee $p2
        i32.eqz
        br_if $B10
        local.get $l4
        local.get $p2
        i32.store offset=16
        local.get $p2
        local.get $l4
        i32.store offset=24
      end
      local.get $p1
      i32.const 20
      i32.add
      i32.load
      local.tee $p2
      i32.eqz
      br_if $B0
      local.get $l4
      i32.const 20
      i32.add
      local.get $p2
      i32.store
      local.get $p2
      local.get $l4
      i32.store offset=24
      return
    end)
  (func $_ZN8dlmalloc8dlmalloc8Dlmalloc7realloc17h1e42fbdcdf2a4cf4E (type $t6) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32)
    i32.const 0
    local.set $l3
    block $B0
      local.get $p2
      i32.const -65588
      i32.gt_u
      br_if $B0
      i32.const 16
      local.get $p2
      i32.const 11
      i32.add
      i32.const -8
      i32.and
      local.get $p2
      i32.const 11
      i32.lt_u
      select
      local.set $l4
      local.get $p1
      i32.const -4
      i32.add
      local.tee $l5
      i32.load
      local.tee $l6
      i32.const -8
      i32.and
      local.set $l7
      block $B1
        block $B2
          block $B3
            block $B4
              block $B5
                block $B6
                  block $B7
                    local.get $l6
                    i32.const 3
                    i32.and
                    i32.eqz
                    br_if $B7
                    local.get $p1
                    i32.const -8
                    i32.add
                    local.tee $l8
                    local.get $l7
                    i32.add
                    local.set $l9
                    local.get $l7
                    local.get $l4
                    i32.ge_u
                    br_if $B6
                    local.get $p0
                    i32.load offset=412
                    local.get $l9
                    i32.eq
                    br_if $B5
                    local.get $p0
                    i32.load offset=408
                    local.get $l9
                    i32.eq
                    br_if $B4
                    local.get $l9
                    i32.const 4
                    i32.add
                    i32.load
                    local.tee $l6
                    i32.const 2
                    i32.and
                    br_if $B1
                    local.get $l6
                    i32.const -8
                    i32.and
                    local.tee $l6
                    local.get $l7
                    i32.add
                    local.tee $l7
                    local.get $l4
                    i32.ge_u
                    br_if $B3
                    br $B1
                  end
                  local.get $l4
                  i32.const 256
                  i32.lt_u
                  br_if $B1
                  local.get $l7
                  local.get $l4
                  i32.const 4
                  i32.or
                  i32.lt_u
                  br_if $B1
                  local.get $l7
                  local.get $l4
                  i32.sub
                  i32.const 131073
                  i32.ge_u
                  br_if $B1
                  br $B2
                end
                local.get $l7
                local.get $l4
                i32.sub
                local.tee $p2
                i32.const 16
                i32.lt_u
                br_if $B2
                local.get $l5
                local.get $l4
                local.get $l6
                i32.const 1
                i32.and
                i32.or
                i32.const 2
                i32.or
                i32.store
                local.get $l8
                local.get $l4
                i32.add
                local.tee $l3
                local.get $p2
                i32.const 3
                i32.or
                i32.store offset=4
                local.get $l9
                local.get $l9
                i32.load offset=4
                i32.const 1
                i32.or
                i32.store offset=4
                local.get $p0
                local.get $l3
                local.get $p2
                call $_ZN8dlmalloc8dlmalloc8Dlmalloc13dispose_chunk17he00c681454a3c3b7E
                br $B2
              end
              local.get $p0
              i32.load offset=404
              local.get $l7
              i32.add
              local.tee $l7
              local.get $l4
              i32.le_u
              br_if $B1
              local.get $l5
              local.get $l4
              local.get $l6
              i32.const 1
              i32.and
              i32.or
              i32.const 2
              i32.or
              i32.store
              local.get $l8
              local.get $l4
              i32.add
              local.tee $p2
              local.get $l7
              local.get $l4
              i32.sub
              local.tee $l3
              i32.const 1
              i32.or
              i32.store offset=4
              local.get $p0
              local.get $l3
              i32.store offset=404
              local.get $p0
              local.get $p2
              i32.store offset=412
              br $B2
            end
            local.get $p0
            i32.load offset=400
            local.get $l7
            i32.add
            local.tee $l7
            local.get $l4
            i32.lt_u
            br_if $B1
            block $B8
              block $B9
                local.get $l7
                local.get $l4
                i32.sub
                local.tee $p2
                i32.const 15
                i32.gt_u
                br_if $B9
                local.get $l5
                local.get $l6
                i32.const 1
                i32.and
                local.get $l7
                i32.or
                i32.const 2
                i32.or
                i32.store
                local.get $l8
                local.get $l7
                i32.add
                local.tee $p2
                local.get $p2
                i32.load offset=4
                i32.const 1
                i32.or
                i32.store offset=4
                i32.const 0
                local.set $p2
                i32.const 0
                local.set $l3
                br $B8
              end
              local.get $l5
              local.get $l4
              local.get $l6
              i32.const 1
              i32.and
              i32.or
              i32.const 2
              i32.or
              i32.store
              local.get $l8
              local.get $l4
              i32.add
              local.tee $l3
              local.get $p2
              i32.const 1
              i32.or
              i32.store offset=4
              local.get $l8
              local.get $l7
              i32.add
              local.tee $l4
              local.get $p2
              i32.store
              local.get $l4
              local.get $l4
              i32.load offset=4
              i32.const -2
              i32.and
              i32.store offset=4
            end
            local.get $p0
            local.get $l3
            i32.store offset=408
            local.get $p0
            local.get $p2
            i32.store offset=400
            br $B2
          end
          local.get $p0
          local.get $l9
          local.get $l6
          call $_ZN8dlmalloc8dlmalloc8Dlmalloc12unlink_chunk17hfe7c47108d1938c5E
          block $B10
            local.get $l7
            local.get $l4
            i32.sub
            local.tee $p2
            i32.const 16
            i32.lt_u
            br_if $B10
            local.get $l5
            local.get $l4
            local.get $l5
            i32.load
            i32.const 1
            i32.and
            i32.or
            i32.const 2
            i32.or
            i32.store
            local.get $l8
            local.get $l4
            i32.add
            local.tee $l3
            local.get $p2
            i32.const 3
            i32.or
            i32.store offset=4
            local.get $l8
            local.get $l7
            i32.add
            local.tee $l4
            local.get $l4
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
            local.get $p0
            local.get $l3
            local.get $p2
            call $_ZN8dlmalloc8dlmalloc8Dlmalloc13dispose_chunk17he00c681454a3c3b7E
            br $B2
          end
          local.get $l5
          local.get $l7
          local.get $l5
          i32.load
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store
          local.get $l8
          local.get $l7
          i32.add
          local.tee $p2
          local.get $p2
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
        end
        local.get $p1
        local.set $l3
        br $B0
      end
      local.get $p0
      local.get $p2
      call $_ZN8dlmalloc8dlmalloc8Dlmalloc6malloc17hb0329e71e24f7e2fE
      local.tee $l4
      i32.eqz
      br_if $B0
      local.get $l4
      local.get $p1
      local.get $p2
      i32.const -4
      i32.const -8
      local.get $l5
      i32.load
      local.tee $l3
      i32.const 3
      i32.and
      select
      local.get $l3
      i32.const -8
      i32.and
      i32.add
      local.tee $l3
      local.get $l3
      local.get $p2
      i32.gt_u
      select
      call $memcpy
      local.set $p2
      local.get $p0
      local.get $p1
      call $_ZN8dlmalloc8dlmalloc8Dlmalloc4free17h7ab57ecacfa2b1c3E
      local.get $p2
      return
    end
    local.get $l3)
  (func $_ZN8dlmalloc8dlmalloc8Dlmalloc13dispose_chunk17he00c681454a3c3b7E (type $t4) (param $p0 i32) (param $p1 i32) (param $p2 i32)
    (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32)
    local.get $p1
    local.get $p2
    i32.add
    local.set $l3
    block $B0
      block $B1
        block $B2
          block $B3
            block $B4
              local.get $p1
              i32.const 4
              i32.add
              i32.load
              local.tee $l4
              i32.const 1
              i32.and
              br_if $B4
              local.get $l4
              i32.const 3
              i32.and
              i32.eqz
              br_if $B3
              local.get $p1
              i32.load
              local.tee $l4
              local.get $p2
              i32.add
              local.set $p2
              block $B5
                local.get $p0
                i32.load offset=408
                local.get $p1
                local.get $l4
                i32.sub
                local.tee $p1
                i32.ne
                br_if $B5
                local.get $l3
                i32.load offset=4
                i32.const 3
                i32.and
                i32.const 3
                i32.ne
                br_if $B4
                local.get $p0
                local.get $p2
                i32.store offset=400
                local.get $l3
                local.get $l3
                i32.load offset=4
                i32.const -2
                i32.and
                i32.store offset=4
                local.get $p1
                local.get $p2
                i32.const 1
                i32.or
                i32.store offset=4
                local.get $l3
                local.get $p2
                i32.store
                return
              end
              local.get $p0
              local.get $p1
              local.get $l4
              call $_ZN8dlmalloc8dlmalloc8Dlmalloc12unlink_chunk17hfe7c47108d1938c5E
            end
            block $B6
              block $B7
                local.get $l3
                i32.const 4
                i32.add
                i32.load
                local.tee $l4
                i32.const 2
                i32.and
                i32.eqz
                br_if $B7
                local.get $l3
                i32.const 4
                i32.add
                local.get $l4
                i32.const -2
                i32.and
                i32.store
                local.get $p1
                local.get $p2
                i32.const 1
                i32.or
                i32.store offset=4
                local.get $p1
                local.get $p2
                i32.add
                local.get $p2
                i32.store
                br $B6
              end
              block $B8
                block $B9
                  local.get $p0
                  i32.load offset=412
                  local.get $l3
                  i32.eq
                  br_if $B9
                  local.get $p0
                  i32.load offset=408
                  local.get $l3
                  i32.eq
                  br_if $B8
                  local.get $p0
                  local.get $l3
                  local.get $l4
                  i32.const -8
                  i32.and
                  local.tee $l4
                  call $_ZN8dlmalloc8dlmalloc8Dlmalloc12unlink_chunk17hfe7c47108d1938c5E
                  local.get $p1
                  local.get $l4
                  local.get $p2
                  i32.add
                  local.tee $p2
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get $p1
                  local.get $p2
                  i32.add
                  local.get $p2
                  i32.store
                  local.get $p1
                  local.get $p0
                  i32.load offset=408
                  i32.ne
                  br_if $B6
                  local.get $p0
                  local.get $p2
                  i32.store offset=400
                  return
                end
                local.get $p0
                local.get $p1
                i32.store offset=412
                local.get $p0
                local.get $p0
                i32.load offset=404
                local.get $p2
                i32.add
                local.tee $p2
                i32.store offset=404
                local.get $p1
                local.get $p2
                i32.const 1
                i32.or
                i32.store offset=4
                local.get $p1
                local.get $p0
                i32.load offset=408
                i32.ne
                br_if $B3
                local.get $p0
                i32.const 0
                i32.store offset=400
                local.get $p0
                i32.const 0
                i32.store offset=408
                return
              end
              local.get $p0
              local.get $p1
              i32.store offset=408
              local.get $p0
              local.get $p0
              i32.load offset=400
              local.get $p2
              i32.add
              local.tee $p2
              i32.store offset=400
              local.get $p1
              local.get $p2
              i32.const 1
              i32.or
              i32.store offset=4
              local.get $p1
              local.get $p2
              i32.add
              local.get $p2
              i32.store
              return
            end
            local.get $p2
            i32.const 256
            i32.lt_u
            br_if $B0
            block $B10
              block $B11
                local.get $p2
                i32.const 8
                i32.shr_u
                local.tee $l4
                br_if $B11
                i32.const 0
                local.set $l3
                br $B10
              end
              i32.const 31
              local.set $l3
              local.get $p2
              i32.const 16777215
              i32.gt_u
              br_if $B10
              local.get $p2
              i32.const 6
              local.get $l4
              i32.clz
              local.tee $l3
              i32.sub
              i32.const 31
              i32.and
              i32.shr_u
              i32.const 1
              i32.and
              local.get $l3
              i32.const 1
              i32.shl
              i32.sub
              i32.const 62
              i32.add
              local.set $l3
            end
            local.get $p1
            i64.const 0
            i64.store offset=16 align=4
            local.get $p1
            i32.const 28
            i32.add
            local.get $l3
            i32.store
            local.get $p0
            local.get $l3
            i32.const 2
            i32.shl
            i32.add
            i32.const 272
            i32.add
            local.set $l4
            block $B12
              block $B13
                block $B14
                  local.get $p0
                  i32.const 4
                  i32.add
                  local.tee $p0
                  i32.load
                  local.tee $l5
                  i32.const 1
                  local.get $l3
                  i32.const 31
                  i32.and
                  i32.shl
                  local.tee $l6
                  i32.and
                  i32.eqz
                  br_if $B14
                  local.get $l4
                  i32.load
                  local.tee $l4
                  i32.const 4
                  i32.add
                  i32.load
                  i32.const -8
                  i32.and
                  local.get $p2
                  i32.ne
                  br_if $B13
                  local.get $l4
                  local.set $p0
                  br $B12
                end
                local.get $p0
                local.get $l5
                local.get $l6
                i32.or
                i32.store
                local.get $l4
                local.get $p1
                i32.store
                local.get $p1
                i32.const 24
                i32.add
                local.get $l4
                i32.store
                br $B1
              end
              local.get $p2
              i32.const 0
              i32.const 25
              local.get $l3
              i32.const 1
              i32.shr_u
              i32.sub
              i32.const 31
              i32.and
              local.get $l3
              i32.const 31
              i32.eq
              select
              i32.shl
              local.set $l3
              loop $L15
                local.get $l4
                local.get $l3
                i32.const 29
                i32.shr_u
                i32.const 4
                i32.and
                i32.add
                i32.const 16
                i32.add
                local.tee $l5
                i32.load
                local.tee $p0
                i32.eqz
                br_if $B2
                local.get $l3
                i32.const 1
                i32.shl
                local.set $l3
                local.get $p0
                local.set $l4
                local.get $p0
                i32.const 4
                i32.add
                i32.load
                i32.const -8
                i32.and
                local.get $p2
                i32.ne
                br_if $L15
              end
            end
            local.get $p0
            i32.load offset=8
            local.tee $p2
            local.get $p1
            i32.store offset=12
            local.get $p0
            local.get $p1
            i32.store offset=8
            local.get $p1
            i32.const 24
            i32.add
            i32.const 0
            i32.store
            local.get $p1
            local.get $p0
            i32.store offset=12
            local.get $p1
            local.get $p2
            i32.store offset=8
          end
          return
        end
        local.get $l5
        local.get $p1
        i32.store
        local.get $p1
        i32.const 24
        i32.add
        local.get $l4
        i32.store
      end
      local.get $p1
      local.get $p1
      i32.store offset=12
      local.get $p1
      local.get $p1
      i32.store offset=8
      return
    end
    local.get $p0
    local.get $p2
    i32.const 3
    i32.shr_u
    local.tee $l3
    i32.const 3
    i32.shl
    i32.add
    i32.const 8
    i32.add
    local.set $p2
    block $B16
      block $B17
        local.get $p0
        i32.load
        local.tee $l4
        i32.const 1
        local.get $l3
        i32.shl
        local.tee $l3
        i32.and
        i32.eqz
        br_if $B17
        local.get $p2
        i32.load offset=8
        local.set $p0
        br $B16
      end
      local.get $p0
      local.get $l4
      local.get $l3
      i32.or
      i32.store
      local.get $p2
      local.set $p0
    end
    local.get $p2
    local.get $p1
    i32.store offset=8
    local.get $p0
    local.get $p1
    i32.store offset=12
    local.get $p1
    local.get $p2
    i32.store offset=12
    local.get $p1
    local.get $p0
    i32.store offset=8)
  (func $_ZN8dlmalloc8dlmalloc8Dlmalloc4free17h7ab57ecacfa2b1c3E (type $t3) (param $p0 i32) (param $p1 i32)
    (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32)
    local.get $p1
    i32.const -8
    i32.add
    local.tee $l2
    local.get $p1
    i32.const -4
    i32.add
    i32.load
    local.tee $l3
    i32.const -8
    i32.and
    local.tee $p1
    i32.add
    local.set $l4
    block $B0
      block $B1
        block $B2
          block $B3
            local.get $l3
            i32.const 1
            i32.and
            br_if $B3
            local.get $l3
            i32.const 3
            i32.and
            i32.eqz
            br_if $B2
            local.get $l2
            i32.load
            local.tee $l3
            local.get $p1
            i32.add
            local.set $p1
            block $B4
              local.get $p0
              i32.load offset=408
              local.get $l2
              local.get $l3
              i32.sub
              local.tee $l2
              i32.ne
              br_if $B4
              local.get $l4
              i32.load offset=4
              i32.const 3
              i32.and
              i32.const 3
              i32.ne
              br_if $B3
              local.get $p0
              local.get $p1
              i32.store offset=400
              local.get $l4
              local.get $l4
              i32.load offset=4
              i32.const -2
              i32.and
              i32.store offset=4
              local.get $l2
              local.get $p1
              i32.const 1
              i32.or
              i32.store offset=4
              local.get $l2
              local.get $p1
              i32.add
              local.get $p1
              i32.store
              return
            end
            local.get $p0
            local.get $l2
            local.get $l3
            call $_ZN8dlmalloc8dlmalloc8Dlmalloc12unlink_chunk17hfe7c47108d1938c5E
          end
          block $B5
            block $B6
              local.get $l4
              i32.const 4
              i32.add
              local.tee $l5
              i32.load
              local.tee $l3
              i32.const 2
              i32.and
              i32.eqz
              br_if $B6
              local.get $l5
              local.get $l3
              i32.const -2
              i32.and
              i32.store
              local.get $l2
              local.get $p1
              i32.const 1
              i32.or
              i32.store offset=4
              local.get $l2
              local.get $p1
              i32.add
              local.get $p1
              i32.store
              br $B5
            end
            block $B7
              block $B8
                local.get $p0
                i32.load offset=412
                local.get $l4
                i32.eq
                br_if $B8
                local.get $p0
                i32.load offset=408
                local.get $l4
                i32.eq
                br_if $B7
                local.get $p0
                local.get $l4
                local.get $l3
                i32.const -8
                i32.and
                local.tee $l3
                call $_ZN8dlmalloc8dlmalloc8Dlmalloc12unlink_chunk17hfe7c47108d1938c5E
                local.get $l2
                local.get $l3
                local.get $p1
                i32.add
                local.tee $p1
                i32.const 1
                i32.or
                i32.store offset=4
                local.get $l2
                local.get $p1
                i32.add
                local.get $p1
                i32.store
                local.get $l2
                local.get $p0
                i32.load offset=408
                i32.ne
                br_if $B5
                local.get $p0
                local.get $p1
                i32.store offset=400
                return
              end
              local.get $p0
              local.get $l2
              i32.store offset=412
              local.get $p0
              local.get $p0
              i32.load offset=404
              local.get $p1
              i32.add
              local.tee $p1
              i32.store offset=404
              local.get $l2
              local.get $p1
              i32.const 1
              i32.or
              i32.store offset=4
              block $B9
                local.get $l2
                local.get $p0
                i32.load offset=408
                i32.ne
                br_if $B9
                local.get $p0
                i32.const 0
                i32.store offset=400
                local.get $p0
                i32.const 0
                i32.store offset=408
              end
              local.get $p0
              i32.const 440
              i32.add
              i32.load
              local.tee $l3
              local.get $p1
              i32.ge_u
              br_if $B2
              local.get $p0
              i32.load offset=412
              local.tee $p1
              i32.eqz
              br_if $B2
              block $B10
                local.get $p0
                i32.load offset=404
                local.tee $l5
                i32.const 41
                i32.lt_u
                br_if $B10
                local.get $p0
                i32.const 424
                i32.add
                local.set $l2
                loop $L11
                  block $B12
                    local.get $l2
                    i32.load
                    local.tee $l4
                    local.get $p1
                    i32.gt_u
                    br_if $B12
                    local.get $l4
                    local.get $l2
                    i32.load offset=4
                    i32.add
                    local.get $p1
                    i32.gt_u
                    br_if $B10
                  end
                  local.get $l2
                  i32.load offset=8
                  local.tee $l2
                  br_if $L11
                end
              end
              block $B13
                block $B14
                  local.get $p0
                  i32.const 432
                  i32.add
                  i32.load
                  local.tee $p1
                  br_if $B14
                  i32.const 4095
                  local.set $l2
                  br $B13
                end
                i32.const 0
                local.set $l2
                loop $L15
                  local.get $l2
                  i32.const 1
                  i32.add
                  local.set $l2
                  local.get $p1
                  i32.load offset=8
                  local.tee $p1
                  br_if $L15
                end
                local.get $l2
                i32.const 4095
                local.get $l2
                i32.const 4095
                i32.gt_u
                select
                local.set $l2
              end
              local.get $p0
              local.get $l2
              i32.store offset=448
              local.get $l5
              local.get $l3
              i32.le_u
              br_if $B2
              local.get $p0
              i32.const 440
              i32.add
              i32.const -1
              i32.store
              return
            end
            local.get $p0
            local.get $l2
            i32.store offset=408
            local.get $p0
            local.get $p0
            i32.load offset=400
            local.get $p1
            i32.add
            local.tee $p1
            i32.store offset=400
            local.get $l2
            local.get $p1
            i32.const 1
            i32.or
            i32.store offset=4
            local.get $l2
            local.get $p1
            i32.add
            local.get $p1
            i32.store
            return
          end
          local.get $p1
          i32.const 256
          i32.lt_u
          br_if $B1
          block $B16
            block $B17
              local.get $p1
              i32.const 8
              i32.shr_u
              local.tee $l3
              br_if $B17
              i32.const 0
              local.set $l4
              br $B16
            end
            i32.const 31
            local.set $l4
            local.get $p1
            i32.const 16777215
            i32.gt_u
            br_if $B16
            local.get $p1
            i32.const 6
            local.get $l3
            i32.clz
            local.tee $l4
            i32.sub
            i32.const 31
            i32.and
            i32.shr_u
            i32.const 1
            i32.and
            local.get $l4
            i32.const 1
            i32.shl
            i32.sub
            i32.const 62
            i32.add
            local.set $l4
          end
          local.get $l2
          i64.const 0
          i64.store offset=16 align=4
          local.get $l2
          i32.const 28
          i32.add
          local.get $l4
          i32.store
          local.get $p0
          local.get $l4
          i32.const 2
          i32.shl
          i32.add
          i32.const 272
          i32.add
          local.set $l3
          block $B18
            block $B19
              block $B20
                block $B21
                  block $B22
                    block $B23
                      local.get $p0
                      i32.const 4
                      i32.add
                      local.tee $l5
                      i32.load
                      local.tee $l6
                      i32.const 1
                      local.get $l4
                      i32.const 31
                      i32.and
                      i32.shl
                      local.tee $l7
                      i32.and
                      i32.eqz
                      br_if $B23
                      local.get $l3
                      i32.load
                      local.tee $l5
                      i32.const 4
                      i32.add
                      i32.load
                      i32.const -8
                      i32.and
                      local.get $p1
                      i32.ne
                      br_if $B22
                      local.get $l5
                      local.set $l4
                      br $B21
                    end
                    local.get $l5
                    local.get $l6
                    local.get $l7
                    i32.or
                    i32.store
                    local.get $l3
                    local.get $l2
                    i32.store
                    local.get $l2
                    i32.const 24
                    i32.add
                    local.get $l3
                    i32.store
                    br $B19
                  end
                  local.get $p1
                  i32.const 0
                  i32.const 25
                  local.get $l4
                  i32.const 1
                  i32.shr_u
                  i32.sub
                  i32.const 31
                  i32.and
                  local.get $l4
                  i32.const 31
                  i32.eq
                  select
                  i32.shl
                  local.set $l3
                  loop $L24
                    local.get $l5
                    local.get $l3
                    i32.const 29
                    i32.shr_u
                    i32.const 4
                    i32.and
                    i32.add
                    i32.const 16
                    i32.add
                    local.tee $l6
                    i32.load
                    local.tee $l4
                    i32.eqz
                    br_if $B20
                    local.get $l3
                    i32.const 1
                    i32.shl
                    local.set $l3
                    local.get $l4
                    local.set $l5
                    local.get $l4
                    i32.const 4
                    i32.add
                    i32.load
                    i32.const -8
                    i32.and
                    local.get $p1
                    i32.ne
                    br_if $L24
                  end
                end
                local.get $l4
                i32.load offset=8
                local.tee $p1
                local.get $l2
                i32.store offset=12
                local.get $l4
                local.get $l2
                i32.store offset=8
                local.get $l2
                i32.const 24
                i32.add
                i32.const 0
                i32.store
                local.get $l2
                local.get $l4
                i32.store offset=12
                local.get $l2
                local.get $p1
                i32.store offset=8
                br $B18
              end
              local.get $l6
              local.get $l2
              i32.store
              local.get $l2
              i32.const 24
              i32.add
              local.get $l5
              i32.store
            end
            local.get $l2
            local.get $l2
            i32.store offset=12
            local.get $l2
            local.get $l2
            i32.store offset=8
          end
          local.get $p0
          local.get $p0
          i32.load offset=448
          i32.const -1
          i32.add
          local.tee $l2
          i32.store offset=448
          local.get $l2
          i32.eqz
          br_if $B0
        end
        return
      end
      local.get $p0
      local.get $p1
      i32.const 3
      i32.shr_u
      local.tee $l4
      i32.const 3
      i32.shl
      i32.add
      i32.const 8
      i32.add
      local.set $p1
      block $B25
        block $B26
          local.get $p0
          i32.load
          local.tee $l3
          i32.const 1
          local.get $l4
          i32.shl
          local.tee $l4
          i32.and
          i32.eqz
          br_if $B26
          local.get $p1
          i32.load offset=8
          local.set $p0
          br $B25
        end
        local.get $p0
        local.get $l3
        local.get $l4
        i32.or
        i32.store
        local.get $p1
        local.set $p0
      end
      local.get $p1
      local.get $l2
      i32.store offset=8
      local.get $p0
      local.get $l2
      i32.store offset=12
      local.get $l2
      local.get $p1
      i32.store offset=12
      local.get $l2
      local.get $p0
      i32.store offset=8
      return
    end
    block $B27
      local.get $p0
      i32.const 432
      i32.add
      i32.load
      local.tee $p1
      br_if $B27
      local.get $p0
      i32.const 4095
      i32.store offset=448
      return
    end
    i32.const 0
    local.set $l2
    loop $L28
      local.get $l2
      i32.const 1
      i32.add
      local.set $l2
      local.get $p1
      i32.load offset=8
      local.tee $p1
      br_if $L28
    end
    local.get $p0
    local.get $l2
    i32.const 4095
    local.get $l2
    i32.const 4095
    i32.gt_u
    select
    i32.store offset=448)
  (func $_ZN8dlmalloc8dlmalloc8Dlmalloc8memalign17hf8d339f9992b4eadE (type $t6) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32)
    i32.const 0
    local.set $l3
    block $B0
      i32.const -65587
      local.get $p1
      i32.const 16
      local.get $p1
      i32.const 16
      i32.gt_u
      select
      local.tee $p1
      i32.sub
      local.get $p2
      i32.le_u
      br_if $B0
      local.get $p0
      local.get $p1
      i32.const 16
      local.get $p2
      i32.const 11
      i32.add
      i32.const -8
      i32.and
      local.get $p2
      i32.const 11
      i32.lt_u
      select
      local.tee $l4
      i32.add
      i32.const 12
      i32.add
      call $_ZN8dlmalloc8dlmalloc8Dlmalloc6malloc17hb0329e71e24f7e2fE
      local.tee $p2
      i32.eqz
      br_if $B0
      local.get $p2
      i32.const -8
      i32.add
      local.set $l3
      block $B1
        block $B2
          local.get $p1
          i32.const -1
          i32.add
          local.tee $l5
          local.get $p2
          i32.and
          br_if $B2
          local.get $l3
          local.set $p1
          br $B1
        end
        local.get $p2
        i32.const -4
        i32.add
        local.tee $l6
        i32.load
        local.tee $l7
        i32.const -8
        i32.and
        local.get $l5
        local.get $p2
        i32.add
        i32.const 0
        local.get $p1
        i32.sub
        i32.and
        i32.const -8
        i32.add
        local.tee $p2
        local.get $p2
        local.get $p1
        i32.add
        local.get $p2
        local.get $l3
        i32.sub
        i32.const 16
        i32.gt_u
        select
        local.tee $p1
        local.get $l3
        i32.sub
        local.tee $p2
        i32.sub
        local.set $l5
        block $B3
          local.get $l7
          i32.const 3
          i32.and
          i32.eqz
          br_if $B3
          local.get $p1
          local.get $l5
          local.get $p1
          i32.load offset=4
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store offset=4
          local.get $p1
          local.get $l5
          i32.add
          local.tee $l5
          local.get $l5
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get $l6
          local.get $p2
          local.get $l6
          i32.load
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store
          local.get $p1
          local.get $p1
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get $p0
          local.get $l3
          local.get $p2
          call $_ZN8dlmalloc8dlmalloc8Dlmalloc13dispose_chunk17he00c681454a3c3b7E
          br $B1
        end
        local.get $l3
        i32.load
        local.set $l3
        local.get $p1
        local.get $l5
        i32.store offset=4
        local.get $p1
        local.get $l3
        local.get $p2
        i32.add
        i32.store
      end
      block $B4
        local.get $p1
        i32.const 4
        i32.add
        i32.load
        local.tee $p2
        i32.const 3
        i32.and
        i32.eqz
        br_if $B4
        local.get $p2
        i32.const -8
        i32.and
        local.tee $l3
        local.get $l4
        i32.const 16
        i32.add
        i32.le_u
        br_if $B4
        local.get $p1
        i32.const 4
        i32.add
        local.get $l4
        local.get $p2
        i32.const 1
        i32.and
        i32.or
        i32.const 2
        i32.or
        i32.store
        local.get $p1
        local.get $l4
        i32.add
        local.tee $p2
        local.get $l3
        local.get $l4
        i32.sub
        local.tee $l4
        i32.const 3
        i32.or
        i32.store offset=4
        local.get $p1
        local.get $l3
        i32.add
        local.tee $l3
        local.get $l3
        i32.load offset=4
        i32.const 1
        i32.or
        i32.store offset=4
        local.get $p0
        local.get $p2
        local.get $l4
        call $_ZN8dlmalloc8dlmalloc8Dlmalloc13dispose_chunk17he00c681454a3c3b7E
      end
      local.get $p1
      i32.const 8
      i32.add
      local.set $l3
    end
    local.get $l3)
  (func $_ZN5alloc5alloc18handle_alloc_error17h02613b455f15ef67E (type $t3) (param $p0 i32) (param $p1 i32)
    local.get $p0
    local.get $p1
    call $rust_oom
    unreachable)
  (func $_ZN5alloc7raw_vec17capacity_overflow17heb1d9eef88f15a21E (type $t0)
    i32.const 1051232
    i32.const 17
    i32.const 1051252
    call $_ZN4core9panicking5panic17hc886a4cb4479b06eE
    unreachable)
  (func $_ZN5alloc6string104_$LT$impl$u20$core..convert..From$LT$alloc..string..String$GT$$u20$for$u20$alloc..vec..Vec$LT$u8$GT$$GT$4from17hc7c291bec8ce730aE (type $t3) (param $p0 i32) (param $p1 i32)
    local.get $p0
    local.get $p1
    i64.load align=4
    i64.store align=4
    local.get $p0
    i32.const 8
    i32.add
    local.get $p1
    i32.const 8
    i32.add
    i32.load
    i32.store)
  (func $_ZN4core3ops8function6FnOnce9call_once17h4d488110c8a675c3E (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    i32.load
    drop
    loop $L0 (result i32)
      br $L0
    end)
  (func $_ZN4core3ptr13drop_in_place17h00c08aab80423b88E (type $t1) (param $p0 i32))
  (func $_ZN4core9panicking18panic_bounds_check17hc3d961e9f5eff2edE (type $t4) (param $p0 i32) (param $p1 i32) (param $p2 i32)
    (local $l3 i32)
    global.get $g0
    i32.const 48
    i32.sub
    local.tee $l3
    global.set $g0
    local.get $l3
    local.get $p1
    i32.store offset=4
    local.get $l3
    local.get $p0
    i32.store
    local.get $l3
    i32.const 28
    i32.add
    i32.const 2
    i32.store
    local.get $l3
    i32.const 44
    i32.add
    i32.const 49
    i32.store
    local.get $l3
    i64.const 2
    i64.store offset=12 align=4
    local.get $l3
    i32.const 1051444
    i32.store offset=8
    local.get $l3
    i32.const 49
    i32.store offset=36
    local.get $l3
    local.get $l3
    i32.const 32
    i32.add
    i32.store offset=24
    local.get $l3
    local.get $l3
    i32.store offset=40
    local.get $l3
    local.get $l3
    i32.const 4
    i32.add
    i32.store offset=32
    local.get $l3
    i32.const 8
    i32.add
    local.get $p2
    call $_ZN4core9panicking9panic_fmt17h6aa2a8f84484b5f7E
    unreachable)
  (func $_ZN4core9panicking5panic17hc886a4cb4479b06eE (type $t4) (param $p0 i32) (param $p1 i32) (param $p2 i32)
    (local $l3 i32)
    global.get $g0
    i32.const 32
    i32.sub
    local.tee $l3
    global.set $g0
    local.get $l3
    i32.const 20
    i32.add
    i32.const 0
    i32.store
    local.get $l3
    i32.const 1051268
    i32.store offset=16
    local.get $l3
    i64.const 1
    i64.store offset=4 align=4
    local.get $l3
    local.get $p1
    i32.store offset=28
    local.get $l3
    local.get $p0
    i32.store offset=24
    local.get $l3
    local.get $l3
    i32.const 24
    i32.add
    i32.store
    local.get $l3
    local.get $p2
    call $_ZN4core9panicking9panic_fmt17h6aa2a8f84484b5f7E
    unreachable)
  (func $_ZN4core5slice24slice_end_index_len_fail17haeb08024239d8a09E (type $t4) (param $p0 i32) (param $p1 i32) (param $p2 i32)
    (local $l3 i32)
    global.get $g0
    i32.const 48
    i32.sub
    local.tee $l3
    global.set $g0
    local.get $l3
    local.get $p1
    i32.store offset=4
    local.get $l3
    local.get $p0
    i32.store
    local.get $l3
    i32.const 28
    i32.add
    i32.const 2
    i32.store
    local.get $l3
    i32.const 44
    i32.add
    i32.const 49
    i32.store
    local.get $l3
    i64.const 2
    i64.store offset=12 align=4
    local.get $l3
    i32.const 1052092
    i32.store offset=8
    local.get $l3
    i32.const 49
    i32.store offset=36
    local.get $l3
    local.get $l3
    i32.const 32
    i32.add
    i32.store offset=24
    local.get $l3
    local.get $l3
    i32.const 4
    i32.add
    i32.store offset=40
    local.get $l3
    local.get $l3
    i32.store offset=32
    local.get $l3
    i32.const 8
    i32.add
    local.get $p2
    call $_ZN4core9panicking9panic_fmt17h6aa2a8f84484b5f7E
    unreachable)
  (func $_ZN4core3fmt9Formatter3pad17hb011277a1901f9f7E (type $t6) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32) (local $l10 i32) (local $l11 i32) (local $l12 i32) (local $l13 i32) (local $l14 i32)
    local.get $p0
    i32.load offset=16
    local.set $l3
    block $B0
      block $B1
        block $B2
          block $B3
            local.get $p0
            i32.load offset=8
            local.tee $l4
            i32.const 1
            i32.eq
            br_if $B3
            local.get $l3
            i32.const 1
            i32.eq
            br_if $B2
            local.get $p0
            i32.load offset=24
            local.get $p1
            local.get $p2
            local.get $p0
            i32.const 28
            i32.add
            i32.load
            i32.load offset=12
            call_indirect (type $t6) $T0
            local.set $l3
            br $B0
          end
          local.get $l3
          i32.const 1
          i32.ne
          br_if $B1
        end
        block $B4
          block $B5
            local.get $p2
            br_if $B5
            i32.const 0
            local.set $p2
            br $B4
          end
          local.get $p1
          local.get $p2
          i32.add
          local.set $l5
          local.get $p0
          i32.const 20
          i32.add
          i32.load
          i32.const 1
          i32.add
          local.set $l6
          i32.const 0
          local.set $l7
          local.get $p1
          local.set $l3
          local.get $p1
          local.set $l8
          loop $L6
            local.get $l3
            i32.const 1
            i32.add
            local.set $l9
            block $B7
              block $B8
                block $B9
                  local.get $l3
                  i32.load8_s
                  local.tee $l10
                  i32.const -1
                  i32.gt_s
                  br_if $B9
                  block $B10
                    block $B11
                      local.get $l9
                      local.get $l5
                      i32.ne
                      br_if $B11
                      i32.const 0
                      local.set $l11
                      local.get $l5
                      local.set $l3
                      br $B10
                    end
                    local.get $l3
                    i32.load8_u offset=1
                    i32.const 63
                    i32.and
                    local.set $l11
                    local.get $l3
                    i32.const 2
                    i32.add
                    local.tee $l9
                    local.set $l3
                  end
                  local.get $l10
                  i32.const 31
                  i32.and
                  local.set $l12
                  block $B12
                    local.get $l10
                    i32.const 255
                    i32.and
                    local.tee $l10
                    i32.const 223
                    i32.gt_u
                    br_if $B12
                    local.get $l11
                    local.get $l12
                    i32.const 6
                    i32.shl
                    i32.or
                    local.set $l10
                    br $B8
                  end
                  block $B13
                    block $B14
                      local.get $l3
                      local.get $l5
                      i32.ne
                      br_if $B14
                      i32.const 0
                      local.set $l13
                      local.get $l5
                      local.set $l14
                      br $B13
                    end
                    local.get $l3
                    i32.load8_u
                    i32.const 63
                    i32.and
                    local.set $l13
                    local.get $l3
                    i32.const 1
                    i32.add
                    local.tee $l9
                    local.set $l14
                  end
                  local.get $l13
                  local.get $l11
                  i32.const 6
                  i32.shl
                  i32.or
                  local.set $l11
                  block $B15
                    local.get $l10
                    i32.const 240
                    i32.ge_u
                    br_if $B15
                    local.get $l11
                    local.get $l12
                    i32.const 12
                    i32.shl
                    i32.or
                    local.set $l10
                    br $B8
                  end
                  block $B16
                    block $B17
                      local.get $l14
                      local.get $l5
                      i32.ne
                      br_if $B17
                      i32.const 0
                      local.set $l10
                      local.get $l9
                      local.set $l3
                      br $B16
                    end
                    local.get $l14
                    i32.const 1
                    i32.add
                    local.set $l3
                    local.get $l14
                    i32.load8_u
                    i32.const 63
                    i32.and
                    local.set $l10
                  end
                  local.get $l11
                  i32.const 6
                  i32.shl
                  local.get $l12
                  i32.const 18
                  i32.shl
                  i32.const 1835008
                  i32.and
                  i32.or
                  local.get $l10
                  i32.or
                  local.tee $l10
                  i32.const 1114112
                  i32.ne
                  br_if $B7
                  br $B4
                end
                local.get $l10
                i32.const 255
                i32.and
                local.set $l10
              end
              local.get $l9
              local.set $l3
            end
            block $B18
              local.get $l6
              i32.const -1
              i32.add
              local.tee $l6
              i32.eqz
              br_if $B18
              local.get $l7
              local.get $l8
              i32.sub
              local.get $l3
              i32.add
              local.set $l7
              local.get $l3
              local.set $l8
              local.get $l5
              local.get $l3
              i32.ne
              br_if $L6
              br $B4
            end
          end
          local.get $l10
          i32.const 1114112
          i32.eq
          br_if $B4
          block $B19
            block $B20
              local.get $l7
              i32.eqz
              br_if $B20
              local.get $l7
              local.get $p2
              i32.eq
              br_if $B20
              i32.const 0
              local.set $l3
              local.get $l7
              local.get $p2
              i32.ge_u
              br_if $B19
              local.get $p1
              local.get $l7
              i32.add
              i32.load8_s
              i32.const -64
              i32.lt_s
              br_if $B19
            end
            local.get $p1
            local.set $l3
          end
          local.get $l7
          local.get $p2
          local.get $l3
          select
          local.set $p2
          local.get $l3
          local.get $p1
          local.get $l3
          select
          local.set $p1
        end
        local.get $l4
        i32.const 1
        i32.eq
        br_if $B1
        local.get $p0
        i32.load offset=24
        local.get $p1
        local.get $p2
        local.get $p0
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type $t6) $T0
        return
      end
      block $B21
        block $B22
          block $B23
            local.get $p2
            i32.eqz
            br_if $B23
            i32.const 0
            local.set $l9
            local.get $p2
            local.set $l10
            local.get $p1
            local.set $l3
            loop $L24
              local.get $l9
              local.get $l3
              i32.load8_u
              i32.const 192
              i32.and
              i32.const 128
              i32.eq
              i32.add
              local.set $l9
              local.get $l3
              i32.const 1
              i32.add
              local.set $l3
              local.get $l10
              i32.const -1
              i32.add
              local.tee $l10
              br_if $L24
            end
            local.get $p2
            local.get $l9
            i32.sub
            local.get $p0
            i32.load offset=12
            local.tee $l6
            i32.ge_u
            br_if $B22
            i32.const 0
            local.set $l9
            local.get $p2
            local.set $l10
            local.get $p1
            local.set $l3
            loop $L25
              local.get $l9
              local.get $l3
              i32.load8_u
              i32.const 192
              i32.and
              i32.const 128
              i32.eq
              i32.add
              local.set $l9
              local.get $l3
              i32.const 1
              i32.add
              local.set $l3
              local.get $l10
              i32.const -1
              i32.add
              local.tee $l10
              br_if $L25
              br $B21
            end
          end
          i32.const 0
          local.set $l9
          local.get $p0
          i32.load offset=12
          local.tee $l6
          br_if $B21
        end
        local.get $p0
        i32.load offset=24
        local.get $p1
        local.get $p2
        local.get $p0
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type $t6) $T0
        return
      end
      i32.const 0
      local.set $l3
      local.get $l9
      local.get $p2
      i32.sub
      local.get $l6
      i32.add
      local.tee $l6
      local.set $l10
      block $B26
        block $B27
          block $B28
            i32.const 0
            local.get $p0
            i32.load8_u offset=32
            local.tee $l9
            local.get $l9
            i32.const 3
            i32.eq
            select
            i32.const 3
            i32.and
            br_table $B26 $B27 $B28 $B27 $B26
          end
          local.get $l6
          i32.const 1
          i32.shr_u
          local.set $l3
          local.get $l6
          i32.const 1
          i32.add
          i32.const 1
          i32.shr_u
          local.set $l10
          br $B26
        end
        i32.const 0
        local.set $l10
        local.get $l6
        local.set $l3
      end
      local.get $l3
      i32.const 1
      i32.add
      local.set $l3
      block $B29
        loop $L30
          local.get $l3
          i32.const -1
          i32.add
          local.tee $l3
          i32.eqz
          br_if $B29
          local.get $p0
          i32.load offset=24
          local.get $p0
          i32.load offset=4
          local.get $p0
          i32.load offset=28
          i32.load offset=16
          call_indirect (type $t2) $T0
          i32.eqz
          br_if $L30
        end
        i32.const 1
        return
      end
      local.get $p0
      i32.load offset=4
      local.set $l9
      i32.const 1
      local.set $l3
      local.get $p0
      i32.load offset=24
      local.get $p1
      local.get $p2
      local.get $p0
      i32.load offset=28
      i32.load offset=12
      call_indirect (type $t6) $T0
      br_if $B0
      local.get $l10
      i32.const 1
      i32.add
      local.set $l3
      local.get $p0
      i32.load offset=28
      local.set $l10
      local.get $p0
      i32.load offset=24
      local.set $p0
      loop $L31
        block $B32
          local.get $l3
          i32.const -1
          i32.add
          local.tee $l3
          br_if $B32
          i32.const 0
          return
        end
        local.get $p0
        local.get $l9
        local.get $l10
        i32.load offset=16
        call_indirect (type $t2) $T0
        i32.eqz
        br_if $L31
      end
      i32.const 1
      return
    end
    local.get $l3)
  (func $_ZN4core3str16slice_error_fail17h26278b2259fb6582E (type $t8) (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32) (param $p4 i32)
    (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32) (local $l10 i32)
    global.get $g0
    i32.const 112
    i32.sub
    local.tee $l5
    global.set $g0
    local.get $l5
    local.get $p3
    i32.store offset=12
    local.get $l5
    local.get $p2
    i32.store offset=8
    i32.const 1
    local.set $l6
    local.get $p1
    local.set $l7
    block $B0
      local.get $p1
      i32.const 257
      i32.lt_u
      br_if $B0
      i32.const 0
      local.get $p1
      i32.sub
      local.set $l8
      i32.const 256
      local.set $l9
      loop $L1
        block $B2
          local.get $l9
          local.get $p1
          i32.ge_u
          br_if $B2
          i32.const 0
          local.set $l6
          local.get $p0
          local.get $l9
          i32.add
          i32.load8_s
          i32.const -65
          i32.le_s
          br_if $B2
          local.get $l9
          local.set $l7
          br $B0
        end
        local.get $l9
        i32.const -1
        i32.add
        local.set $l7
        i32.const 0
        local.set $l6
        local.get $l9
        i32.const 1
        i32.eq
        br_if $B0
        local.get $l8
        local.get $l9
        i32.add
        local.set $l10
        local.get $l7
        local.set $l9
        local.get $l10
        i32.const 1
        i32.ne
        br_if $L1
      end
    end
    local.get $l5
    local.get $l7
    i32.store offset=20
    local.get $l5
    local.get $p0
    i32.store offset=16
    local.get $l5
    i32.const 0
    i32.const 5
    local.get $l6
    select
    i32.store offset=28
    local.get $l5
    i32.const 1051268
    i32.const 1052208
    local.get $l6
    select
    i32.store offset=24
    block $B3
      block $B4
        block $B5
          block $B6
            local.get $p2
            local.get $p1
            i32.gt_u
            local.tee $l6
            br_if $B6
            local.get $p3
            local.get $p1
            i32.gt_u
            br_if $B6
            local.get $p2
            local.get $p3
            i32.gt_u
            br_if $B5
            block $B7
              block $B8
                local.get $p2
                i32.eqz
                br_if $B8
                local.get $p1
                local.get $p2
                i32.eq
                br_if $B8
                local.get $p1
                local.get $p2
                i32.le_u
                br_if $B7
                local.get $p0
                local.get $p2
                i32.add
                i32.load8_s
                i32.const -64
                i32.lt_s
                br_if $B7
              end
              local.get $p3
              local.set $p2
            end
            local.get $l5
            local.get $p2
            i32.store offset=32
            block $B9
              block $B10
                local.get $p2
                i32.eqz
                br_if $B10
                local.get $p2
                local.get $p1
                i32.ne
                br_if $B9
              end
              local.get $p2
              local.set $l6
              br $B4
            end
            local.get $p1
            i32.const 1
            i32.add
            local.set $l9
            loop $L11
              block $B12
                local.get $p2
                local.get $p1
                i32.ge_u
                br_if $B12
                local.get $p0
                local.get $p2
                i32.add
                i32.load8_s
                i32.const -64
                i32.lt_s
                br_if $B12
                local.get $l5
                i32.const 36
                i32.add
                local.set $l9
                local.get $p2
                local.set $l6
                br $B3
              end
              local.get $p2
              i32.const -1
              i32.add
              local.set $l6
              local.get $p2
              i32.const 1
              i32.eq
              br_if $B4
              local.get $l9
              local.get $p2
              i32.eq
              local.set $p3
              local.get $l6
              local.set $p2
              local.get $p3
              br_if $B4
              br $L11
            end
          end
          local.get $l5
          local.get $p2
          local.get $p3
          local.get $l6
          select
          i32.store offset=40
          local.get $l5
          i32.const 48
          i32.add
          i32.const 20
          i32.add
          i32.const 3
          i32.store
          local.get $l5
          i32.const 72
          i32.add
          i32.const 20
          i32.add
          i32.const 50
          i32.store
          local.get $l5
          i32.const 84
          i32.add
          i32.const 50
          i32.store
          local.get $l5
          i64.const 3
          i64.store offset=52 align=4
          local.get $l5
          i32.const 1052248
          i32.store offset=48
          local.get $l5
          i32.const 49
          i32.store offset=76
          local.get $l5
          local.get $l5
          i32.const 72
          i32.add
          i32.store offset=64
          local.get $l5
          local.get $l5
          i32.const 24
          i32.add
          i32.store offset=88
          local.get $l5
          local.get $l5
          i32.const 16
          i32.add
          i32.store offset=80
          local.get $l5
          local.get $l5
          i32.const 40
          i32.add
          i32.store offset=72
          local.get $l5
          i32.const 48
          i32.add
          local.get $p4
          call $_ZN4core9panicking9panic_fmt17h6aa2a8f84484b5f7E
          unreachable
        end
        local.get $l5
        i32.const 100
        i32.add
        i32.const 50
        i32.store
        local.get $l5
        i32.const 72
        i32.add
        i32.const 20
        i32.add
        i32.const 50
        i32.store
        local.get $l5
        i32.const 84
        i32.add
        i32.const 49
        i32.store
        local.get $l5
        i32.const 48
        i32.add
        i32.const 20
        i32.add
        i32.const 4
        i32.store
        local.get $l5
        i64.const 4
        i64.store offset=52 align=4
        local.get $l5
        i32.const 1052308
        i32.store offset=48
        local.get $l5
        i32.const 49
        i32.store offset=76
        local.get $l5
        local.get $l5
        i32.const 72
        i32.add
        i32.store offset=64
        local.get $l5
        local.get $l5
        i32.const 24
        i32.add
        i32.store offset=96
        local.get $l5
        local.get $l5
        i32.const 16
        i32.add
        i32.store offset=88
        local.get $l5
        local.get $l5
        i32.const 12
        i32.add
        i32.store offset=80
        local.get $l5
        local.get $l5
        i32.const 8
        i32.add
        i32.store offset=72
        local.get $l5
        i32.const 48
        i32.add
        local.get $p4
        call $_ZN4core9panicking9panic_fmt17h6aa2a8f84484b5f7E
        unreachable
      end
      local.get $l5
      i32.const 36
      i32.add
      local.set $l9
    end
    block $B13
      local.get $l6
      local.get $p1
      i32.eq
      br_if $B13
      i32.const 1
      local.set $p3
      block $B14
        block $B15
          block $B16
            block $B17
              local.get $p0
              local.get $l6
              i32.add
              local.tee $l7
              i32.load8_s
              local.tee $p2
              i32.const -1
              i32.gt_s
              br_if $B17
              i32.const 0
              local.set $p3
              local.get $p0
              local.get $p1
              i32.add
              local.tee $p1
              local.set $p0
              block $B18
                local.get $l7
                i32.const 1
                i32.add
                local.get $p1
                i32.eq
                br_if $B18
                local.get $l7
                i32.const 2
                i32.add
                local.set $p0
                local.get $l7
                i32.load8_u offset=1
                i32.const 63
                i32.and
                local.set $p3
              end
              local.get $p2
              i32.const 31
              i32.and
              local.set $l7
              local.get $p2
              i32.const 255
              i32.and
              i32.const 223
              i32.gt_u
              br_if $B16
              local.get $p3
              local.get $l7
              i32.const 6
              i32.shl
              i32.or
              local.set $p2
              br $B15
            end
            local.get $l5
            local.get $p2
            i32.const 255
            i32.and
            i32.store offset=36
            local.get $l5
            i32.const 40
            i32.add
            local.set $p1
            br $B14
          end
          i32.const 0
          local.set $l8
          local.get $p1
          local.set $l10
          block $B19
            local.get $p0
            local.get $p1
            i32.eq
            br_if $B19
            local.get $p0
            i32.const 1
            i32.add
            local.set $l10
            local.get $p0
            i32.load8_u
            i32.const 63
            i32.and
            local.set $l8
          end
          local.get $l8
          local.get $p3
          i32.const 6
          i32.shl
          i32.or
          local.set $p3
          block $B20
            local.get $p2
            i32.const 255
            i32.and
            i32.const 240
            i32.ge_u
            br_if $B20
            local.get $p3
            local.get $l7
            i32.const 12
            i32.shl
            i32.or
            local.set $p2
            br $B15
          end
          i32.const 0
          local.set $p2
          block $B21
            local.get $l10
            local.get $p1
            i32.eq
            br_if $B21
            local.get $l10
            i32.load8_u
            i32.const 63
            i32.and
            local.set $p2
          end
          local.get $p3
          i32.const 6
          i32.shl
          local.get $l7
          i32.const 18
          i32.shl
          i32.const 1835008
          i32.and
          i32.or
          local.get $p2
          i32.or
          local.tee $p2
          i32.const 1114112
          i32.eq
          br_if $B13
        end
        local.get $l5
        local.get $p2
        i32.store offset=36
        i32.const 1
        local.set $p3
        local.get $l5
        i32.const 40
        i32.add
        local.set $p1
        local.get $p2
        i32.const 128
        i32.lt_u
        br_if $B14
        i32.const 2
        local.set $p3
        local.get $p2
        i32.const 2048
        i32.lt_u
        br_if $B14
        i32.const 3
        i32.const 4
        local.get $p2
        i32.const 65536
        i32.lt_u
        select
        local.set $p3
      end
      local.get $l5
      local.get $l6
      i32.store offset=40
      local.get $l5
      local.get $p3
      local.get $l6
      i32.add
      i32.store offset=44
      local.get $l5
      i32.const 48
      i32.add
      i32.const 20
      i32.add
      i32.const 5
      i32.store
      local.get $l5
      i32.const 108
      i32.add
      i32.const 50
      i32.store
      local.get $l5
      i32.const 100
      i32.add
      i32.const 50
      i32.store
      local.get $l5
      i32.const 72
      i32.add
      i32.const 20
      i32.add
      i32.const 51
      i32.store
      local.get $l5
      i32.const 84
      i32.add
      i32.const 52
      i32.store
      local.get $l5
      i64.const 5
      i64.store offset=52 align=4
      local.get $l5
      i32.const 1052392
      i32.store offset=48
      local.get $l5
      local.get $p1
      i32.store offset=88
      local.get $l5
      local.get $l9
      i32.store offset=80
      local.get $l5
      i32.const 49
      i32.store offset=76
      local.get $l5
      local.get $l5
      i32.const 72
      i32.add
      i32.store offset=64
      local.get $l5
      local.get $l5
      i32.const 24
      i32.add
      i32.store offset=104
      local.get $l5
      local.get $l5
      i32.const 16
      i32.add
      i32.store offset=96
      local.get $l5
      local.get $l5
      i32.const 32
      i32.add
      i32.store offset=72
      local.get $l5
      i32.const 48
      i32.add
      local.get $p4
      call $_ZN4core9panicking9panic_fmt17h6aa2a8f84484b5f7E
      unreachable
    end
    i32.const 1051305
    i32.const 43
    local.get $p4
    call $_ZN4core9panicking5panic17hc886a4cb4479b06eE
    unreachable)
  (func $_ZN4core9panicking9panic_fmt17h6aa2a8f84484b5f7E (type $t3) (param $p0 i32) (param $p1 i32)
    (local $l2 i32)
    global.get $g0
    i32.const 16
    i32.sub
    local.tee $l2
    global.set $g0
    local.get $l2
    local.get $p1
    i32.store offset=12
    local.get $l2
    local.get $p0
    i32.store offset=8
    local.get $l2
    i32.const 1051376
    i32.store offset=4
    local.get $l2
    i32.const 1051268
    i32.store
    local.get $l2
    call $rust_begin_unwind
    unreachable)
  (func $_ZN4core5slice22slice_index_order_fail17h0c8c4de89bda6894E (type $t4) (param $p0 i32) (param $p1 i32) (param $p2 i32)
    (local $l3 i32)
    global.get $g0
    i32.const 48
    i32.sub
    local.tee $l3
    global.set $g0
    local.get $l3
    local.get $p1
    i32.store offset=4
    local.get $l3
    local.get $p0
    i32.store
    local.get $l3
    i32.const 28
    i32.add
    i32.const 2
    i32.store
    local.get $l3
    i32.const 44
    i32.add
    i32.const 49
    i32.store
    local.get $l3
    i64.const 2
    i64.store offset=12 align=4
    local.get $l3
    i32.const 1052144
    i32.store offset=8
    local.get $l3
    i32.const 49
    i32.store offset=36
    local.get $l3
    local.get $l3
    i32.const 32
    i32.add
    i32.store offset=24
    local.get $l3
    local.get $l3
    i32.const 4
    i32.add
    i32.store offset=40
    local.get $l3
    local.get $l3
    i32.store offset=32
    local.get $l3
    i32.const 8
    i32.add
    local.get $p2
    call $_ZN4core9panicking9panic_fmt17h6aa2a8f84484b5f7E
    unreachable)
  (func $_ZN4core5slice26slice_start_index_len_fail17h46c23795afd32c64E (type $t4) (param $p0 i32) (param $p1 i32) (param $p2 i32)
    (local $l3 i32)
    global.get $g0
    i32.const 48
    i32.sub
    local.tee $l3
    global.set $g0
    local.get $l3
    local.get $p1
    i32.store offset=4
    local.get $l3
    local.get $p0
    i32.store
    local.get $l3
    i32.const 28
    i32.add
    i32.const 2
    i32.store
    local.get $l3
    i32.const 44
    i32.add
    i32.const 49
    i32.store
    local.get $l3
    i64.const 2
    i64.store offset=12 align=4
    local.get $l3
    i32.const 1052060
    i32.store offset=8
    local.get $l3
    i32.const 49
    i32.store offset=36
    local.get $l3
    local.get $l3
    i32.const 32
    i32.add
    i32.store offset=24
    local.get $l3
    local.get $l3
    i32.const 4
    i32.add
    i32.store offset=40
    local.get $l3
    local.get $l3
    i32.store offset=32
    local.get $l3
    i32.const 8
    i32.add
    local.get $p2
    call $_ZN4core9panicking9panic_fmt17h6aa2a8f84484b5f7E
    unreachable)
  (func $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17hf76888becbde89b4E (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    i64.load32_u
    i32.const 1
    local.get $p1
    call $_ZN4core3fmt3num3imp7fmt_u6417h93f5bc195622e061E)
  (func $_ZN4core3fmt5write17hb395f946a5ce2cabE (type $t6) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32) (local $l10 i32) (local $l11 i32) (local $l12 i32)
    global.get $g0
    i32.const 48
    i32.sub
    local.tee $l3
    global.set $g0
    local.get $l3
    i32.const 36
    i32.add
    local.get $p1
    i32.store
    local.get $l3
    i32.const 3
    i32.store8 offset=40
    local.get $l3
    i64.const 137438953472
    i64.store offset=8
    local.get $l3
    local.get $p0
    i32.store offset=32
    local.get $l3
    i32.const 0
    i32.store offset=24
    local.get $l3
    i32.const 0
    i32.store offset=16
    block $B0
      block $B1
        block $B2
          block $B3
            block $B4
              local.get $p2
              i32.load offset=8
              local.tee $l4
              i32.eqz
              br_if $B4
              local.get $p2
              i32.load
              local.set $l5
              local.get $p2
              i32.load offset=4
              local.tee $l6
              local.get $p2
              i32.const 12
              i32.add
              i32.load
              local.tee $l7
              local.get $l7
              local.get $l6
              i32.gt_u
              select
              local.tee $l8
              i32.eqz
              br_if $B3
              local.get $p0
              local.get $l5
              i32.load
              local.get $l5
              i32.load offset=4
              local.get $p1
              i32.load offset=12
              call_indirect (type $t6) $T0
              br_if $B1
              local.get $l5
              i32.const 12
              i32.add
              local.set $p0
              local.get $p2
              i32.load offset=20
              local.set $l9
              local.get $p2
              i32.load offset=16
              local.set $l10
              local.get $l8
              local.set $l11
              loop $L5
                local.get $l3
                local.get $l4
                i32.const 28
                i32.add
                i32.load8_u
                i32.store8 offset=40
                local.get $l3
                local.get $l4
                i32.const 4
                i32.add
                i64.load align=4
                i64.const 32
                i64.rotl
                i64.store offset=8
                local.get $l4
                i32.const 24
                i32.add
                i32.load
                local.set $p2
                i32.const 0
                local.set $l7
                i32.const 0
                local.set $p1
                block $B6
                  block $B7
                    block $B8
                      local.get $l4
                      i32.const 20
                      i32.add
                      i32.load
                      br_table $B7 $B8 $B6 $B7
                    end
                    block $B9
                      local.get $p2
                      local.get $l9
                      i32.lt_u
                      br_if $B9
                      local.get $p2
                      local.get $l9
                      i32.const 1051880
                      call $_ZN4core9panicking18panic_bounds_check17hc3d961e9f5eff2edE
                      unreachable
                    end
                    local.get $p2
                    i32.const 3
                    i32.shl
                    local.set $l12
                    i32.const 0
                    local.set $p1
                    local.get $l10
                    local.get $l12
                    i32.add
                    local.tee $l12
                    i32.load offset=4
                    i32.const 53
                    i32.ne
                    br_if $B6
                    local.get $l12
                    i32.load
                    i32.load
                    local.set $p2
                  end
                  i32.const 1
                  local.set $p1
                end
                local.get $l3
                local.get $p2
                i32.store offset=20
                local.get $l3
                local.get $p1
                i32.store offset=16
                local.get $l4
                i32.const 16
                i32.add
                i32.load
                local.set $p2
                block $B10
                  block $B11
                    block $B12
                      local.get $l4
                      i32.const 12
                      i32.add
                      i32.load
                      br_table $B11 $B12 $B10 $B11
                    end
                    block $B13
                      local.get $p2
                      local.get $l9
                      i32.lt_u
                      br_if $B13
                      local.get $p2
                      local.get $l9
                      i32.const 1051880
                      call $_ZN4core9panicking18panic_bounds_check17hc3d961e9f5eff2edE
                      unreachable
                    end
                    local.get $p2
                    i32.const 3
                    i32.shl
                    local.set $p1
                    local.get $l10
                    local.get $p1
                    i32.add
                    local.tee $p1
                    i32.load offset=4
                    i32.const 53
                    i32.ne
                    br_if $B10
                    local.get $p1
                    i32.load
                    i32.load
                    local.set $p2
                  end
                  i32.const 1
                  local.set $l7
                end
                local.get $l3
                local.get $p2
                i32.store offset=28
                local.get $l3
                local.get $l7
                i32.store offset=24
                block $B14
                  local.get $l4
                  i32.load
                  local.tee $p2
                  local.get $l9
                  i32.ge_u
                  br_if $B14
                  local.get $l10
                  local.get $p2
                  i32.const 3
                  i32.shl
                  i32.add
                  local.tee $p2
                  i32.load
                  local.get $l3
                  i32.const 8
                  i32.add
                  local.get $p2
                  i32.load offset=4
                  call_indirect (type $t2) $T0
                  br_if $B1
                  local.get $l11
                  i32.const -1
                  i32.add
                  local.tee $l11
                  i32.eqz
                  br_if $B2
                  local.get $l4
                  i32.const 32
                  i32.add
                  local.set $l4
                  local.get $p0
                  i32.const -4
                  i32.add
                  local.set $p2
                  local.get $p0
                  i32.load
                  local.set $p1
                  local.get $p0
                  i32.const 8
                  i32.add
                  local.set $p0
                  local.get $l3
                  i32.load offset=32
                  local.get $p2
                  i32.load
                  local.get $p1
                  local.get $l3
                  i32.load offset=36
                  i32.load offset=12
                  call_indirect (type $t6) $T0
                  i32.eqz
                  br_if $L5
                  br $B1
                end
              end
              local.get $p2
              local.get $l9
              i32.const 1051864
              call $_ZN4core9panicking18panic_bounds_check17hc3d961e9f5eff2edE
              unreachable
            end
            local.get $p2
            i32.load
            local.set $l5
            local.get $p2
            i32.load offset=4
            local.tee $l6
            local.get $p2
            i32.const 20
            i32.add
            i32.load
            local.tee $l4
            local.get $l4
            local.get $l6
            i32.gt_u
            select
            local.tee $l8
            i32.eqz
            br_if $B3
            local.get $p2
            i32.load offset=16
            local.set $l4
            local.get $p0
            local.get $l5
            i32.load
            local.get $l5
            i32.load offset=4
            local.get $p1
            i32.load offset=12
            call_indirect (type $t6) $T0
            br_if $B1
            local.get $l5
            i32.const 12
            i32.add
            local.set $p0
            local.get $l8
            local.set $p2
            loop $L15
              local.get $l4
              i32.load
              local.get $l3
              i32.const 8
              i32.add
              local.get $l4
              i32.const 4
              i32.add
              i32.load
              call_indirect (type $t2) $T0
              br_if $B1
              local.get $p2
              i32.const -1
              i32.add
              local.tee $p2
              i32.eqz
              br_if $B2
              local.get $l4
              i32.const 8
              i32.add
              local.set $l4
              local.get $p0
              i32.const -4
              i32.add
              local.set $p1
              local.get $p0
              i32.load
              local.set $l7
              local.get $p0
              i32.const 8
              i32.add
              local.set $p0
              local.get $l3
              i32.load offset=32
              local.get $p1
              i32.load
              local.get $l7
              local.get $l3
              i32.load offset=36
              i32.load offset=12
              call_indirect (type $t6) $T0
              i32.eqz
              br_if $L15
              br $B1
            end
          end
          i32.const 0
          local.set $l8
        end
        block $B16
          local.get $l6
          local.get $l8
          i32.le_u
          br_if $B16
          local.get $l3
          i32.load offset=32
          local.get $l5
          local.get $l8
          i32.const 3
          i32.shl
          i32.add
          local.tee $l4
          i32.load
          local.get $l4
          i32.load offset=4
          local.get $l3
          i32.load offset=36
          i32.load offset=12
          call_indirect (type $t6) $T0
          br_if $B1
        end
        i32.const 0
        local.set $l4
        br $B0
      end
      i32.const 1
      local.set $l4
    end
    local.get $l3
    i32.const 48
    i32.add
    global.set $g0
    local.get $l4)
  (func $_ZN71_$LT$core..ops..range..Range$LT$Idx$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h2e46a5c0d45e01feE (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32) (local $l3 i32) (local $l4 i32)
    global.get $g0
    i32.const 32
    i32.sub
    local.tee $l2
    global.set $g0
    block $B0
      block $B1
        local.get $p0
        local.get $p1
        call $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17he367d82e7bbd21f5E
        br_if $B1
        local.get $p1
        i32.const 28
        i32.add
        i32.load
        local.set $l3
        local.get $p1
        i32.load offset=24
        local.set $l4
        local.get $l2
        i32.const 28
        i32.add
        i32.const 0
        i32.store
        local.get $l2
        i32.const 1051268
        i32.store offset=24
        local.get $l2
        i64.const 1
        i64.store offset=12 align=4
        local.get $l2
        i32.const 1051272
        i32.store offset=8
        local.get $l4
        local.get $l3
        local.get $l2
        i32.const 8
        i32.add
        call $_ZN4core3fmt5write17hb395f946a5ce2cabE
        i32.eqz
        br_if $B0
      end
      local.get $l2
      i32.const 32
      i32.add
      global.set $g0
      i32.const 1
      return
    end
    local.get $p0
    i32.const 4
    i32.add
    local.get $p1
    call $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17he367d82e7bbd21f5E
    local.set $p1
    local.get $l2
    i32.const 32
    i32.add
    global.set $g0
    local.get $p1)
  (func $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17he367d82e7bbd21f5E (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32) (local $l3 i32) (local $l4 i32)
    global.get $g0
    i32.const 128
    i32.sub
    local.tee $l2
    global.set $g0
    block $B0
      block $B1
        block $B2
          block $B3
            block $B4
              local.get $p1
              i32.load
              local.tee $l3
              i32.const 16
              i32.and
              br_if $B4
              local.get $p0
              i32.load
              local.set $l4
              local.get $l3
              i32.const 32
              i32.and
              br_if $B3
              local.get $l4
              i64.extend_i32_u
              i32.const 1
              local.get $p1
              call $_ZN4core3fmt3num3imp7fmt_u6417h93f5bc195622e061E
              local.set $p0
              br $B2
            end
            local.get $p0
            i32.load
            local.set $l4
            i32.const 0
            local.set $p0
            loop $L5
              local.get $l2
              local.get $p0
              i32.add
              i32.const 127
              i32.add
              local.get $l4
              i32.const 15
              i32.and
              local.tee $l3
              i32.const 48
              i32.or
              local.get $l3
              i32.const 87
              i32.add
              local.get $l3
              i32.const 10
              i32.lt_u
              select
              i32.store8
              local.get $p0
              i32.const -1
              i32.add
              local.set $p0
              local.get $l4
              i32.const 4
              i32.shr_u
              local.tee $l4
              br_if $L5
            end
            local.get $p0
            i32.const 128
            i32.add
            local.tee $l4
            i32.const 129
            i32.ge_u
            br_if $B1
            local.get $p1
            i32.const 1
            i32.const 1051608
            i32.const 2
            local.get $l2
            local.get $p0
            i32.add
            i32.const 128
            i32.add
            i32.const 0
            local.get $p0
            i32.sub
            call $_ZN4core3fmt9Formatter12pad_integral17h05ee6133195a52bcE
            local.set $p0
            br $B2
          end
          i32.const 0
          local.set $p0
          loop $L6
            local.get $l2
            local.get $p0
            i32.add
            i32.const 127
            i32.add
            local.get $l4
            i32.const 15
            i32.and
            local.tee $l3
            i32.const 48
            i32.or
            local.get $l3
            i32.const 55
            i32.add
            local.get $l3
            i32.const 10
            i32.lt_u
            select
            i32.store8
            local.get $p0
            i32.const -1
            i32.add
            local.set $p0
            local.get $l4
            i32.const 4
            i32.shr_u
            local.tee $l4
            br_if $L6
          end
          local.get $p0
          i32.const 128
          i32.add
          local.tee $l4
          i32.const 129
          i32.ge_u
          br_if $B0
          local.get $p1
          i32.const 1
          i32.const 1051608
          i32.const 2
          local.get $l2
          local.get $p0
          i32.add
          i32.const 128
          i32.add
          i32.const 0
          local.get $p0
          i32.sub
          call $_ZN4core3fmt9Formatter12pad_integral17h05ee6133195a52bcE
          local.set $p0
        end
        local.get $l2
        i32.const 128
        i32.add
        global.set $g0
        local.get $p0
        return
      end
      local.get $l4
      i32.const 128
      i32.const 1051592
      call $_ZN4core5slice26slice_start_index_len_fail17h46c23795afd32c64E
      unreachable
    end
    local.get $l4
    i32.const 128
    i32.const 1051592
    call $_ZN4core5slice26slice_start_index_len_fail17h46c23795afd32c64E
    unreachable)
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h128e23c99f6446a5E (type $t10) (param $p0 i32) (result i64)
    i64.const 5966890128770411197)
  (func $_ZN60_$LT$core..cell..BorrowError$u20$as$u20$core..fmt..Debug$GT$3fmt17hd3dc522e1f283df9E (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p1
    i32.load offset=24
    i32.const 1051280
    i32.const 11
    local.get $p1
    i32.const 28
    i32.add
    i32.load
    i32.load offset=12
    call_indirect (type $t6) $T0)
  (func $_ZN63_$LT$core..cell..BorrowMutError$u20$as$u20$core..fmt..Debug$GT$3fmt17hced3ea94d7abc7c3E (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p1
    i32.load offset=24
    i32.const 1051291
    i32.const 14
    local.get $p1
    i32.const 28
    i32.add
    i32.load
    i32.load offset=12
    call_indirect (type $t6) $T0)
  (func $_ZN4core6option13expect_failed17hafe643dc99f2fb33E (type $t4) (param $p0 i32) (param $p1 i32) (param $p2 i32)
    (local $l3 i32)
    global.get $g0
    i32.const 48
    i32.sub
    local.tee $l3
    global.set $g0
    local.get $l3
    local.get $p1
    i32.store offset=12
    local.get $l3
    local.get $p0
    i32.store offset=8
    local.get $l3
    i32.const 36
    i32.add
    i32.const 1
    i32.store
    local.get $l3
    i64.const 1
    i64.store offset=20 align=4
    local.get $l3
    i32.const 1051348
    i32.store offset=16
    local.get $l3
    i32.const 50
    i32.store offset=44
    local.get $l3
    local.get $l3
    i32.const 40
    i32.add
    i32.store offset=32
    local.get $l3
    local.get $l3
    i32.const 8
    i32.add
    i32.store offset=40
    local.get $l3
    i32.const 16
    i32.add
    local.get $p2
    call $_ZN4core9panicking9panic_fmt17h6aa2a8f84484b5f7E
    unreachable)
  (func $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h1a51066d15be9a53E (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p1
    local.get $p0
    i32.load
    local.get $p0
    i32.load offset=4
    call $_ZN4core3fmt9Formatter3pad17hb011277a1901f9f7E)
  (func $_ZN4core6option18expect_none_failed17h6878b94853f7ecf6E (type $t8) (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32) (param $p4 i32)
    (local $l5 i32)
    global.get $g0
    i32.const 64
    i32.sub
    local.tee $l5
    global.set $g0
    local.get $l5
    local.get $p1
    i32.store offset=12
    local.get $l5
    local.get $p0
    i32.store offset=8
    local.get $l5
    local.get $p3
    i32.store offset=20
    local.get $l5
    local.get $p2
    i32.store offset=16
    local.get $l5
    i32.const 44
    i32.add
    i32.const 2
    i32.store
    local.get $l5
    i32.const 60
    i32.add
    i32.const 54
    i32.store
    local.get $l5
    i64.const 2
    i64.store offset=28 align=4
    local.get $l5
    i32.const 1051360
    i32.store offset=24
    local.get $l5
    i32.const 50
    i32.store offset=52
    local.get $l5
    local.get $l5
    i32.const 48
    i32.add
    i32.store offset=40
    local.get $l5
    local.get $l5
    i32.const 16
    i32.add
    i32.store offset=56
    local.get $l5
    local.get $l5
    i32.const 8
    i32.add
    i32.store offset=48
    local.get $l5
    i32.const 24
    i32.add
    local.get $p4
    call $_ZN4core9panicking9panic_fmt17h6aa2a8f84484b5f7E
    unreachable)
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hf0970a00b42f5ba2E (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    i32.load
    local.get $p1
    local.get $p0
    i32.load offset=4
    i32.load offset=12
    call_indirect (type $t2) $T0)
  (func $_ZN4core5panic9PanicInfo7message17h1ce7bd5bc7e6939cE (type $t5) (param $p0 i32) (result i32)
    local.get $p0
    i32.load offset=8)
  (func $_ZN4core5panic9PanicInfo8location17h96ba60a01800530cE (type $t5) (param $p0 i32) (result i32)
    local.get $p0
    i32.load offset=12)
  (func $_ZN4core5panic8Location6caller17hbeb99f2804420dffE (type $t5) (param $p0 i32) (result i32)
    local.get $p0)
  (func $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h44ce8e8e61187795E (type $t6) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32)
    global.get $g0
    i32.const 48
    i32.sub
    local.tee $l3
    global.set $g0
    block $B0
      block $B1
        local.get $p2
        br_if $B1
        i32.const 0
        local.set $l4
        br $B0
      end
      local.get $l3
      i32.const 40
      i32.add
      local.set $l5
      block $B2
        block $B3
          block $B4
            loop $L5
              block $B6
                local.get $p0
                i32.load offset=8
                i32.load8_u
                i32.eqz
                br_if $B6
                local.get $p0
                i32.load
                i32.const 1051516
                i32.const 4
                local.get $p0
                i32.load offset=4
                i32.load offset=12
                call_indirect (type $t6) $T0
                br_if $B2
              end
              local.get $l3
              i32.const 10
              i32.store offset=40
              local.get $l3
              i64.const 4294967306
              i64.store offset=32
              local.get $l3
              local.get $p2
              i32.store offset=28
              local.get $l3
              i32.const 0
              i32.store offset=24
              local.get $l3
              local.get $p2
              i32.store offset=20
              local.get $l3
              local.get $p1
              i32.store offset=16
              local.get $l3
              i32.const 8
              i32.add
              i32.const 10
              local.get $p1
              local.get $p2
              call $_ZN4core5slice6memchr6memchr17h0f2bc0ed161f00a2E
              block $B7
                block $B8
                  block $B9
                    block $B10
                      local.get $l3
                      i32.load offset=8
                      i32.const 1
                      i32.ne
                      br_if $B10
                      local.get $l3
                      i32.load offset=12
                      local.set $l4
                      loop $L11
                        local.get $l3
                        local.get $l4
                        local.get $l3
                        i32.load offset=24
                        i32.add
                        i32.const 1
                        i32.add
                        local.tee $l4
                        i32.store offset=24
                        block $B12
                          block $B13
                            local.get $l4
                            local.get $l3
                            i32.load offset=36
                            local.tee $l6
                            i32.ge_u
                            br_if $B13
                            local.get $l3
                            i32.load offset=20
                            local.set $l7
                            br $B12
                          end
                          local.get $l3
                          i32.load offset=20
                          local.tee $l7
                          local.get $l4
                          i32.lt_u
                          br_if $B12
                          local.get $l6
                          i32.const 5
                          i32.ge_u
                          br_if $B4
                          local.get $l3
                          i32.load offset=16
                          local.get $l4
                          local.get $l6
                          i32.sub
                          local.tee $l8
                          i32.add
                          local.tee $l9
                          local.get $l5
                          i32.eq
                          br_if $B8
                          local.get $l9
                          local.get $l5
                          local.get $l6
                          call $bcmp
                          i32.eqz
                          br_if $B8
                        end
                        local.get $l3
                        i32.load offset=28
                        local.tee $l9
                        local.get $l4
                        i32.lt_u
                        br_if $B9
                        local.get $l7
                        local.get $l9
                        i32.lt_u
                        br_if $B9
                        local.get $l3
                        local.get $l6
                        local.get $l3
                        i32.const 16
                        i32.add
                        i32.add
                        i32.const 23
                        i32.add
                        i32.load8_u
                        local.get $l3
                        i32.load offset=16
                        local.get $l4
                        i32.add
                        local.get $l9
                        local.get $l4
                        i32.sub
                        call $_ZN4core5slice6memchr6memchr17h0f2bc0ed161f00a2E
                        local.get $l3
                        i32.load offset=4
                        local.set $l4
                        local.get $l3
                        i32.load
                        i32.const 1
                        i32.eq
                        br_if $L11
                      end
                    end
                    local.get $l3
                    local.get $l3
                    i32.load offset=28
                    i32.store offset=24
                  end
                  local.get $p0
                  i32.load offset=8
                  i32.const 0
                  i32.store8
                  local.get $p2
                  local.set $l4
                  br $B7
                end
                local.get $p0
                i32.load offset=8
                i32.const 1
                i32.store8
                local.get $l8
                i32.const 1
                i32.add
                local.set $l4
              end
              local.get $p0
              i32.load offset=4
              local.set $l9
              local.get $p0
              i32.load
              local.set $l6
              block $B14
                block $B15
                  block $B16
                    local.get $l4
                    i32.eqz
                    br_if $B16
                    local.get $p2
                    local.get $l4
                    i32.eq
                    br_if $B16
                    block $B17
                      local.get $p2
                      local.get $l4
                      i32.le_u
                      br_if $B17
                      local.get $p1
                      local.get $l4
                      i32.add
                      local.tee $l7
                      i32.load8_s
                      i32.const -65
                      i32.gt_s
                      br_if $B15
                    end
                    local.get $p1
                    local.get $p2
                    i32.const 0
                    local.get $l4
                    i32.const 1051520
                    call $_ZN4core3str16slice_error_fail17h26278b2259fb6582E
                    unreachable
                  end
                  local.get $l6
                  local.get $p1
                  local.get $l4
                  local.get $l9
                  i32.load offset=12
                  call_indirect (type $t6) $T0
                  br_if $B2
                  br $B14
                end
                local.get $l6
                local.get $p1
                local.get $l4
                local.get $l9
                i32.load offset=12
                call_indirect (type $t6) $T0
                br_if $B2
                local.get $l7
                i32.load8_s
                i32.const -65
                i32.le_s
                br_if $B3
              end
              local.get $p1
              local.get $l4
              i32.add
              local.set $p1
              local.get $p2
              local.get $l4
              i32.sub
              local.tee $p2
              br_if $L5
            end
            i32.const 0
            local.set $l4
            br $B0
          end
          local.get $l6
          i32.const 4
          i32.const 1052192
          call $_ZN4core5slice24slice_end_index_len_fail17haeb08024239d8a09E
          unreachable
        end
        local.get $p1
        local.get $p2
        local.get $l4
        local.get $p2
        i32.const 1051536
        call $_ZN4core3str16slice_error_fail17h26278b2259fb6582E
        unreachable
      end
      i32.const 1
      local.set $l4
    end
    local.get $l3
    i32.const 48
    i32.add
    global.set $g0
    local.get $l4)
  (func $_ZN4core5slice6memchr6memchr17h0f2bc0ed161f00a2E (type $t11) (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32)
    (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32)
    i32.const 0
    local.set $l4
    block $B0
      block $B1
        block $B2
          block $B3
            i32.const 0
            local.get $p2
            i32.sub
            i32.const 3
            i32.and
            local.tee $l5
            i32.eqz
            br_if $B3
            local.get $p3
            local.get $l5
            local.get $l5
            local.get $p3
            i32.gt_u
            select
            local.tee $l6
            i32.eqz
            br_if $B3
            i32.const 0
            local.set $l5
            local.get $p1
            i32.const 255
            i32.and
            local.set $l4
            loop $L4
              local.get $p2
              local.get $l5
              i32.add
              i32.load8_u
              local.get $l4
              i32.eq
              br_if $B2
              local.get $l6
              local.get $l5
              i32.const 1
              i32.add
              local.tee $l5
              i32.ne
              br_if $L4
            end
            local.get $l6
            local.set $l4
          end
          local.get $p3
          i32.const 8
          i32.lt_u
          br_if $B1
          local.get $l4
          local.get $p3
          i32.const -8
          i32.add
          local.tee $l7
          i32.gt_u
          br_if $B1
          local.get $p1
          i32.const 255
          i32.and
          i32.const 16843009
          i32.mul
          local.set $l5
          block $B5
            loop $L6
              local.get $p2
              local.get $l4
              i32.add
              local.tee $l6
              i32.const 4
              i32.add
              i32.load
              local.get $l5
              i32.xor
              local.tee $l8
              i32.const -1
              i32.xor
              local.get $l8
              i32.const -16843009
              i32.add
              i32.and
              local.get $l6
              i32.load
              local.get $l5
              i32.xor
              local.tee $l6
              i32.const -1
              i32.xor
              local.get $l6
              i32.const -16843009
              i32.add
              i32.and
              i32.or
              i32.const -2139062144
              i32.and
              br_if $B5
              local.get $l4
              i32.const 8
              i32.add
              local.tee $l4
              local.get $l7
              i32.le_u
              br_if $L6
            end
          end
          local.get $l4
          local.get $p3
          i32.le_u
          br_if $B1
          local.get $l4
          local.get $p3
          i32.const 1051960
          call $_ZN4core5slice26slice_start_index_len_fail17h46c23795afd32c64E
          unreachable
        end
        i32.const 1
        local.set $l6
        br $B0
      end
      i32.const 0
      local.set $l5
      i32.const 0
      local.set $l6
      block $B7
        local.get $l4
        local.get $p3
        i32.eq
        br_if $B7
        local.get $p2
        local.get $l4
        i32.add
        local.set $p2
        local.get $p3
        local.get $l4
        i32.sub
        local.set $l8
        i32.const 0
        local.set $l5
        local.get $p1
        i32.const 255
        i32.and
        local.set $l6
        block $B8
          loop $L9
            local.get $p2
            local.get $l5
            i32.add
            i32.load8_u
            local.get $l6
            i32.eq
            br_if $B8
            local.get $l8
            local.get $l5
            i32.const 1
            i32.add
            local.tee $l5
            i32.ne
            br_if $L9
          end
          i32.const 0
          local.set $l6
          local.get $l8
          local.get $l4
          i32.add
          local.set $l5
          br $B0
        end
        i32.const 1
        local.set $l6
        local.get $l5
        local.set $l5
      end
      local.get $l5
      local.get $l4
      i32.add
      local.set $l5
    end
    local.get $p0
    local.get $l5
    i32.store offset=4
    local.get $p0
    local.get $l6
    i32.store)
  (func $_ZN4core3fmt8builders10DebugTuple5field17h6c7d284ba7c32ea1E (type $t6) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i64) (local $l8 i64)
    global.get $g0
    i32.const 64
    i32.sub
    local.tee $l3
    global.set $g0
    i32.const 1
    local.set $l4
    block $B0
      local.get $p0
      i32.load8_u offset=8
      br_if $B0
      local.get $p0
      i32.load offset=4
      local.set $l5
      block $B1
        local.get $p0
        i32.load
        local.tee $l6
        i32.load8_u
        i32.const 4
        i32.and
        br_if $B1
        i32.const 1
        local.set $l4
        local.get $l6
        i32.load offset=24
        i32.const 1051554
        i32.const 1051558
        local.get $l5
        select
        i32.const 2
        i32.const 1
        local.get $l5
        select
        local.get $l6
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type $t6) $T0
        br_if $B0
        local.get $p1
        local.get $p0
        i32.load
        local.get $p2
        i32.load offset=12
        call_indirect (type $t2) $T0
        local.set $l4
        br $B0
      end
      block $B2
        local.get $l5
        br_if $B2
        i32.const 1
        local.set $l4
        local.get $l6
        i32.load offset=24
        i32.const 1051556
        i32.const 2
        local.get $l6
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type $t6) $T0
        br_if $B0
        local.get $p0
        i32.load
        local.set $l6
      end
      i32.const 1
      local.set $l4
      local.get $l3
      i32.const 1
      i32.store8 offset=23
      local.get $l3
      i32.const 52
      i32.add
      i32.const 1051492
      i32.store
      local.get $l3
      local.get $l6
      i64.load offset=24 align=4
      i64.store offset=8
      local.get $l3
      local.get $l3
      i32.const 23
      i32.add
      i32.store offset=16
      local.get $l6
      i64.load offset=8 align=4
      local.set $l7
      local.get $l6
      i64.load offset=16 align=4
      local.set $l8
      local.get $l3
      local.get $l6
      i32.load8_u offset=32
      i32.store8 offset=56
      local.get $l3
      local.get $l8
      i64.store offset=40
      local.get $l3
      local.get $l7
      i64.store offset=32
      local.get $l3
      local.get $l6
      i64.load align=4
      i64.store offset=24
      local.get $l3
      local.get $l3
      i32.const 8
      i32.add
      i32.store offset=48
      local.get $p1
      local.get $l3
      i32.const 24
      i32.add
      local.get $p2
      i32.load offset=12
      call_indirect (type $t2) $T0
      br_if $B0
      local.get $l3
      i32.load offset=48
      i32.const 1051552
      i32.const 2
      local.get $l3
      i32.load offset=52
      i32.load offset=12
      call_indirect (type $t6) $T0
      local.set $l4
    end
    local.get $p0
    local.get $l4
    i32.store8 offset=8
    local.get $p0
    local.get $p0
    i32.load offset=4
    i32.const 1
    i32.add
    i32.store offset=4
    local.get $l3
    i32.const 64
    i32.add
    global.set $g0
    local.get $p0)
  (func $_ZN4core3fmt8builders10DebugTuple6finish17h6ed5b55943d7a61eE (type $t5) (param $p0 i32) (result i32)
    (local $l1 i32) (local $l2 i32) (local $l3 i32)
    local.get $p0
    i32.load8_u offset=8
    local.set $l1
    block $B0
      local.get $p0
      i32.load offset=4
      local.tee $l2
      i32.eqz
      br_if $B0
      local.get $l1
      i32.const 255
      i32.and
      local.set $l3
      i32.const 1
      local.set $l1
      block $B1
        local.get $l3
        br_if $B1
        block $B2
          local.get $l2
          i32.const 1
          i32.ne
          br_if $B2
          local.get $p0
          i32.load8_u offset=9
          i32.eqz
          br_if $B2
          local.get $p0
          i32.load
          local.tee $l3
          i32.load8_u
          i32.const 4
          i32.and
          br_if $B2
          i32.const 1
          local.set $l1
          local.get $l3
          i32.load offset=24
          i32.const 1051559
          i32.const 1
          local.get $l3
          i32.const 28
          i32.add
          i32.load
          i32.load offset=12
          call_indirect (type $t6) $T0
          br_if $B1
        end
        local.get $p0
        i32.load
        local.tee $l1
        i32.load offset=24
        i32.const 1051560
        i32.const 1
        local.get $l1
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type $t6) $T0
        local.set $l1
      end
      local.get $p0
      local.get $l1
      i32.store8 offset=8
    end
    local.get $l1
    i32.const 255
    i32.and
    i32.const 0
    i32.ne)
  (func $_ZN4core3fmt8builders10DebugInner5entry17h34fa990cab34ee4dE (type $t4) (param $p0 i32) (param $p1 i32) (param $p2 i32)
    (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i64) (local $l7 i64)
    global.get $g0
    i32.const 64
    i32.sub
    local.tee $l3
    global.set $g0
    i32.const 1
    local.set $l4
    block $B0
      local.get $p0
      i32.load8_u offset=4
      br_if $B0
      local.get $p0
      i32.load8_u offset=5
      local.set $l4
      block $B1
        local.get $p0
        i32.load
        local.tee $l5
        i32.load8_u
        i32.const 4
        i32.and
        br_if $B1
        block $B2
          local.get $l4
          i32.const 255
          i32.and
          i32.eqz
          br_if $B2
          i32.const 1
          local.set $l4
          local.get $l5
          i32.load offset=24
          i32.const 1051554
          i32.const 2
          local.get $l5
          i32.const 28
          i32.add
          i32.load
          i32.load offset=12
          call_indirect (type $t6) $T0
          br_if $B0
          local.get $p0
          i32.load
          local.set $l5
        end
        local.get $p1
        local.get $l5
        local.get $p2
        i32.load offset=12
        call_indirect (type $t2) $T0
        local.set $l4
        br $B0
      end
      block $B3
        local.get $l4
        i32.const 255
        i32.and
        br_if $B3
        i32.const 1
        local.set $l4
        local.get $l5
        i32.load offset=24
        i32.const 1051561
        i32.const 1
        local.get $l5
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type $t6) $T0
        br_if $B0
        local.get $p0
        i32.load
        local.set $l5
      end
      i32.const 1
      local.set $l4
      local.get $l3
      i32.const 1
      i32.store8 offset=23
      local.get $l3
      i32.const 52
      i32.add
      i32.const 1051492
      i32.store
      local.get $l3
      local.get $l5
      i64.load offset=24 align=4
      i64.store offset=8
      local.get $l3
      local.get $l3
      i32.const 23
      i32.add
      i32.store offset=16
      local.get $l5
      i64.load offset=8 align=4
      local.set $l6
      local.get $l5
      i64.load offset=16 align=4
      local.set $l7
      local.get $l3
      local.get $l5
      i32.load8_u offset=32
      i32.store8 offset=56
      local.get $l3
      local.get $l7
      i64.store offset=40
      local.get $l3
      local.get $l6
      i64.store offset=32
      local.get $l3
      local.get $l5
      i64.load align=4
      i64.store offset=24
      local.get $l3
      local.get $l3
      i32.const 8
      i32.add
      i32.store offset=48
      local.get $p1
      local.get $l3
      i32.const 24
      i32.add
      local.get $p2
      i32.load offset=12
      call_indirect (type $t2) $T0
      br_if $B0
      local.get $l3
      i32.load offset=48
      i32.const 1051552
      i32.const 2
      local.get $l3
      i32.load offset=52
      i32.load offset=12
      call_indirect (type $t6) $T0
      local.set $l4
    end
    local.get $p0
    i32.const 1
    i32.store8 offset=5
    local.get $p0
    local.get $l4
    i32.store8 offset=4
    local.get $l3
    i32.const 64
    i32.add
    global.set $g0)
  (func $_ZN4core3fmt8builders8DebugSet5entry17h4da9ac0fd443c627E (type $t6) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    local.get $p0
    local.get $p1
    local.get $p2
    call $_ZN4core3fmt8builders10DebugInner5entry17h34fa990cab34ee4dE
    local.get $p0)
  (func $_ZN4core3fmt8builders9DebugList6finish17h15497983fc988cedE (type $t5) (param $p0 i32) (result i32)
    (local $l1 i32)
    i32.const 1
    local.set $l1
    block $B0
      local.get $p0
      i32.load8_u offset=4
      br_if $B0
      local.get $p0
      i32.load
      local.tee $p0
      i32.load offset=24
      i32.const 1051563
      i32.const 1
      local.get $p0
      i32.const 28
      i32.add
      i32.load
      i32.load offset=12
      call_indirect (type $t6) $T0
      local.set $l1
    end
    local.get $l1)
  (func $_ZN4core3fmt5Write10write_char17h0660426ba5d037baE (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32) (local $l3 i32)
    global.get $g0
    i32.const 16
    i32.sub
    local.tee $l2
    global.set $g0
    local.get $l2
    i32.const 0
    i32.store offset=12
    block $B0
      block $B1
        block $B2
          block $B3
            local.get $p1
            i32.const 128
            i32.lt_u
            br_if $B3
            local.get $p1
            i32.const 2048
            i32.lt_u
            br_if $B2
            local.get $l2
            i32.const 12
            i32.add
            local.set $l3
            local.get $p1
            i32.const 65536
            i32.ge_u
            br_if $B1
            local.get $l2
            local.get $p1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get $l2
            local.get $p1
            i32.const 12
            i32.shr_u
            i32.const 224
            i32.or
            i32.store8 offset=12
            local.get $l2
            local.get $p1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            i32.const 3
            local.set $p1
            br $B0
          end
          local.get $l2
          local.get $p1
          i32.store8 offset=12
          local.get $l2
          i32.const 12
          i32.add
          local.set $l3
          i32.const 1
          local.set $p1
          br $B0
        end
        local.get $l2
        local.get $p1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get $l2
        local.get $p1
        i32.const 6
        i32.shr_u
        i32.const 192
        i32.or
        i32.store8 offset=12
        local.get $l2
        i32.const 12
        i32.add
        local.set $l3
        i32.const 2
        local.set $p1
        br $B0
      end
      local.get $l2
      local.get $p1
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=15
      local.get $l2
      local.get $p1
      i32.const 18
      i32.shr_u
      i32.const 240
      i32.or
      i32.store8 offset=12
      local.get $l2
      local.get $p1
      i32.const 6
      i32.shr_u
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=14
      local.get $l2
      local.get $p1
      i32.const 12
      i32.shr_u
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=13
      i32.const 4
      local.set $p1
    end
    local.get $p0
    local.get $l3
    local.get $p1
    call $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h44ce8e8e61187795E
    local.set $p1
    local.get $l2
    i32.const 16
    i32.add
    global.set $g0
    local.get $p1)
  (func $_ZN4core3fmt5Write9write_fmt17h3781fd8c2a82affaE (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32)
    global.get $g0
    i32.const 32
    i32.sub
    local.tee $l2
    global.set $g0
    local.get $l2
    local.get $p0
    i32.store offset=4
    local.get $l2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get $p1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get $l2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get $p1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get $l2
    local.get $p1
    i64.load align=4
    i64.store offset=8
    local.get $l2
    i32.const 4
    i32.add
    i32.const 1051812
    local.get $l2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17hb395f946a5ce2cabE
    local.set $p1
    local.get $l2
    i32.const 32
    i32.add
    global.set $g0
    local.get $p1)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17he83ce1db16bdf500E (type $t6) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    local.get $p0
    i32.load
    local.get $p1
    local.get $p2
    call $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h44ce8e8e61187795E)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h2670637b4af27d11E (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32) (local $l3 i32)
    global.get $g0
    i32.const 16
    i32.sub
    local.tee $l2
    global.set $g0
    local.get $p0
    i32.load
    local.set $p0
    local.get $l2
    i32.const 0
    i32.store offset=12
    block $B0
      block $B1
        block $B2
          block $B3
            local.get $p1
            i32.const 128
            i32.lt_u
            br_if $B3
            local.get $p1
            i32.const 2048
            i32.lt_u
            br_if $B2
            local.get $l2
            i32.const 12
            i32.add
            local.set $l3
            local.get $p1
            i32.const 65536
            i32.ge_u
            br_if $B1
            local.get $l2
            local.get $p1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get $l2
            local.get $p1
            i32.const 12
            i32.shr_u
            i32.const 224
            i32.or
            i32.store8 offset=12
            local.get $l2
            local.get $p1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            i32.const 3
            local.set $p1
            br $B0
          end
          local.get $l2
          local.get $p1
          i32.store8 offset=12
          local.get $l2
          i32.const 12
          i32.add
          local.set $l3
          i32.const 1
          local.set $p1
          br $B0
        end
        local.get $l2
        local.get $p1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get $l2
        local.get $p1
        i32.const 6
        i32.shr_u
        i32.const 192
        i32.or
        i32.store8 offset=12
        local.get $l2
        i32.const 12
        i32.add
        local.set $l3
        i32.const 2
        local.set $p1
        br $B0
      end
      local.get $l2
      local.get $p1
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=15
      local.get $l2
      local.get $p1
      i32.const 18
      i32.shr_u
      i32.const 240
      i32.or
      i32.store8 offset=12
      local.get $l2
      local.get $p1
      i32.const 6
      i32.shr_u
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=14
      local.get $l2
      local.get $p1
      i32.const 12
      i32.shr_u
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=13
      i32.const 4
      local.set $p1
    end
    local.get $p0
    local.get $l3
    local.get $p1
    call $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h44ce8e8e61187795E
    local.set $p1
    local.get $l2
    i32.const 16
    i32.add
    global.set $g0
    local.get $p1)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17hba895e75ebac3cc2E (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32)
    global.get $g0
    i32.const 32
    i32.sub
    local.tee $l2
    global.set $g0
    local.get $l2
    local.get $p0
    i32.load
    i32.store offset=4
    local.get $l2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get $p1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get $l2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get $p1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get $l2
    local.get $p1
    i64.load align=4
    i64.store offset=8
    local.get $l2
    i32.const 4
    i32.add
    i32.const 1051812
    local.get $l2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17hb395f946a5ce2cabE
    local.set $p1
    local.get $l2
    i32.const 32
    i32.add
    global.set $g0
    local.get $p1)
  (func $_ZN59_$LT$core..fmt..Arguments$u20$as$u20$core..fmt..Display$GT$3fmt17h88a4e919f59e7b36E (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32) (local $l3 i32)
    global.get $g0
    i32.const 32
    i32.sub
    local.tee $l2
    global.set $g0
    local.get $p1
    i32.const 28
    i32.add
    i32.load
    local.set $l3
    local.get $p1
    i32.load offset=24
    local.set $p1
    local.get $l2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get $p0
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get $l2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get $p0
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get $l2
    local.get $p0
    i64.load align=4
    i64.store offset=8
    local.get $p1
    local.get $l3
    local.get $l2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17hb395f946a5ce2cabE
    local.set $p0
    local.get $l2
    i32.const 32
    i32.add
    global.set $g0
    local.get $p0)
  (func $_ZN4core3fmt9Formatter12pad_integral17h05ee6133195a52bcE (type $t12) (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32) (param $p4 i32) (param $p5 i32) (result i32)
    (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32) (local $l10 i32) (local $l11 i32)
    block $B0
      block $B1
        local.get $p1
        i32.eqz
        br_if $B1
        i32.const 43
        i32.const 1114112
        local.get $p0
        i32.load
        local.tee $l6
        i32.const 1
        i32.and
        local.tee $p1
        select
        local.set $l7
        local.get $p1
        local.get $p5
        i32.add
        local.set $l8
        br $B0
      end
      local.get $p5
      i32.const 1
      i32.add
      local.set $l8
      local.get $p0
      i32.load
      local.set $l6
      i32.const 45
      local.set $l7
    end
    block $B2
      block $B3
        local.get $l6
        i32.const 4
        i32.and
        br_if $B3
        i32.const 0
        local.set $p2
        br $B2
      end
      i32.const 0
      local.set $l9
      block $B4
        local.get $p3
        i32.eqz
        br_if $B4
        local.get $p3
        local.set $l10
        local.get $p2
        local.set $p1
        loop $L5
          local.get $l9
          local.get $p1
          i32.load8_u
          i32.const 192
          i32.and
          i32.const 128
          i32.eq
          i32.add
          local.set $l9
          local.get $p1
          i32.const 1
          i32.add
          local.set $p1
          local.get $l10
          i32.const -1
          i32.add
          local.tee $l10
          br_if $L5
        end
      end
      local.get $l8
      local.get $p3
      i32.add
      local.get $l9
      i32.sub
      local.set $l8
    end
    i32.const 1
    local.set $p1
    block $B6
      block $B7
        local.get $p0
        i32.load offset=8
        i32.const 1
        i32.eq
        br_if $B7
        local.get $p0
        local.get $l7
        local.get $p2
        local.get $p3
        call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h81dd0f8b1c9d1dd3E
        br_if $B6
        local.get $p0
        i32.load offset=24
        local.get $p4
        local.get $p5
        local.get $p0
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type $t6) $T0
        local.set $p1
        br $B6
      end
      block $B8
        local.get $p0
        i32.const 12
        i32.add
        i32.load
        local.tee $l9
        local.get $l8
        i32.gt_u
        br_if $B8
        local.get $p0
        local.get $l7
        local.get $p2
        local.get $p3
        call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h81dd0f8b1c9d1dd3E
        br_if $B6
        local.get $p0
        i32.load offset=24
        local.get $p4
        local.get $p5
        local.get $p0
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type $t6) $T0
        return
      end
      block $B9
        block $B10
          block $B11
            block $B12
              block $B13
                local.get $l6
                i32.const 8
                i32.and
                i32.eqz
                br_if $B13
                local.get $p0
                i32.load offset=4
                local.set $l6
                local.get $p0
                i32.const 48
                i32.store offset=4
                local.get $p0
                i32.load8_u offset=32
                local.set $l11
                i32.const 1
                local.set $p1
                local.get $p0
                i32.const 1
                i32.store8 offset=32
                local.get $p0
                local.get $l7
                local.get $p2
                local.get $p3
                call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h81dd0f8b1c9d1dd3E
                br_if $B6
                i32.const 0
                local.set $p1
                local.get $l9
                local.get $l8
                i32.sub
                local.tee $l10
                local.set $p3
                i32.const 1
                local.get $p0
                i32.load8_u offset=32
                local.tee $l9
                local.get $l9
                i32.const 3
                i32.eq
                select
                i32.const 3
                i32.and
                br_table $B10 $B11 $B12 $B11 $B10
              end
              i32.const 0
              local.set $p1
              local.get $l9
              local.get $l8
              i32.sub
              local.tee $l9
              local.set $l8
              block $B14
                block $B15
                  block $B16
                    i32.const 1
                    local.get $p0
                    i32.load8_u offset=32
                    local.tee $l10
                    local.get $l10
                    i32.const 3
                    i32.eq
                    select
                    i32.const 3
                    i32.and
                    br_table $B14 $B15 $B16 $B15 $B14
                  end
                  local.get $l9
                  i32.const 1
                  i32.shr_u
                  local.set $p1
                  local.get $l9
                  i32.const 1
                  i32.add
                  i32.const 1
                  i32.shr_u
                  local.set $l8
                  br $B14
                end
                i32.const 0
                local.set $l8
                local.get $l9
                local.set $p1
              end
              local.get $p1
              i32.const 1
              i32.add
              local.set $p1
              loop $L17
                local.get $p1
                i32.const -1
                i32.add
                local.tee $p1
                i32.eqz
                br_if $B9
                local.get $p0
                i32.load offset=24
                local.get $p0
                i32.load offset=4
                local.get $p0
                i32.load offset=28
                i32.load offset=16
                call_indirect (type $t2) $T0
                i32.eqz
                br_if $L17
              end
              i32.const 1
              return
            end
            local.get $l10
            i32.const 1
            i32.shr_u
            local.set $p1
            local.get $l10
            i32.const 1
            i32.add
            i32.const 1
            i32.shr_u
            local.set $p3
            br $B10
          end
          i32.const 0
          local.set $p3
          local.get $l10
          local.set $p1
        end
        local.get $p1
        i32.const 1
        i32.add
        local.set $p1
        block $B18
          loop $L19
            local.get $p1
            i32.const -1
            i32.add
            local.tee $p1
            i32.eqz
            br_if $B18
            local.get $p0
            i32.load offset=24
            local.get $p0
            i32.load offset=4
            local.get $p0
            i32.load offset=28
            i32.load offset=16
            call_indirect (type $t2) $T0
            i32.eqz
            br_if $L19
          end
          i32.const 1
          return
        end
        local.get $p0
        i32.load offset=4
        local.set $l10
        i32.const 1
        local.set $p1
        local.get $p0
        i32.load offset=24
        local.get $p4
        local.get $p5
        local.get $p0
        i32.load offset=28
        i32.load offset=12
        call_indirect (type $t6) $T0
        br_if $B6
        local.get $p3
        i32.const 1
        i32.add
        local.set $l9
        local.get $p0
        i32.load offset=28
        local.set $p3
        local.get $p0
        i32.load offset=24
        local.set $p2
        block $B20
          loop $L21
            local.get $l9
            i32.const -1
            i32.add
            local.tee $l9
            i32.eqz
            br_if $B20
            i32.const 1
            local.set $p1
            local.get $p2
            local.get $l10
            local.get $p3
            i32.load offset=16
            call_indirect (type $t2) $T0
            br_if $B6
            br $L21
          end
        end
        local.get $p0
        local.get $l11
        i32.store8 offset=32
        local.get $p0
        local.get $l6
        i32.store offset=4
        i32.const 0
        return
      end
      local.get $p0
      i32.load offset=4
      local.set $l10
      i32.const 1
      local.set $p1
      local.get $p0
      local.get $l7
      local.get $p2
      local.get $p3
      call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h81dd0f8b1c9d1dd3E
      br_if $B6
      local.get $p0
      i32.load offset=24
      local.get $p4
      local.get $p5
      local.get $p0
      i32.load offset=28
      i32.load offset=12
      call_indirect (type $t6) $T0
      br_if $B6
      local.get $l8
      i32.const 1
      i32.add
      local.set $l9
      local.get $p0
      i32.load offset=28
      local.set $p3
      local.get $p0
      i32.load offset=24
      local.set $p0
      loop $L22
        block $B23
          local.get $l9
          i32.const -1
          i32.add
          local.tee $l9
          br_if $B23
          i32.const 0
          return
        end
        i32.const 1
        local.set $p1
        local.get $p0
        local.get $l10
        local.get $p3
        i32.load offset=16
        call_indirect (type $t2) $T0
        i32.eqz
        br_if $L22
      end
    end
    local.get $p1)
  (func $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h81dd0f8b1c9d1dd3E (type $t9) (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32) (result i32)
    (local $l4 i32)
    block $B0
      block $B1
        local.get $p1
        i32.const 1114112
        i32.eq
        br_if $B1
        i32.const 1
        local.set $l4
        local.get $p0
        i32.load offset=24
        local.get $p1
        local.get $p0
        i32.const 28
        i32.add
        i32.load
        i32.load offset=16
        call_indirect (type $t2) $T0
        br_if $B0
      end
      block $B2
        local.get $p2
        br_if $B2
        i32.const 0
        return
      end
      local.get $p0
      i32.load offset=24
      local.get $p2
      local.get $p3
      local.get $p0
      i32.const 28
      i32.add
      i32.load
      i32.load offset=12
      call_indirect (type $t6) $T0
      local.set $l4
    end
    local.get $l4)
  (func $_ZN4core3fmt9Formatter9write_fmt17hc1fb6c199a6a4c9dE (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32) (local $l3 i32)
    global.get $g0
    i32.const 32
    i32.sub
    local.tee $l2
    global.set $g0
    local.get $p0
    i32.const 28
    i32.add
    i32.load
    local.set $l3
    local.get $p0
    i32.load offset=24
    local.set $p0
    local.get $l2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get $p1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get $l2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get $p1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get $l2
    local.get $p1
    i64.load align=4
    i64.store offset=8
    local.get $p0
    local.get $l3
    local.get $l2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17hb395f946a5ce2cabE
    local.set $p1
    local.get $l2
    i32.const 32
    i32.add
    global.set $g0
    local.get $p1)
  (func $_ZN4core3fmt9Formatter15debug_lower_hex17he16ae5aeaad8d5abE (type $t5) (param $p0 i32) (result i32)
    local.get $p0
    i32.load8_u
    i32.const 16
    i32.and
    i32.const 4
    i32.shr_u)
  (func $_ZN4core3fmt9Formatter15debug_upper_hex17h8b72eec9a9ee7d24E (type $t5) (param $p0 i32) (result i32)
    local.get $p0
    i32.load8_u
    i32.const 32
    i32.and
    i32.const 5
    i32.shr_u)
  (func $_ZN4core3fmt9Formatter11debug_tuple17h242798767252cce4E (type $t11) (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32)
    local.get $p0
    local.get $p1
    i32.load offset=24
    local.get $p2
    local.get $p3
    local.get $p1
    i32.const 28
    i32.add
    i32.load
    i32.load offset=12
    call_indirect (type $t6) $T0
    i32.store8 offset=8
    local.get $p0
    local.get $p1
    i32.store
    local.get $p0
    local.get $p3
    i32.eqz
    i32.store8 offset=9
    local.get $p0
    i32.const 0
    i32.store offset=4)
  (func $_ZN4core3fmt9Formatter10debug_list17h4df433c222cafce6E (type $t3) (param $p0 i32) (param $p1 i32)
    (local $l2 i32)
    local.get $p1
    i32.load offset=24
    i32.const 1051562
    i32.const 1
    local.get $p1
    i32.const 28
    i32.add
    i32.load
    i32.load offset=12
    call_indirect (type $t6) $T0
    local.set $l2
    local.get $p0
    i32.const 0
    i32.store8 offset=5
    local.get $p0
    local.get $l2
    i32.store8 offset=4
    local.get $p0
    local.get $p1
    i32.store)
  (func $_ZN40_$LT$str$u20$as$u20$core..fmt..Debug$GT$3fmt17h4e37a0e4f747f286E (type $t6) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32) (local $l10 i32) (local $l11 i32) (local $l12 i32) (local $l13 i32) (local $l14 i64)
    i32.const 1
    local.set $l3
    block $B0
      block $B1
        local.get $p2
        i32.load offset=24
        i32.const 34
        local.get $p2
        i32.const 28
        i32.add
        i32.load
        i32.load offset=16
        call_indirect (type $t2) $T0
        br_if $B1
        block $B2
          block $B3
            local.get $p1
            br_if $B3
            i32.const 0
            local.set $l4
            br $B2
          end
          local.get $p0
          local.get $p1
          i32.add
          local.set $l5
          i32.const 0
          local.set $l4
          local.get $p0
          local.set $l6
          local.get $p0
          local.set $l7
          i32.const 0
          local.set $l8
          block $B4
            loop $L5
              local.get $l6
              i32.const 1
              i32.add
              local.set $l9
              block $B6
                block $B7
                  block $B8
                    local.get $l6
                    i32.load8_s
                    local.tee $l10
                    i32.const -1
                    i32.gt_s
                    br_if $B8
                    block $B9
                      block $B10
                        local.get $l9
                        local.get $l5
                        i32.ne
                        br_if $B10
                        i32.const 0
                        local.set $l11
                        local.get $l5
                        local.set $l6
                        br $B9
                      end
                      local.get $l6
                      i32.load8_u offset=1
                      i32.const 63
                      i32.and
                      local.set $l11
                      local.get $l6
                      i32.const 2
                      i32.add
                      local.tee $l9
                      local.set $l6
                    end
                    local.get $l10
                    i32.const 31
                    i32.and
                    local.set $l3
                    block $B11
                      local.get $l10
                      i32.const 255
                      i32.and
                      local.tee $l10
                      i32.const 223
                      i32.gt_u
                      br_if $B11
                      local.get $l11
                      local.get $l3
                      i32.const 6
                      i32.shl
                      i32.or
                      local.set $l11
                      br $B7
                    end
                    block $B12
                      block $B13
                        local.get $l6
                        local.get $l5
                        i32.ne
                        br_if $B13
                        i32.const 0
                        local.set $l12
                        local.get $l5
                        local.set $l13
                        br $B12
                      end
                      local.get $l6
                      i32.load8_u
                      i32.const 63
                      i32.and
                      local.set $l12
                      local.get $l6
                      i32.const 1
                      i32.add
                      local.tee $l9
                      local.set $l13
                    end
                    local.get $l12
                    local.get $l11
                    i32.const 6
                    i32.shl
                    i32.or
                    local.set $l11
                    block $B14
                      local.get $l10
                      i32.const 240
                      i32.ge_u
                      br_if $B14
                      local.get $l11
                      local.get $l3
                      i32.const 12
                      i32.shl
                      i32.or
                      local.set $l11
                      br $B7
                    end
                    block $B15
                      block $B16
                        local.get $l13
                        local.get $l5
                        i32.ne
                        br_if $B16
                        i32.const 0
                        local.set $l10
                        local.get $l9
                        local.set $l6
                        br $B15
                      end
                      local.get $l13
                      i32.const 1
                      i32.add
                      local.set $l6
                      local.get $l13
                      i32.load8_u
                      i32.const 63
                      i32.and
                      local.set $l10
                    end
                    local.get $l11
                    i32.const 6
                    i32.shl
                    local.get $l3
                    i32.const 18
                    i32.shl
                    i32.const 1835008
                    i32.and
                    i32.or
                    local.get $l10
                    i32.or
                    local.tee $l11
                    i32.const 1114112
                    i32.ne
                    br_if $B6
                    br $B4
                  end
                  local.get $l10
                  i32.const 255
                  i32.and
                  local.set $l11
                end
                local.get $l9
                local.set $l6
              end
              i32.const 2
              local.set $l9
              i32.const 116
              local.set $l13
              block $B17
                block $B18
                  block $B19
                    block $B20
                      block $B21
                        block $B22
                          block $B23
                            block $B24
                              local.get $l11
                              i32.const -9
                              i32.add
                              br_table $B18 $B23 $B21 $B21 $B24 $B21 $B21 $B21 $B21 $B21 $B21 $B21 $B21 $B21 $B21 $B21 $B21 $B21 $B21 $B21 $B21 $B21 $B21 $B21 $B21 $B20 $B21 $B21 $B21 $B21 $B20 $B22
                            end
                            i32.const 114
                            local.set $l13
                            br $B18
                          end
                          i32.const 110
                          local.set $l13
                          br $B18
                        end
                        local.get $l11
                        i32.const 92
                        i32.eq
                        br_if $B20
                      end
                      block $B25
                        local.get $l11
                        call $_ZN4core7unicode12unicode_data15grapheme_extend6lookup17he3cc23a69ca36d6aE
                        br_if $B25
                        local.get $l11
                        call $_ZN4core7unicode9printable12is_printable17h04f2efbc69a32118E
                        br_if $B17
                      end
                      local.get $l11
                      i32.const 1
                      i32.or
                      i32.clz
                      i32.const 2
                      i32.shr_u
                      i32.const 7
                      i32.xor
                      i64.extend_i32_u
                      i64.const 21474836480
                      i64.or
                      local.set $l14
                      i32.const 3
                      local.set $l9
                      br $B19
                    end
                  end
                  local.get $l11
                  local.set $l13
                end
                block $B26
                  block $B27
                    local.get $l8
                    local.get $l4
                    i32.lt_u
                    br_if $B27
                    block $B28
                      local.get $l4
                      i32.eqz
                      br_if $B28
                      local.get $l4
                      local.get $p1
                      i32.eq
                      br_if $B28
                      local.get $l4
                      local.get $p1
                      i32.ge_u
                      br_if $B27
                      local.get $p0
                      local.get $l4
                      i32.add
                      i32.load8_s
                      i32.const -65
                      i32.le_s
                      br_if $B27
                    end
                    block $B29
                      local.get $l8
                      i32.eqz
                      br_if $B29
                      local.get $l8
                      local.get $p1
                      i32.eq
                      br_if $B29
                      local.get $l8
                      local.get $p1
                      i32.ge_u
                      br_if $B27
                      local.get $p0
                      local.get $l8
                      i32.add
                      i32.load8_s
                      i32.const -65
                      i32.le_s
                      br_if $B27
                    end
                    local.get $p2
                    i32.load offset=24
                    local.get $p0
                    local.get $l4
                    i32.add
                    local.get $l8
                    local.get $l4
                    i32.sub
                    local.get $p2
                    i32.load offset=28
                    i32.load offset=12
                    call_indirect (type $t6) $T0
                    i32.eqz
                    br_if $B26
                    i32.const 1
                    return
                  end
                  local.get $p0
                  local.get $p1
                  local.get $l4
                  local.get $l8
                  i32.const 1051896
                  call $_ZN4core3str16slice_error_fail17h26278b2259fb6582E
                  unreachable
                end
                loop $L30
                  local.get $l9
                  local.set $l10
                  i32.const 1
                  local.set $l3
                  i32.const 92
                  local.set $l4
                  i32.const 1
                  local.set $l9
                  block $B31
                    block $B32
                      block $B33
                        block $B34
                          block $B35
                            block $B36
                              local.get $l10
                              br_table $B34 $B35 $B31 $B36 $B34
                            end
                            block $B37
                              block $B38
                                block $B39
                                  block $B40
                                    local.get $l14
                                    i64.const 32
                                    i64.shr_u
                                    i32.wrap_i64
                                    i32.const 255
                                    i32.and
                                    br_table $B34 $B37 $B38 $B39 $B40 $B33 $B34
                                  end
                                  local.get $l14
                                  i64.const -1095216660481
                                  i64.and
                                  i64.const 12884901888
                                  i64.or
                                  local.set $l14
                                  i32.const 3
                                  local.set $l9
                                  i32.const 117
                                  local.set $l4
                                  br $B31
                                end
                                local.get $l14
                                i64.const -1095216660481
                                i64.and
                                i64.const 8589934592
                                i64.or
                                local.set $l14
                                i32.const 3
                                local.set $l9
                                i32.const 123
                                local.set $l4
                                br $B31
                              end
                              i32.const 48
                              i32.const 87
                              local.get $l13
                              local.get $l14
                              i32.wrap_i64
                              local.tee $l9
                              i32.const 2
                              i32.shl
                              i32.const 28
                              i32.and
                              i32.shr_u
                              i32.const 15
                              i32.and
                              local.tee $l4
                              i32.const 10
                              i32.lt_u
                              select
                              local.get $l4
                              i32.add
                              local.set $l4
                              block $B41
                                local.get $l9
                                i32.eqz
                                br_if $B41
                                local.get $l14
                                i64.const -1
                                i64.add
                                i64.const 4294967295
                                i64.and
                                local.get $l14
                                i64.const -4294967296
                                i64.and
                                i64.or
                                local.set $l14
                                br $B32
                              end
                              local.get $l14
                              i64.const -1095216660481
                              i64.and
                              i64.const 4294967296
                              i64.or
                              local.set $l14
                              br $B32
                            end
                            local.get $l14
                            i64.const -1095216660481
                            i64.and
                            local.set $l14
                            i32.const 3
                            local.set $l9
                            i32.const 125
                            local.set $l4
                            br $B31
                          end
                          i32.const 0
                          local.set $l9
                          local.get $l13
                          local.set $l4
                          br $B31
                        end
                        i32.const 1
                        local.set $l9
                        block $B42
                          local.get $l11
                          i32.const 128
                          i32.lt_u
                          br_if $B42
                          i32.const 2
                          local.set $l9
                          local.get $l11
                          i32.const 2048
                          i32.lt_u
                          br_if $B42
                          i32.const 3
                          i32.const 4
                          local.get $l11
                          i32.const 65536
                          i32.lt_u
                          select
                          local.set $l9
                        end
                        local.get $l9
                        local.get $l8
                        i32.add
                        local.set $l4
                        br $B17
                      end
                      local.get $l14
                      i64.const -1095216660481
                      i64.and
                      i64.const 17179869184
                      i64.or
                      local.set $l14
                    end
                    i32.const 3
                    local.set $l9
                  end
                  local.get $p2
                  i32.load offset=24
                  local.get $l4
                  local.get $p2
                  i32.load offset=28
                  i32.load offset=16
                  call_indirect (type $t2) $T0
                  i32.eqz
                  br_if $L30
                  br $B1
                end
              end
              local.get $l8
              local.get $l7
              i32.sub
              local.get $l6
              i32.add
              local.set $l8
              local.get $l6
              local.set $l7
              local.get $l5
              local.get $l6
              i32.ne
              br_if $L5
            end
          end
          local.get $l4
          i32.eqz
          br_if $B2
          local.get $l4
          local.get $p1
          i32.eq
          br_if $B2
          local.get $l4
          local.get $p1
          i32.ge_u
          br_if $B0
          local.get $p0
          local.get $l4
          i32.add
          i32.load8_s
          i32.const -65
          i32.le_s
          br_if $B0
        end
        i32.const 1
        local.set $l3
        local.get $p2
        i32.load offset=24
        local.get $p0
        local.get $l4
        i32.add
        local.get $p1
        local.get $l4
        i32.sub
        local.get $p2
        i32.load offset=28
        i32.load offset=12
        call_indirect (type $t6) $T0
        br_if $B1
        local.get $p2
        i32.load offset=24
        i32.const 34
        local.get $p2
        i32.load offset=28
        i32.load offset=16
        call_indirect (type $t2) $T0
        return
      end
      local.get $l3
      return
    end
    local.get $p0
    local.get $p1
    local.get $l4
    local.get $p1
    i32.const 1051912
    call $_ZN4core3str16slice_error_fail17h26278b2259fb6582E
    unreachable)
  (func $_ZN4core7unicode12unicode_data15grapheme_extend6lookup17he3cc23a69ca36d6aE (type $t5) (param $p0 i32) (result i32)
    (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32)
    block $B0
      block $B1
        block $B2
          i32.const 0
          i32.const 15
          local.get $p0
          i32.const 68900
          i32.lt_u
          select
          local.tee $l1
          local.get $l1
          i32.const 8
          i32.add
          local.tee $l1
          local.get $l1
          i32.const 2
          i32.shl
          i32.const 1053944
          i32.add
          i32.load
          i32.const 11
          i32.shl
          local.get $p0
          i32.const 11
          i32.shl
          local.tee $l1
          i32.gt_u
          select
          local.tee $l2
          local.get $l2
          i32.const 4
          i32.add
          local.tee $l2
          local.get $l2
          i32.const 2
          i32.shl
          i32.const 1053944
          i32.add
          i32.load
          i32.const 11
          i32.shl
          local.get $l1
          i32.gt_u
          select
          local.tee $l2
          local.get $l2
          i32.const 2
          i32.add
          local.tee $l2
          local.get $l2
          i32.const 2
          i32.shl
          i32.const 1053944
          i32.add
          i32.load
          i32.const 11
          i32.shl
          local.get $l1
          i32.gt_u
          select
          local.tee $l2
          local.get $l2
          i32.const 1
          i32.add
          local.tee $l2
          local.get $l2
          i32.const 2
          i32.shl
          i32.const 1053944
          i32.add
          i32.load
          i32.const 11
          i32.shl
          local.get $l1
          i32.gt_u
          select
          local.tee $l2
          i32.const 2
          i32.shl
          i32.const 1053944
          i32.add
          i32.load
          i32.const 11
          i32.shl
          local.tee $l3
          local.get $l1
          i32.eq
          local.get $l3
          local.get $l1
          i32.lt_u
          i32.add
          local.get $l2
          i32.add
          local.tee $l1
          i32.const 30
          i32.gt_u
          br_if $B2
          i32.const 689
          local.set $l4
          block $B3
            local.get $l1
            i32.const 30
            i32.eq
            br_if $B3
            local.get $l1
            i32.const 2
            i32.shl
            i32.const 1053948
            i32.add
            i32.load
            i32.const 21
            i32.shr_u
            local.set $l4
          end
          i32.const 0
          local.set $l2
          block $B4
            local.get $l1
            i32.const -1
            i32.add
            local.tee $l3
            local.get $l1
            i32.gt_u
            br_if $B4
            local.get $l3
            i32.const 31
            i32.ge_u
            br_if $B0
            local.get $l3
            i32.const 2
            i32.shl
            i32.const 1053944
            i32.add
            i32.load
            i32.const 2097151
            i32.and
            local.set $l2
          end
          block $B5
            local.get $l4
            local.get $l1
            i32.const 2
            i32.shl
            i32.const 1053944
            i32.add
            i32.load
            i32.const 21
            i32.shr_u
            local.tee $l1
            i32.const 1
            i32.add
            i32.eq
            br_if $B5
            local.get $p0
            local.get $l2
            i32.sub
            local.set $l2
            local.get $l1
            i32.const 689
            local.get $l1
            i32.const 689
            i32.gt_u
            select
            local.set $l3
            local.get $l4
            i32.const -1
            i32.add
            local.set $l4
            i32.const 0
            local.set $p0
            loop $L6
              local.get $l3
              local.get $l1
              i32.eq
              br_if $B1
              local.get $p0
              local.get $l1
              i32.const 1054068
              i32.add
              i32.load8_u
              i32.add
              local.tee $p0
              local.get $l2
              i32.gt_u
              br_if $B5
              local.get $l4
              local.get $l1
              i32.const 1
              i32.add
              local.tee $l1
              i32.ne
              br_if $L6
            end
            local.get $l4
            local.set $l1
          end
          local.get $l1
          i32.const 1
          i32.and
          return
        end
        local.get $l1
        i32.const 31
        i32.const 1053896
        call $_ZN4core9panicking18panic_bounds_check17hc3d961e9f5eff2edE
        unreachable
      end
      local.get $l3
      i32.const 689
      i32.const 1053912
      call $_ZN4core9panicking18panic_bounds_check17hc3d961e9f5eff2edE
      unreachable
    end
    local.get $l3
    i32.const 31
    i32.const 1053928
    call $_ZN4core9panicking18panic_bounds_check17hc3d961e9f5eff2edE
    unreachable)
  (func $_ZN4core7unicode9printable12is_printable17h04f2efbc69a32118E (type $t5) (param $p0 i32) (result i32)
    (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32)
    block $B0
      block $B1
        block $B2
          block $B3
            block $B4
              block $B5
                block $B6
                  block $B7
                    block $B8
                      block $B9
                        local.get $p0
                        i32.const 65536
                        i32.lt_u
                        br_if $B9
                        local.get $p0
                        i32.const 131072
                        i32.lt_u
                        br_if $B8
                        i32.const 0
                        local.set $l1
                        local.get $p0
                        i32.const -201547
                        i32.add
                        i32.const 716213
                        i32.lt_u
                        br_if $B1
                        local.get $p0
                        i32.const -195102
                        i32.add
                        i32.const 1506
                        i32.lt_u
                        br_if $B1
                        local.get $p0
                        i32.const -191457
                        i32.add
                        i32.const 3103
                        i32.lt_u
                        br_if $B1
                        local.get $p0
                        i32.const -183970
                        i32.add
                        i32.const 14
                        i32.lt_u
                        br_if $B1
                        local.get $p0
                        i32.const 2097150
                        i32.and
                        i32.const 178206
                        i32.eq
                        br_if $B1
                        local.get $p0
                        i32.const -173790
                        i32.add
                        i32.const 34
                        i32.lt_u
                        br_if $B1
                        local.get $p0
                        i32.const -177973
                        i32.add
                        i32.const 11
                        i32.lt_u
                        br_if $B1
                        local.get $p0
                        i32.const 918000
                        i32.lt_u
                        return
                      end
                      local.get $p0
                      i32.const 65280
                      i32.and
                      i32.const 8
                      i32.shr_u
                      local.set $l2
                      i32.const 1052504
                      local.set $l3
                      i32.const 0
                      local.set $l4
                      local.get $p0
                      i32.const 255
                      i32.and
                      local.set $l5
                      loop $L10
                        local.get $l3
                        i32.const 2
                        i32.add
                        local.set $l6
                        local.get $l4
                        local.get $l3
                        i32.load8_u offset=1
                        local.tee $l1
                        i32.add
                        local.set $l7
                        block $B11
                          local.get $l3
                          i32.load8_u
                          local.tee $l3
                          local.get $l2
                          i32.eq
                          br_if $B11
                          local.get $l3
                          local.get $l2
                          i32.gt_u
                          br_if $B2
                          local.get $l7
                          local.set $l4
                          local.get $l6
                          local.set $l3
                          local.get $l6
                          i32.const 1052586
                          i32.ne
                          br_if $L10
                          br $B2
                        end
                        local.get $l7
                        local.get $l4
                        i32.lt_u
                        br_if $B7
                        local.get $l7
                        i32.const 290
                        i32.gt_u
                        br_if $B6
                        local.get $l4
                        i32.const 1052586
                        i32.add
                        local.set $l3
                        block $B12
                          loop $L13
                            local.get $l1
                            i32.eqz
                            br_if $B12
                            local.get $l1
                            i32.const -1
                            i32.add
                            local.set $l1
                            local.get $l3
                            i32.load8_u
                            local.set $l4
                            local.get $l3
                            i32.const 1
                            i32.add
                            local.set $l3
                            local.get $l4
                            local.get $l5
                            i32.ne
                            br_if $L13
                          end
                          i32.const 0
                          local.set $l1
                          br $B1
                        end
                        local.get $l7
                        local.set $l4
                        local.get $l6
                        local.set $l3
                        local.get $l6
                        i32.const 1052586
                        i32.ne
                        br_if $L10
                        br $B2
                      end
                    end
                    local.get $p0
                    i32.const 65280
                    i32.and
                    i32.const 8
                    i32.shr_u
                    local.set $l2
                    i32.const 1053185
                    local.set $l3
                    i32.const 0
                    local.set $l4
                    local.get $p0
                    i32.const 255
                    i32.and
                    local.set $l5
                    loop $L14
                      local.get $l3
                      i32.const 2
                      i32.add
                      local.set $l6
                      local.get $l4
                      local.get $l3
                      i32.load8_u offset=1
                      local.tee $l1
                      i32.add
                      local.set $l7
                      block $B15
                        local.get $l3
                        i32.load8_u
                        local.tee $l3
                        local.get $l2
                        i32.eq
                        br_if $B15
                        local.get $l3
                        local.get $l2
                        i32.gt_u
                        br_if $B3
                        local.get $l7
                        local.set $l4
                        local.get $l6
                        local.set $l3
                        local.get $l6
                        i32.const 1053261
                        i32.ne
                        br_if $L14
                        br $B3
                      end
                      local.get $l7
                      local.get $l4
                      i32.lt_u
                      br_if $B5
                      local.get $l7
                      i32.const 175
                      i32.gt_u
                      br_if $B4
                      local.get $l4
                      i32.const 1053261
                      i32.add
                      local.set $l3
                      block $B16
                        loop $L17
                          local.get $l1
                          i32.eqz
                          br_if $B16
                          local.get $l1
                          i32.const -1
                          i32.add
                          local.set $l1
                          local.get $l3
                          i32.load8_u
                          local.set $l4
                          local.get $l3
                          i32.const 1
                          i32.add
                          local.set $l3
                          local.get $l4
                          local.get $l5
                          i32.ne
                          br_if $L17
                        end
                        i32.const 0
                        local.set $l1
                        br $B1
                      end
                      local.get $l7
                      local.set $l4
                      local.get $l6
                      local.set $l3
                      local.get $l6
                      i32.const 1053261
                      i32.ne
                      br_if $L14
                      br $B3
                    end
                  end
                  local.get $l4
                  local.get $l7
                  i32.const 1052472
                  call $_ZN4core5slice22slice_index_order_fail17h0c8c4de89bda6894E
                  unreachable
                end
                local.get $l7
                i32.const 290
                i32.const 1052472
                call $_ZN4core5slice24slice_end_index_len_fail17haeb08024239d8a09E
                unreachable
              end
              local.get $l4
              local.get $l7
              i32.const 1052472
              call $_ZN4core5slice22slice_index_order_fail17h0c8c4de89bda6894E
              unreachable
            end
            local.get $l7
            i32.const 175
            i32.const 1052472
            call $_ZN4core5slice24slice_end_index_len_fail17haeb08024239d8a09E
            unreachable
          end
          local.get $p0
          i32.const 65535
          i32.and
          local.set $l5
          i32.const 1053436
          local.set $l3
          i32.const 1
          local.set $l1
          block $B18
            loop $L19
              local.get $l3
              i32.const 1
              i32.add
              local.set $p0
              block $B20
                block $B21
                  local.get $l3
                  i32.load8_u
                  local.tee $l4
                  i32.const 24
                  i32.shl
                  i32.const 24
                  i32.shr_s
                  local.tee $l7
                  i32.const 0
                  i32.lt_s
                  br_if $B21
                  local.get $p0
                  local.set $l3
                  br $B20
                end
                local.get $p0
                i32.const 1053855
                i32.eq
                br_if $B18
                local.get $l7
                i32.const 127
                i32.and
                i32.const 8
                i32.shl
                local.get $l3
                i32.load8_u offset=1
                i32.or
                local.set $l4
                local.get $l3
                i32.const 2
                i32.add
                local.set $l3
              end
              local.get $l5
              local.get $l4
              i32.sub
              local.tee $l5
              i32.const 0
              i32.lt_s
              br_if $B1
              local.get $l1
              i32.const 1
              i32.xor
              local.set $l1
              local.get $l3
              i32.const 1053855
              i32.ne
              br_if $L19
              br $B1
            end
          end
          i32.const 1051305
          i32.const 43
          i32.const 1052488
          call $_ZN4core9panicking5panic17hc886a4cb4479b06eE
          unreachable
        end
        local.get $p0
        i32.const 65535
        i32.and
        local.set $l5
        i32.const 1052876
        local.set $l3
        i32.const 1
        local.set $l1
        loop $L22
          local.get $l3
          i32.const 1
          i32.add
          local.set $p0
          block $B23
            block $B24
              local.get $l3
              i32.load8_u
              local.tee $l4
              i32.const 24
              i32.shl
              i32.const 24
              i32.shr_s
              local.tee $l7
              i32.const 0
              i32.lt_s
              br_if $B24
              local.get $p0
              local.set $l3
              br $B23
            end
            local.get $p0
            i32.const 1053185
            i32.eq
            br_if $B0
            local.get $l7
            i32.const 127
            i32.and
            i32.const 8
            i32.shl
            local.get $l3
            i32.load8_u offset=1
            i32.or
            local.set $l4
            local.get $l3
            i32.const 2
            i32.add
            local.set $l3
          end
          local.get $l5
          local.get $l4
          i32.sub
          local.tee $l5
          i32.const 0
          i32.lt_s
          br_if $B1
          local.get $l1
          i32.const 1
          i32.xor
          local.set $l1
          local.get $l3
          i32.const 1053185
          i32.ne
          br_if $L22
        end
      end
      local.get $l1
      i32.const 1
      i32.and
      return
    end
    i32.const 1051305
    i32.const 43
    i32.const 1052488
    call $_ZN4core9panicking5panic17hc886a4cb4479b06eE
    unreachable)
  (func $_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hd7770bbf948948ffE (type $t6) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    local.get $p2
    local.get $p0
    local.get $p1
    call $_ZN4core3fmt9Formatter3pad17hb011277a1901f9f7E)
  (func $_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17h5472f29c33f4c4c9E (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i64) (local $l6 i32)
    i32.const 1
    local.set $l2
    block $B0
      local.get $p1
      i32.load offset=24
      i32.const 39
      local.get $p1
      i32.const 28
      i32.add
      i32.load
      i32.load offset=16
      call_indirect (type $t2) $T0
      br_if $B0
      i32.const 116
      local.set $l3
      i32.const 2
      local.set $l4
      block $B1
        block $B2
          block $B3
            block $B4
              block $B5
                block $B6
                  local.get $p0
                  i32.load
                  local.tee $p0
                  i32.const -9
                  i32.add
                  br_table $B1 $B5 $B3 $B3 $B6 $B3 $B3 $B3 $B3 $B3 $B3 $B3 $B3 $B3 $B3 $B3 $B3 $B3 $B3 $B3 $B3 $B3 $B3 $B3 $B3 $B2 $B3 $B3 $B3 $B3 $B2 $B4
                end
                i32.const 114
                local.set $l3
                i32.const 2
                local.set $l4
                br $B1
              end
              i32.const 110
              local.set $l3
              i32.const 2
              local.set $l4
              br $B1
            end
            local.get $p0
            i32.const 92
            i32.eq
            br_if $B2
          end
          block $B7
            block $B8
              block $B9
                local.get $p0
                call $_ZN4core7unicode12unicode_data15grapheme_extend6lookup17he3cc23a69ca36d6aE
                i32.eqz
                br_if $B9
                local.get $p0
                i32.const 1
                i32.or
                i32.clz
                i32.const 2
                i32.shr_u
                i32.const 7
                i32.xor
                i64.extend_i32_u
                i64.const 21474836480
                i64.or
                local.set $l5
                br $B8
              end
              block $B10
                local.get $p0
                call $_ZN4core7unicode9printable12is_printable17h04f2efbc69a32118E
                i32.eqz
                br_if $B10
                i32.const 1
                local.set $l4
                br $B7
              end
              local.get $p0
              i32.const 1
              i32.or
              i32.clz
              i32.const 2
              i32.shr_u
              i32.const 7
              i32.xor
              i64.extend_i32_u
              i64.const 21474836480
              i64.or
              local.set $l5
            end
            i32.const 3
            local.set $l4
          end
          local.get $p0
          local.set $l3
          br $B1
        end
        local.get $p0
        local.set $l3
        i32.const 2
        local.set $l4
      end
      loop $L11
        local.get $l4
        local.set $l6
        i32.const 92
        local.set $p0
        i32.const 1
        local.set $l2
        i32.const 1
        local.set $l4
        block $B12
          block $B13
            block $B14
              block $B15
                block $B16
                  block $B17
                    local.get $l6
                    br_table $B15 $B16 $B12 $B17 $B15
                  end
                  block $B18
                    block $B19
                      block $B20
                        block $B21
                          local.get $l5
                          i64.const 32
                          i64.shr_u
                          i32.wrap_i64
                          i32.const 255
                          i32.and
                          br_table $B15 $B18 $B19 $B20 $B21 $B14 $B15
                        end
                        local.get $l5
                        i64.const -1095216660481
                        i64.and
                        i64.const 12884901888
                        i64.or
                        local.set $l5
                        i32.const 117
                        local.set $p0
                        br $B13
                      end
                      local.get $l5
                      i64.const -1095216660481
                      i64.and
                      i64.const 8589934592
                      i64.or
                      local.set $l5
                      i32.const 123
                      local.set $p0
                      br $B13
                    end
                    i32.const 48
                    i32.const 87
                    local.get $l3
                    local.get $l5
                    i32.wrap_i64
                    local.tee $l4
                    i32.const 2
                    i32.shl
                    i32.const 28
                    i32.and
                    i32.shr_u
                    i32.const 15
                    i32.and
                    local.tee $p0
                    i32.const 10
                    i32.lt_u
                    select
                    local.get $p0
                    i32.add
                    local.set $p0
                    block $B22
                      local.get $l4
                      i32.eqz
                      br_if $B22
                      local.get $l5
                      i64.const -1
                      i64.add
                      i64.const 4294967295
                      i64.and
                      local.get $l5
                      i64.const -4294967296
                      i64.and
                      i64.or
                      local.set $l5
                      br $B13
                    end
                    local.get $l5
                    i64.const -1095216660481
                    i64.and
                    i64.const 4294967296
                    i64.or
                    local.set $l5
                    br $B13
                  end
                  local.get $l5
                  i64.const -1095216660481
                  i64.and
                  local.set $l5
                  i32.const 125
                  local.set $p0
                  br $B13
                end
                i32.const 0
                local.set $l4
                local.get $l3
                local.set $p0
                br $B12
              end
              local.get $p1
              i32.load offset=24
              i32.const 39
              local.get $p1
              i32.load offset=28
              i32.load offset=16
              call_indirect (type $t2) $T0
              return
            end
            local.get $l5
            i64.const -1095216660481
            i64.and
            i64.const 17179869184
            i64.or
            local.set $l5
          end
          i32.const 3
          local.set $l4
        end
        local.get $p1
        i32.load offset=24
        local.get $p0
        local.get $p1
        i32.load offset=28
        i32.load offset=16
        call_indirect (type $t2) $T0
        i32.eqz
        br_if $L11
      end
    end
    local.get $l2)
  (func $_ZN4core5slice6memchr7memrchr17hf1e7486ad49e8b64E (type $t11) (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32)
    (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32) (local $l10 i32)
    local.get $p3
    i32.const 0
    local.get $p3
    i32.const 0
    local.get $p2
    i32.sub
    i32.const 3
    i32.and
    local.tee $l4
    i32.sub
    i32.const 7
    i32.and
    local.get $p3
    local.get $l4
    i32.lt_u
    local.tee $l5
    select
    local.tee $l6
    i32.sub
    local.set $l7
    block $B0
      block $B1
        block $B2
          block $B3
            local.get $p3
            local.get $l6
            i32.lt_u
            br_if $B3
            local.get $p3
            local.get $l4
            local.get $l5
            select
            local.set $l8
            local.get $p2
            local.get $l7
            i32.add
            local.get $p2
            local.get $p3
            i32.add
            local.tee $l4
            i32.sub
            local.set $l5
            local.get $l4
            i32.const -1
            i32.add
            local.set $l4
            local.get $p1
            i32.const 255
            i32.and
            local.set $l9
            block $B4
              loop $L5
                local.get $l6
                i32.eqz
                br_if $B4
                local.get $l5
                i32.const 1
                i32.add
                local.set $l5
                local.get $l6
                i32.const -1
                i32.add
                local.set $l6
                local.get $l4
                i32.load8_u
                local.set $l10
                local.get $l4
                i32.const -1
                i32.add
                local.set $l4
                local.get $l10
                local.get $l9
                i32.ne
                br_if $L5
              end
              local.get $l7
              local.get $l5
              i32.sub
              local.set $l6
              br $B1
            end
            local.get $p1
            i32.const 255
            i32.and
            i32.const 16843009
            i32.mul
            local.set $l4
            block $B6
              loop $L7
                local.get $l7
                local.tee $l6
                local.get $l8
                i32.le_u
                br_if $B6
                local.get $l6
                i32.const -8
                i32.add
                local.set $l7
                local.get $p2
                local.get $l6
                i32.add
                local.tee $l5
                i32.const -4
                i32.add
                i32.load
                local.get $l4
                i32.xor
                local.tee $l10
                i32.const -1
                i32.xor
                local.get $l10
                i32.const -16843009
                i32.add
                i32.and
                local.get $l5
                i32.const -8
                i32.add
                i32.load
                local.get $l4
                i32.xor
                local.tee $l5
                i32.const -1
                i32.xor
                local.get $l5
                i32.const -16843009
                i32.add
                i32.and
                i32.or
                i32.const -2139062144
                i32.and
                i32.eqz
                br_if $L7
              end
            end
            local.get $l6
            local.get $p3
            i32.gt_u
            br_if $B2
            local.get $p2
            i32.const -1
            i32.add
            local.set $l5
            local.get $p1
            i32.const 255
            i32.and
            local.set $l10
            loop $L8
              block $B9
                local.get $l6
                br_if $B9
                i32.const 0
                local.set $l4
                br $B0
              end
              local.get $l5
              local.get $l6
              i32.add
              local.set $l4
              local.get $l6
              i32.const -1
              i32.add
              local.set $l6
              local.get $l4
              i32.load8_u
              local.get $l10
              i32.eq
              br_if $B1
              br $L8
            end
          end
          local.get $l7
          local.get $p3
          i32.const 1051976
          call $_ZN4core5slice26slice_start_index_len_fail17h46c23795afd32c64E
          unreachable
        end
        local.get $l6
        local.get $p3
        i32.const 1051992
        call $_ZN4core5slice24slice_end_index_len_fail17haeb08024239d8a09E
        unreachable
      end
      i32.const 1
      local.set $l4
    end
    local.get $p0
    local.get $l6
    i32.store offset=4
    local.get $p0
    local.get $l4
    i32.store)
  (func $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i8$GT$3fmt17h74ea3e673a2ac4f8E (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32) (local $l3 i32) (local $l4 i32)
    global.get $g0
    i32.const 128
    i32.sub
    local.tee $l2
    global.set $g0
    local.get $p0
    i32.load8_u
    local.set $l3
    i32.const 0
    local.set $p0
    loop $L0
      local.get $l2
      local.get $p0
      i32.add
      i32.const 127
      i32.add
      local.get $l3
      i32.const 15
      i32.and
      local.tee $l4
      i32.const 48
      i32.or
      local.get $l4
      i32.const 87
      i32.add
      local.get $l4
      i32.const 10
      i32.lt_u
      select
      i32.store8
      local.get $p0
      i32.const -1
      i32.add
      local.set $p0
      local.get $l3
      i32.const 4
      i32.shr_u
      i32.const 15
      i32.and
      local.tee $l3
      br_if $L0
    end
    block $B1
      local.get $p0
      i32.const 128
      i32.add
      local.tee $l3
      i32.const 129
      i32.lt_u
      br_if $B1
      local.get $l3
      i32.const 128
      i32.const 1051592
      call $_ZN4core5slice26slice_start_index_len_fail17h46c23795afd32c64E
      unreachable
    end
    local.get $p1
    i32.const 1
    i32.const 1051608
    i32.const 2
    local.get $l2
    local.get $p0
    i32.add
    i32.const 128
    i32.add
    i32.const 0
    local.get $p0
    i32.sub
    call $_ZN4core3fmt9Formatter12pad_integral17h05ee6133195a52bcE
    local.set $p0
    local.get $l2
    i32.const 128
    i32.add
    global.set $g0
    local.get $p0)
  (func $_ZN4core3fmt3num3imp51_$LT$impl$u20$core..fmt..Display$u20$for$u20$u8$GT$3fmt17h98c236a29d0072e5E (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    i64.load8_u
    i32.const 1
    local.get $p1
    call $_ZN4core3fmt3num3imp7fmt_u6417h93f5bc195622e061E)
  (func $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17h7dfebd7501684a06E (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32) (local $l3 i32) (local $l4 i32)
    global.get $g0
    i32.const 128
    i32.sub
    local.tee $l2
    global.set $g0
    local.get $p0
    i32.load
    local.set $l3
    i32.const 0
    local.set $p0
    loop $L0
      local.get $l2
      local.get $p0
      i32.add
      i32.const 127
      i32.add
      local.get $l3
      i32.const 15
      i32.and
      local.tee $l4
      i32.const 48
      i32.or
      local.get $l4
      i32.const 87
      i32.add
      local.get $l4
      i32.const 10
      i32.lt_u
      select
      i32.store8
      local.get $p0
      i32.const -1
      i32.add
      local.set $p0
      local.get $l3
      i32.const 4
      i32.shr_u
      local.tee $l3
      br_if $L0
    end
    block $B1
      local.get $p0
      i32.const 128
      i32.add
      local.tee $l3
      i32.const 129
      i32.lt_u
      br_if $B1
      local.get $l3
      i32.const 128
      i32.const 1051592
      call $_ZN4core5slice26slice_start_index_len_fail17h46c23795afd32c64E
      unreachable
    end
    local.get $p1
    i32.const 1
    i32.const 1051608
    i32.const 2
    local.get $l2
    local.get $p0
    i32.add
    i32.const 128
    i32.add
    i32.const 0
    local.get $p0
    i32.sub
    call $_ZN4core3fmt9Formatter12pad_integral17h05ee6133195a52bcE
    local.set $p0
    local.get $l2
    i32.const 128
    i32.add
    global.set $g0
    local.get $p0)
  (func $_ZN4core3fmt3num3imp7fmt_u6417h93f5bc195622e061E (type $t13) (param $p0 i64) (param $p1 i32) (param $p2 i32) (result i32)
    (local $l3 i32) (local $l4 i32) (local $l5 i64) (local $l6 i32) (local $l7 i32) (local $l8 i32)
    global.get $g0
    i32.const 48
    i32.sub
    local.tee $l3
    global.set $g0
    i32.const 39
    local.set $l4
    block $B0
      block $B1
        local.get $p0
        i64.const 10000
        i64.ge_u
        br_if $B1
        local.get $p0
        local.set $l5
        br $B0
      end
      i32.const 39
      local.set $l4
      loop $L2
        local.get $l3
        i32.const 9
        i32.add
        local.get $l4
        i32.add
        local.tee $l6
        i32.const -4
        i32.add
        local.get $p0
        local.get $p0
        i64.const 10000
        i64.div_u
        local.tee $l5
        i64.const 10000
        i64.mul
        i64.sub
        i32.wrap_i64
        local.tee $l7
        i32.const 65535
        i32.and
        i32.const 100
        i32.div_u
        local.tee $l8
        i32.const 1
        i32.shl
        i32.const 1051610
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        local.get $l6
        i32.const -2
        i32.add
        local.get $l7
        local.get $l8
        i32.const 100
        i32.mul
        i32.sub
        i32.const 65535
        i32.and
        i32.const 1
        i32.shl
        i32.const 1051610
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        local.get $l4
        i32.const -4
        i32.add
        local.set $l4
        local.get $p0
        i64.const 99999999
        i64.gt_u
        local.set $l6
        local.get $l5
        local.set $p0
        local.get $l6
        br_if $L2
      end
    end
    block $B3
      local.get $l5
      i32.wrap_i64
      local.tee $l6
      i32.const 99
      i32.le_s
      br_if $B3
      local.get $l3
      i32.const 9
      i32.add
      local.get $l4
      i32.const -2
      i32.add
      local.tee $l4
      i32.add
      local.get $l5
      i32.wrap_i64
      local.tee $l6
      local.get $l6
      i32.const 65535
      i32.and
      i32.const 100
      i32.div_u
      local.tee $l6
      i32.const 100
      i32.mul
      i32.sub
      i32.const 65535
      i32.and
      i32.const 1
      i32.shl
      i32.const 1051610
      i32.add
      i32.load16_u align=1
      i32.store16 align=1
    end
    block $B4
      block $B5
        local.get $l6
        i32.const 10
        i32.lt_s
        br_if $B5
        local.get $l3
        i32.const 9
        i32.add
        local.get $l4
        i32.const -2
        i32.add
        local.tee $l4
        i32.add
        local.get $l6
        i32.const 1
        i32.shl
        i32.const 1051610
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        br $B4
      end
      local.get $l3
      i32.const 9
      i32.add
      local.get $l4
      i32.const -1
      i32.add
      local.tee $l4
      i32.add
      local.get $l6
      i32.const 48
      i32.add
      i32.store8
    end
    local.get $p2
    local.get $p1
    i32.const 1051268
    i32.const 0
    local.get $l3
    i32.const 9
    i32.add
    local.get $l4
    i32.add
    i32.const 39
    local.get $l4
    i32.sub
    call $_ZN4core3fmt9Formatter12pad_integral17h05ee6133195a52bcE
    local.set $l4
    local.get $l3
    i32.const 48
    i32.add
    global.set $g0
    local.get $l4)
  (func $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i8$GT$3fmt17haa011cd9b81643deE (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32) (local $l3 i32) (local $l4 i32)
    global.get $g0
    i32.const 128
    i32.sub
    local.tee $l2
    global.set $g0
    local.get $p0
    i32.load8_u
    local.set $l3
    i32.const 0
    local.set $p0
    loop $L0
      local.get $l2
      local.get $p0
      i32.add
      i32.const 127
      i32.add
      local.get $l3
      i32.const 15
      i32.and
      local.tee $l4
      i32.const 48
      i32.or
      local.get $l4
      i32.const 55
      i32.add
      local.get $l4
      i32.const 10
      i32.lt_u
      select
      i32.store8
      local.get $p0
      i32.const -1
      i32.add
      local.set $p0
      local.get $l3
      i32.const 4
      i32.shr_u
      i32.const 15
      i32.and
      local.tee $l3
      br_if $L0
    end
    block $B1
      local.get $p0
      i32.const 128
      i32.add
      local.tee $l3
      i32.const 129
      i32.lt_u
      br_if $B1
      local.get $l3
      i32.const 128
      i32.const 1051592
      call $_ZN4core5slice26slice_start_index_len_fail17h46c23795afd32c64E
      unreachable
    end
    local.get $p1
    i32.const 1
    i32.const 1051608
    i32.const 2
    local.get $l2
    local.get $p0
    i32.add
    i32.const 128
    i32.add
    i32.const 0
    local.get $p0
    i32.sub
    call $_ZN4core3fmt9Formatter12pad_integral17h05ee6133195a52bcE
    local.set $p0
    local.get $l2
    i32.const 128
    i32.add
    global.set $g0
    local.get $p0)
  (func $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17h2c02422bfe9eb594E (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32) (local $l3 i32) (local $l4 i32)
    global.get $g0
    i32.const 128
    i32.sub
    local.tee $l2
    global.set $g0
    local.get $p0
    i32.load
    local.set $l3
    i32.const 0
    local.set $p0
    loop $L0
      local.get $l2
      local.get $p0
      i32.add
      i32.const 127
      i32.add
      local.get $l3
      i32.const 15
      i32.and
      local.tee $l4
      i32.const 48
      i32.or
      local.get $l4
      i32.const 55
      i32.add
      local.get $l4
      i32.const 10
      i32.lt_u
      select
      i32.store8
      local.get $p0
      i32.const -1
      i32.add
      local.set $p0
      local.get $l3
      i32.const 4
      i32.shr_u
      local.tee $l3
      br_if $L0
    end
    block $B1
      local.get $p0
      i32.const 128
      i32.add
      local.tee $l3
      i32.const 129
      i32.lt_u
      br_if $B1
      local.get $l3
      i32.const 128
      i32.const 1051592
      call $_ZN4core5slice26slice_start_index_len_fail17h46c23795afd32c64E
      unreachable
    end
    local.get $p1
    i32.const 1
    i32.const 1051608
    i32.const 2
    local.get $l2
    local.get $p0
    i32.add
    i32.const 128
    i32.add
    i32.const 0
    local.get $p0
    i32.sub
    call $_ZN4core3fmt9Formatter12pad_integral17h05ee6133195a52bcE
    local.set $p0
    local.get $l2
    i32.const 128
    i32.add
    global.set $g0
    local.get $p0)
  (func $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17h093b80017617dc71E (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i64)
    local.get $p0
    i32.load
    local.tee $p0
    i64.extend_i32_s
    local.tee $l2
    local.get $l2
    i64.const 63
    i64.shr_s
    local.tee $l2
    i64.add
    local.get $l2
    i64.xor
    local.get $p0
    i32.const -1
    i32.xor
    i32.const 31
    i32.shr_u
    local.get $p1
    call $_ZN4core3fmt3num3imp7fmt_u6417h93f5bc195622e061E)
  (func $memcpy (type $t6) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    (local $l3 i32)
    block $B0
      local.get $p2
      i32.eqz
      br_if $B0
      local.get $p0
      local.set $l3
      loop $L1
        local.get $l3
        local.get $p1
        i32.load8_u
        i32.store8
        local.get $p1
        i32.const 1
        i32.add
        local.set $p1
        local.get $l3
        i32.const 1
        i32.add
        local.set $l3
        local.get $p2
        i32.const -1
        i32.add
        local.tee $p2
        br_if $L1
      end
    end
    local.get $p0)
  (func $bcmp (type $t6) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    (local $l3 i32) (local $l4 i32) (local $l5 i32)
    i32.const 0
    local.set $l3
    block $B0
      local.get $p2
      i32.eqz
      br_if $B0
      block $B1
        loop $L2
          local.get $p0
          i32.load8_u
          local.tee $l4
          local.get $p1
          i32.load8_u
          local.tee $l5
          i32.ne
          br_if $B1
          local.get $p0
          i32.const 1
          i32.add
          local.set $p0
          local.get $p1
          i32.const 1
          i32.add
          local.set $p1
          local.get $p2
          i32.const -1
          i32.add
          local.tee $p2
          i32.eqz
          br_if $B0
          br $L2
        end
      end
      local.get $l4
      local.get $l5
      i32.sub
      local.set $l3
    end
    local.get $l3)
  (table $T0 63 63 funcref)
  (memory $memory 17)
  (global $g0 (mut i32) (i32.const 1048576))
  (global $__data_end i32 (i32.const 1055298))
  (global $__heap_base i32 (i32.const 1055298))
  (export "memory" (memory 0))
  (export "main" (func $main))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (elem $e0 (i32.const 1) $_ZN4core3ptr13drop_in_place17hf76295713a90097aE $_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h99b8727af158158aE $_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h7cb1f56a4edd3851E $_ZN9rust_test4main17h5541390f7c4dfc2eE $_ZN59_$LT$core..fmt..Arguments$u20$as$u20$core..fmt..Display$GT$3fmt17h88a4e919f59e7b36E $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h3525f3f9a0adc297E $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h5d7b0744a6e3b82fE $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17h093b80017617dc71E $_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h43e9c0fc7264942bE $_ZN60_$LT$std..io..error..Error$u20$as$u20$core..fmt..Display$GT$3fmt17h63dc9f43fba9e471E $_ZN3std5alloc24default_alloc_error_hook17hc03eb1d26ecad9f0E $_ZN4core3ptr13drop_in_place17h068db193d06fd1fbE $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17hea8a3f0a0a103a61E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h2566d3d077c1c374E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17hdec56c84cea7811cE $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h39f86d728dd50852E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h37953ac5dd826edfE $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17hdfc78445ae553426E $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h6b90d67ef72b6162E $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h3ad6b7dc2f5cec71E $_ZN63_$LT$core..cell..BorrowMutError$u20$as$u20$core..fmt..Debug$GT$3fmt17hced3ea94d7abc7c3E $_ZN60_$LT$core..cell..BorrowError$u20$as$u20$core..fmt..Debug$GT$3fmt17hd3dc522e1f283df9E $_ZN4core3ptr13drop_in_place17h8fa49d8f224e9668E $_ZN62_$LT$std..ffi..c_str..NulError$u20$as$u20$core..fmt..Debug$GT$3fmt17hdbe9560da47ba080E $_ZN4core3ptr13drop_in_place17h07bb3761885668acE $_ZN82_$LT$std..sys_common..poison..PoisonError$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h063158582f775d68E $_ZN4core3ptr13drop_in_place17h198d7b3663eb2020E $_ZN3std5error5Error5cause17h2b069ed21176965fE $_ZN3std5error5Error7type_id17hd6272b78ea2ed3abE $_ZN3std5error5Error9backtrace17hd42982a9d518845cE $_ZN243_$LT$std..error..$LT$impl$u20$core..convert..From$LT$alloc..string..String$GT$$u20$for$u20$alloc..boxed..Box$LT$dyn$u20$std..error..Error$u2b$core..marker..Sync$u2b$core..marker..Send$GT$$GT$..from..StringError$u20$as$u20$std..error..Error$GT$11description17hbc74e6da51e8498dE $_ZN244_$LT$std..error..$LT$impl$u20$core..convert..From$LT$alloc..string..String$GT$$u20$for$u20$alloc..boxed..Box$LT$dyn$u20$std..error..Error$u2b$core..marker..Sync$u2b$core..marker..Send$GT$$GT$..from..StringError$u20$as$u20$core..fmt..Display$GT$3fmt17hd22e6bf5f24cc832E $_ZN242_$LT$std..error..$LT$impl$u20$core..convert..From$LT$alloc..string..String$GT$$u20$for$u20$alloc..boxed..Box$LT$dyn$u20$std..error..Error$u2b$core..marker..Sync$u2b$core..marker..Send$GT$$GT$..from..StringError$u20$as$u20$core..fmt..Debug$GT$3fmt17hc54eb153061f3528E $_ZN4core3ptr13drop_in_place17h33cb88e18f1743e1E $_ZN80_$LT$std..io..Write..write_fmt..Adaptor$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h4a28ff28c6c8fdc2E $_ZN4core3fmt5Write10write_char17h0246824b0281d4ecE $_ZN4core3fmt5Write9write_fmt17h2f805ac767637cb6E $_ZN3std4sync4once4Once9call_once28_$u7b$$u7b$closure$u7d$$u7d$17h34d413a2f6944e79E $_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h9e4f16104979e5cbE $_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h1bfafad7a2b0fca0E $_ZN4core3ptr13drop_in_place17hf58877e2af32fc2aE $_ZN90_$LT$std..panicking..begin_panic_handler..PanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$8take_box17h327c8a118334ec23E $_ZN90_$LT$std..panicking..begin_panic_handler..PanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$3get17he829a193e7bd084bE $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hc4ed596920ed0f82E $_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$8take_box17h3148b39bb58acd57E $_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$3get17hdc67d47d21f85001E $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hac9d42c1070e325fE $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17haddd63b3d1b18b68E $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17hf76888becbde89b4E $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h1a51066d15be9a53E $_ZN71_$LT$core..ops..range..Range$LT$Idx$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h2e46a5c0d45e01feE $_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17h5472f29c33f4c4c9E $_ZN4core3ops8function6FnOnce9call_once17h4d488110c8a675c3E $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hf0970a00b42f5ba2E $_ZN4core3ptr13drop_in_place17h00c08aab80423b88E $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h128e23c99f6446a5E $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h44ce8e8e61187795E $_ZN4core3fmt5Write10write_char17h0660426ba5d037baE $_ZN4core3fmt5Write9write_fmt17h3781fd8c2a82affaE $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17he83ce1db16bdf500E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h2670637b4af27d11E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17hba895e75ebac3cc2E)
  (data $d0 (i32.const 1048576) "\01\00\00\00\04\00\00\00\04\00\00\00\02\00\00\00\02\00\00\00\03\00\00\00test!\0a\00\00\18\00\10\00\06\00\00\00 \00\10\00,\00\10\00/rustc/e2be5f568d1f60365b825530f5b5cb722460591b/library/core/src/slice/mod.rs\00\00\00\0c\00\00\00\04\00\00\00\04\00\00\00\0d\00\00\00\0e\00\00\00\0f\00\00\00\0c\00\00\00\04\00\00\00\04\00\00\00\10\00\00\00\11\00\00\00\12\00\00\00\0c\00\00\00\04\00\00\00\04\00\00\00\13\00\00\00already borrowedalready mutably borrowedassertion failed: `(left == right)`\0a  left: ``,\0a right: ``\00\00\e8\00\10\00-\00\00\00\15\01\10\00\0c\00\00\00!\01\10\00\01\00\00\00\0c\00\00\00\00\00\00\00\01\00\00\00\14\00\00\00assertion failed: mid <= self.len()\000\00\10\00M\00\00\00\cf\04\00\00\09\00\00\00called `Option::unwrap()` on a `None` value\00\0c\00\00\00\00\00\00\00\01\00\00\00\15\00\00\00\0c\00\00\00\00\00\00\00\01\00\00\00\16\00\00\00\17\00\00\00\10\00\00\00\04\00\00\00\18\00\00\00called `Result::unwrap()` on an `Err` value\00\19\00\00\00\08\00\00\00\04\00\00\00\1a\00\00\00\0c\00\00\00\04\00\00\00\04\00\00\00\06\00\00\00library/std/src/thread/mod.rs\00\00\00(\02\10\00\1d\00\00\00n\03\00\00*\00\00\00inconsistent park state\00(\02\10\00\1d\00\00\00|\03\00\00\13\00\00\00\02\00\00\00`: \00\e8\00\10\00-\00\00\00\15\01\10\00\0c\00\00\00\84\02\10\00\03\00\00\00park state changed unexpectedly\00\a0\02\10\00\1f\00\00\00(\02\10\00\1d\00\00\00y\03\00\00\0d\00\00\00failed to generate unique thread ID: bitspace exhausted\00(\02\10\00\1d\00\00\00\0c\04\00\00\11\00\00\00(\02\10\00\1d\00\00\00\12\04\00\00*\00\00\00thread name may not contain interior null bytes\00(\02\10\00\1d\00\00\00P\04\00\00*\00\00\00inconsistent state in unpark(\02\10\00\1d\00\00\00\86\04\00\00\12\00\00\00(\02\10\00\1d\00\00\00\94\04\00\00%\00\00\00\1b\00\00\00\0c\00\00\00\04\00\00\00\1c\00\00\00\1d\00\00\00\1e\00\00\00\1f\00\00\00\1c\00\00\00 \00\00\00!\00\00\00library/std/src/io/buffered.rs\00\00\d4\03\10\00\1e\00\00\00f\02\00\00)\00\00\00\d4\03\10\00\1e\00\00\00\a8\02\00\00\1d\00\00\00unexpected end of fileother os erroroperation interruptedwrite zerotimed outinvalid datainvalid input parameteroperation would blockentity already existsbroken pipeaddress not availableaddress in usenot connectedconnection abortedconnection resetconnection refusedpermission deniedentity not found\00\00\00<\01\10\00\00\00\00\00 (os error )<\01\10\00\00\00\00\00H\05\10\00\0b\00\00\00S\05\10\00\01\00\00\00library/std/src/io/stdio.rscannot access stdout during shutdown\00l\05\10\00\1b\00\00\00\1b\02\00\003\00\00\00l\05\10\00\1b\00\00\00t\02\00\00\14\00\00\00failed printing to : \00\00\00\cc\05\10\00\13\00\00\00\df\05\10\00\02\00\00\00l\05\10\00\1b\00\00\00\82\03\00\00\09\00\00\00l\05\10\00\1b\00\00\00w\03\00\00\1a\00\00\00l\05\10\00\1b\00\00\00z\03\00\00\14\00\00\00stdoutformatter error\00\00\00\22\00\00\00\0c\00\00\00\04\00\00\00#\00\00\00$\00\00\00%\00\00\00attempted to use a condition variable with two mutexeslibrary/std/src/sync/condvar.rs\00\00\00\8a\06\10\00\1f\00\00\00?\02\00\00\12\00\00\00\0c\00\00\00\04\00\00\00\04\00\00\00&\00\00\00'\00\00\00library/std/src/sync/once.rs\d0\06\10\00\1c\00\00\00\0c\01\00\002\00\00\00assertion failed: state_and_queue & STATE_MASK == RUNNING\00\00\00\d0\06\10\00\1c\00\00\00\af\01\00\00\15\00\00\00Once instance has previously been poisoned\00\00\d0\06\10\00\1c\00\00\00\8f\01\00\00\15\00\00\00\d0\06\10\00\1c\00\00\00\f0\01\00\00\09\00\00\00\d0\06\10\00\1c\00\00\00\fc\01\00\005\00\00\00assertion failed: queue != DONElibrary/std/src/sys_common/at_exit_imp.rs\c3\07\10\00)\00\00\001\00\00\00\0d\00\00\00PoisonError { inner: .. }library/std/src/sys_common/thread_info.rs\00\00\15\08\10\00)\00\00\00\15\00\00\00\16\00\00\00\15\08\10\00)\00\00\00\16\00\00\00\18\00\00\00\15\08\10\00)\00\00\00\19\00\00\00\15\00\00\00\15\08\10\00)\00\00\00(\00\00\00$\00\00\00assertion failed: c.borrow().is_none()\00\00\15\08\10\00)\00\00\00(\00\00\00\1a\00\00\00\15\08\10\00)\00\00\00)\00\00\00\22\00\00\00\0c\00\00\00\04\00\00\00\04\00\00\00(\00\00\00library/std/src/panicking.rs\d8\08\10\00\1c\00\00\00\e1\01\00\00\1f\00\00\00\d8\08\10\00\1c\00\00\00\e2\01\00\00\1e\00\00\00)\00\00\00\10\00\00\00\04\00\00\00*\00\00\00+\00\00\00\1b\00\00\00\0c\00\00\00\04\00\00\00,\00\00\00\0c\00\00\00\08\00\00\00\04\00\00\00-\00\00\00.\00\00\00\0c\00\00\00\08\00\00\00\04\00\00\00/\00\00\00NulError\0c\00\00\00\04\00\00\00\04\00\00\000\00\00\00operation successfulcondvar wait not supportedlibrary/std/src/sys/wasm/../unsupported/condvar.rs\a2\09\10\002\00\00\00\15\00\00\00\09\00\00\00cannot recursively acquire mutexlibrary/std/src/sys/wasm/../unsupported/mutex.rs\04\0a\10\000\00\00\00\16\00\00\00\09\00\00\00library/alloc/src/raw_vec.rscapacity overflow\00\00\00D\0a\10\00\1c\00\00\00\1e\02\00\00\05\00\00\00`..\00\85\0a\10\00\02\00\00\00BorrowErrorBorrowMutErrorcalled `Option::unwrap()` on a `None` value\84\0a\10\00\00\00\00\00: \00\00\84\0a\10\00\00\00\00\00\dc\0a\10\00\02\00\00\007\00\00\00\00\00\00\00\01\00\00\008\00\00\00index out of bounds: the len is  but the index is \00\00\00\0b\10\00 \00\00\00 \0b\10\00\12\00\00\00library/core/src/fmt/builders.rs7\00\00\00\0c\00\00\00\04\00\00\009\00\00\00:\00\00\00;\00\00\00    D\0b\10\00 \00\00\000\00\00\00!\00\00\00D\0b\10\00 \00\00\001\00\00\00\12\00\00\00,\0a, (\0a(,)\0a[]library/core/src/fmt/num.rs\00\ac\0b\10\00\1b\00\00\00T\00\00\00\14\00\00\000x00010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899\00\007\00\00\00\04\00\00\00\04\00\00\00<\00\00\00=\00\00\00>\00\00\00library/core/src/fmt/mod.rs\00\bc\0c\10\00\1b\00\00\00W\04\00\00\11\00\00\00\bc\0c\10\00\1b\00\00\00a\04\00\00$\00\00\00\bc\0c\10\00\1b\00\00\00\f2\07\00\00\1e\00\00\00\bc\0c\10\00\1b\00\00\00\f9\07\00\00\16\00\00\00library/core/src/slice/memchr.rs\18\0d\10\00 \00\00\00R\00\00\00\05\00\00\00\18\0d\10\00 \00\00\00i\00\00\00\1a\00\00\00\18\0d\10\00 \00\00\00\83\00\00\00\05\00\00\00range start index  out of range for slice of length h\0d\10\00\12\00\00\00z\0d\10\00\22\00\00\00range end index \ac\0d\10\00\10\00\00\00z\0d\10\00\22\00\00\00slice index starts at  but ends at \00\cc\0d\10\00\16\00\00\00\e2\0d\10\00\0d\00\00\00library/core/src/str/pattern.rs\00\00\0e\10\00\1f\00\00\00\b0\01\00\00&\00\00\00[...]byte index  is out of bounds of `\00\005\0e\10\00\0b\00\00\00@\0e\10\00\16\00\00\00\84\0a\10\00\01\00\00\00begin <= end ( <= ) when slicing `\00\00p\0e\10\00\0e\00\00\00~\0e\10\00\04\00\00\00\82\0e\10\00\10\00\00\00\84\0a\10\00\01\00\00\00 is not a char boundary; it is inside  (bytes ) of `5\0e\10\00\0b\00\00\00\b4\0e\10\00&\00\00\00\da\0e\10\00\08\00\00\00\e2\0e\10\00\06\00\00\00\84\0a\10\00\01\00\00\00library/core/src/unicode/printable.rs\00\00\00\10\0f\10\00%\00\00\00\0a\00\00\00\1c\00\00\00\10\0f\10\00%\00\00\00\1a\00\00\006\00\00\00\00\01\03\05\05\06\06\03\07\06\08\08\09\11\0a\1c\0b\19\0c\14\0d\10\0e\0d\0f\04\10\03\12\12\13\09\16\01\17\05\18\02\19\03\1a\07\1c\02\1d\01\1f\16 \03+\03,\02-\0b.\010\031\022\01\a7\02\a9\02\aa\04\ab\08\fa\02\fb\05\fd\04\fe\03\ff\09\adxy\8b\8d\a20WX\8b\8c\90\1c\1d\dd\0e\0fKL\fb\fc./?\5c]_\b5\e2\84\8d\8e\91\92\a9\b1\ba\bb\c5\c6\c9\ca\de\e4\e5\ff\00\04\11\12)147:;=IJ]\84\8e\92\a9\b1\b4\ba\bb\c6\ca\ce\cf\e4\e5\00\04\0d\0e\11\12)14:;EFIJ^de\84\91\9b\9d\c9\ce\cf\0d\11)EIWde\8d\91\a9\b4\ba\bb\c5\c9\df\e4\e5\f0\0d\11EIde\80\84\b2\bc\be\bf\d5\d7\f0\f1\83\85\8b\a4\a6\be\bf\c5\c7\ce\cf\da\dbH\98\bd\cd\c6\ce\cfINOWY^_\89\8e\8f\b1\b6\b7\bf\c1\c6\c7\d7\11\16\17[\5c\f6\f7\fe\ff\80\0dmq\de\df\0e\0f\1fno\1c\1d_}~\ae\af\bb\bc\fa\16\17\1e\1fFGNOXZ\5c^~\7f\b5\c5\d4\d5\dc\f0\f1\f5rs\8ftu\96/_&./\a7\af\b7\bf\c7\cf\d7\df\9a@\97\980\8f\1f\c0\c1\ce\ffNOZ[\07\08\0f\10'/\ee\efno7=?BE\90\91\fe\ffSgu\c8\c9\d0\d1\d8\d9\e7\fe\ff\00 _\22\82\df\04\82D\08\1b\04\06\11\81\ac\0e\80\ab5(\0b\80\e0\03\19\08\01\04/\044\04\07\03\01\07\06\07\11\0aP\0f\12\07U\07\03\04\1c\0a\09\03\08\03\07\03\02\03\03\03\0c\04\05\03\0b\06\01\0e\15\05:\03\11\07\06\05\10\07W\07\02\07\15\0dP\04C\03-\03\01\04\11\06\0f\0c:\04\1d%_ m\04j%\80\c8\05\82\b0\03\1a\06\82\fd\03Y\07\15\0b\17\09\14\0c\14\0cj\06\0a\06\1a\06Y\07+\05F\0a,\04\0c\04\01\031\0b,\04\1a\06\0b\03\80\ac\06\0a\06!?L\04-\03t\08<\03\0f\03<\078\08+\05\82\ff\11\18\08/\11-\03 \10!\0f\80\8c\04\82\97\19\0b\15\88\94\05/\05;\07\02\0e\18\09\80\b3-t\0c\80\d6\1a\0c\05\80\ff\05\80\df\0c\ee\0d\03\84\8d\037\09\81\5c\14\80\b8\08\80\cb*8\03\0a\068\08F\08\0c\06t\0b\1e\03Z\04Y\09\80\83\18\1c\0a\16\09L\04\80\8a\06\ab\a4\0c\17\041\a1\04\81\da&\07\0c\05\05\80\a5\11\81m\10x(*\06L\04\80\8d\04\80\be\03\1b\03\0f\0d\00\06\01\01\03\01\04\02\08\08\09\02\0a\05\0b\02\0e\04\10\01\11\02\12\05\13\11\14\01\15\02\17\02\19\0d\1c\05\1d\08$\01j\03k\02\bc\02\d1\02\d4\0c\d5\09\d6\02\d7\02\da\01\e0\05\e1\02\e8\02\ee \f0\04\f8\02\f9\02\fa\02\fb\01\0c';>NO\8f\9e\9e\9f\06\07\096=>V\f3\d0\d1\04\14\1867VW\7f\aa\ae\af\bd5\e0\12\87\89\8e\9e\04\0d\0e\11\12)14:EFIJNOde\5c\b6\b7\1b\1c\07\08\0a\0b\14\1769:\a8\a9\d8\d9\097\90\91\a8\07\0a;>fi\8f\92o_\ee\efZb\9a\9b'(U\9d\a0\a1\a3\a4\a7\a8\ad\ba\bc\c4\06\0b\0c\15\1d:?EQ\a6\a7\cc\cd\a0\07\19\1a\22%>?\c5\c6\04 #%&(38:HJLPSUVXZ\5c^`cefksx}\7f\8a\a4\aa\af\b0\c0\d0\ae\afy\ccno\93^\22{\05\03\04-\03f\03\01/.\80\82\1d\031\0f\1c\04$\09\1e\05+\05D\04\0e*\80\aa\06$\04$\04(\084\0b\01\80\90\817\09\16\0a\08\80\989\03c\08\090\16\05!\03\1b\05\01@8\04K\05/\04\0a\07\09\07@ '\04\0c\096\03:\05\1a\07\04\0c\07PI73\0d3\07.\08\0a\81&RN(\08*V\1c\14\17\09N\04\1e\0fC\0e\19\07\0a\06H\08'\09u\0b?A*\06;\05\0a\06Q\06\01\05\10\03\05\80\8bb\1eH\08\0a\80\a6^\22E\0b\0a\06\0d\139\07\0a6,\04\10\80\c0<dS\0cH\09\0aFE\1bH\08S\1d9\81\07F\0a\1d\03GI7\03\0e\08\0a\069\07\0a\816\19\80\b7\01\0f2\0d\83\9bfu\0b\80\c4\8a\bc\84/\8f\d1\82G\a1\b9\829\07*\04\02`&\0aF\0a(\05\13\82\b0[eK\049\07\11@\05\0b\02\0e\97\f8\08\84\d6*\09\a2\f7\81\1f1\03\11\04\08\81\8c\89\04k\05\0d\03\09\07\10\93`\80\f6\0as\08n\17F\80\9a\14\0cW\09\19\80\87\81G\03\85B\0f\15\85P+\80\d5-\03\1a\04\02\81p:\05\01\85\00\80\d7)L\04\0a\04\02\83\11DL=\80\c2<\06\01\04U\05\1b4\02\81\0e,\04d\0cV\0a\80\ae8\1d\0d,\04\09\07\02\0e\06\80\9a\83\d8\08\0d\03\0d\03t\0cY\07\0c\14\0c\048\08\0a\06(\08\22N\81T\0c\15\03\03\05\07\09\19\07\07\09\03\0d\07)\80\cb%\0a\84\06library/core/src/unicode/unicode_data.rs\00\9f\14\10\00(\00\00\00K\00\00\00(\00\00\00\9f\14\10\00(\00\00\00W\00\00\00\16\00\00\00\9f\14\10\00(\00\00\00R\00\00\00>\00\00\00\00\03\00\00\83\04 \00\91\05`\00]\13\a0\00\12\17\a0\1e\0c \e0\1e\ef, +*0\a0+o\a6`,\02\a8\e0,\1e\fb\e0-\00\fe\a05\9e\ff\e05\fd\01a6\01\0a\a16$\0da7\ab\0e\e18/\18!90\1caF\f3\1e\a1J\f0jaNOo\a1N\9d\bc!Oe\d1\e1O\00\da!P\00\e0\e1Q0\e1aS\ec\e2\a1T\d0\e8\e1T \00.U\f0\01\bfU\00p\00\07\00-\01\01\01\02\01\02\01\01H\0b0\15\10\01e\07\02\06\02\02\01\04#\01\1e\1b[\0b:\09\09\01\18\04\01\09\01\03\01\05+\03w\0f\01 7\01\01\01\04\08\04\01\03\07\0a\02\1d\01:\01\01\01\02\04\08\01\09\01\0a\02\1a\01\02\029\01\04\02\04\02\02\03\03\01\1e\02\03\01\0b\029\01\04\05\01\02\04\01\14\02\16\06\01\01:\01\01\02\01\04\08\01\07\03\0a\02\1e\01;\01\01\01\0c\01\09\01(\01\03\019\03\05\03\01\04\07\02\0b\02\1d\01:\01\02\01\02\01\03\01\05\02\07\02\0b\02\1c\029\02\01\01\02\04\08\01\09\01\0a\02\1d\01H\01\04\01\02\03\01\01\08\01Q\01\02\07\0c\08b\01\02\09\0b\06J\02\1b\01\01\01\01\017\0e\01\05\01\02\05\0b\01$\09\01f\04\01\06\01\02\02\02\19\02\04\03\10\04\0d\01\02\02\06\01\0f\01\00\03\00\03\1d\03\1d\02\1e\02@\02\01\07\08\01\02\0b\09\01-\03w\02\22\01v\03\04\02\09\01\06\03\db\02\02\01:\01\01\07\01\01\01\01\02\08\06\0a\02\010\11?\040\07\01\01\05\01(\09\0c\02 \04\02\02\01\038\01\01\02\03\01\01\03:\08\02\02\98\03\01\0d\01\07\04\01\06\01\03\02\c6:\01\05\00\01\c3!\00\03\8d\01` \00\06i\02\00\04\01\0a \02P\02\00\01\03\01\04\01\19\02\05\01\97\02\1a\12\0d\01&\08\19\0b.\030\01\02\04\02\02'\01C\06\02\02\02\02\0c\01\08\01/\013\01\01\03\02\02\05\02\01\01*\02\08\01\ee\01\02\01\04\01\00\01\00\10\10\10\00\02\00\01\e2\01\95\05\00\03\01\02\05\04(\03\04\01\a5\02\00\04\00\02\99\0b\b0\016\0f8\031\04\02\02E\03$\05\01\08>\01\0c\024\09\0a\04\02\01_\03\02\01\01\02\06\01\a0\01\03\08\15\029\02\01\01\01\01\16\01\0e\07\03\05\c3\08\02\03\01\01\17\01Q\01\02\06\01\01\02\01\01\02\01\02\eb\01\02\04\06\02\01\02\1b\02U\08\02\01\01\02j\01\01\01\02\06\01\01e\03\02\04\01\05\00\09\01\02\f5\01\0a\02\01\01\04\01\90\04\02\02\04\01 \0a(\06\02\04\08\01\09\06\02\03.\0d\01\02\00\07\01\06\01\01R\16\02\07\01\02\01\02z\06\03\01\01\02\01\07\01\01H\02\03\01\01\01\00\02\00\05;\07\00\01?\04Q\01\00\02\00\01\01\03\04\05\08\08\02\07\1e\04\94\03\007\042\08\01\0e\01\16\05\01\0f\00\07\01\11\02\07\01\02\01\05\00\07\00\04\00\07m\07\00`\80\f0\00")
  (data $d1 (i32.const 1054760) "\01\00\00\00\00\00\00\00"))
