(module
  (memory (export "memory") 48)
  (func $_start (result i32)
  	(local i32 i32)
  	i32.const 0
	i32.const 0
	i32.store
	i32.const 0
	local.set 0
	i32.const 1024
	i32.const 1024
	i32.mul
	local.set 1
	;; store values
	loop
		local.get 0
		i32.const 8
		i32.add
		local.tee 0
		local.get 0
		local.get 1
		i32.add
		local.get 0
		;; store to mem
		i32.load
		i32.store
		local.get 0
		local.get 1
		i32.add
		i32.const 4
		i32.add
		local.get 0
		;; store to mem
		i32.load
		i32.store
		local.get 1
		i32.ne
		br_if 0
	end
	i32.const 0
  )
  (export "_start" (func $_start))
)
