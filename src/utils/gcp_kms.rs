use super::hyper_utils::create_ssl_client;
use anyhow;
use base64;
use google_cloudkms1::{
    api::DecryptRequest,
    oauth2::{
        authenticator::{ApplicationDefaultCredentialsTypes, Authenticator},
        read_authorized_user_secret, ApplicationDefaultCredentialsAuthenticator,
        ApplicationDefaultCredentialsFlowOpts, AuthorizedUserAuthenticator,
    },
    CloudKMS,
};
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use std::{default::Default, path::Path, env};

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

pub async fn get_cloud_kms() -> CloudKMS {
    let authenticator = get_authenticator().await;

    CloudKMS::new(create_ssl_client(), authenticator)
}

pub async fn decrypt_ciphertext(
    cloud_kms: CloudKMS,
    project_id: &str,
    location: &str,
    key_ring: &str,
    key: &str,
    ciphertext: &str,
) -> anyhow::Result<String> {
    let mut request = DecryptRequest::default();
    request.ciphertext = Some(ciphertext.to_string());

    let result = cloud_kms
        .projects()
        .locations_key_rings_crypto_keys_decrypt(
            request,
            format!(
                "projects/{}/locations/{}/keyRings/{}/cryptoKeys/{}",
                project_id, location, key_ring, key
            )
            .as_str(),
        )
        .doit()
        .await;

    match result {
        Ok((_, secret_version)) => Ok(String::from_utf8(
            base64::decode(secret_version.plaintext.unwrap()).unwrap(),
        )
        .unwrap()),
        Err(err) => Err(anyhow::anyhow!(err)),
    }
}
