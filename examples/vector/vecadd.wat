(module
  (func $_start (result v128)
    (local $l2 v128)
    (f32.const 0.5)
    (f32x4.splat)
    (local.set $l2)
    (local.get $l2)
    (local.get $l2)
    (f32x4.add)
  )
  (export "_start" (func $_start))
)
