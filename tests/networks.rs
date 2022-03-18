use podtender::error::PodtenderError;
use podtender::networks::parameter_types::*;
use podtender::networks::response_types::InspectNetworkResponse;

use podtender::example_values_trait::ExampleValues;

use std::collections::HashMap;

mod utils;

#[tokio::test]
async fn remove_network() {
    let podman_service = utils::setup();
    let network_name = String::from("remove_network");

    utils::create_network(&network_name);

    let parameter = RemoveNetworkParameter {
        network_name: network_name.clone(),
        force: Some(true),
    };

    let podtender_result = podman_service.networks().remove(parameter).await;

    match podtender_result {
        Ok(result) => {
            assert_eq!(
                result.get(0).as_ref().unwrap().name.as_ref().unwrap(),
                &network_name
            );
        }
        Err(podtender_error) => {
            utils::delete_network(&network_name);
            utils::print_path_if_serde_error(&podtender_error);
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn remove_network_from_example() {
    let podman_service = utils::setup();

    let parameter = RemoveNetworkParameter::example();
    let network_name = parameter.network_name.clone();

    utils::create_network(&network_name);

    let podtender_result = podman_service.networks().remove(parameter).await;

    match podtender_result {
        Ok(result) => {
            assert_eq!(
                result.get(0).as_ref().unwrap().name.as_ref().unwrap(),
                &network_name
            );
        }
        Err(podtender_error) => {
            utils::delete_network(&network_name);
            utils::print_path_if_serde_error(&podtender_error);
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn connect_container_to_network_from_example() {
    let podman_service = utils::setup();

    let parameter = ConnectContainerParameter::example();

    let network_name = parameter.network_name.clone();
    let container_name = parameter.container.clone();

    //default network mode in rootless mode is/was slirp4netns but bridge is needed to connect containers
    let network_mode = String::from("bridge");

    utils::create_network_with_subnet(&network_name, "192.168.123.0/24");
    utils::create_container_with_network_mode(&container_name, &network_mode);
    utils::start_container(&container_name);

    let podtender_result = podman_service.networks().connect_container(parameter).await;

    utils::delete_container(&container_name);
    utils::delete_network(&network_name);

    match podtender_result {
        Ok(_) => {}
        Err(podtender_error) => {
            utils::print_path_if_serde_error(&podtender_error);
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn disconnect_container_from_network() {
    let podman_service = utils::setup();
    let network_name = String::from("disconnect_container_from_network");
    let container_name = String::from("disconnect_container_from_network_container");
    //default network mode in rootless mode is/was slirp4netns but bridge is needed to connect containers
    let network_mode = String::from("bridge");

    utils::create_network(&network_name);
    utils::create_container_with_network_mode(&container_name, &network_mode);
    utils::connect_container_to_network(&network_name, &container_name);

    let parameter = DisconnectContainerParameter {
        network_name: network_name.clone(),
        container: container_name.clone(),
        force: Some(true),
    };

    let podtender_result = podman_service
        .networks()
        .disconnect_container(parameter)
        .await;

    utils::delete_container(&container_name);
    utils::delete_network(&network_name);

    match podtender_result {
        Ok(_) => {}
        Err(podtender_error) => {
            utils::print_path_if_serde_error(&podtender_error);
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn disconnect_container_from_network_from_example() {
    let podman_service = utils::setup();

    let parameter = DisconnectContainerParameter::example();

    let network_name = parameter.network_name.clone();
    let container_name = parameter.container.clone();
    //default network mode in rootless mode is/was slirp4netns but bridge is needed to connect containers
    let network_mode = String::from("bridge");

    utils::create_network(&network_name);
    utils::create_container_with_network_mode(&container_name, &network_mode);
    utils::connect_container_to_network(&network_name, &container_name);

    let podtender_result = podman_service
        .networks()
        .disconnect_container(parameter)
        .await;

    utils::delete_container(&container_name);
    utils::delete_network(&network_name);

    match podtender_result {
        Ok(_) => {}
        Err(podtender_error) => {
            utils::print_path_if_serde_error(&podtender_error);
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn network_exists() {
    let podman_service = utils::setup();
    let network_name = String::from("network_exists");

    utils::create_network(&network_name);

    let parameter = NetworkExistsParameter {
        network_name: network_name.clone(),
    };

    let podtender_result = podman_service.networks().exists(parameter).await;

    utils::delete_network(&network_name);

    match podtender_result {
        Ok(_) => {}
        Err(podtender_error) => {
            utils::print_path_if_serde_error(&podtender_error);
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn network_exists_from_example() {
    let podman_service = utils::setup();

    let parameter = NetworkExistsParameter::example();

    let network_name = parameter.network_name.clone();

    utils::create_network(&network_name);

    let podtender_result = podman_service.networks().exists(parameter).await;

    utils::delete_network(&network_name);

    match podtender_result {
        Ok(_) => {}
        Err(podtender_error) => {
            utils::print_path_if_serde_error(&podtender_error);
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn network_does_not_exists() {
    let podman_service = utils::setup();
    let network_name = String::from("network_does_not_exists");

    let parameter = NetworkExistsParameter {
        network_name: network_name.clone(),
    };

    let podtender_error = podman_service
        .networks()
        .exists(parameter)
        .await
        .err()
        .expect("Network id/name existed or podman error.");

    if let PodtenderError::PodmanErrorResponse(err) = podtender_error {
        assert_eq!(404, err.response_code);
    } else {
        utils::print_path_if_serde_error(&podtender_error);
        panic!(
            "Wrong error, expected RequestError got {:#?}",
            podtender_error
        );
    }
}

#[tokio::test]
async fn network_create_from_example() {
    let podman_service = utils::setup();

    let parameter = CreateNetworkParameter::example();
    let network_name = parameter.name.as_ref().unwrap().clone();

    let podtender_result = podman_service.networks().create(parameter).await;
    utils::delete_network(&network_name);

    match podtender_result {
        Ok(_) => {}
        Err(podtender_error) => {
            utils::print_path_if_serde_error(&podtender_error);
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn list_networks() {
    let podman_service = utils::setup();
    let network_name1 = String::from("list_networks1");
    let network_name2 = String::from("list_networks2");
    let network_name3 = String::from("list_networks3");

    utils::create_network(&network_name1);
    utils::create_network(&network_name2);
    utils::create_network(&network_name3);

    let mut filter_map: HashMap<String, Vec<String>> = HashMap::new();
    filter_map.insert(
        String::from("name"),
        vec![
            String::from(&network_name1),
            String::from(&network_name2),
            String::from(&network_name3),
        ],
    );

    let parameter = ListNetworksParameter {
        filters: Some(filter_map),
    };

    let podtender_result = podman_service.networks().list(parameter).await;

    utils::delete_network(&network_name1);
    utils::delete_network(&network_name2);
    utils::delete_network(&network_name3);

    match podtender_result {
        Ok(response) => {
            assert_eq!(3, response.len());
            for element in response {
                assert!(element.name.unwrap().contains("list_networks"));
            }
        }
        Err(podtender_error) => {
            utils::print_path_if_serde_error(&podtender_error);
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn list_networks_from_example() {
    let podman_service = utils::setup();
    let network_name1 = String::from("list_networks_from_example1");
    let network_name2 = String::from("list_networks_from_example2");
    let network_name3 = String::from("list_networks_from_example3");

    let parameter = ListNetworksParameter::example();

    let filter_map = parameter.filters.as_ref().unwrap().clone();
    let label_vec = filter_map.get("label");
    let label = label_vec.unwrap().first().unwrap().to_owned();

    utils::create_network_with_label(&network_name1, &label);
    utils::create_network_with_label(&network_name2, &label);
    utils::create_network_with_label(&network_name3, &label);

    let podtender_result = podman_service.networks().list(parameter).await;

    utils::delete_network(&network_name1);
    utils::delete_network(&network_name2);
    utils::delete_network(&network_name3);

    match podtender_result {
        Ok(response) => {
            assert_eq!(3, response.len());
            for element in response {
                assert!(element.name.unwrap().contains("list_networks_from_example"));
            }
        }
        Err(podtender_error) => {
            utils::print_path_if_serde_error(&podtender_error);
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn prune_networks_no_example() {
    let podman_service = utils::setup();
    let network_name1 = String::from("prune_networks_no_example1");
    let network_name2 = String::from("prune_networks_no_example2");
    let network_name3 = String::from("prune_networks_no_example3");

    let label = String::from("prune=yes");

    utils::create_network_with_label(&network_name1, &label);
    utils::create_network_with_label(&network_name2, &label);
    utils::create_network_with_label(&network_name3, &label);

    let mut filter_map: HashMap<String, Vec<String>> = HashMap::new();
    filter_map.insert(String::from("label"), vec![String::from("prune")]);

    let parameter = PruneNetworksParameter {
        filters: Some(filter_map),
    };

    let podtender_result = podman_service.networks().prune(parameter).await;

    utils::delete_network(&network_name1);
    utils::delete_network(&network_name2);
    utils::delete_network(&network_name3);

    match podtender_result {
        Ok(response) => {
            assert_eq!(3, response.len());
            for element in response {
                assert!(element.name.unwrap().contains("prune_networks"));
            }
        }
        Err(podtender_error) => {
            utils::print_path_if_serde_error(&podtender_error);
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn prune_networks_from_example() {
    let podman_service = utils::setup();
    let network_name1 = String::from("prune_networks_from_example1");
    let network_name2 = String::from("prune_networks_from_example2");
    let network_name3 = String::from("prune_networks_from_example3");

    let parameter = PruneNetworksParameter::example();
    let filter_map = parameter.filters.as_ref().unwrap().clone();
    let label_vec = filter_map.get("label");
    let label = label_vec.unwrap().first().unwrap().to_owned();

    utils::create_network_with_label(&network_name1, &label);
    utils::create_network_with_label(&network_name2, &label);
    utils::create_network_with_label(&network_name3, &label);

    let podtender_result = podman_service.networks().prune(parameter).await;

    utils::delete_network(&network_name1);
    utils::delete_network(&network_name2);
    utils::delete_network(&network_name3);

    match podtender_result {
        Ok(response) => {
            assert_eq!(3, response.len());
            for element in response {
                assert!(element
                    .name
                    .unwrap()
                    .contains("prune_networks_from_example"));
            }
        }
        Err(podtender_error) => {
            utils::print_path_if_serde_error(&podtender_error);
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn network_inspect() {
    let podman_service = utils::setup();
    let network_name = String::from("network_inspect");

    utils::create_network(&network_name);

    let parameter = InspectNetworkParameter {
        network_name: network_name.clone(),
    };

    let podtender_result = podman_service.networks().inspect(parameter).await;
    if let Err(x) = &podtender_result {
        utils::print_path_if_serde_error(x);
    }

    let podman_command_result: Result<Vec<InspectNetworkResponse>, _> =
        serde_json::from_slice(utils::inspect_network(&network_name).stdout.as_slice());
    let podman_command_result = {
        match podman_command_result {
            Ok(result) => result,
            Err(err) => {
                panic!("Serializing podman command output failed: {}", err);
            }
        }
    };
    let podman_command_entry = podman_command_result
        .get(0)
        .expect("No results returned by podman command");
    utils::delete_network(&network_name);
    match podtender_result {
        Ok(result) => {
            assert_eq!(podman_command_entry, &result);
        }
        Err(podtender_error) => {
            utils::print_path_if_serde_error(&podtender_error);
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn network_inspect_from_example() {
    let podman_service = utils::setup();

    let parameter = InspectNetworkParameter::example();
    let network_name = parameter.network_name.clone();

    utils::create_network(&network_name);

    let podman_command_result: Result<Vec<InspectNetworkResponse>, _> =
        serde_json::from_slice(utils::inspect_network(&network_name).stdout.as_slice());
    let podman_command_result = {
        match podman_command_result {
            Ok(result) => result,
            Err(err) => {
                panic!("Serializing podman command output failed: {}", err);
            }
        }
    };
    let podman_command_entry = podman_command_result
        .get(0)
        .expect("No results returned by podman command");

    let podtender_result = podman_service.networks().inspect(parameter).await;
    utils::delete_network(&network_name);
    match podtender_result {
        Ok(result) => {
            assert_eq!(podman_command_entry, &result);
        }
        Err(podtender_error) => {
            utils::print_path_if_serde_error(&podtender_error);
            panic!("{:#?}", podtender_error);
        }
    }
}
