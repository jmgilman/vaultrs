use dockertest::{Composition, DockerOperations, DockerTest};
use futures::future::Future;

/// A server configuration which can be converted into a [Composition] to be
/// ran as a container.
///
/// Types implementing this trait should collect all required configuration data
/// and use [ServerConfig::to_comp] to convert it into a [Composition] to be run
/// by a [TestInstance]. The associated [Server] for this configuration should
/// later recieve a copy of this configuration to perform any additional steps
/// required.
pub trait ServerConfig {
    /// Creates a [Composition] from this configuration
    fn to_comp(&self) -> Composition;

    /// Creates a new [TestInstance] and automatically adds the [Composition]
    /// from this configuration to it.
    fn to_instance(&self) -> TestInstance;
}

/// An instance of a server that is created after it's associated [Composition]
/// has been brought up.
///
/// This trait is the main vehicle for tests to interact with the running
/// container. It should encompass all logic necessary for tests to successfully
/// interact with it.
pub trait Server {
    type Config;
    fn new(ops: &DockerOperations, config: &Self::Config) -> Self;
}

/// A single test instance made up of several [Composition]s which
/// are brought up for executing tests.
///
/// Use the `run` method to run the containers and perform the test logic.
pub struct TestInstance {
    pub instance: DockerTest,
}

impl TestInstance {
    /// Returns a new [TestInstance] configured with the given [Composition]s.
    pub fn new(mut servers: Vec<Composition>) -> TestInstance {
        let mut instance = DockerTest::new();
        servers.drain(..).for_each(|s| instance.add_composition(s));

        TestInstance { instance }
    }

    /// Adds a [Composition] to this [TestInstance].
    pub fn add(&mut self, comp: Composition) {
        self.instance.add_composition(comp);
    }

    /// Runs all containers from the associated [Composition]s, verifying they
    /// are running according to their startup conditions, and then calls the
    /// passed closure with runtime details.
    ///
    /// This is the main method for running the [TestInstance]. All test logic
    /// should be encompassed within the passed closure. Creating and destroying
    /// containers happens upon entering/leaving the closure.
    pub fn run<T, F>(self, fun: T)
    where
        T: FnOnce(DockerOperations) -> F + Send + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.instance.run(|ops| async move {
            (fun)(ops).await;
        });
    }
}
