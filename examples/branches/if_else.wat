(module
  (func $_start (result i32)
  	(local i32)
   	i32.const 0
	if 
		i32.const 1
		local.set 0
		i32.const 2
		if
			i32.const 3
			local.set 0
		else
			i32.const 5
			local.set 0
		end
	end
	local.get 0
  )
  (export "_start" (func $_start))
)
