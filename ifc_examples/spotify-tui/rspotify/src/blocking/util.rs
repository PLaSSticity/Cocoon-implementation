//! utils function
use chrono::prelude::*;
use rand::distributions::Alphanumeric;
use rand::{self, Rng};
use webbrowser;

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::io;
use std::string::ToString;

use super::oauth2::{SpotifyOAuth, TokenInfo};

/// convert datetime to unix timestampe
pub fn datetime_to_timestamp(elapsed: u32) -> i64 {
    let utc: DateTime<Utc> = Utc::now();
    utc.timestamp() + i64::from(elapsed)
}
/// generate `length` random chars
pub fn generate_random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .collect()
}

/// convert map to `query_string`, for example:
/// convert
/// `{"redirect_uri":"my_uri",
///  "state":"my-state"
///  "scope":"test-scope"}`
/// to
/// `redirect_uri=my_uri&state=my-state&scope=test-scope`
/// Since hashmap is not sorted, so the order of key-value-pairs
/// may differ from times
pub fn convert_map_to_string<
    K: Debug + Eq + Hash + ToString,
    V: Debug + ToString,
    S: ::std::hash::BuildHasher,
>(
    map: &HashMap<K, V, S>,
) -> String {
    let mut string: String = String::new();
    for (key, value) in map.iter() {
        string.push_str(&key.to_string());
        string.push_str("=");
        string.push_str(&value.to_string());
        string.push_str("&");
    }
    string
}

/// convert query string to map, for example:
/// convert
/// `redirect_uri=my_uri&state=my-state&scope=test-scope`
/// to
/// `{"redirect_uri":"my_uri",
///  "state":"my-state"
///  "scope":"test-scope"}`
pub fn convert_str_to_map(query_str: &mut str) -> HashMap<&str, &str> {
    let mut map: HashMap<&str, &str> = HashMap::new();
    let tokens: Vec<&str> = query_str
        .split('&')
        .filter(|token| !token.is_empty())
        .collect();
    for token in tokens {
        let vec: Vec<&str> = token.split('=').collect();
        map.insert(vec[0], vec[1]);
    }
    map
}

pub fn request_token(spotify_oauth: &mut SpotifyOAuth) {
    let state = generate_random_string(16);
    let auth_url = spotify_oauth.get_authorize_url(Some(&state), None);
    match webbrowser::open(&auth_url) {
        Ok(_) => println!("Opened {} in your browser", auth_url),
        Err(why) => eprintln!("Error {:?};Please navigate here [{:?}] ", why, auth_url),
    }
}

pub fn process_token(spotify_oauth: &mut SpotifyOAuth, input: &mut String) -> Option<TokenInfo> {
    match spotify_oauth.parse_response_code(input) {
        Some(code) => spotify_oauth.get_access_token(&code),
        None => None,
    }
}

pub fn process_token_without_cache(
    spotify_oauth: &mut SpotifyOAuth,
    input: &mut String,
) -> Option<TokenInfo> {
    match spotify_oauth.parse_response_code(input) {
        Some(code) => spotify_oauth.get_access_token_without_cache(&code),
        None => None,
    }
}

/// get tokenInfo by Authorization
pub fn get_token(spotify_oauth: &mut SpotifyOAuth) -> Option<TokenInfo> {
    match spotify_oauth.get_cached_token() {
        Some(token_info) => Some(token_info),
        None => {
            request_token(spotify_oauth);
            println!("Enter the URL you were redirected to: ");
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => process_token(spotify_oauth, &mut input),
                Err(_) => None,
            }
        }
    }
}

/// get tokenInfo by Authorization without cache.
pub fn get_token_without_cache(spotify_oauth: &mut SpotifyOAuth) -> Option<TokenInfo> {
    request_token(spotify_oauth);
    println!("Enter the URL you were redirected to: ");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => process_token_without_cache(spotify_oauth, &mut input),
        Err(_) => None,
    }
}

/// get tokenInfo by authorization and code
pub fn get_token_by_code(spotify_oauth: &mut SpotifyOAuth, code: &str) -> Option<TokenInfo> {
    spotify_oauth.get_access_token(&code)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_covert_str_to_map() {
        let mut query_url = String::from("redirect_uri=my_uri&state=my-state&scope=test-scope&");
        let parameters = convert_str_to_map(&mut query_url);
        match parameters.get("redirect_uri") {
            Some(redirect_uri) => {
                assert_eq!(redirect_uri, &"my_uri");
                println!("{:?}", redirect_uri);
            }
            None => panic!("failed"),
        }
    }
    #[test]
    fn test_convert_map_to_string() {
        let mut map = HashMap::new();
        map.insert("redirect_uri", "my_uri");
        map.insert("state", "my-state");
        map.insert("scope", "test-scope");
        let result = convert_map_to_string(&map);
        // hashmap is not sorted, so the order of key-value-pairs will not
        // follow the insert order
        assert!(result.contains("redirect_uri=my_uri&"));
        assert!(result.contains("state=my-state&"));
        assert!(result.contains("scope=test-scope&"));
    }
}
