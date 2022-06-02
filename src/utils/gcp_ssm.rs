use base64;
use google_secretmanager1::{
    oauth2::{
        authenticator::ApplicationDefaultCredentialsTypes, read_authorized_user_secret,
        ApplicationDefaultCredentialsAuthenticator, ApplicationDefaultCredentialsFlowOpts,
        AuthorizedUserAuthenticator,
    },
    Result, SecretManager,
};
use hyper::{client::HttpConnector, Client};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use std::{default::Default, path::Path};

fn create_ssl_client() -> Client<HttpsConnector<HttpConnector>> {
    Client::builder().build(
        HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .enable_http2()
            .build(),
    )
}

async fn get_application_default_credentials_authenticator(
) -> google_secretmanager1::oauth2::authenticator::Authenticator<HttpsConnector<HttpConnector>> {
    let opts = ApplicationDefaultCredentialsFlowOpts::default();

    match ApplicationDefaultCredentialsAuthenticator::with_client(opts, create_ssl_client()).await {
        ApplicationDefaultCredentialsTypes::InstanceMetadata(auth_builder) => auth_builder
            .hyper_client(create_ssl_client())
            // .persist_tokens_to_disk("./output/application_default_credentials.json")
            .build()
            .await
            .unwrap(),
        ApplicationDefaultCredentialsTypes::ServiceAccount(auth_builder) => auth_builder
            .hyper_client(create_ssl_client())
            // .persist_tokens_to_disk("./output/application_default_credentials.json")
            .build()
            .await
            .unwrap(),
    }
}

async fn get_authorized_user_authenticator(
) -> google_secretmanager1::oauth2::authenticator::Authenticator<HttpsConnector<HttpConnector>> {
    let home_dir = format!(
        "{}/.config/gcloud/application_default_credentials.json",
        env!("HOME")
    );
    let authorized_user_secret = read_authorized_user_secret(Path::new(&home_dir))
        .await
        .unwrap();

    AuthorizedUserAuthenticator::with_client(authorized_user_secret, create_ssl_client())
        // .persist_tokens_to_disk("./output/authorized_user_credentials.json")
        .build()
        .await
        .unwrap()
}

pub async fn get_secret_manager() -> Result<SecretManager> {
    let authenticator = get_authorized_user_authenticator().await;

    Ok(SecretManager::new(create_ssl_client(), authenticator))
}

pub async fn access_secret_version(
    secret_manager: SecretManager,
    project_id: &str,
    secret_name: &str,
    version: u16,
) -> Result<String> {
    let (_, secret_version) = secret_manager
        .projects()
        .secrets_versions_access(
            format!(
                "projects/{}/secrets/{}/versions/{}",
                project_id, secret_name, version
            )
            .as_str(),
        )
        .doit()
        .await?;

    Ok(
        String::from_utf8(base64::decode(secret_version.payload.unwrap().data.unwrap()).unwrap())
            .unwrap(),
    )
}
