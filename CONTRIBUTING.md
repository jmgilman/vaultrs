# Architecture

The architecture of the source directory is as such:

* `src/` - Crate root directory
* `src/api` - Root directory containing raw endpoints and supporting functions
* `src/client.rs` - Source for the client
* `src/error.rs` - Contains the common error enum for this crate
* `src/lib.rs` - Crate root file
* `src/*.rs` - Main API functions that wrap endpoints located in `src/api`

For example, the PKI engine is organized as such:

* `src/api/pki/requests.rs` - Contains all endpoints associated with this engine
* `src/api/pki/responses.rs` - Contains all responses from the endpoints
* `src/pki.rs` - Contains the high level functions for interacting with the 
   engine

Additionally, the `src/pki.rs` file is further organized into modules which help
break up the API functions available. For example, `pki::certs` contains
functions for working with certificates and `pki::roles` contains functions for
configuring roles. 

This library leans heavily on [rustify](https://docs.rs/rustify/0.1.0/rustify/)
in order to scaffold the Vault API endpoints. Hashicorp also has [extensive
documentation](https://developer.hashicorp.com/vault/api-docs) available for all supported
endpoints. Vault uses the concepts of *secret engines* in order to categorize
the functionality offered by the software. For example, there's a PKI engine,
KV engine, SSH engine, various database engines, etc. This library takes
advantage of this categorization by breaking up the API interface to reflect the
functionality available. As seen in the previous section, each engine has a 
dedicated directory under `/api` which contains the requests and responses of
the API endpoints for that engine and then a single `{engine}.rs` file at the
crate root which provides the high level API functions for the engine. 

## Endpoints

```rust
/// ## Generate Root
/// This endpoint generates a new self-signed CA certificate and private key. If
/// the path ends with exported, the private key will be returned in the
/// response; if it is internal the private key will not be returned and cannot
/// be retrieved later.
///
/// * Path: {self.mount}/root/generate/{self.cert_type}
/// * Method: POST
/// * Response: [Option<GenerateRootResponse]
/// * Reference: https://developer.hashicorp.com/vault/api-docssecret/pki#generate-root
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/root/generate/{self.cert_type}",
    method = "POST",
    response = "Option<GenerateRootResponse>",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct GenerateRootRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
    pub cert_type: String,
    pub alt_names: Option<String>,
    pub common_name: Option<String>,
    pub country: Option<Vec<String>>,
    pub exclude_cn_from_sans: Option<bool>,
    pub format: Option<String>,
    pub locality: Option<Vec<String>>,
    pub key_bits: Option<u64>,
    pub key_type: Option<String>,
    pub ip_sans: Option<String>,
    pub max_path_length: Option<i32>,
    pub organization: Option<Vec<String>>,
    pub other_sans: Option<Vec<String>>,
    pub ou: Option<Vec<String>>,
    pub permitted_dns_domains: Vec<String>,
    pub postal_code: Option<Vec<String>>,
    pub private_key_format: Option<String>,
    pub province: Option<Vec<String>>,
    pub serial_number: Option<String>,
    pub street_address: Option<Vec<String>>,
    pub ttl: Option<String>,
    pub uri_sans: Option<String>,
}
```

Above is the definition for the endpoint which generates root certificates in
the PKI engine. Since there are *many* endpoints available in Vault, all 
definitions should include documentation comments that match the above format.
This includes a short description of the endpoint (usually copied off the Vault
API docs), basic details about the endpoint (method type, response, etc.) and
then a link to where the endpoint is documented in the Vault API docs. 

For a full explanation of how the `rustify_derive` macro works, see 
[the docs]((https://docs.rs/rustify/0.1.0/rustify/)). A brief overview follows:

* The `path` parameter defines the relative API path of the endpoint. The client
  will automatically handle prepending the API version to the URLs and so it can
  be freely omitted. This parameter allows basic interpolation using curly 
  braces as seen above. It's important to mark any fields being used in 
  interpolation with `[serde(skip)]` to prevent `serde` from serializing them
  in the request body.
* The `method` parameter defines the HTTP method that this endpoint is
  expecting. Note that some endpoints share the same `path` but expect different
  HTTP methods to be used.
* The `result` parameter defines the concrete type that this endpoint is
  expected to return. The response from the Vault server will be attempted to be
  deserialized into this type. Thus, the target type should derive
  `serde::Deserialize`. Most responses from the Vault server are wrapped in a
  common format where the actual payload is located in the `data` field. The
  library will automatically strip this wrapper, escalating any errors found, and
  return the payload. Thus, the result type only needs to specify the format of
  the payload and not the entire wrapper. For endpoints that don't return any
  result, the result parameter can be omitted entirely. 
* The `builder` parameter adds some useful methods associated with using
  `derive_builder` and most endpoints should include this. 

In some cases an endpoint may return a payload that is unknown by the library
ahead of time. For example, the KV engine can return a payload that is specific
to the secret being read. In cases like these it's possible to add a generic
type to the `result` parameter, however, there are 
[some limitations](https://github.com/rust-lang/rust/issues/23246) imposed by
how associated types work in Rust. Specifically, we have to use `PhantomData`
in the struct fields in order to prevent the compiler from complaining about
unused generics. See 
[UnwrapRequest](https://github.com/jmgilman/vaultrs/blob/master/src/api/sys/requests.rs#L70)
for an example of this. 

Alternatively, you can simply set the result/field to a `serde_json::Value` and
then deserialize it in the higher level API function. See the
[read](https://github.com/jmgilman/vaultrs/blob/master/src/kv2.rs#L94) function
for an example of this. 

The remaining derviations for the endpoint are summarized below:

* The `Builder` derivation is used to make building requests easier. The fields
  of the endpoint should indicate which are required and which are optional by
  using an `Option`. Combined with `setter(into)` and `setter(strip_option)` the
  build process for the end-user becomes much easier. The 
  `skip_serializing_none` attribute informs `serde` to ignore fields that have
  a value of `Option::None`. Combined with `Default` any unset fields in the
  build process will default to `Option::None` and therefore not be serialized
  in the request. This is important because an empty field in some cases can
  actually erase the stored value. 
* The `Serialize` derivation is required as the request body consists of the
  result of serializing the endpoint. Thus, any fields you don't want to be 
  serialized as part of the body should have the `#[serde(skip)]` tag added.

## High-level Functions

```rust
/// Generates a new root CA
///
/// See [GenerateRootRequest]
pub async fn generate(
    client: &impl Client,
    mount: &str,
    cert_type: &str,
    opts: Option<&mut GenerateRootRequestBuilder>,
) -> Result<Option<GenerateRootResponse>, ClientError> {
    let mut t = GenerateRootRequest::builder();
    let endpoint = opts
        .unwrap_or(&mut t)
        .mount(mount)
        .cert_type(cert_type)
        .build()
        .unwrap();
    api::exec_with_result(client, endpoint).await
}
```

Above is the high-level API function associated with the `GenerateRootRequest`
endpoint. Most high-level functions will look identical with the only changes
being related to the required fields, whether it accepts additional options,
and the return type. A few notes:

* All functions should have a succinct summary of what they do and link back to
  the documentation of the underlying endpoint. This allows the end-user to
  avoid a cluttered intellisense result when interacting with the API but still
  be able to easily click to the more detailed documentation added to the
  endpoint.
* Executing endpoints requires a `VaultClient` and so all functions should take
  a reference to one.
* Most secret engines can be mounted multiple times at different mount points.
  Thus, the end-user must specify which mounted secret engine they are targeting
  with the request.
* The high-level API function should require the end-user pass all mandatory
  parameters for the endpoint. In the above example, the `cert_type` field in
  the `GenerateRootRequest` is required and would result in an error if it was
  omitted. Adding it to the function parameter list eases the burden of the
  end-user by specifying which ones are required.
* Any optional request parameters should be handled by allowing the end-user
  to specify a `Builder` variant of the request. As seen above, we generate
  an empty builder and then use `unwrap_or()` to attempt to use the passed
  options but defaulting to our empty one if none was passed. We then attach
  the required parameters to the builder before executing.
* The `vaultrs::api` module contains functions for executing endpoints. In
  particular:
    * The `api::exec_with_result` function executes the endpoint and returns the
      result as specified by the endpoint. This function will fail if an empty
      HTTP response or an empty `data` field is returned.
    * The `api::exec_with_empty_result` function executes the endpoint and
      expects the server to returned a wrapped response but the `data` field
      for that response is empty. This only occurs in rare occasions. 
    * The `api::exec_with_empty` function executes the endpoint and expects the
      server to return an empty HTTP response. Most API endpoints that operate
      on server-side data will return an empty HTTP response.

The `api::*` functions will take care of most of the work of executing the
endpoint for you. This includes automatically attaching the Vault token to
requests as well as logging warnings found in the wrapper or returning an
`Err` when the Vault server rejects a request. In most cases the Vault server
will include an error message when a request fails and the library will
automatically propagate it as needed.

# Adding functionality

1. Create a new directory under `src/api` for the engine type if it's not
   already been added.
2. Add endpoints to `src/api/{engine}/requests.rs` and their associated
   responses to `src/api/{engine}/responses.rs`.
3. Add high level functions that use the endpoints in `src/{engine}.rs`. 
4. Add tests for each high level function in `tests/{engine}.rs`. 

# Testing

All tests use a live instance of the Vault server for testing against since
mocking cannot verify the endpoint structures are accurate and valid. This also 
allows pinning to specific versions of Vault and adding support for newer 
versions as needed. While the tests are not intended to test the Vault server 
itself, it's recommended to  perform necessary setup to imitate end-user
behavior.

Bear in mind that the response of an endpoint may change based on the input
given by the user. For example, a different response is generated by the root
CA generation endpoint depending on if an internal or external CA is requested.
It's therefore important to mark fields as `Optional<>` where necessary and 
attempt to test all cases if possible. 

Since some engines can have dozens of endpoints it's often better to define
sub-functions for testing each endpoint and then combining them all together
into a single test function. This allows you to setup the Vault server once and
then test each endpoint and dramatically improves speed over testing each
endpoint individually.
