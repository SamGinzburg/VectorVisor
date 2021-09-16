(module
  (memory (export "memory") 32)
  (func $_start (result i32)
  	(local i32 i32)
  	i32.const 0
	i32.const 0
	i32.store
	i32.const 0
	local.set 0
	i32.const 0
	local.set 1
	loop
		loop
			local.get 0
			i32.load
			i32.const 2048
			i32.add
			local.tee 0
			local.get 0
			i32.store
			local.get 0
			i32.load
			i32.const 2048000
			i32.ne
			br_if 0
		end
		i32.const 0
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
