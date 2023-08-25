use reqwest::blocking::Client;

use std::env;
use std::io;

use oauth2::pkce::PkceToken;
use oauth2::util;

fn build_auth_url(
    auth_url: &str,
    redirect_uri: &str,
    client_id: &str,
    pkce_token: &PkceToken,
    state: &str,
) -> String {
    format!(
        "{}?{}&{}&{}&{}&{}&{}&{}",
        auth_url,
        "response_type=code",
        "scope=photos",
        format!("client_id={}", client_id),
        format!("state={}", state),
        format!("redirect_uri={}", redirect_uri),
        format!("code_challenge={}", pkce_token.code_challenge),
        "code_challenge_method=S256"
    )
}

fn build_code_redeem_body(
    redirect_uri: &str,
    client_id: &str,
    client_secret: &str,
    pkce_token: &PkceToken,
    auth_code: &str,
) -> String {
    format!(
        "{{ {}, {}, {}, {}, {}, {}, {} }}",
        "\"grant_type\": \"authorization_code\"",
        format!(" \"redirect_uri\": \"{}\"", redirect_uri),
        format!(" \"client_id\": \"{}\"", client_id),
        format!(" \"client_secret\": \"{}\"", client_secret),
        format!(" \"redirect_uri\": \"{}\"", redirect_uri),
        format!(" \"code_verifier\": \"{}\"", pkce_token.code_verifier),
        format!(" \"code\": \"{}\"", auth_code)
    )
}

fn main() {
    let client = Client::new();
    let auth_endpoint = env::var("OAUTH_AUTH_ENDPOINT").unwrap();
    let client_id = env::var("PUBLIC_CLIENT_KEY").unwrap();
    let pkce_token = PkceToken::new();
    let state = util::generate_random_string(10);
    let redirect_uri = "https://example-app.com/redirect";
    let auth_url = build_auth_url(
        &auth_endpoint,
        redirect_uri,
        &client_id,
        &pkce_token,
        &state,
    );

    println!(
        "Client running.\nAuthorization url: {}\nState: {}\nCode verifier: {}",
        auth_url, state, pkce_token.code_verifier
    );

    println!("Enter code: ");
    let mut s = String::new();
    let auth_code = match io::stdin().read_line(&mut s) {
        Ok(_) => s.trim(),
        _ => panic!("Could not read code."),
    };

    println!("Code: {}", auth_code);

    let token_endpoint = env::var("OAUTH_TOKEN_ENDPOINT").unwrap();
    let client_secret = env::var("SECRET_CLIENT_KEY").unwrap();
    let token_body = build_code_redeem_body(
        redirect_uri,
        &client_id,
        &client_secret,
        &pkce_token,
        auth_code,
    );
    println!("Token body: {}", token_body);

    let response = client.post(token_endpoint).body(token_body).send().unwrap();
    println!("Response: {:?}", response);
    println!("Response text: {}", response.text().unwrap());
}
