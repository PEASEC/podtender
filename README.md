![podtender logo](logo/podtender-logo-source.svg)

# Podtender
An async rust client library for the [Podman REST API](https://docs.podman.io/en/latest/Reference.html).

## Usage

### Example
```rust
use tokio;
use podtender::podman_service::PodmanService;

#[tokio::main]
async fn main() {
  let podman_service = PodmanService::new("path/to/podman/socket");
  let podtender_result = podman_service.system().get_info().await.unwrap();
  println!("{podtender_result:?}");
}
```
The [integration tests](https://github.com/PEASEC/podtender/tree/main/tests) also act as examples for how to use the crate.

### Podman setup
The [Podman service](https://docs.podman.io/en/latest/markdown/podman-system-service.1.html) needs to be set up before the crate can be used. 
```shell
# Set up podman service
podman system service unix:///home/`whoami`/podman.sock --log-level=debug --time=50
# Test if the socket is up and running
curl --unix-socket /home/`whoami`/podman.sock http://d/v4.0.0/libpod/info
```
### Supported Podman version
We aim to support the latest Podman version only. Currently, this is 4.1.x.

### Crate features
#### Builder pattern via derive builder
The `builder` feature enables the builder pattern for request types. This is implemented with the [builder derive macro](https://github.com/colin-kiegel/rust-derive-builder).
IDE support for code created by macros may be limited.
```rust
// with builder feature
let create_volume_parameter = CreateVolumeParameterBuilder::default()
    .driver("local".to_owned())
    .volume_name("some_volume_name".to_owned())
    .options(some_previously_created_hashmap)
    .build()
    .expect("Error building CreateVolumeParameter");
// without builder feature
let create_volume_parameter = CreateVolumeParameter {
    driver: Some("local".to_owned()),
    labels: None,
    volume_name: Some("some_volume_name".to_owned())
    options: Some(some_previously_created_hashmap),
}
```

#### Example parameters used in tests
The `examples` feature enables initialized parameters via the `ExampleValues` trait. This is mostly intended for tests and
to showcase a usable example parameter per API operation.

#### Tracing
`enable-tracing` enables logs/tracing powered by [Tokio's tracing crate](https://github.com/tokio-rs/tracing).

### Requirements
Podtender uses [hyper](https://github.com/hyperium/hyper) and requires [tokio](https://github.com/tokio-rs/tokio).
An active Podman socket is needed to communicate with Podman.

### Project structure
The Podman socket and network operations are internally managed by the `PodmanService` struct.

The [Podman API](https://docs.podman.io/en/latest/_static/api.html?version=v4.0) categorizes endpoints into multiple sections. This crate is based on that structure.
The [pods](src/pods) module contains all api call functions, parameter types and response types for the implemented
[pods](https://docs.podman.io/en/latest/_static/api.html?version=v4.0#tag/pods) API endpoints.

Every endpoint category module contains:
* the parameter types, used to configure the request
* the api call functions used to make the request 
* the response types into which the response gets deserialized

To start a pod, you would configure `podtender::pods::parameter_types::PodStatsParameter`
use it with `podman_service.pods().start(parameter).await` and expect `podtender::pods::response_types::StartPodResponse`.


## Notes
* Only unix socket is currently supported
* Podtender currently only builds on Linux since tokio only builds support for unix sockets on Linux 
* Podman (API) is treated as trusted and in testing as source of truth
* To create a container in bridge network mode, use this as starting point:
  ```
  CreateContainerParameter {
        netns: Some(Namespace {
            nsmode: Some("bridge".to_owned()),
            value: None,
        }),
        ..Default::default()
    };
  ```
* The following error indicates the Podman socket not being available. If this happens in tests, rerunning recreates the socket for the defined ttl and should get rid of the error.
  ```
   HyperError(
       hyper::Error(
           Connect,
           Os {
               code: 2,
               kind: NotFound,
               message: "No such file or directory",
           },
       ),
   )
   ```
* The `connect_container_to_network_from_example` test require the `192.168.123.0/24` subnet to be unused on the machine executing the tests. 
* `2021-05-26T10:42:00+02:00` timestamps are supported by Podman, this crate currently only supports dates as strings.
* Podman only supports query array in this format: `containers=container1&containers=container2`
* Tests create the Podman service socket at `unix:///home/{user}/{socket_name}.sock`, the `socket_name` can be changed in [tests/utils](tests/utils/mod.rs).
* Tests build images from the Dockerfiles in [test_container](https://github.com/PEASEC/podtender/tree/main/test_container).

## Acknowledgments
* This work was created at Science and Technology for Peace and Security (PEASEC), Technical University of Darmstadt, www.peasec.de, and supported by funds of the German Government’s Special Purpose Fund held at Landwirtschaftliche Rentenbank in the projects Geobox-II and AgriRegio.
  * Contributors under those funds:
    * Julian Schindel
    * Franz Kuntke
* The overall structure of this crate was inspired by [shiplift](https://github.com/softprops/shiplift), created by [softprops (Doug Tangren)](https://github.com/softprops).

## License
Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in `podtender` by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.


## Current State

### Currently supported API endpoints:
- [ ] Containers
  - [ ] Commit
  - [x] Delete Container
  - [ ] Copy files into a Container
  - [ ] Attach to a container
  - [ ] Report on changes to the container's filesystem; adds, deletes or modifications
  - [x] Checkpoint a container (currently not tested, CRIU requires root privileges)
  - [x] Check if a container exists
  - [x] Export a container
  - [x] Run a container's healtcheck
  - [x] Initialize a container
  - [x] Inspect container
  - [x] Kill container
  - [x] Get container logs
  - [x] Mount a container
  - [x] Pause a container
  - [x] Rename an existing container
  - [ ] Resize a Container's TTY
  - [x] Restart a container
  - [ ] Restore a container
  - [x] Start a container
  - [ ] Get stats for a container (deprecated, use "Get stats for one or more containers")
  - [x] Stop a container
  - [x] List processes
    - [x] streaming
    - [x] non streaming
  - [x] Unmount a container
  - [x] Unpause container
  - [ ] Wait on a container
  - [x] Create a container
  - [x] List containers
  - [x] Delete stopped containers
  - [ ] Show mounted containers
  - [x] Get stats for one or more containers
    - [x] streaming
    - [x] non streaming
  - [ ] Generate a Kubernetes YAML file
  - [ ] Remove pods from play kube
  - [ ] Play a Kubernetes YAML file
- [ ] exec
  - [ ] Create an exec instance
  - [ ] Inspect an exec instance
  - [ ] Resize an exec instance
  - [ ] Start an exec instance
- [ ] images
  - [ ] Create image
  - [x] Remove an image from the local storage
  - [ ] Report on changes to image's filesystem; adds, deletes or modifications
  - [x] Image exists
  - [x] Export an image
  - [ ] History of an image
  - [x] Inspect an Image
  - [ ] Push image
  - [ ] Tag an image
  - [ ] Image tree
  - [ ] Untag an image
  - [ ] Export multiple images
  - [x] Import image
  - [x] List images
  - [x] Load image
  - [x] Prune unused images
  - [x] Pull images
  - [ ] Remove one or more images from the storage
  - [x] Search images
- [ ] manifests
  - [ ] Remove
  - [ ] Add image
  - [ ] Exists
  - [ ] Inspect
  - [ ] Push
  - [ ] Create
- [x] networks
  - [x] Remove a network
  - [x] Connect container to network
  - [x] Disconnect container from network
  - [x] Network exists
  - [x] Inspect a network
  - [x] Create network
  - [x] List networks
  - [x] Delete unused networks
- [ ] pods
  - [ ] Generate Systemd Units
  - [ ] Generate a Kubernetes YAML file
  - [x] Remove pod
  - [x] Pod exists
  - [x] Inspect pod
  - [x] Kill a pod
  - [x] Pause a pod
  - [x] Restart a pod
  - [x] Start a pod
  - [x] Stop a pod
  - [x] List processes
    - [x] streaming
    - [x] non streaming
  - [x] Unpause a pod
  - [x] Create a pod
  - [x] List pods
  - [x] Prune unused pods
  - [x] Get stats for one or more pods (currently broken, see Podman [issue](https://github.com/containers/podman/issues/13213))
- [x] volumes
  - [x] Remove volume
  - [x] Volume exists
  - [x] Inspect volume
  - [x] Create a volume
  - [x] List volumes
  - [x] Prune volumes
- [ ] secrets
  - [ ] Remove secret
  - [ ] Inspect secret
  - [ ] Create secret
  - [ ] List secrets
- [ ] system
  - [ ] Ping service
  - [ ] Get events
    - [x] streaming
    - [ ] non streaming
  - [x] Get info
  - [ ] Show disk usage
  - [ ] Prune unused data
  - [ ] Component version information

### Planned features
- [ ] Support DateTime Json fields (currently represented as `String`)
- [ ] Full support of the Podman API (Docker compat is out of scope)
