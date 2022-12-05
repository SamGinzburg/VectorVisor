;; see https://github.com/WebAssembly/WASI/blob/master/phases/snapshot/witx/wasi_snapshot_preview1.witx

(module
  (type $t0 (func (param i32 i32 i32 i32) (result i32)))
  (type $t1 (func (param i32)))
  (type $t2 (func (param i32 i32) (result i32)))
  (type $t3 (func (param i32 i32) (result i32)))
  (type $t4 (func (param i32 i32)))
  (import "env" "serverless_invoke" (func $serverless_invoke (type $t3)))
  (import "env" "serverless_response" (func $serverless_response (type $t4)))

  (func $_start  (result)
     loop
        i32.const 1024 ;; resp ptr
        ;; call serverless invoke
        i32.const 1024 ;; invoke ptr
        i32.const 524288
        call $serverless_invoke ;; pushes the resp len onto the stack
        ;; call serverless response
        call $serverless_response
        br 0
     end 
    )
  (memory $memory 32)
  (export "memory" (memory 0))
  (export "_start" (func $_start))
)
