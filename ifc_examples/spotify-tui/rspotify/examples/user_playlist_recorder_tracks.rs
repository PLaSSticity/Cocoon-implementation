extern crate rspotify;

use rspotify::client::Spotify;
use rspotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::util::get_token;

#[tokio::main]
async fn main() {
    // Set client_id and client_secret in .env file or
    // export CLIENT_ID="your client_id"
    // export CLIENT_SECRET="secret"
    // export REDIRECT_URI=your-direct-uri

    // Or set client_id, client_secret,redirect_uri explictly
    // let oauth = SpotifyOAuth::default()
    //     .client_id("this-is-my-client-id")
    //     .client_secret("this-is-my-client-secret")
    //     .redirect_uri("http://localhost:8888/callback")
    //     .build();

    let mut oauth = SpotifyOAuth::default()
        .scope("playlist-modify-private playlist-modify-public")
        .build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            // Or set client_id and client_secret explictly
            // let client_credential = SpotifyClientCredentials::default()
            //     .client_id("this-is-my-client-id")
            //     .client_secret("this-is-my-client-secret")
            //     .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            //this is my(samray's) user_id and playlist_id, so just change
            // user_id and playlist_id to yours, or you will get a 403 forbidden error
            let user_id = "2257tjys2e2u2ygfke42niy2q";
            let playlist_id = "5jAOgWXCBKuinsGiZxjDQ5";
            let range_start = 0;
            let insert_before = 1;
            let range_length = 1;
            let result = spotify
                .user_playlist_recorder_tracks(
                    user_id,
                    playlist_id,
                    range_start,
                    range_length,
                    insert_before,
                    None,
                )
                .await;
            println!("result:{:?}", result);
        }
        None => println!("auth failed"),
    };
}
