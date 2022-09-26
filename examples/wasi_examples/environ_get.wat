
(module
    (type $t0 (func))

    (import "wasi_unstable" "environ_get" 
        (func $get_env (param i32 i32) (result i32)))
    (import "wasi_unstable" "fd_write" 
        (func $print (param $fd i32) 
	             (param $iovec i32)
		           (param $len i32)
		           (param $written i32) (result i32)))
    (import "wasi_unstable" "proc_exit" 
        (func $exit (param i32)))

    (memory 1)
    (export "memory" (memory 0))

    ;;(data (i32.const 100))
    (data (i32.const 104) "\n")

    (func $newline (type $t0)
	    ;; new line
        i32.const 32  ;; offset
        i32.const 104  ;; buf*
        i32.store align=2
        i32.const 36  ;; offset
        i32.const 1  ;; buf_len
        i32.store offset=0 align=2

        i32.const 1  ;; 1 for stdout
        i32.const 32  ;; 0 as we stored the beginning of __wasi_ciovec_t
        i32.const 1  ;; how many I/O vectors are passed in, just one in our case
        i32.const 100 ;; nwritten
        call $print
	      drop
    )

    (func $_start (type $t0) (local i32 i32 i32)
        i32.const 0  ;; offset for environ pointer
        i32.const 4  ;; value 
        i32.store align=2
	      ;; so we have environ as the first slot in memory 0-4

        i32.const 4  ;; offset for environ pointer
        i32.const 0  ;; value 
        i32.store align=2
	      ;; this is the first pointer for environ 

        i32.const 64  ;; offset for environ_buf*
        i32.const 0   ;; value 
        i32.store align=2
        ;; the second slot is pointer to a buffer to write the argument string data
        ;; so this needs to be able to store all the data of the arguments, 
        ;; environ will contain pointers into this area after environ_get has been
        ;; called.
        i32.const 0  ;; address 0 for environ
        i32.const 64 ;; address 64 for environ_buf
        call $get_env 
	      drop

	      ;; set up __wasi_ciovec_t (const void buf* and size_t buf_len) for environ[0]
        i32.const 32  ;; offset
        i32.const 64  ;; buf*
        i32.store align=2
        i32.const 36  ;; offset
        i32.const 6  ;; buf_len
        i32.store offset=0 align=2

        i32.const 1  ;; 1 for stdout
        i32.const 32  ;; 0 as we stored the beginning of __wasi_ciovec_t
        i32.const 1  ;; how many I/O vectors are passed in, just one in our case
        i32.const 100 ;; nwritten
        call $print
	      drop
        call $newline
        ;; set up __wasi_ciovec_t for environ[1]
        i32.const 32  ;; offset
        i32.const 70  ;; buf* 
        i32.store align=2
        i32.const 36  ;; offset
        i32.const 6  ;; buf_len
        i32.store offset=0 align=2

        i32.const 1  ;; 1 for stdout
        i32.const 32  ;; 0 as we stored the beginning of __wasi_ciovec_t
        i32.const 1  ;; how many I/O vectors are passed in, just one in our case
        i32.const 100 ;; nwritten
        call $print
	      drop
	      call $newline
        i32.const 100  ;; offset
        i32.load offset=0 ;; length written
        call $exit
    )
      (export "_start" (func $_start))
)