mod images;
mod setup;
pub(crate) use images::{Vault, TESTED_VERSION};
pub use images::{KUB_ACCOUNT_NAME, KUB_NAMESPACE};
pub use setup::{TestBuilder, POSTGRES_PASSWORD, POSTGRES_USER};
