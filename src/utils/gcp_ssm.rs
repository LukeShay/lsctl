use super::hyper_utils::create_ssl_client;
use anyhow;
use base64;
use google_secretmanager1::{
    oauth2::{
        authenticator::{ApplicationDefaultCredentialsTypes, Authenticator},
        read_authorized_user_secret, ApplicationDefaultCredentialsAuthenticator,
        ApplicationDefaultCredentialsFlowOpts, AuthorizedUserAuthenticator,
    },
    SecretManager,
};
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use std::{default::Default, env, path::Path};

async fn get_authenticator() -> Authenticator<HttpsConnector<HttpConnector>> {
    match read_authorized_user_secret(Path::new(
        format!(
            "{}/.config/gcloud/application_default_credentials.json",
            env::var("HOME").unwrap()
        )
        .as_str(),
    ))
    .await
    {
        Ok(authorized_user_secret) => {
            AuthorizedUserAuthenticator::with_client(authorized_user_secret, create_ssl_client())
                .build()
                .await
                .unwrap()
        }
        Err(_) => {
            let opts = ApplicationDefaultCredentialsFlowOpts::default();

            match ApplicationDefaultCredentialsAuthenticator::with_client(opts, create_ssl_client())
                .await
            {
                ApplicationDefaultCredentialsTypes::InstanceMetadata(auth_builder) => auth_builder
                    .hyper_client(create_ssl_client())
                    .build()
                    .await
                    .unwrap(),
                ApplicationDefaultCredentialsTypes::ServiceAccount(auth_builder) => auth_builder
                    .hyper_client(create_ssl_client())
                    .build()
                    .await
                    .unwrap(),
            }
        }
    }
}

pub async fn get_secret_manager() -> SecretManager {
    let authenticator = get_authenticator().await;

    SecretManager::new(create_ssl_client(), authenticator)
}

pub async fn access_secret_version(
    secret_manager: SecretManager,
    project_id: &str,
    secret_name: &str,
    version: u16,
) -> anyhow::Result<String> {
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

    anyhow::Ok(
        String::from_utf8(base64::decode(secret_version.payload.unwrap().data.unwrap()).unwrap())
            .unwrap(),
    )
}
