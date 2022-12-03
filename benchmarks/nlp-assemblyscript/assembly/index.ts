// Import Console (for writing to stdout), and FileSystem (for reading/writing files)
// from "as-wasi". An API for working with WASI in AssemblyScript much easier.
import { Console, FileSystem, Descriptor } from "as-wasi/assembly";
import { JSON, JSONEncoder } from "assemblyscript-json/assembly"; 
import { listen, vectorvisor_barrier } from "./env";
import { stopWords, initSet, getSet } from "./stop";

function abort(message: usize, fileName: usize, line: u32, column: u32): void {
    Console.log("fname: " + fileName.toString());
    Console.log("message: " + message.toString());
    Console.log("line: " + line.toString());
    unreachable()
}

initSet();

let set: Set<string> = getSet();

/*
 * Func inputs:
 *      tweets: string[]
 */
function main(input: JSON.Obj): Uint8Array | null {
    let tweets: JSON.Arr | null = input.getArr("tweets");
    if (tweets != null) {

        //vectorvisor_barrier();

        let strTweets: string[] = tweets._arr.map<string>((val: JSON.Value): string => val.toString());

        // Split each tweet (tokenize)
        let tokenize: string[][] = strTweets.map<string[]>((val: string): string[] => val.split(" "));

        //vectorvisor_barrier();

        // Remove empty values and stop words
        let filtered: string[][] = tokenize.map<string[]>((arr: string[]): string[] =>
                                        arr.filter((word: string): bool => {
                                               if (set.has(word)) {
                                                   return false; 
                                               } else {
                                                   return true;
                                               }
                                           }));

        //vectorvisor_barrier();

        // Get the array of hashtags for each tweet
        let hashtags: string[][] = filtered.map<string[]>((tweet: string[]): string[] => 
                                      tweet.filter((word: string): bool => {
                                          if (word.charAt(0) == '#' && word.charAt(1) != "") {
                                              return true;
                                          } else {
                                              return false;
                                          }
                                      }));

        //vectorvisor_barrier();

        let encoder = new JSONEncoder();
        encoder.pushArray("tokenized");
        for (let tweet_idx = 0; tweet_idx < filtered.length; tweet_idx++) {
            let wordLen: i32 = filtered[tweet_idx].length;
            for (let word_idx = 0; word_idx < wordLen; word_idx++) {
                encoder.setString(null, filtered[tweet_idx][word_idx]);
            }
        }
        encoder.popArray();

        encoder.pushArray("hashtags");
        for (let tweet_idx = 0; tweet_idx < hashtags.length; tweet_idx++) {
            let wordLen: i32 = hashtags[tweet_idx].length;
            for (let word_idx = 0; word_idx < wordLen; word_idx++) {
                encoder.setString(null, hashtags[tweet_idx][word_idx]);
            }
        }
        encoder.popArray();

        let json: Uint8Array = encoder.serialize();
        return json;
    }

    // else we failed somehow...
    //unreachable()
    return null;
}


listen(1024*512, main);
