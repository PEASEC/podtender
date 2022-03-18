use lazy_static::lazy_static;
use podtender::error::PodtenderError;
use podtender::error::PodtenderError::SerdeJsonErrorWithPath;
use podtender::podman_service::PodmanService;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::thread;
use std::time;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

// #[allow(dead_code)] is need on most methods to calm clippy down.

pub static TESTCONTAINER_IMAGE_NAME: &str = "testcontainer";
static API_SOCKET_NAME: &str = "podtender_test_socket";
// In seconds.
static API_TTL: u32 = 300;

lazy_static! {
    pub static ref PODMAN_PATH: PathBuf = PathBuf::from("podman");
}

lazy_static! {
    pub static ref BUILT_TEST_IMAGE: bool = build_test_image();
}

lazy_static! {
    pub static ref PODMAN_SERVICE: PodmanService =
        PodmanService::new(setup_podman_api(API_SOCKET_NAME, API_TTL).as_str());
}

lazy_static! {
    pub static ref SUBSCRIBER_SET: bool = set_global_subscriber();
}

pub fn set_global_subscriber() -> bool {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "podtender=trace".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    true
}

#[allow(dead_code)]
pub fn setup() -> &'static PodmanService {
    assert!(*BUILT_TEST_IMAGE);
    assert!(*SUBSCRIBER_SET);
    &PODMAN_SERVICE
}

/// Start the podman api for `ttl` seconds on `unix:///home/{user}/socket_name.sock`.
/// `user` is the current system username, `socket_name` is supplied via function parameter.
#[allow(dead_code)]
pub fn setup_podman_api(socket_name: &str, ttl: u32) -> String {
    let user = Command::new("whoami")
        .output()
        .expect("failed to execute podman command");

    let user = String::from(
        String::from_utf8(Vec::from(user.stdout.as_slice()))
            .unwrap()
            .trim(),
    );

    let socket = format!("unix:///home/{}/{}.sock", user, socket_name);
    let ttl = format!("--time={}", ttl);

    Command::new(PODMAN_PATH.as_path())
        .arg("system")
        .arg("service")
        .arg(&socket)
        .arg(ttl)
        .spawn()
        .expect("failed to execute podman command");
    // otherwise the socket isn't always ready, really annoying to debug
    thread::sleep(time::Duration::from_secs(5));
    format!("/home/{}/{}.sock", user, socket_name)
}

/// Return podman system information.
#[allow(dead_code)]
pub fn get_info() -> Output {
    Command::new(PODMAN_PATH.as_path())
        .arg("info")
        .arg("--format=json")
        .output()
        .expect("failed to execute podman command")
}

/// Start a container named `container_name` for testing purposes.
#[allow(dead_code)]
pub fn run_container(container_name: &str) {
    Command::new(PODMAN_PATH.as_path())
        .arg("run")
        .arg("-d")
        .arg(format!("--name={}", container_name))
        .arg(format!("{}:latest", TESTCONTAINER_IMAGE_NAME))
        .status()
        .expect("failed to execute podman command");
}

/// Start a container named `container_name` for testing purposes.
#[allow(dead_code)]
pub fn start_container(container_name: &str) {
    Command::new(PODMAN_PATH.as_path())
        .arg("start")
        .arg(container_name)
        .status()
        .expect("failed to execute podman command");
}

/// Pause a container named `container_name` for testing purposes.
/// Only supported with cgroup v2
#[allow(dead_code)]
pub fn pause_container(container_name: &str) {
    Command::new(PODMAN_PATH.as_path())
        .arg("pause")
        .arg(container_name)
        .status()
        .expect("failed to execute podman command");
}

/// Mount a container named `container_name` for testing purposes.
#[allow(dead_code)]
pub fn mount_container(container_name: &str) {
    Command::new(PODMAN_PATH.as_path())
        .arg("mount")
        .arg(container_name)
        .status()
        .expect("failed to execute podman command");
}

/// Create a container named `container_name` for testing purposes.
#[allow(dead_code)]
pub fn create_container(container_name: &str) {
    Command::new(PODMAN_PATH.as_path())
        .arg("create")
        .arg(format!("--name={}", container_name))
        .arg(format!("{}:latest", TESTCONTAINER_IMAGE_NAME))
        .status()
        .expect("failed to execute podman command");
}

/// Create a container named `container_name` with label `label` for testing purposes.
#[allow(dead_code)]
pub fn create_container_with_label(container_name: &str, label: &str) {
    Command::new(PODMAN_PATH.as_path())
        .arg("create")
        .arg("--label")
        .arg(label)
        .arg(format!("--name={}", container_name))
        .arg(format!("{}:latest", TESTCONTAINER_IMAGE_NAME))
        .status()
        .expect("failed to execute podman command");
}

/// Create a container named `container_name` with networkmode `network_mode` for testing purposes.
#[allow(dead_code)]
pub fn create_container_with_network_mode(container_name: &str, network_mode: &str) {
    Command::new(PODMAN_PATH.as_path())
        .arg("create")
        .arg(format!("--name={}", container_name))
        .arg(format!("--network={}", network_mode))
        .arg(format!("{}:latest", TESTCONTAINER_IMAGE_NAME))
        .status()
        .expect("failed to execute podman command");
}

/// Create a container named `container_name` with network `network` for testing purposes.
#[allow(dead_code)]
pub fn create_container_with_network(container_name: &str, network: &str) {
    Command::new(PODMAN_PATH.as_path())
        .arg("create")
        .arg(format!("--name={}", container_name))
        .arg(format!("--net {}", network))
        .arg(format!("{}:latest", TESTCONTAINER_IMAGE_NAME))
        .status()
        .expect("failed to execute podman command");
}

/// Create a container named `container_name` with portmapping `portmapping` for testing purposes.
#[allow(dead_code)]
pub fn create_container_with_portmapping(container_name: &str, portmapping: &str) {
    Command::new(PODMAN_PATH.as_path())
        .arg("create")
        .arg(format!("--name={}", container_name))
        .arg(format!("--publish={}", portmapping))
        .arg(format!("{}:latest", TESTCONTAINER_IMAGE_NAME))
        .status()
        .expect("failed to execute podman command");
}

/// Create a container named `container_name` with pod `pod_name` for testing purposes.
#[allow(dead_code)]
pub fn create_container_with_pod(container_name: &str, pod_name: &str) {
    Command::new(PODMAN_PATH.as_path())
        .arg("create")
        .arg(format!("--name={}", container_name))
        .arg(format!("--pod={}", pod_name))
        .arg(format!("{}:latest", TESTCONTAINER_IMAGE_NAME))
        .status()
        .expect("failed to execute podman command");
}

/// Stop a container named `container_name` for testing purposes.
/// `-t 0` to force immediate stop
#[allow(dead_code)]
pub fn stop_container(container_name: &str) {
    Command::new(PODMAN_PATH.as_path())
        .arg("stop")
        .arg("-t")
        .arg("0")
        .arg("-i")
        .arg(container_name)
        .status()
        .expect("failed to execute podman command");
}

/// Delete a container named `container_name` for testing purposes.
#[allow(dead_code)]
pub fn delete_container(container_name: &str) {
    Command::new(PODMAN_PATH.as_path())
        .arg("rm")
        .arg("--force")
        .arg("-t=0")
        .arg("--ignore")
        .arg(container_name)
        .status()
        .expect("failed to execute podman command");
}

/// Export a container name `container_name` for testing purposes
#[allow(dead_code)]
pub fn export_container(container_name: &str) -> Output {
    Command::new(PODMAN_PATH.as_path())
        .arg("export")
        .arg(container_name)
        .output()
        .expect("failed to execute podman command")
}

/// Inspect a container named `container_name` for testing purposes.
#[allow(dead_code)]
pub fn inspect_container(container_name: &str) -> Output {
    Command::new(PODMAN_PATH.as_path())
        .arg("container")
        .arg("inspect")
        .arg(container_name)
        .output()
        .expect("failed to execute podman command")
}

/// Use podman cli to build the container from  test_container/Dockerfile
#[allow(dead_code)]
pub fn build_test_image() -> bool {
    use std::env;
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    Command::new(PODMAN_PATH.as_path())
        .arg("build")
        .arg("--format")
        .arg("docker")
        .arg("-t")
        .arg(format!("{}:latest", TESTCONTAINER_IMAGE_NAME))
        .arg("-f")
        .arg(format!("{}/test_container/Dockerfile", cargo_manifest_dir))
        .status()
        .expect("failed to execute podman command")
        .success()
}

/// Use podman cli to build the container from  test_container/Dockerfile2
/// Already built images can't be assigned a label as far as I can tell to we need to build a new and different image to avoid just tagging the usual testimage.
#[allow(dead_code)]
pub fn build_test_image_with_label(image_name: &str, label: &str) -> bool {
    use std::env;
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    Command::new(PODMAN_PATH.as_path())
        .arg("build")
        .arg("--format")
        .arg("docker")
        .arg(format!("--label={}", label))
        .arg("-t")
        .arg(format!("{}:latest", image_name))
        .arg("-f")
        .arg(format!("{}/test_container/Dockerfile2", cargo_manifest_dir))
        .status()
        .expect("failed to execute podman command")
        .success()
}

#[allow(dead_code)]
pub fn build_test_image_to_delete(image_name: &str) -> bool {
    use std::env;
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    Command::new(PODMAN_PATH.as_path())
        .arg("build")
        .arg("--format")
        .arg("docker")
        .arg("-t")
        .arg(format!("{}:latest", image_name))
        .arg("-f")
        .arg(format!("{}/test_container/Dockerfile3", cargo_manifest_dir))
        .status()
        .expect("failed to execute podman command")
        .success()
}

/// Create a network named `network_name` for testing purposes.
#[allow(dead_code)]
pub fn create_network(network_name: &str) {
    Command::new(PODMAN_PATH.as_path())
        .arg("network")
        .arg("create")
        .arg(network_name)
        .status()
        .expect("failed to execute podman command");
}

/// Create a network named `network_name` for testing purposes.
#[allow(dead_code)]
pub fn create_network_with_subnet(network_name: &str, subnet: &str) {
    Command::new(PODMAN_PATH.as_path())
        .arg("network")
        .arg("create")
        .arg("--subnet")
        .arg(subnet)
        .arg(network_name)
        .status()
        .expect("failed to execute podman command");
}

/// Create a network named `network_name` for testing purposes.
#[allow(dead_code)]
pub fn create_network_with_label(network_name: &str, label: &str) {
    Command::new(PODMAN_PATH.as_path())
        .arg("network")
        .arg("create")
        .arg(network_name)
        .arg("--label")
        .arg(label)
        .status()
        .expect("failed to execute podman command");
}

/// Delete a network named `network_name` for testing purposes.
#[allow(dead_code)]
pub fn delete_network(network_name: &str) {
    Command::new(PODMAN_PATH.as_path())
        .arg("network")
        .arg("rm")
        .arg("--force")
        .arg(network_name)
        .status()
        .expect("failed to execute podman command");
}

/// Inspect a network named `network_name` for testing purposes.
#[allow(dead_code)]
pub fn inspect_network(network_name: &str) -> Output {
    Command::new(PODMAN_PATH.as_path())
        .arg("network")
        .arg("inspect")
        .arg(network_name)
        .output()
        .expect("failed to execute podman command")
}

/// Delete a network named `network_name` for testing purposes.
#[allow(dead_code)]
pub fn connect_container_to_network(network_name: &str, container_name: &str) {
    Command::new(PODMAN_PATH.as_path())
        .arg("network")
        .arg("connect")
        .arg(network_name)
        .arg(container_name)
        .status()
        .expect("failed to execute podman command");
}

/// Delete a volume named `volume_name` for testing purposes.
#[allow(dead_code)]
pub fn delete_volume(volume_name: &str) {
    Command::new(PODMAN_PATH.as_path())
        .arg("volume")
        .arg("rm")
        .arg("--force")
        .arg(volume_name)
        .status()
        .expect("failed to execute podman command");
}

/// Create a volume named `volume_name` for testing purposes.
#[allow(dead_code)]
pub fn create_volume(volume_name: &str) {
    Command::new(PODMAN_PATH.as_path())
        .arg("volume")
        .arg("create")
        .arg(volume_name)
        .status()
        .expect("failed to execute podman command");
}

/// Create a volume named `volume_name` for testing purposes.
#[allow(dead_code)]
pub fn create_volume_with_label(volume_name: &str, label: &str) {
    Command::new(PODMAN_PATH.as_path())
        .arg("volume")
        .arg("create")
        .arg(volume_name)
        .arg("--label")
        .arg(label)
        .status()
        .expect("failed to execute podman command");
}

/// Delete a pod named `pod_name` for testing purposes.
#[allow(dead_code)]
pub fn delete_pod(pod_name: &str) {
    Command::new(PODMAN_PATH.as_path())
        .arg("pod")
        .arg("rm")
        .arg("--force")
        .arg("--ignore")
        .arg(pod_name)
        .status()
        .expect("failed to execute podman command");
}

/// Stop a pod named `pod_name` for testing purposes.
#[allow(dead_code)]
pub fn stop_pod(pod_name: &str) {
    Command::new(PODMAN_PATH.as_path())
        .arg("pod")
        .arg("stop")
        .arg("-t=0")
        .arg("--ignore")
        .arg(pod_name)
        .status()
        .expect("failed to execute podman command");
}

/// Create a pod named `pod_name` for testing purposes.
#[allow(dead_code)]
pub fn create_pod(pod_name: &str) -> String {
    let pod = Command::new(PODMAN_PATH.as_path())
        .arg("pod")
        .arg("create")
        .arg("--name")
        .arg(pod_name)
        .output()
        .expect("failed to execute podman command");
    String::from(
        String::from_utf8(Vec::from(pod.stdout.as_slice()))
            .unwrap()
            .trim(),
    )
}

/// Create a pod named `pod_name` for testing purposes.
#[allow(dead_code)]
pub fn create_pod_with_label(pod_name: &str, label: &str) -> String {
    let pod = Command::new(PODMAN_PATH.as_path())
        .arg("pod")
        .arg("create")
        .arg("--name")
        .arg(pod_name)
        .arg("--label")
        .arg(label)
        .output()
        .expect("failed to execute podman command");
    String::from(
        String::from_utf8(Vec::from(pod.stdout.as_slice()))
            .unwrap()
            .trim(),
    )
}

/// Start a pod named `pod_name` for testing purposes.
#[allow(dead_code)]
pub fn start_pod(pod_name: &str) {
    Command::new(PODMAN_PATH.as_path())
        .arg("pod")
        .arg("start")
        .arg(pod_name)
        .status()
        .expect("failed to execute podman command");
}

/// Pause a pod named `pod_name` for testing purposes.
#[allow(dead_code)]
pub fn pause_pod(pod_name: &str) {
    Command::new(PODMAN_PATH.as_path())
        .arg("pod")
        .arg("pause")
        .arg(pod_name)
        .status()
        .expect("failed to execute podman command");
}

/// Pull an image for testing purposes.
#[allow(dead_code)]
pub fn pull_image(image_name: &str) {
    Command::new(PODMAN_PATH.as_path())
        .arg("pull")
        .arg(image_name)
        .status()
        .expect("failed to execute podman command");
}

/// Tag an image for testing purposes.
#[allow(dead_code)]
pub fn tag_image(image_name: &str, tag_name: &str) {
    Command::new(PODMAN_PATH.as_path())
        .arg("tag")
        .arg(image_name)
        .arg(tag_name)
        .status()
        .expect("failed to execute podman command");
}

/// Untag an image for testing purposes.
#[allow(dead_code)]
pub fn untag_image(image_name: &str, tag_name: &str) {
    Command::new(PODMAN_PATH.as_path())
        .arg("untag")
        .arg(image_name)
        .arg(tag_name)
        .status()
        .expect("failed to execute podman command");
}
/// Delete an image for testing purposes.
#[allow(dead_code)]
pub fn delete_image(image_name: &str) {
    Command::new(PODMAN_PATH.as_path())
        .arg("rmi")
        .arg(image_name)
        .arg("-f")
        .status()
        .expect("failed to execute podman command");
}

/// Export an image name `image_name` for testing purposes
#[allow(dead_code)]
pub fn export_image(image_name: &str) -> Output {
    Command::new(PODMAN_PATH.as_path())
        .arg("save")
        .arg("--format")
        .arg("oci-archive")
        .arg(image_name)
        .output()
        .expect("failed to execute podman command")
}

/// Inspect an image name `image_name` for testing purposes
#[allow(dead_code)]
pub fn inspect_image(image_name: &str) -> Output {
    Command::new(PODMAN_PATH.as_path())
        .arg("image")
        .arg("inspect")
        .arg(image_name)
        .output()
        .expect("failed to execute podman command")
}

/// Check whether the error is a `SerdeJsonErrorWithPath`, if yes, print the location and error message to stderr.
#[allow(dead_code)]
pub fn print_path_if_serde_error(podtender_error: &PodtenderError) {
    if let SerdeJsonErrorWithPath(test) = podtender_error {
        eprintln!("{}", test.inner());
        eprintln!("{}", test.path())
    }
}
