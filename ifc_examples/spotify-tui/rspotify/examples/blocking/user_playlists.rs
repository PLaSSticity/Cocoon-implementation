extern crate rspotify;

use rspotify::blocking::client::Spotify;
use rspotify::blocking::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::blocking::util::get_token;

fn main() {
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

    let mut spotify_oauth = SpotifyOAuth::default()
        .scope("playlist-read-private, playlist-read-collaborative")
        .build();
    match get_token(&mut spotify_oauth) {
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
            //this is my(samray's) user_id, so just change
            // user_id to yours, or you will get a 403 forbidden error
            let user_id = "2257tjys2e2u2ygfke42niy2q";
            let playlists = spotify.user_playlists(user_id, Some(10), None);
            println!("{:?}", playlists);
        }
        None => println!("auth failed"),
    };
}
