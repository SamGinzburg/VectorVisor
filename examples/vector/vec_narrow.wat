(module
  (func $_start (result v128)
    (local $l2 v128)
    ;;(f32.const 0.5)
    ;;(f32x4.splat)
    (v128.const i32x4 0x90000001 0x00000002 0x00000003 0x00000004)
    (v128.const i32x4 0x00000001 0x00000002 0x00000003 0x00000004)
    (i8x16.narrow_i16x8_u)
  )
  (export "_start" (func $_start))
)
