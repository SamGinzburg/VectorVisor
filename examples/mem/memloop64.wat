(module
  (memory (export "memory") 32)
  (func $_start (result i32)
  	(local i64 i32)
  	i32.const 0
	i64.const 0
	i64.store
	i64.const 0
	local.set 0
	i32.const 0
	local.set 1
	loop
		loop
			local.get 0
			i32.wrap_i64
			i64.load
			i64.const 2048
			i64.add
			local.tee 0
			i32.wrap_i64
			local.get 0
			i64.store
			local.get 0
			i32.wrap_i64
			i64.load
			i64.const 2048000
			i64.ne
			br_if 0
		end
		i64.const 0
		local.set 0
		local.get 1
		i32.const 1
		i32.add
		local.tee 1
		i32.const 100000
		i32.ne
		br_if 0
	end
	local.get 1
	i32.load
  )
  (export "_start" (func $_start))
)
