#[macro_use]
extern crate tracing;

mod common;

use common::{VaultServer, VaultServerHelper};
use dockertest_server::servers::cloud::localstack::LocalStackServer;
use test_log::test;
use vaultrs::client::Client;
use vaultrs::error::ClientError;

#[test]
fn test_auth() {
    let test = common::new_aws_test();

    test.run(|instance| async move {
        let server: VaultServer = instance.server();
        let localstack: LocalStackServer = instance.server();
        let client = server.client();
        let endpoint = setup_auth_engine(&server, &client).await.unwrap();

        crate::config::client::test_set(&localstack, &client, &endpoint).await;
        crate::config::client::test_read(&client, &endpoint).await;
        crate::config::client::test_delete(&client, &endpoint).await;

        crate::config::identity::test_set(&client, &endpoint).await;
        crate::config::identity::test_read(&client, &endpoint).await;

        crate::config::certificate::test_create(&client, &endpoint).await;
        crate::config::certificate::test_read(&client, &endpoint).await;
        crate::config::certificate::test_list(&client, &endpoint).await;
        crate::config::certificate::test_delete(&client, &endpoint).await;

        crate::config::sts::test_create(&client, &endpoint).await;
        crate::config::sts::test_read(&client, &endpoint).await;
        crate::config::sts::test_list(&client, &endpoint).await;
        crate::config::sts::test_delete(&client, &endpoint).await;

        crate::config::tidy::identity_access_list::test_set(&client, &endpoint).await;
        crate::config::tidy::identity_access_list::test_read(&client, &endpoint).await;
        crate::config::tidy::identity_access_list::test_delete(&client, &endpoint).await;

        crate::config::tidy::role_tag_deny_list::test_set(&client, &endpoint).await;
        crate::config::tidy::role_tag_deny_list::test_read(&client, &endpoint).await;
        crate::config::tidy::role_tag_deny_list::test_delete(&client, &endpoint).await;

        crate::role::test_create_iam(&client, &endpoint).await;
        crate::role::test_create_ec2(&client, &endpoint).await;
        crate::role::test_read(&client, &endpoint).await;
        crate::role::test_list(&client, &endpoint).await;

        let role_tag = crate::role::test_create_tag(&client, &endpoint).await;
        crate::role_tag_deny_list::test_create(&client, &endpoint, &role_tag).await;
        crate::role_tag_deny_list::test_read(&client, &endpoint, &role_tag).await;
        crate::role_tag_deny_list::test_list(&client, &endpoint, &role_tag).await;
        crate::role_tag_deny_list::test_tidy(&client, &endpoint).await;
        crate::role_tag_deny_list::test_delete(&client, &endpoint, &role_tag).await;

        // role is needed for role_tag_deny_list operations
        crate::role::test_delete(&client, &endpoint).await;

        crate::identity_access_list::test_list(&client, &endpoint).await;
        crate::identity_access_list::test_tidy(&client, &endpoint).await;
    });
}

#[test]
fn test_secret_engine() {
    let test = common::new_aws_test();

    test.run(|instance| async move {
        let server: VaultServer = instance.server();
        let localstack: LocalStackServer = instance.server();
        let client = server.client();
        let endpoint = setup_secret_engine(&server, &client).await.unwrap();

        crate::secretengine::config::test_set(&localstack, &client, &endpoint).await;
        crate::secretengine::config::test_get(&client, &endpoint).await;
        crate::secretengine::config::test_rotate(&client, &endpoint).await;
        crate::secretengine::config::test_set_lease(&client, &endpoint).await;
        crate::secretengine::config::test_read_lease(&client, &endpoint).await;

        crate::secretengine::roles::test_create_update(&client, &endpoint).await;
        crate::secretengine::roles::test_read(&client, &endpoint).await;
        crate::secretengine::roles::test_list(&client, &endpoint).await;
        crate::secretengine::roles::test_credentials(&client, &endpoint).await;
        crate::secretengine::roles::test_credentials_sts(&client, &endpoint).await;
        crate::secretengine::roles::test_delete(&client, &endpoint).await;
    });
}

#[derive(Debug)]
pub struct AwsAuthEndpoint {
    pub path: String,
    pub role_name: String,
}

pub struct AwsSecretEngineEndpoint {
    pub path: String,
}

async fn setup_auth_engine(
    server: &VaultServer,
    client: &impl Client,
) -> Result<AwsAuthEndpoint, ClientError> {
    debug!("setting up AWS auth engine");

    let path = "aws_test";
    let role_name = "test";

    // Mount the AppRole auth engine
    server.mount_auth(client, path, "aws").await?;

    // configure aws client
    Ok(AwsAuthEndpoint {
        path: path.to_string(),
        role_name: role_name.to_string(),
    })
}

async fn setup_secret_engine(
    server: &VaultServer,
    client: &impl Client,
) -> Result<AwsSecretEngineEndpoint, ClientError> {
    debug!("setting up AWS secret engine");

    let path = "aws_test";

    server.mount_secret(client, path, "aws").await?;

    Ok(AwsSecretEngineEndpoint {
        path: path.to_string(),
    })
}

mod config {
    pub mod client {
        use dockertest_server::servers::cloud::localstack::LocalStackServer;
        use vaultrs::{api::auth::aws::requests::ConfigureClientRequest, auth::aws};

        use crate::{AwsAuthEndpoint, Client};

        pub async fn test_set(
            localstack: &LocalStackServer,
            client: &impl Client,
            endpoint: &AwsAuthEndpoint,
        ) {
            let res = aws::config::client::set(
                client,
                &endpoint.path,
                Some(
                    &mut ConfigureClientRequest::builder()
                        .access_key("test")
                        .secret_key("test")
                        .sts_region("local")
                        .endpoint(localstack.internal_url())
                        .sts_endpoint(localstack.internal_url())
                        .iam_endpoint(localstack.internal_url()),
                ),
            )
            .await;

            assert!(res.is_ok());
        }

        pub async fn test_read(client: &impl Client, endpoint: &AwsAuthEndpoint) {
            let res = aws::config::client::read(client, &endpoint.path).await;
            assert!(res.is_ok());
            assert_eq!(res.unwrap().access_key, Some("test".to_string()));
        }

        pub async fn test_delete(client: &impl Client, endpoint: &AwsAuthEndpoint) {
            let res = aws::config::client::delete(client, &endpoint.path).await;
            assert!(res.is_ok());
        }
    }

    pub mod identity {
        use vaultrs::{api::auth::aws::requests::ConfigureIdentityRequest, auth::aws};

        use crate::{AwsAuthEndpoint, Client};

        pub async fn test_set(client: &impl Client, endpoint: &AwsAuthEndpoint) {
            let res = aws::config::identity::set(
                client,
                &endpoint.path,
                Some(
                    &mut ConfigureIdentityRequest::builder()
                        .iam_alias("unique_id")
                        .ec2_alias("instance_id"),
                ),
            )
            .await;
            assert!(res.is_ok());
        }

        pub async fn test_read(client: &impl Client, endpoint: &AwsAuthEndpoint) {
            let res = aws::config::identity::read(client, &endpoint.path).await;
            assert!(res.is_ok());

            let res = res.unwrap();
            assert_eq!(res.iam_alias, Some("unique_id".to_string()));
            assert_eq!(res.ec2_alias, Some("instance_id".to_string()));
        }
    }

    pub mod certificate {
        use base64::{engine::general_purpose, Engine as _};
        use vaultrs::auth::aws;

        use crate::{AwsAuthEndpoint, Client};

        const CERT_NAME: &str = "test_cert";
        const CERT: &str = include_str!("files/aws.crt");

        pub async fn test_create(client: &impl Client, endpoint: &AwsAuthEndpoint) {
            let res = aws::config::certificate::create(
                client,
                &endpoint.path,
                CERT_NAME,
                &general_purpose::STANDARD.encode(CERT),
                None,
            )
            .await;
            assert!(res.is_ok());
        }

        pub async fn test_read(client: &impl Client, endpoint: &AwsAuthEndpoint) {
            let res = aws::config::certificate::read(client, &endpoint.path, CERT_NAME).await;
            assert!(res.is_ok());
            assert_eq!(res.unwrap().aws_public_cert, CERT)
        }

        pub async fn test_delete(client: &impl Client, endpoint: &AwsAuthEndpoint) {
            let res = aws::config::certificate::delete(client, &endpoint.path, CERT_NAME).await;
            assert!(res.is_ok())
        }

        pub async fn test_list(client: &impl Client, endpoint: &AwsAuthEndpoint) {
            let res = aws::config::certificate::list(client, &endpoint.path).await;
            assert!(res.is_ok());
            assert_eq!(res.unwrap().keys, vec![CERT_NAME])
        }
    }

    pub mod sts {
        use vaultrs::auth::aws;

        use crate::{AwsAuthEndpoint, Client};

        const SATELLITE_ACCOUNT_ID: &str = "000000000001";
        const ROLE_NAME: &str = "SomeRole";

        pub async fn test_create(client: &impl Client, endpoint: &AwsAuthEndpoint) {
            let res =
                aws::config::sts::create(client, &endpoint.path, SATELLITE_ACCOUNT_ID, ROLE_NAME)
                    .await;
            assert!(res.is_ok())
        }

        pub async fn test_read(client: &impl Client, endpoint: &AwsAuthEndpoint) {
            let res = aws::config::sts::read(client, &endpoint.path, SATELLITE_ACCOUNT_ID).await;
            assert!(res.is_ok());
            assert!(res.unwrap().sts_role.ends_with(ROLE_NAME));
        }

        pub async fn test_list(client: &impl Client, endpoint: &AwsAuthEndpoint) {
            let res = aws::config::sts::list(client, &endpoint.path).await;
            assert!(res.is_ok());
            assert_eq!(res.unwrap().keys, [SATELLITE_ACCOUNT_ID]);
        }

        pub async fn test_delete(client: &impl Client, endpoint: &AwsAuthEndpoint) {
            let res = aws::config::sts::delete(client, &endpoint.path, SATELLITE_ACCOUNT_ID).await;
            assert!(res.is_ok())
        }
    }

    pub mod tidy {
        pub mod identity_access_list {
            use vaultrs::{
                api::auth::aws::requests::ConfigureIdentityAccessListTidyOperationRequest,
                auth::aws,
            };

            use crate::{AwsAuthEndpoint, Client};

            pub async fn test_set(client: &impl Client, endpoint: &AwsAuthEndpoint) {
                let res = aws::config::tidy::identity_access_list::set(
                    client,
                    &endpoint.path,
                    Some(
                        &mut ConfigureIdentityAccessListTidyOperationRequest::builder()
                            .safety_buffer("24h")
                            .disable_periodic_tidy(true),
                    ),
                )
                .await;

                assert!(res.is_ok())
            }
            pub async fn test_read(client: &impl Client, endpoint: &AwsAuthEndpoint) {
                let res =
                    aws::config::tidy::identity_access_list::read(client, &endpoint.path).await;

                assert!(res.is_ok());

                let res = res.unwrap();
                assert_eq!(res.safety_buffer, 86400);
                assert!(res.disable_periodic_tidy);
            }
            pub async fn test_delete(client: &impl Client, endpoint: &AwsAuthEndpoint) {
                let res =
                    aws::config::tidy::identity_access_list::read(client, &endpoint.path).await;

                assert!(res.is_ok());
            }
        }

        pub mod role_tag_deny_list {
            use vaultrs::{
                api::auth::aws::requests::ConfigureRoleTagDenyListTidyOperationRequest, auth::aws,
            };

            use crate::{AwsAuthEndpoint, Client};

            pub async fn test_set(client: &impl Client, endpoint: &AwsAuthEndpoint) {
                let res = aws::config::tidy::role_tag_deny_list::set(
                    client,
                    &endpoint.path,
                    Some(
                        &mut ConfigureRoleTagDenyListTidyOperationRequest::builder()
                            .safety_buffer("24h"),
                    ),
                )
                .await;

                assert!(res.is_ok())
            }

            pub async fn test_read(client: &impl Client, endpoint: &AwsAuthEndpoint) {
                let res = aws::config::tidy::role_tag_deny_list::read(client, &endpoint.path).await;
                assert!(res.is_ok());

                let res = res.unwrap();
                assert_eq!(res.safety_buffer, 86400);
                assert!(!res.disable_periodic_tidy);
            }

            pub async fn test_delete(client: &impl Client, endpoint: &AwsAuthEndpoint) {
                let res = aws::config::tidy::role_tag_deny_list::read(client, &endpoint.path).await;
                assert!(res.is_ok());
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

    use crate::{AwsAuthEndpoint, Client};

    const ROLE_NAME_IAM: &str = "test_role_iam";
    const ROLE_NAME_EC2: &str = "test_role_ec2";

    pub async fn test_create_iam(client: &impl Client, endpoint: &AwsAuthEndpoint) {
        let res = aws::role::create(
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
        .await;

        assert!(res.is_ok())
    }

    pub async fn test_create_ec2(client: &impl Client, endpoint: &AwsAuthEndpoint) {
        let res = aws::role::create(
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
        .await;

        assert!(res.is_ok())
    }

    pub async fn test_read(client: &impl Client, endpoint: &AwsAuthEndpoint) {
        let res = aws::role::read(client, &endpoint.path, ROLE_NAME_IAM).await;
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap().bound_iam_principal_arn.unwrap_or_default(),
            ["000000000001"]
        );
    }

    pub async fn test_list(client: &impl Client, endpoint: &AwsAuthEndpoint) {
        let res = aws::role::list(client, &endpoint.path).await;
        assert!(res.is_ok());

        let res = res.unwrap();
        assert!(res.keys.contains(&ROLE_NAME_IAM.to_string()));
        assert!(res.keys.contains(&ROLE_NAME_EC2.to_string()));
    }

    pub async fn test_delete(client: &impl Client, endpoint: &AwsAuthEndpoint) {
        let res = aws::role::delete(client, &endpoint.path, ROLE_NAME_IAM).await;
        assert!(res.is_ok());

        let res = aws::role::delete(client, &endpoint.path, ROLE_NAME_EC2).await;
        assert!(res.is_ok());
    }

    pub async fn test_create_tag(
        client: &impl Client,
        endpoint: &AwsAuthEndpoint,
    ) -> CreateRoleTagResponse {
        // role_tag is only used with ec2 auth method
        let res = aws::role::create_tag(
            client,
            &endpoint.path,
            ROLE_NAME_EC2,
            Some(&mut CreateRoleTagRequest::builder().max_ttl("48h")),
        )
        .await;

        assert!(res.is_ok());

        res.unwrap()
    }
}

mod identity_access_list {
    use vaultrs::{api::auth::aws::requests::TidyIdentityAccessListEntriesRequest, auth::aws};

    use crate::{AwsAuthEndpoint, Client, ClientError};

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
        let res = aws::identity_access_list::tidy(
            client,
            &endpoint.path,
            Some(&mut TidyIdentityAccessListEntriesRequest::builder().safety_buffer("12h")),
        )
        .await;

        assert!(res.is_ok());
    }
}

mod role_tag_deny_list {
    use vaultrs::{
        api::auth::aws::{requests::TidyDenyListTagsRequest, responses::CreateRoleTagResponse},
        auth::aws,
    };

    use crate::{AwsAuthEndpoint, Client};

    pub async fn test_create(
        client: &impl Client,
        endpoint: &AwsAuthEndpoint,
        role_tag: &CreateRoleTagResponse,
    ) {
        let res =
            aws::role_tag_deny_list::create(client, &endpoint.path, &role_tag.tag_value).await;

        assert!(res.is_ok());
    }

    pub async fn test_read(
        client: &impl Client,
        endpoint: &AwsAuthEndpoint,
        role_tag: &CreateRoleTagResponse,
    ) {
        let res = aws::role_tag_deny_list::read(client, &endpoint.path, &role_tag.tag_value).await;
        assert!(res.is_ok());
    }

    pub async fn test_list(
        client: &impl Client,
        endpoint: &AwsAuthEndpoint,
        role_tag: &CreateRoleTagResponse,
    ) {
        let res = aws::role_tag_deny_list::list(client, &endpoint.path).await;
        assert!(res.is_ok());
        assert!(res.unwrap().keys.contains(&role_tag.tag_value));
    }

    pub async fn test_delete(
        client: &impl Client,
        endpoint: &AwsAuthEndpoint,
        role_tag: &CreateRoleTagResponse,
    ) {
        let res =
            aws::role_tag_deny_list::delete(client, &endpoint.path, &role_tag.tag_value).await;
        assert!(res.is_ok());
    }

    pub async fn test_tidy(client: &impl Client, endpoint: &AwsAuthEndpoint) {
        let res = aws::role_tag_deny_list::tidy(
            client,
            &endpoint.path,
            Some(&mut TidyDenyListTagsRequest::builder().safety_buffer("8h")),
        )
        .await;
        assert!(res.is_ok())
    }
}

pub mod secretengine {

    pub mod config {
        use dockertest_server::servers::cloud::localstack::LocalStackServer;
        use vaultrs::{api::aws::requests::SetConfigurationRequest, aws};

        use crate::{AwsSecretEngineEndpoint, Client};

        pub async fn test_set(
            localstack: &LocalStackServer,
            client: &impl Client,
            endpoint: &AwsSecretEngineEndpoint,
        ) {
            let res = aws::config::set(
                client,
                &endpoint.path,
                "test",
                "test",
                Some(
                    SetConfigurationRequest::builder()
                        .max_retries(3)
                        .region("eu-central-1")
                        .sts_endpoint(localstack.internal_url())
                        .iam_endpoint(localstack.internal_url()),
                ),
            )
            .await;

            assert!(res.is_ok())
        }

        pub async fn test_get(client: &impl Client, endpoint: &AwsSecretEngineEndpoint) {
            let res = aws::config::get(client, &endpoint.path).await;

            assert!(res.is_ok());

            let data = res.unwrap();
            assert!(data.access_key == "test");
            assert!(data.max_retries == 3);
            assert!(data.region == "eu-central-1");
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
            let res = aws::config::set_lease(client, &endpoint.path, "1h", "6h").await;

            assert!(res.is_ok());
        }

        pub async fn test_read_lease(client: &impl Client, endpoint: &AwsSecretEngineEndpoint) {
            let res = aws::config::read_lease(client, &endpoint.path).await;

            assert!(res.is_ok());

            let data = res.unwrap();

            // response looks like "1h0m0s"
            assert!(data.lease.starts_with("1h"));
            assert!(data.lease_max.starts_with("6h"));
        }
    }

    pub mod roles {
        use vaultrs::{
            api::aws::requests::{
                CreateUpdateRoleRequest, GenerateCredentialsRequest, GenerateCredentialsStsRequest,
            },
            aws,
        };

        use crate::{AwsSecretEngineEndpoint, Client};

        pub const TEST_ROLE: &str = "test_role";
        pub const TEST_ARN: &str = "arn:aws:iam::123456789012:role/test_role";

        pub async fn test_create_update(client: &impl Client, endpoint: &AwsSecretEngineEndpoint) {
            let res = aws::roles::create_update(
                client,
                &endpoint.path,
                TEST_ROLE,
                "assumed_role",
                Some(CreateUpdateRoleRequest::builder().role_arns(vec![TEST_ARN.to_string()])),
            )
            .await;

            assert!(res.is_ok())
        }

        pub async fn test_read(client: &impl Client, endpoint: &AwsSecretEngineEndpoint) {
            let res = aws::roles::read(client, &endpoint.path, TEST_ROLE).await;

            assert!(res.is_ok());

            let data = res.unwrap();
            let roles = data.role_arns.unwrap();

            assert!(data.credential_type == "assumed_role");
            assert!(roles[0] == TEST_ARN);
            assert!(roles.len() == 1);
        }

        pub async fn test_list(client: &impl Client, endpoint: &AwsSecretEngineEndpoint) {
            let res = aws::roles::list(client, &endpoint.path).await;

            assert!(res.is_ok());

            let data = res.unwrap();
            assert!(data.keys[0] == TEST_ROLE);
            assert!(data.keys.len() == 1);
        }

        pub async fn test_credentials(client: &impl Client, endpoint: &AwsSecretEngineEndpoint) {
            let res = aws::roles::credentials(
                client,
                &endpoint.path,
                TEST_ROLE,
                Some(GenerateCredentialsRequest::builder().ttl("3h")),
            )
            .await;

            assert!(res.is_ok());

            let data = res.unwrap();
            assert!(data.access_key.starts_with("ASIA"));
            assert!(!data.secret_key.is_empty());
            assert!(!data.security_token.unwrap().is_empty());
        }

        pub async fn test_credentials_sts(
            client: &impl Client,
            endpoint: &AwsSecretEngineEndpoint,
        ) {
            let res = aws::roles::credentials_sts(
                client,
                &endpoint.path,
                TEST_ROLE,
                Some(GenerateCredentialsStsRequest::builder().ttl("3h")),
            )
            .await;

            assert!(res.is_ok());

            let data = res.unwrap();
            assert!(data.access_key.starts_with("ASIA"));
            assert!(!data.secret_key.is_empty());
            assert!(!data.security_token.unwrap().is_empty());
        }

        pub async fn test_delete(client: &impl Client, endpoint: &AwsSecretEngineEndpoint) {
            let res = aws::roles::delete(client, &endpoint.path, TEST_ROLE).await;

            assert!(res.is_ok());

            // check deletion actually worked, list should be empty (Vault returns 404)
            let res_after = aws::roles::list(client, &endpoint.path).await;
            assert!(res_after.is_err());
        }
    }
}
