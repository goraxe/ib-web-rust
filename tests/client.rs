use std::env;

use ib_web_rust::apis::{
    authorization_token_api::generate_token, configuration::Configuration,
    trading_portfolio_api::portfolio_accounts_get, trading_session_api::tickle_post,
};

use tokio_test::{assert_err, assert_ok};

#[tokio::test]
async fn account_generate_token() {
    let configuration = Configuration::new();

    let token = generate_token(&configuration, "client_credentials", "", "", None).await;
    println!("{:?}", token);
    assert_ok!(token);
}

#[tokio::test]
async fn account_generate_token_basic_auth() {
    let username = match env::var("IB_USERNAME") {
        Ok(val) => val,
        Err(_) => {
            panic!("IB_USERNAME not set");
        }
    };

    let password = match env::var("IB_PASSWORD") {
        Ok(val) => val,
        Err(_) => {
            panic!("IB_PASSWORD not set");
        }
    };
    let configuration = Configuration {
        basic_auth: Some((username, Some(password.to_string()))),
        ..Configuration::default()
    };

    let token = generate_token(&configuration, "client_credentials", "", "", None).await;
    println!("{:?}", token);
    assert_ok!(token);
}

#[tokio::test]
async fn portfolio_accounts() {
    let username = match env::var("IB_USERNAME") {
        Ok(val) => val,
        Err(_) => {
            panic!("IB_USERNAME not set");
        }
    };

    let password = match env::var("IB_PASSWORD") {
        Ok(val) => val,
        Err(_) => {
            panic!("IB_PASSWORD not set");
        }
    };
    let configuration = Configuration {
        basic_auth: Some((username, Some(password.to_string()))),
        ..Configuration::default()
    };
    let response = portfolio_accounts_get(&configuration).await;
    println!("{:?}", response);
    assert_ok!(response);
}

#[tokio::test]
async fn trading_tickle_no_auth() {
    let configuration = Configuration::new();
    let response = tickle_post(&configuration).await;
    assert_err!(response);
}

#[tokio::test]
async fn trading_tickle_basic_auth() {
    let username = match env::var("IB_USERNAME") {
        Ok(val) => val,
        Err(_) => {
            panic!("IB_USERNAME not set");
        }
    };

    let password = match env::var("IB_PASSWORD") {
        Ok(val) => val,
        Err(_) => {
            panic!("IB_PASSWORD not set");
        }
    };
    let configuration = Configuration {
        basic_auth: Some((username, Some(password.to_string()))),
        ..Configuration::default()
    };
    let response = tickle_post(&configuration).await;
    assert_ok!(response);
}
