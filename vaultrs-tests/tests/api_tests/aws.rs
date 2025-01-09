use crate::common::Test;
use tracing::debug;
use vaultrs::client::Client;
use vaultrs::error::ClientError;
use vaultrs::sys::{auth, mount};

#[tokio::test]
#[ignore]
async fn test_auth() {
    let test = Test::builder().with_localstack(["iam", "sts"]).await;
    let client = test.client();
    let endpoint = setup_auth_engine(client).await.unwrap();

    config::client::test_set(test.localstack_url().unwrap(), client, &endpoint).await;
    config::client::test_read(client, &endpoint).await;
    config::client::test_delete(client, &endpoint).await;

    config::identity::test_set(client, &endpoint).await;
    config::identity::test_read(client, &endpoint).await;

    config::certificate::test_create(client, &endpoint).await;
    config::certificate::test_read(client, &endpoint).await;
    config::certificate::test_list(client, &endpoint).await;
    config::certificate::test_delete(client, &endpoint).await;

    config::sts::test_create(client, &endpoint).await;
    config::sts::test_read(client, &endpoint).await;
    config::sts::test_list(client, &endpoint).await;
    config::sts::test_delete(client, &endpoint).await;

    config::tidy::identity_access_list::test_set(client, &endpoint).await;
    config::tidy::identity_access_list::test_read(client, &endpoint).await;
    config::tidy::identity_access_list::test_delete(client, &endpoint).await;

    config::tidy::role_tag_deny_list::test_set(client, &endpoint).await;
    config::tidy::role_tag_deny_list::test_read(client, &endpoint).await;
    config::tidy::role_tag_deny_list::test_delete(client, &endpoint).await;

    role::test_create_iam(client, &endpoint).await;
    role::test_create_ec2(client, &endpoint).await;
    role::test_read(client, &endpoint).await;
    role::test_list(client, &endpoint).await;

    let role_tag = role::test_create_tag(client, &endpoint).await;
    role_tag_deny_list::test_create(client, &endpoint, &role_tag).await;
    role_tag_deny_list::test_read(client, &endpoint, &role_tag).await;
    role_tag_deny_list::test_list(client, &endpoint, &role_tag).await;
    role_tag_deny_list::test_tidy(client, &endpoint).await;
    role_tag_deny_list::test_delete(client, &endpoint, &role_tag).await;

    // role is needed for role_tag_deny_list operations
    role::test_delete(client, &endpoint).await;

    identity_access_list::test_list(client, &endpoint).await;
    identity_access_list::test_tidy(client, &endpoint).await;
}

#[tokio::test]
#[ignore]
async fn test_secret_engine() {
    let test = Test::builder().with_localstack(["iam", "sts"]).await;

    let client = test.client();
    let endpoint = setup_secret_engine(client).await.unwrap();

    secretengine::config::test_set(test.localstack_url().unwrap(), client, &endpoint).await;
    secretengine::config::test_get(client, &endpoint).await;
    secretengine::config::test_rotate(client, &endpoint).await;
    secretengine::config::test_set_lease(client, &endpoint).await;
    secretengine::config::test_read_lease(client, &endpoint).await;

    secretengine::roles::test_create_update(client, &endpoint).await;
    secretengine::roles::test_read(client, &endpoint).await;
    secretengine::roles::test_list(client, &endpoint).await;
    secretengine::roles::test_credentials(client, &endpoint).await;
    secretengine::roles::test_credentials_sts(client, &endpoint).await;
    secretengine::roles::test_delete(client, &endpoint).await;
}

#[derive(Debug)]
pub struct AwsAuthEndpoint {
    pub path: String,
}

pub struct AwsSecretEngineEndpoint {
    pub path: String,
}

async fn setup_auth_engine(client: &impl Client) -> Result<AwsAuthEndpoint, ClientError> {
    debug!("setting up AWS auth engine");

    let path = "aws_test";

    // Mount the AppRole auth engine
    auth::enable(client, path, "aws", None).await.unwrap();

    // configure aws client
    Ok(AwsAuthEndpoint {
        path: path.to_string(),
    })
}

async fn setup_secret_engine(client: &impl Client) -> Result<AwsSecretEngineEndpoint, ClientError> {
    debug!("setting up AWS secret engine");

    let path = "aws_test";
    mount::enable(client, path, "aws", None).await.unwrap();
    Ok(AwsSecretEngineEndpoint {
        path: path.to_string(),
    })
}

mod config {
    pub mod client {
        use vaultrs::{api::auth::aws::requests::ConfigureClientRequest, auth::aws};

        use super::super::{AwsAuthEndpoint, Client};

        pub async fn test_set(
            localstack_url: &str,
            client: &impl Client,
            endpoint: &AwsAuthEndpoint,
        ) {
            aws::config::client::set(
                client,
                &endpoint.path,
                Some(
                    &mut ConfigureClientRequest::builder()
                        .access_key("test")
                        .secret_key("test")
                        .sts_region("local")
                        .endpoint(localstack_url)
                        .sts_endpoint(localstack_url)
                        .iam_endpoint(localstack_url),
                ),
            )
            .await
            .unwrap();
        }

        pub async fn test_read(client: &impl Client, endpoint: &AwsAuthEndpoint) {
            let config = aws::config::client::read(client, &endpoint.path)
                .await
                .unwrap();
            assert_eq!(config.access_key, Some("test".to_string()));
        }

        pub async fn test_delete(client: &impl Client, endpoint: &AwsAuthEndpoint) {
            aws::config::client::delete(client, &endpoint.path)
                .await
                .unwrap();
        }
    }

    pub mod identity {
        use vaultrs::{api::auth::aws::requests::ConfigureIdentityRequest, auth::aws};

        use super::super::{AwsAuthEndpoint, Client};

        pub async fn test_set(client: &impl Client, endpoint: &AwsAuthEndpoint) {
            aws::config::identity::set(
                client,
                &endpoint.path,
                Some(
                    &mut ConfigureIdentityRequest::builder()
                        .iam_alias("unique_id")
                        .ec2_alias("instance_id"),
                ),
            )
            .await
            .unwrap();
        }

        pub async fn test_read(client: &impl Client, endpoint: &AwsAuthEndpoint) {
            let identity = aws::config::identity::read(client, &endpoint.path)
                .await
                .unwrap();

            assert_eq!(identity.iam_alias, Some("unique_id".to_string()));
            assert_eq!(identity.ec2_alias, Some("instance_id".to_string()));
        }
    }

    pub mod certificate {
        use base64::{engine::general_purpose, Engine as _};
        use vaultrs::auth::aws;

        use super::super::{AwsAuthEndpoint, Client};

        const CERT_NAME: &str = "test_cert";
        const CERT: &str = include_str!("../files/aws.crt");

        pub async fn test_create(client: &impl Client, endpoint: &AwsAuthEndpoint) {
            aws::config::certificate::create(
                client,
                &endpoint.path,
                CERT_NAME,
                &general_purpose::STANDARD.encode(CERT),
                None,
            )
            .await
            .unwrap();
        }

        pub async fn test_read(client: &impl Client, endpoint: &AwsAuthEndpoint) {
            let certificate = aws::config::certificate::read(client, &endpoint.path, CERT_NAME)
                .await
                .unwrap();
            assert_eq!(certificate.aws_public_cert, CERT)
        }

        pub async fn test_delete(client: &impl Client, endpoint: &AwsAuthEndpoint) {
            aws::config::certificate::delete(client, &endpoint.path, CERT_NAME)
                .await
                .unwrap();
        }

        pub async fn test_list(client: &impl Client, endpoint: &AwsAuthEndpoint) {
            let certificate = aws::config::certificate::list(client, &endpoint.path)
                .await
                .unwrap();
            assert_eq!(certificate.keys, vec![CERT_NAME])
        }
    }

    pub mod sts {
        use vaultrs::auth::aws;

        use super::super::{AwsAuthEndpoint, Client};

        const SATELLITE_ACCOUNT_ID: &str = "000000000001";
        const ROLE_NAME: &str = "SomeRole";

        pub async fn test_create(client: &impl Client, endpoint: &AwsAuthEndpoint) {
            aws::config::sts::create(client, &endpoint.path, SATELLITE_ACCOUNT_ID, ROLE_NAME)
                .await
                .unwrap();
        }

        pub async fn test_read(client: &impl Client, endpoint: &AwsAuthEndpoint) {
            let sts = aws::config::sts::read(client, &endpoint.path, SATELLITE_ACCOUNT_ID)
                .await
                .unwrap();
            assert!(sts.sts_role.ends_with(ROLE_NAME));
        }

        pub async fn test_list(client: &impl Client, endpoint: &AwsAuthEndpoint) {
            let sts = aws::config::sts::list(client, &endpoint.path)
                .await
                .unwrap();
            assert_eq!(sts.keys, [SATELLITE_ACCOUNT_ID]);
        }

        pub async fn test_delete(client: &impl Client, endpoint: &AwsAuthEndpoint) {
            aws::config::sts::delete(client, &endpoint.path, SATELLITE_ACCOUNT_ID)
                .await
                .unwrap();
        }
    }

    pub mod tidy {
        pub mod identity_access_list {
            use vaultrs::{
                api::auth::aws::requests::ConfigureIdentityAccessListTidyOperationRequest,
                auth::aws,
            };

            use super::super::super::{AwsAuthEndpoint, Client};

            pub async fn test_set(client: &impl Client, endpoint: &AwsAuthEndpoint) {
                aws::config::tidy::identity_access_list::set(
                    client,
                    &endpoint.path,
                    Some(
                        &mut ConfigureIdentityAccessListTidyOperationRequest::builder()
                            .safety_buffer("24h")
                            .disable_periodic_tidy(true),
                    ),
                )
                .await
                .unwrap();
            }
            pub async fn test_read(client: &impl Client, endpoint: &AwsAuthEndpoint) {
                let iacl = aws::config::tidy::identity_access_list::read(client, &endpoint.path)
                    .await
                    .unwrap();

                assert_eq!(iacl.safety_buffer, 86400);
                assert!(iacl.disable_periodic_tidy);
            }
            pub async fn test_delete(client: &impl Client, endpoint: &AwsAuthEndpoint) {
                aws::config::tidy::identity_access_list::read(client, &endpoint.path)
                    .await
                    .unwrap();
            }
        }

        pub mod role_tag_deny_list {
            use vaultrs::{
                api::auth::aws::requests::ConfigureRoleTagDenyListTidyOperationRequest, auth::aws,
            };

            use super::super::super::{AwsAuthEndpoint, Client};

            pub async fn test_set(client: &impl Client, endpoint: &AwsAuthEndpoint) {
                aws::config::tidy::role_tag_deny_list::set(
                    client,
                    &endpoint.path,
                    Some(
                        &mut ConfigureRoleTagDenyListTidyOperationRequest::builder()
                            .safety_buffer("24h"),
                    ),
                )
                .await
                .unwrap();
            }

            pub async fn test_read(client: &impl Client, endpoint: &AwsAuthEndpoint) {
                let denied_tags =
                    aws::config::tidy::role_tag_deny_list::read(client, &endpoint.path)
                        .await
                        .unwrap();
                assert_eq!(denied_tags.safety_buffer, 86400);
                assert!(!denied_tags.disable_periodic_tidy);
            }

            pub async fn test_delete(client: &impl Client, endpoint: &AwsAuthEndpoint) {
                aws::config::tidy::role_tag_deny_list::read(client, &endpoint.path)
                    .await
                    .unwrap();
            }
        }
    }
}

mod role {
    use vaultrs::{
        api::auth::aws::{
            requests::{CreateRoleRequest, CreateRoleTagRequest},
            responses::CreateRoleTagResponse,
        },
        auth::aws,
    };

    use super::{AwsAuthEndpoint, Client};

    const ROLE_NAME_IAM: &str = "test_role_iam";
    const ROLE_NAME_EC2: &str = "test_role_ec2";

    pub async fn test_create_iam(client: &impl Client, endpoint: &AwsAuthEndpoint) {
        aws::role::create(
            client,
            &endpoint.path,
            ROLE_NAME_IAM,
            Some(
                &mut CreateRoleRequest::builder()
                    .auth_type("iam")
                    .bound_iam_principal_arn(["000000000001".to_string()])
                    .resolve_aws_unique_ids(false),
            ),
        )
        .await
        .unwrap();
    }

    pub async fn test_create_ec2(client: &impl Client, endpoint: &AwsAuthEndpoint) {
        aws::role::create(
            client,
            &endpoint.path,
            ROLE_NAME_EC2,
            Some(
                &mut CreateRoleRequest::builder()
                    .auth_type("ec2")
                    .role_tag("Testing")
                    .bound_ec2_instance_id(["i-1234567890abcdef0".to_string()]),
            ),
        )
        .await
        .unwrap();
    }

    pub async fn test_read(client: &impl Client, endpoint: &AwsAuthEndpoint) {
        let role = aws::role::read(client, &endpoint.path, ROLE_NAME_IAM)
            .await
            .unwrap();
        assert_eq!(
            role.bound_iam_principal_arn.unwrap_or_default(),
            ["000000000001"]
        );
    }

    pub async fn test_list(client: &impl Client, endpoint: &AwsAuthEndpoint) {
        let roles = aws::role::list(client, &endpoint.path).await.unwrap();

        assert!(roles.keys.contains(&ROLE_NAME_IAM.to_string()));
        assert!(roles.keys.contains(&ROLE_NAME_EC2.to_string()));
    }

    pub async fn test_delete(client: &impl Client, endpoint: &AwsAuthEndpoint) {
        aws::role::delete(client, &endpoint.path, ROLE_NAME_IAM)
            .await
            .unwrap();

        aws::role::delete(client, &endpoint.path, ROLE_NAME_EC2)
            .await
            .unwrap();
    }

    pub async fn test_create_tag(
        client: &impl Client,
        endpoint: &AwsAuthEndpoint,
    ) -> CreateRoleTagResponse {
        // role_tag is only used with ec2 auth method
        aws::role::create_tag(
            client,
            &endpoint.path,
            ROLE_NAME_EC2,
            Some(&mut CreateRoleTagRequest::builder().max_ttl("48h")),
        )
        .await
        .unwrap()
    }
}

mod identity_access_list {
    use vaultrs::{api::auth::aws::requests::TidyIdentityAccessListEntriesRequest, auth::aws};

    use super::{AwsAuthEndpoint, Client, ClientError};

    pub async fn test_list(client: &impl Client, endpoint: &AwsAuthEndpoint) {
        let res = aws::identity_access_list::list(client, &endpoint.path).await;
        assert!(match res {
            // vault returns 404 instead of empty list
            // <https://github.com/hashicorp/vault/issues/1365>
            Err(ClientError::APIError { code, errors: _ }) => code == 404,
            _ => false,
        })
    }

    pub async fn test_tidy(client: &impl Client, endpoint: &AwsAuthEndpoint) {
        aws::identity_access_list::tidy(
            client,
            &endpoint.path,
            Some(&mut TidyIdentityAccessListEntriesRequest::builder().safety_buffer("12h")),
        )
        .await
        .unwrap();
    }
}

mod role_tag_deny_list {
    use vaultrs::{
        api::auth::aws::{requests::TidyDenyListTagsRequest, responses::CreateRoleTagResponse},
        auth::aws,
    };

    use super::{AwsAuthEndpoint, Client};

    pub async fn test_create(
        client: &impl Client,
        endpoint: &AwsAuthEndpoint,
        role_tag: &CreateRoleTagResponse,
    ) {
        aws::role_tag_deny_list::create(client, &endpoint.path, &role_tag.tag_value)
            .await
            .unwrap();
    }

    pub async fn test_read(
        client: &impl Client,
        endpoint: &AwsAuthEndpoint,
        role_tag: &CreateRoleTagResponse,
    ) {
        aws::role_tag_deny_list::read(client, &endpoint.path, &role_tag.tag_value)
            .await
            .unwrap();
    }

    pub async fn test_list(
        client: &impl Client,
        endpoint: &AwsAuthEndpoint,
        role_tag: &CreateRoleTagResponse,
    ) {
        let denied_tags = aws::role_tag_deny_list::list(client, &endpoint.path)
            .await
            .unwrap();
        assert!(denied_tags.keys.contains(&role_tag.tag_value));
    }

    pub async fn test_delete(
        client: &impl Client,
        endpoint: &AwsAuthEndpoint,
        role_tag: &CreateRoleTagResponse,
    ) {
        aws::role_tag_deny_list::delete(client, &endpoint.path, &role_tag.tag_value)
            .await
            .unwrap();
    }

    pub async fn test_tidy(client: &impl Client, endpoint: &AwsAuthEndpoint) {
        aws::role_tag_deny_list::tidy(
            client,
            &endpoint.path,
            Some(&mut TidyDenyListTagsRequest::builder().safety_buffer("8h")),
        )
        .await
        .unwrap();
    }
}

pub mod secretengine {

    pub mod config {
        use vaultrs::{api::aws::requests::SetConfigurationRequest, aws};

        use super::super::{AwsSecretEngineEndpoint, Client};

        pub async fn test_set(
            localstack_url: &str,
            client: &impl Client,
            endpoint: &AwsSecretEngineEndpoint,
        ) {
            aws::config::set(
                client,
                &endpoint.path,
                "test",
                "test",
                Some(
                    SetConfigurationRequest::builder()
                        .max_retries(3)
                        .region("eu-central-1")
                        .sts_endpoint(localstack_url)
                        .iam_endpoint(localstack_url),
                ),
            )
            .await
            .unwrap();
        }

        pub async fn test_get(client: &impl Client, endpoint: &AwsSecretEngineEndpoint) {
            let config = aws::config::get(client, &endpoint.path).await.unwrap();

            assert!(config.access_key == "test");
            assert!(config.max_retries == 3);
            assert!(config.region == "eu-central-1");
        }

        // Doesn't work with Localstack, probably because of limitation with IAM APIs implementation
        // and Vault method of rotating keys
        // let's keep the call at least to avoid obvious errors, but it will return 500 with Vault + Localstack
        pub async fn test_rotate(client: &impl Client, endpoint: &AwsSecretEngineEndpoint) {
            let _res = aws::config::rotate(client, &endpoint.path).await;

            // assert!(res.is_ok());
            // assert!(res.unwrap().access_key.starts_with("AKIA"));
        }

        pub async fn test_set_lease(client: &impl Client, endpoint: &AwsSecretEngineEndpoint) {
            aws::config::set_lease(client, &endpoint.path, "1h", "6h")
                .await
                .unwrap();
        }

        pub async fn test_read_lease(client: &impl Client, endpoint: &AwsSecretEngineEndpoint) {
            let lease = aws::config::read_lease(client, &endpoint.path)
                .await
                .unwrap();

            // response looks like "1h0m0s"
            assert!(lease.lease.starts_with("1h"));
            assert!(lease.lease_max.starts_with("6h"));
        }
    }

    pub mod roles {
        use vaultrs::{
            api::aws::requests::{
                CreateUpdateRoleRequest, GenerateCredentialsRequest, GenerateCredentialsStsRequest,
            },
            aws,
        };

        use super::super::{AwsSecretEngineEndpoint, Client};

        pub const TEST_ROLE: &str = "test_role";
        pub const TEST_ARN: &str = "arn:aws:iam::123456789012:role/test_role";

        pub async fn test_create_update(client: &impl Client, endpoint: &AwsSecretEngineEndpoint) {
            aws::roles::create_update(
                client,
                &endpoint.path,
                TEST_ROLE,
                "assumed_role",
                Some(CreateUpdateRoleRequest::builder().role_arns(vec![TEST_ARN.to_string()])),
            )
            .await
            .unwrap();
        }

        pub async fn test_read(client: &impl Client, endpoint: &AwsSecretEngineEndpoint) {
            let data = aws::roles::read(client, &endpoint.path, TEST_ROLE)
                .await
                .unwrap();

            let roles = data.role_arns.unwrap();

            assert!(data.credential_type == "assumed_role");
            assert_eq!(roles, [TEST_ARN]);
        }

        pub async fn test_list(client: &impl Client, endpoint: &AwsSecretEngineEndpoint) {
            let roles = aws::roles::list(client, &endpoint.path).await.unwrap();
            assert_eq!(roles.keys, [TEST_ROLE]);
        }

        pub async fn test_credentials(client: &impl Client, endpoint: &AwsSecretEngineEndpoint) {
            let role = aws::roles::read(client, &endpoint.path, TEST_ROLE)
                .await
                .unwrap();
            dbg!(role);
            let data = aws::roles::credentials(
                client,
                &endpoint.path,
                TEST_ROLE,
                Some(
                    GenerateCredentialsRequest::builder()
                        .ttl("3h")
                        .role_arn(TEST_ARN.to_string()),
                ),
            )
            .await
            .unwrap();
            dbg!(&data);

            assert!(data.access_key.starts_with("LSIA"));
            assert!(!data.secret_key.is_empty());
            assert!(!data.security_token.unwrap().is_empty());
        }

        pub async fn test_credentials_sts(
            client: &impl Client,
            endpoint: &AwsSecretEngineEndpoint,
        ) {
            let roles = aws::roles::credentials_sts(
                client,
                &endpoint.path,
                TEST_ROLE,
                Some(GenerateCredentialsStsRequest::builder().ttl("3h")),
            )
            .await
            .unwrap();

            assert!(roles.access_key.starts_with("LSIA"));
            assert!(!roles.secret_key.is_empty());
            assert!(!roles.security_token.unwrap().is_empty());
        }

        pub async fn test_delete(client: &impl Client, endpoint: &AwsSecretEngineEndpoint) {
            aws::roles::delete(client, &endpoint.path, TEST_ROLE)
                .await
                .unwrap();

            // check deletion actually worked, list should be empty (Vault returns 404)
            let res_after = aws::roles::list(client, &endpoint.path).await;
            assert!(res_after.is_err());
        }
    }
}
