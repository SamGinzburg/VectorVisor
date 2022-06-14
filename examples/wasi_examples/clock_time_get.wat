(module
  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  ;; Type declarations
  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  (type  (func (param i32 i64 i32)     (result i32)))
  (type  (func (param i32 i32 i32 i32) (result i32)))

  (type (func))
  (type (func (result i32)))
  (type  (func (param i32) (result i32)))
  (type  (func (param i32 i32 i32) (result i32)))

  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  ;; Declare the use of native "OS" functions accessible through the WebAssembly System Interface (WASI)
  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  (import "wasi_unstable" "clock_time_get" (func $wasi_unstable.clock_time_get (type 0)))
  (import "wasi_unstable" "fd_write"       (func $wasi_unstable.fd_write       (type 1)))

  ;; *******************************************************************************************************************
  ;; Private API functions
  ;; *******************************************************************************************************************

  ;; Line feed character
  (func $line-feed (type 3) (result i32) i32.const 10)

  ;; Bit masks for accessing either the upper or lower nybbles of a byte
  (func $hex-mask-upper (type 3) (result i32) i32.const 240)  ;; Returns 0xF0
  (func $hex-mask-lower (type 3) (result i32) i32.const 15)   ;; Returns 0x0F

  ;; Increment/decrement functions
  (func $incr (type 4) (param i32) (result i32) (i32.add (get_local 0) (i32.const 1)))
  (func $decr (type 4) (param i32) (result i32) (i32.sub (get_local 0) (i32.const 1)))

  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  ;; Convert a byte's upper or lower nybbles to the corresponding ASCII character
  ;; Input  : [ i32 ]        Integer in the range 0x0 to 0xF
  ;; Output : [ i32 ]        ASCII character of input value left on the stack
  ;;
  ;; To access a byte's upper nybble, mask out the lower nybble by AND'ing it with 0xF0
  ;; For example, if we receive input 6c
  ;;       6    C
  ;;       0110 1100
  ;;   AND 1111 0000
  ;;    -> 0110 0000
  ;;
  ;; Shift right by 4 bits to move the relevant bits to the junior half of the byte
  ;;       0000 0110
  ;;
  ;; Perform a call_indirect using 0000 0110 as the function index, thus invoking function $hex6
  ;; Function $hex6 returns the ASCII value for "6", that is, integer 54
  ;;
  ;; Similarly, to access a byte's lower nybble, mask out the upper nybble by AND'ing it with 0x0F
  ;;       6    C
  ;;       0110 1100
  ;;   AND 0000 1111
  ;;    -> 0000 1100
  ;;
  ;; No shift right is required here because the necessary bits are already in the junior half of the byte
  ;;
  ;; Perform a call_indirect using 0000 1100 as the function index, thus invoking function $hexC
  ;; Function $hexC returns the ASCII value for "c", that is, integer 99
  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  (func $upper-nybble-to-char (type 4) (param i32) (result i32)
    (call_indirect (type 3)
      (i32.shr_u
        (i32.and
          (i32.load8_u (get_local 0))
          (call $hex-mask-upper)
        )
        (i32.const 4)
      )
    )
  )

  (func $lower-nybble-to-char (type 4) (param i32) (result i32)
    (call_indirect (type 3)
      (i32.and
        (i32.load8_u (get_local 0))
        (call $hex-mask-lower)
      )
    )
  )

  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  ;; Declare a function table for hex to char conversion
  ;; The offset of each function in the table corresponds to the hex value that needs to be converted
  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  (table 16 funcref)
  (elem (i32.const 0)
        $hex0 $hex1 $hex2 $hex3 $hex4 $hex5 $hex6 $hex7 $hex8 $hex9 $hexA $hexB $hexC $hexD $hexE $hexF
  )

  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  ;; Function table functions
  ;; Each function returns the ASCII character corresponding to its table index
  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  (func $hex0 (type 3) (result i32) i32.const 48)             ;; ASCII "0"
  (func $hex1 (type 3) (result i32) i32.const 49)             ;; ASCII "1"
  (func $hex2 (type 3) (result i32) i32.const 50)             ;; ASCII "2"
  (func $hex3 (type 3) (result i32) i32.const 51)             ;; ASCII "3"
  (func $hex4 (type 3) (result i32) i32.const 52)             ;; ASCII "4"
  (func $hex5 (type 3) (result i32) i32.const 53)             ;; ASCII "5"
  (func $hex6 (type 3) (result i32) i32.const 54)             ;; ASCII "6"
  (func $hex7 (type 3) (result i32) i32.const 55)             ;; ASCII "7"
  (func $hex8 (type 3) (result i32) i32.const 56)             ;; ASCII "8"
  (func $hex9 (type 3) (result i32) i32.const 57)             ;; ASCII "9"
  (func $hexA (type 3) (result i32) i32.const 97)             ;; ASCII "a"
  (func $hexB (type 3) (result i32) i32.const 98)             ;; ASCII "b"
  (func $hexC (type 3) (result i32) i32.const 99)             ;; ASCII "c"
  (func $hexD (type 3) (result i32) i32.const 100)            ;; ASCII "d"
  (func $hexE (type 3) (result i32) i32.const 101)            ;; ASCII "e"
  (func $hexF (type 3) (result i32) i32.const 102)            ;; ASCII "f"

  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  ;; Convert a numeric binary value to an ASCII hex string.
  ;; This function assumes that the numeric binary data is stored in litte-endian format
  ;; Input        : [ 0 : i32        Offset to start of binary data
  ;;                , 1    : i32        Length of binary data
  ;;                , 2 : i32        Location at which the resulting character string will be written
  ;;                ]
  ;; Output       : [ 3 : i32 ]         Length of generated character string
  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  (func $bin-to-hex-str (type 5)
        (param i32)
        (param i32)
        (param i32)
        (result i32)

    (local i32)
    (local i32)

    (block
      ;; Calculate offset of lowest order byte of binary data
      ;; 3 = 0 + $bin_len - 1
      (set_local 3 (i32.add (get_local 0) (call $decr (get_local 1))))

      ;; Initialise loop counter
      (set_local 4 (get_local 1))

      (loop
        ;; Terminate the loop if the counter has reached zero
        (br_if 1 (i32.eq (get_local 4) (i32.const 0)))

        ;; Transform the upper nybble of the current byte into text format
        ;; Write the resulting ASCII character to the offset held in 2
        (i32.store8 (get_local 2) (call $upper-nybble-to-char (get_local 3)))
        (set_local 2 (call $incr (get_local 2)))

        ;; Now transform the lower nybble...
        (i32.store8 (get_local 2) (call $lower-nybble-to-char (get_local 3)))
        (set_local 2  (call $incr (get_local 2)))

        ;; Update loop variables
        (set_local 3 (call $decr (get_local 3)))
        (set_local 4       (call $decr (get_local 4)))

        ;; Restart loop
        (br 0)
      )
    )

    ;; Return the length of the generated character string
    (i32.mul (get_local 1) (i32.const 2))
  )

  ;; *******************************************************************************************************************
  ;; Public API functions
  ;; *******************************************************************************************************************

  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  ;; What's the time Mr WASI?
  ;; Input        : []
  ;; Output       : []
  ;; Side-effects : Writes 17 bytes of ASCII time data to standard out
  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  (func $_start (type 2)
    (local i32)
    (local i32)
    (local i32)
    (local i32)

    (set_local 0 (i32.const 8))
    (set_local 1 (i32.const 8))
    (set_local 2 (i32.const 20))

    (call $wasi_unstable.clock_time_get
      (i32.const 0)           ;; Clock id 0 = Realtime clock
      (i64.const 1)           ;; Precision
      (get_local 0) ;; Offset of returned data
    )
    drop
    
    ;; Convert binary data to an ASCII hex string
    (set_local 3 (call $bin-to-hex-str (get_local 0)
                                              (get_local 1)
                                              (get_local 2)
                        )
    )

    ;; Store a terminating line feed at the end of the text string (2 + 3)
    (i32.store (i32.add (get_local 2) (get_local 3)) (call $line-feed))

    ;; Generated character string is now one byte longer
    (set_local 3 (i32.add (get_local 3) (i32.const 1)))

    ;; Store offset of string data at offset 0
    (i32.store (i32.const 0) (get_local 2))

    ;; Store length of string data (3) at offset 4
    (i32.store (i32.const 4) (get_local 3))

    ;; Write string time value to standard out
    (call $wasi_unstable.fd_write
      (i32.const 1)      ;; fd 1 = standard out
      (i32.const 0)      ;; Location of offset/length data to be written
      (i32.const 1)      ;; Number of strings to write
      (i32.const 100)    ;; Location at which the number of bytes written will be stored (not that we care...)
    )
    drop                 ;; This function returns [], so we must discard the remaining value from the top of the stack
  )

  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  ;; Declare the use of one 64Kb memory page and export it using the name "memory"
  ;; If you want to call this WASM module from a JavaScript interface such as Wasmer-js, then this interface expects to
  ;; be able to access WASM memory using exactly the name "memory"
  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  (memory (export "memory") 1)

  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  ;; Export functions for public API
  ;; - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  (export "_start"          (func $_start))
)