mod utils;
use futures::stream::StreamExt;
use podtender::example_values_trait::ExampleValues;
use podtender::system::parameter_types::EventsParameter;
use podtender::system::response_types::GetInfoResponse;
use std::collections::HashMap;

#[tokio::test]
async fn get_info() {
    let podman_service = utils::setup();

    let podtender_result = podman_service.system().get_info().await;

    match podtender_result {
        Ok(mut podtender_result) => {
            let mut podman_command_result: GetInfoResponse =
                serde_json::from_slice(utils::get_info().stdout.as_slice())
                    .expect("failed to deserialize podman command output");

            // Set remote_socket=None since cli and api return different results.
            podman_command_result.host.as_mut().unwrap().remote_socket = None;
            podtender_result.host.as_mut().unwrap().remote_socket = None;

            // uptime and free memory differ with every call to the api
            podman_command_result.host.as_mut().unwrap().uptime = None;
            podman_command_result.host.as_mut().unwrap().mem_free = None;
            podtender_result.host.as_mut().unwrap().uptime = None;
            podtender_result.host.as_mut().unwrap().mem_free = None;

            // container store can be different when running all tests
            podman_command_result
                .store
                .as_mut()
                .unwrap()
                .container_store = None;
            podtender_result.store.as_mut().unwrap().container_store = None;

            for (key_1, value_1) in podman_command_result
                .store
                .as_mut()
                .unwrap()
                .graph_status
                .as_ref()
                .unwrap()
            {
                assert_eq!(
                    value_1,
                    podtender_result
                        .store
                        .as_ref()
                        .unwrap()
                        .graph_status
                        .as_ref()
                        .unwrap()
                        .get(key_1)
                        .unwrap()
                );
            }
            podman_command_result
                .store
                .as_mut()
                .unwrap()
                .graph_status
                .as_mut()
                .unwrap()
                .clear();

            podtender_result
                .store
                .as_mut()
                .unwrap()
                .graph_status
                .as_mut()
                .unwrap()
                .clear();

            assert_eq!(podman_command_result, podtender_result);
        }
        Err(podtender_error) => {
            utils::print_path_if_serde_error(&podtender_error);
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn get_events_streaming() {
    let podman_service = utils::setup();
    let container_name = String::from("get_events_streaming");
    let mut filters = HashMap::new();
    filters.insert(String::from("container"), vec![container_name.clone()]);

    utils::create_container(&container_name);
    utils::delete_container(&container_name);

    let parameter = EventsParameter {
        filters: Some(filters),
        since: Some(String::from("3m")),
        stream: true,
        ..Default::default()
    };
    let mut events = podman_service
        .system()
        .get_events_streaming(parameter)
        .await
        .expect("Could not create events stream");
    while let Some(event) = events.next().await {
        match event {
            Ok(event) => {
                assert_eq!(
                    event
                        .actor
                        .attributes
                        .as_ref()
                        .unwrap()
                        .get("image")
                        .unwrap(),
                    &format!("localhost/{}:latest", utils::TESTCONTAINER_IMAGE_NAME)
                );
                assert_ne!(
                    event.actor.attributes.unwrap().get("image").unwrap(),
                    "not_what_should_be_the_image_name"
                );
                return;
            }
            Err(err) => {
                panic!("{:#?}", err);
            }
        }
    }
    panic!("No Events got returned");
}

#[tokio::test]
async fn get_events_streaming_from_example() {
    let podman_service = utils::setup();
    let parameter = EventsParameter::example();

    let filter_map = parameter.filters.as_ref().unwrap().clone();
    let container_vec = filter_map.get("container");
    let container_name = container_vec.unwrap().first().unwrap().to_owned();

    utils::create_container(&container_name);
    utils::delete_container(&container_name);

    let mut events = podman_service
        .system()
        .get_events_streaming(parameter)
        .await
        .expect("Could not create events stream");
    while let Some(event) = events.next().await {
        match event {
            Ok(event) => {
                assert_eq!(
                    event
                        .actor
                        .attributes
                        .as_ref()
                        .unwrap()
                        .get("image")
                        .unwrap(),
                    &format!("localhost/{}:latest", utils::TESTCONTAINER_IMAGE_NAME)
                );
                assert_ne!(
                    event.actor.attributes.unwrap().get("image").unwrap(),
                    "not_what_should_be_the_image_name"
                );
                return;
            }
            Err(err) => {
                panic!("{:#?}", err);
            }
        }
    }
    panic!("No Events got returned");
}
