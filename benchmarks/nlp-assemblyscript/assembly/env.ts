// Declare vectorvisor imports

export declare function serverless_invoke(buf: i32, len: i32): i32
export declare function serverless_response(buf: i32, len: i32): void
export declare function vectorvisor_barrier(): void

import { JSON } from "assemblyscript-json/assembly"; 

// Create a wrapper function to invoke these for us

export function listen(buf_size: i32, fn: (input: JSON.Obj) => Uint8Array | null): void {
    let buf = __alloc(sizeof<u8>() * buf_size);
    while (true) {
        let len = serverless_invoke(buf as i32, buf_size);
        if (len > 0) {
            let input: string = String.UTF8.decodeUnsafe(buf, len);
            let jsonObj: JSON.Obj = <JSON.Obj>(JSON.parse(input));

            let resp = fn(jsonObj);

            if (resp != null) {
                serverless_response(changetype<i32>(resp), resp.byteLength)
            } else {
                serverless_response(0, 0);
            }
        } else {
            serverless_response(0, 0);
        }
    }
}
