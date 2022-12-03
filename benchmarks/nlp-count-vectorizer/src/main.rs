use vtext::tokenize::{VTextTokenizerParams,Tokenizer};
use wasm_serverless_invoke::wasm_handler::WasmHandler;
use wasm_serverless_invoke::wasm_handler::vectorvisor_barrier;
use wasm_serverless_invoke::wasm_handler::SerializationFormat::Json;
use serde::{Deserialize, Serialize};
use stop_words;
use std::collections::HashSet;
use lazy_static::lazy_static;

lazy_static! {
    static ref WORD_SET: HashSet<String> = stop_words::get(stop_words::LANGUAGE::English).into_iter().collect(); 
}

#[derive(Debug, Deserialize)]
struct FuncInput {
    tweets: Vec<String>
}

#[derive(Debug, Serialize)]
struct FuncResponse {
    tokenized: Vec<Vec<String>>,
    hashtags: Vec<Vec<String>>
}

#[inline(never)]
fn remove_stop_words(tweets: &mut Vec<Vec<String>>, word_set: HashSet<String>) -> () {
    for mut v in &mut tweets.into_iter() {
        v.retain(|word| !word_set.contains(word));
    }
}

#[inline(never)]
fn extract_hashtags(tweets: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut hashtag_tweets = vec![];
    for tweet_idx in 0..tweets.len() {
        let mut hashtags = vec![];
        for word_idx in 0..tweets[tweet_idx].len() {
            if tweets[tweet_idx][word_idx] == "#" && tweets[tweet_idx][word_idx+1] != "" {
                hashtags.push(tweets[tweet_idx][word_idx+1].clone());
            }
        }
        hashtag_tweets.push(hashtags);
    }

    hashtag_tweets
}

#[inline(never)]
fn tokenize(inputs: Vec<String>, tok: &dyn Tokenizer) -> Vec<Vec<String>> {
    let mut results = vec![];
    for tweet in inputs {
        let mut str_vec: Vec<String> = vec![];
        for token in tok.tokenize(&tweet) {
            str_vec.push(token.to_string());
        }
        results.push(str_vec);
    }
    results
}

#[inline(never)]
fn tokenize_inputs(event: FuncInput) -> FuncResponse {

    //unsafe { vectorvisor_barrier() };

    let tok = VTextTokenizerParams::default().lang("en").build().unwrap();
    let mut tweets = tokenize(event.tweets, &tok);

    //unsafe { vectorvisor_barrier() };

    let word_set: HashSet<String> = WORD_SET.clone();
    
    remove_stop_words(&mut tweets, word_set);

    //unsafe { vectorvisor_barrier() };

    let hashtags = extract_hashtags(&tweets);

    //unsafe { vectorvisor_barrier() };

    FuncResponse { tokenized: tweets, hashtags: hashtags }
}

fn main() {
    let handler = WasmHandler::new(&tokenize_inputs);
    handler.run_with_format(1024*512, Json);
}
