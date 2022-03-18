mod utils;
use futures::stream::StreamExt;
use podtender::containers::parameter_types::*;
use podtender::containers::response_types::InspectContainerResponse;
use podtender::error::PodtenderError;
use podtender::example_values_trait::ExampleValues;
use serial_test::serial;
use std::collections::HashMap;

#[tokio::test]
async fn create_container_from_example() {
    let podman_service = utils::setup();

    let parameter = CreateContainerParameter {
        image: Some(String::from(utils::TESTCONTAINER_IMAGE_NAME)),
        ..CreateContainerParameter::example()
    };

    let podtender_result = podman_service.containers().create(parameter.clone()).await;

    utils::delete_container(&parameter.name.unwrap());

    if let Err(err) = podtender_result {
        panic!("{:#?}", err);
    }
}

#[tokio::test]
async fn create_container_from_image_with_name() {
    let podman_service = utils::setup();
    let container_name = String::from("create_container_from_image_with_name");

    let parameter = CreateContainerParameter {
        name: Some(container_name.clone()),
        image: Some(String::from(utils::TESTCONTAINER_IMAGE_NAME)),
        ..Default::default()
    };

    let podtender_result = podman_service.containers().create(parameter.clone()).await;

    utils::delete_container(&parameter.name.unwrap());

    if let Err(err) = podtender_result {
        panic!("{:#?}", err);
    }
}

#[tokio::test]
async fn create_container_from_image_with_network() {
    let podman_service = utils::setup();
    let network_name = String::from("create_container_from_image_with_network");

    let mut network: HashMap<String, PerNetworkOptions> = HashMap::new();
    network.insert(network_name.clone(), PerNetworkOptions::default());

    utils::create_network(network_name.as_str());

    let parameter = CreateContainerParameter {
        image: Some(String::from(utils::TESTCONTAINER_IMAGE_NAME)),
        netns: Some(Namespace {
            nsmode: Some("bridge".to_owned()),
            value: None,
        }),
        networks: Some(network),
        ..Default::default()
    };

    let podtender_result = podman_service.containers().create(parameter).await;

    match podtender_result {
        Ok(podtender_result) => {
            utils::delete_container(&podtender_result.id);
            utils::delete_network(network_name.as_str());
        }
        Err(podtender_error) => {
            utils::delete_network(network_name.as_str());
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn create_container_from_image_with_network_and_portmapping() {
    let podman_service = utils::setup();
    let network_name = String::from("create_container_from_image_with_network_and_portmapping");

    let mut network: HashMap<String, PerNetworkOptions> = HashMap::new();
    network.insert(network_name.clone(), PerNetworkOptions::default());

    utils::create_network(network_name.as_str());

    let portmapping = PortMapping {
        container_port: Some(1234),
        host_ip: Some(String::from("127.0.0.1")),
        host_port: Some(1234),
        protocol: Some(String::from("tcp,udp,sctp")),
        range: Some(1),
    };

    let parameter = CreateContainerParameter {
        networks: Some(network),
        image: Some(String::from(utils::TESTCONTAINER_IMAGE_NAME)),
        netns: Some(Namespace {
            nsmode: Some("bridge".to_owned()),
            value: None,
        }),
        portmappings: Some(vec![portmapping]),
        ..Default::default()
    };

    let podtender_result = podman_service.containers().create(parameter).await;

    match podtender_result {
        Ok(podtender_result) => {
            utils::delete_container(&podtender_result.id);
            utils::delete_network(network_name.as_str());
        }
        Err(podtender_error) => {
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
#[serial(list_container)]
async fn list_container_from_example() {
    let podman_service = utils::setup();
    let container_name = String::from("list_container_from_example");
    let label = String::from("list_example");
    utils::create_container_with_label(&container_name, &label);

    let parameter = ListContainersParameter::example();

    let podtender_result = podman_service.containers().list(parameter).await;

    utils::delete_container(&container_name);

    match podtender_result {
        Ok(response) => {
            assert_eq!(1, response.len());
            for element in response {
                assert!(element.names.unwrap().contains(&container_name));
            }
        }
        Err(podtender_error) => {
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
#[serial(list_container)]
async fn list_container_from_example_wrong_label() {
    let podman_service = utils::setup();
    let container_name = String::from("list_container_from_example_wrong_label");
    let label = String::from("wrong");
    utils::create_container_with_label(&container_name, &label);

    let parameter = ListContainersParameter::example();

    let podtender_result = podman_service.containers().list(parameter).await;

    utils::delete_container(&container_name);

    match podtender_result {
        Ok(response) => {
            assert_eq!(0, response.len());
        }
        Err(podtender_error) => {
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn list_container_with_portmapping() {
    let podman_service = utils::setup();
    let container_name = String::from("list_container_with_portmapping");

    utils::create_container_with_portmapping(&container_name, "8080:8080");

    let parameter = ListContainersParameter {
        all: Some(true),
        ..Default::default()
    };

    let podtender_result = podman_service.containers().list(parameter).await;

    utils::delete_container(&container_name);

    match podtender_result {
        Ok(response) => {
            let mut found = false;
            for element in response {
                if element.names.unwrap().contains(&container_name) {
                    found = true;
                }
            }
            assert!(found, "Failed to find a container with the correct name");
        }
        Err(podtender_error) => {
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
#[serial(list_container)]
async fn list_containers() {
    let podman_service = utils::setup();
    let container_name = String::from("list_containers");
    let label = String::from("list=yes");

    utils::create_container_with_label(&container_name, &label);

    let mut filter_map: HashMap<String, Vec<String>> = HashMap::new();
    filter_map.insert(String::from("label"), vec![String::from("list")]);

    // without all, only running containers are shown
    let parameter = ListContainersParameter {
        all: Some(true),
        filters: Some(filter_map),
        ..Default::default()
    };

    let podtender_result = podman_service.containers().list(parameter).await;

    match podtender_result {
        Ok(response) => {
            assert_eq!(1, response.len());
            for element in response {
                assert!(element
                    .names
                    .unwrap()
                    .contains(&String::from("list_containers")));
            }
            utils::delete_container(&container_name);
        }
        Err(podtender_error) => {
            utils::delete_container(&container_name);
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn delete_container_from_example() {
    let podman_service = utils::setup();

    let parameter = DeleteContainerParameter::example();
    let container_name = parameter.container_name.clone();
    utils::run_container(container_name.as_str());

    let podtender_result = podman_service.containers().delete(parameter).await;

    if let Err(err) = &podtender_result {
        utils::delete_container(container_name.as_str());
        panic!("{:#?}", err);
    }

    assert!(podtender_result.is_ok(), "Container deletion failed.");
}

#[tokio::test]
async fn delete_no_container() {
    let podman_service = utils::setup();
    let container_name = String::from("delete_no_container");

    let parameter = DeleteContainerParameter {
        container_name,
        ..Default::default()
    };

    let podtender_result = podman_service
        .containers()
        .delete(parameter)
        .await
        .err()
        .expect("Container id/name existed or podman error.");

    if let PodtenderError::PodmanErrorResponse(err) = podtender_result {
        assert_eq!(404, err.response_code);
    } else {
        panic!(
            "Wrong error, expected PodmanErrorResponse got {:#?}",
            podtender_result
        )
    }
}

#[tokio::test]
async fn delete_container_with_force() {
    let podman_service = utils::setup();
    let container_name = String::from("delete_container_with_force");

    utils::run_container(container_name.as_str());

    let parameter = DeleteContainerParameter {
        container_name: container_name.clone(),
        force: Some(true),
        ..Default::default()
    };

    let podtender_result = podman_service.containers().delete(parameter).await;
    if let Err(err) = &podtender_result {
        utils::delete_container(container_name.as_str());
        panic!("{:#?}", err);
    }

    assert!(podtender_result.is_ok(), "Container deletion failed.");
}

#[tokio::test]
async fn delete_container() {
    let podman_service = utils::setup();
    let container_name = String::from("delete_container");

    utils::run_container(container_name.as_str());
    utils::stop_container(container_name.as_str());

    let parameter = DeleteContainerParameter {
        container_name: container_name.clone(),
        ..Default::default()
    };

    let podtender_result = podman_service.containers().delete(parameter).await;
    if let Err(err) = &podtender_result {
        utils::delete_container(container_name.as_str());
        panic!("{:#?}", err);
    }

    assert!(podtender_result.is_ok(), "Container deletion failed.");
}

#[tokio::test]
async fn start_container_from_example() {
    let podman_service = utils::setup();

    let parameter = StartContainerParameter::example();

    let container_name = parameter.container_name.clone();
    utils::create_container(container_name.as_str());

    let podtender_result = podman_service.containers().start(parameter).await;
    if let Err(err) = &podtender_result {
        println!("{:#?}", err);
    }
    utils::stop_container(container_name.as_str());
    utils::delete_container(container_name.as_str());

    assert!(podtender_result.is_ok(), "Container start failed.");
}

#[tokio::test]
async fn start_container() {
    let podman_service = utils::setup();
    let container_name = String::from("start_container");

    utils::create_container(container_name.as_str());

    let parameter = StartContainerParameter {
        container_name: container_name.clone(),
        detach_keys: Some(String::from("ctrl-p,ctrl-q")),
    };

    let podtender_result = podman_service.containers().start(parameter).await;
    if let Err(err) = &podtender_result {
        println!("{:#?}", err);
    }
    utils::stop_container(container_name.as_str());
    utils::delete_container(container_name.as_str());

    assert!(podtender_result.is_ok(), "Container start failed.");
}

#[tokio::test]
async fn stop_container_from_example() {
    let podman_service = utils::setup();

    let parameter = StopContainerParameter::example();
    let container_name = parameter.container_name.clone();

    utils::run_container(container_name.as_str());

    match podman_service.containers().stop(parameter).await {
        Ok(_) => {
            utils::delete_container(container_name.as_str());
        }
        Err(err) => {
            utils::delete_container(container_name.as_str());
            panic!("{:#?}", err);
        }
    }
}

#[tokio::test]
async fn stop_container() {
    let podman_service = utils::setup();
    let container_name = String::from("stop_container");

    utils::run_container(container_name.as_str());

    let parameter = StopContainerParameter {
        container_name: container_name.clone(),
        all: Some(false),
        ignore: Some(false),
        timeout: Some(5),
    };

    match podman_service.containers().stop(parameter).await {
        Ok(_) => {
            utils::delete_container(container_name.as_str());
        }
        Err(err) => {
            utils::delete_container(container_name.as_str());
            panic!("{:#?}", err);
        }
    }
}

// Needs root for CRIU to be able to checkpoint a container
#[tokio::test]
async fn checkpoint_container_from_example() {
    let podman_service = utils::setup();
    let parameter = CheckpointContainerParameter::example();

    let container_name = parameter.container_name.clone();

    utils::run_container(&container_name);

    let export = podman_service.containers().checkpoint(parameter).await;
    if let Err(err) = export {
        utils::print_path_if_serde_error(&err);
        utils::delete_container(&container_name);
        panic!("{:#?}", err);
    }
    let mut export = export.unwrap();

    let mut file: Vec<u8> = vec![];

    while let Some(chunk) = export.next().await {
        match chunk {
            Ok(mut chunk) => {
                file.append(&mut chunk);
            }
            Err(err) => {
                utils::delete_container(&container_name);
                panic!("{:#?}", err);
            }
        }
    }
    utils::delete_container(&container_name);
    if !file.is_empty() {
        return;
    }
    panic!("No export got returned");
}

#[tokio::test]
async fn container_exists_from_example() {
    let podman_service = utils::setup();

    let parameter = ContainerExistsParameter::example();

    let container_name = parameter.container_name.clone();
    let container_name_does_not_exist =
        String::from("container_exists_from_example_does_not_exist");
    let parameter_does_not_exist = ContainerExistsParameter {
        container_name: container_name_does_not_exist.clone(),
    };

    utils::create_container(&container_name);

    let podtender_result = podman_service.containers().exists(parameter).await;
    let podtender_result_not_exist = podman_service
        .containers()
        .exists(parameter_does_not_exist)
        .await;

    utils::delete_container(&container_name);

    match podtender_result {
        Ok(_) => {}
        Err(podtender_error) => {
            utils::print_path_if_serde_error(&podtender_error);
            panic!("{:#?}", podtender_error);
        }
    }

    if podtender_result_not_exist.is_ok() {
        panic!("container found which should not exist");
    }
}

#[tokio::test]
async fn export_container_from_example() {
    let podman_service = utils::setup();
    let parameter = ExportContainerParameter::example();

    let container_name = parameter.container_name.clone();

    utils::run_container(&container_name);

    let export = podman_service.containers().export(parameter).await;
    if let Err(err) = export {
        utils::print_path_if_serde_error(&err);
        utils::delete_container(&container_name);
        panic!("{:#?}", err);
    }
    let mut export = export.unwrap();

    let mut file: Vec<u8> = vec![];

    while let Some(chunk) = export.next().await {
        match chunk {
            Ok(mut chunk) => {
                file.append(&mut chunk);
            }
            Err(err) => {
                utils::delete_container(&container_name);
                panic!("{:#?}", err);
            }
        }
    }

    if !file.is_empty() {
        let check = utils::export_container(&container_name);
        let stdout = check.stdout;
        utils::delete_container(&container_name);
        if file == stdout {
            return;
        }
        panic!("Exports do not match.")
    }
    utils::delete_container(&container_name);
    panic!("No export got returned");
}

#[tokio::test]
async fn healthcheck_container_from_example() {
    let podman_service = utils::setup();
    let parameter = HealthcheckContainerParameter::example();
    let container_name = parameter.container_name.clone();

    utils::run_container(&container_name);

    match podman_service.containers().healthcheck(parameter).await {
        Ok(_) => {
            utils::delete_container(container_name.as_str());
        }
        Err(err) => {
            utils::delete_container(container_name.as_str());
            utils::print_path_if_serde_error(&err);
            panic!("{:#?}", err);
        }
    }
}

#[tokio::test]
async fn initialize_container_from_example() {
    let podman_service = utils::setup();
    let parameter = InitializeContainerParameter::example();
    let container_name = parameter.container_name.clone();

    utils::create_container(&container_name);

    match podman_service.containers().initialize(parameter).await {
        Ok(_) => {
            utils::delete_container(container_name.as_str());
        }
        Err(err) => {
            utils::delete_container(container_name.as_str());
            utils::print_path_if_serde_error(&err);
            panic!("{:#?}", err);
        }
    }
}

#[tokio::test]
async fn container_inspect_from_example() {
    let podman_service = utils::setup();

    let parameter = InspectContainerParameter::example();
    let container_name = parameter.container_name.clone();

    utils::create_container(&container_name);

    let podtender_result = podman_service.containers().inspect(parameter).await;

    match podtender_result {
        Ok(mut result) => {
            let mut podman_command_result: Vec<InspectContainerResponse> =
                serde_json::from_slice(utils::inspect_container(&container_name).stdout.as_slice())
                    .expect("failed to deserialize podman command output");
            utils::delete_container(&container_name);
            let podman_command_result = podman_command_result.get_mut(0).unwrap();
            podman_command_result.size_rootfs = None;
            result.size_rootfs = None;
            podman_command_result.size_rw = None;
            result.size_rw = None;

            assert_eq!(podman_command_result, &result);
        }
        Err(podtender_error) => {
            utils::print_path_if_serde_error(&podtender_error);
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn kill_container_from_example() {
    let podman_service = utils::setup();
    let parameter = KillContainerParameter::example();
    let container_name = parameter.container_name.clone();

    utils::create_container(&container_name);
    utils::start_container(&container_name);

    let result = podman_service.containers().kill(parameter).await;

    utils::delete_container(container_name.as_str());

    if let Err(err) = result {
        utils::print_path_if_serde_error(&err);
        panic!("{:#?}", err);
    }
}

#[tokio::test]
async fn logs_container_from_example() {
    let podman_service = utils::setup();
    let parameter = ContainerLogsParameter::example();

    let container_name = parameter.container_name.clone();

    utils::create_container(&container_name);
    utils::start_container(&container_name);

    let logs = podman_service.containers().logs(parameter).await;
    if let Err(err) = logs {
        utils::print_path_if_serde_error(&err);
        utils::delete_container(&container_name);
        panic!("{:#?}", err);
    }
    let mut logs = logs.unwrap();
    let mut counter: usize = 0;

    while let Some(Ok(line)) = logs.next().await {
        if !line.contains("Container still running.") {
            utils::delete_container(&container_name);
            panic!("Logs don't contain expected string.")
        }
        if counter > 3 {
            break;
        }
        counter += 1;
    }
    utils::delete_container(&container_name);
    if counter <= 3 {
        panic!("Failed to read as much logs as intended")
    }
}

#[tokio::test]
async fn mount_container_from_example() {
    let podman_service = utils::setup();
    let parameter = MountContainerParameter::example();
    let container_name = parameter.container_name.clone();

    utils::create_container(&container_name);
    utils::start_container(&container_name);

    match podman_service.containers().mount(parameter).await {
        Ok(_) => {
            utils::delete_container(container_name.as_str());
        }
        Err(err) => {
            utils::delete_container(container_name.as_str());
            utils::print_path_if_serde_error(&err);
            panic!("{:#?}", err);
        }
    }
}

#[tokio::test]
async fn list_processes() {
    let podman_service = utils::setup();
    let parameter = ListContainerProcessesParameter {
        container_name: String::from("pod_list_processes"),
        delay: None,
        ps_args: None,
        stream: Some(false),
    };

    let container_name = parameter.container_name.clone();

    utils::create_container(&container_name);
    utils::start_container(&container_name);

    let podtender_result = podman_service.containers().list_processes(parameter).await;

    utils::delete_container(&container_name);
    match podtender_result {
        Ok(result) => {
            assert_eq!(
                vec![
                    String::from("USER"),
                    String::from("PID"),
                    String::from("PPID"),
                    String::from("%CPU"),
                    String::from("ELAPSED"),
                    String::from("TTY"),
                    String::from("TIME"),
                    String::from("COMMAND")
                ],
                result.titles.unwrap()
            );
        }
        Err(podtender_error) => {
            utils::print_path_if_serde_error(&podtender_error);
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn list_processes_from_example_streaming() {
    let podman_service = utils::setup();
    let parameter = ListContainerProcessesParameter::example();

    let container_name = parameter.container_name.clone();

    utils::create_container(&container_name);
    utils::start_container(&container_name);

    let mut processes = podman_service
        .containers()
        .list_processes_streaming(parameter)
        .await
        .expect("Could not create processes stream");

    while let Some(process_entry) = processes.next().await {
        match process_entry {
            Ok(process_entry) => {
                assert_eq!(
                    vec![
                        String::from("USER"),
                        String::from("PID"),
                        String::from("PPID"),
                        String::from("%CPU"),
                        String::from("ELAPSED"),
                        String::from("TTY"),
                        String::from("TIME"),
                        String::from("COMMAND")
                    ],
                    process_entry.titles.unwrap()
                );

                utils::delete_container(&container_name);
                return;
            }
            Err(err) => {
                utils::delete_container(&container_name);
                panic!("{:#?}", err);
            }
        }
    }
    utils::delete_container(&container_name);
    panic!("No processes got returned");
}

#[tokio::test]
async fn pause_container_from_example() {
    let podman_service = utils::setup();
    let parameter = PauseContainerParameter::example();
    let container_name = parameter.container_name.clone();

    utils::create_container(&container_name);
    utils::start_container(&container_name);

    match podman_service.containers().pause(parameter).await {
        Ok(_) => {
            utils::delete_container(container_name.as_str());
        }
        Err(err) => {
            utils::delete_container(container_name.as_str());
            utils::print_path_if_serde_error(&err);
            panic!("{:#?}", err);
        }
    }
}

#[tokio::test]
async fn rename_container_from_example() {
    let podman_service = utils::setup();
    let parameter = RenameContainerParameter::example();
    let container_name = parameter.container_name.clone();
    let container_renamed_name = parameter.rename_to.clone();

    utils::create_container(&container_name);
    utils::start_container(&container_name);

    match podman_service.containers().rename(parameter).await {
        Ok(_) => {
            utils::delete_container(container_renamed_name.as_str());
        }
        Err(err) => {
            utils::delete_container(container_name.as_str());
            utils::print_path_if_serde_error(&err);
            panic!("{:#?}", err);
        }
    }
}

#[tokio::test]
async fn restart_container_from_example() {
    let podman_service = utils::setup();
    let parameter = RestartContainerParameter::example();
    let container_name = parameter.container_name.clone();

    utils::create_container(&container_name);
    utils::start_container(&container_name);

    match podman_service.containers().restart(parameter).await {
        Ok(_) => {
            utils::delete_container(container_name.as_str());
        }
        Err(err) => {
            utils::delete_container(container_name.as_str());
            utils::print_path_if_serde_error(&err);
            panic!("{:#?}", err);
        }
    }
}

#[tokio::test]
async fn unmount_container_from_example() {
    let podman_service = utils::setup();
    let parameter = UnmountContainerParameter::example();
    let container_name = parameter.container_name.clone();

    utils::create_container(&container_name);

    podman_service
        .containers()
        .mount(MountContainerParameter {
            container_name: container_name.clone(),
        })
        .await
        .unwrap();

    match podman_service.containers().unmount(parameter).await {
        Ok(_) => {
            utils::delete_container(container_name.as_str());
        }
        Err(err) => {
            utils::delete_container(container_name.as_str());
            utils::print_path_if_serde_error(&err);
            panic!("{:#?}", err);
        }
    }
}

#[tokio::test]
async fn unpause_container_from_example() {
    let podman_service = utils::setup();
    let parameter = UnpauseContainerParameter::example();
    let container_name = parameter.container_name.clone();

    utils::create_container(&container_name);
    utils::start_container(&container_name);
    utils::pause_container(&container_name);

    match podman_service.containers().unpause(parameter).await {
        Ok(_) => {
            utils::delete_container(container_name.as_str());
        }
        Err(err) => {
            utils::delete_container(container_name.as_str());
            utils::print_path_if_serde_error(&err);
            panic!("{:#?}", err);
        }
    }
}

#[tokio::test]
async fn stats_from_example_streaming() {
    let podman_service = utils::setup();
    let parameter = ContainersStatsParameter::example();

    let container_name1 = parameter.container_names.get(0).unwrap().clone();
    let container_name2 = parameter.container_names.get(1).unwrap().clone();

    utils::create_container(&container_name1);
    utils::start_container(&container_name1);
    utils::create_container(&container_name2);
    utils::start_container(&container_name2);

    let stats = podman_service.containers().stats_stream(parameter).await;
    if let Err(err) = stats {
        utils::print_path_if_serde_error(&err);
        utils::delete_container(&container_name1);
        utils::delete_container(&container_name2);
        panic!("{:#?}", err);
    }

    let mut stats = stats.unwrap();

    while let Some(stats_entry) = stats.next().await {
        match stats_entry {
            Ok(stats_entry) => {
                utils::delete_container(&container_name1);
                utils::delete_container(&container_name2);

                assert!(stats_entry
                    .stats
                    .as_ref()
                    .unwrap()
                    .get(0)
                    .as_ref()
                    .unwrap()
                    .name
                    .as_ref()
                    .unwrap()
                    .contains("ContainersStatsParameter"));
                assert!(stats_entry
                    .stats
                    .as_ref()
                    .unwrap()
                    .get(1)
                    .as_ref()
                    .unwrap()
                    .name
                    .as_ref()
                    .unwrap()
                    .contains("ContainersStatsParameter"));
                return;
            }
            Err(err) => {
                utils::delete_container(&container_name1);
                utils::delete_container(&container_name2);
                panic!("{:#?}", err);
            }
        }
    }
    panic!("No processes got returned");
}

#[tokio::test]
async fn stats_from_example() {
    let podman_service = utils::setup();

    let container_name1 = String::from("stats_from_example_1");
    let container_name2 = String::from("stats_from_example_2");
    let parameter = ContainersStatsParameter {
        container_names: vec![container_name1.clone(), container_name2.clone()],
        ..ContainersStatsParameter::example()
    };

    utils::create_container(&container_name1);
    utils::start_container(&container_name1);
    utils::create_container(&container_name2);
    utils::start_container(&container_name2);

    let stats = podman_service.containers().stats(parameter).await;
    if let Err(err) = stats {
        utils::print_path_if_serde_error(&err);
        utils::delete_container(&container_name1);
        utils::delete_container(&container_name2);
        panic!("{:#?}", err);
    }

    let stats = stats.unwrap();

    utils::delete_container(&container_name1);
    utils::delete_container(&container_name2);

    assert!(stats
        .stats
        .as_ref()
        .unwrap()
        .get(0)
        .as_ref()
        .unwrap()
        .name
        .as_ref()
        .unwrap()
        .contains("stats_from_example"));
    assert!(stats
        .stats
        .as_ref()
        .unwrap()
        .get(1)
        .as_ref()
        .unwrap()
        .name
        .as_ref()
        .unwrap()
        .contains("stats_from_example"));
}

#[tokio::test]
async fn prune_containers_from_example() {
    let podman_service = utils::setup();
    let container_name1 = String::from("prune_containers_from_example1");
    let container_name2 = String::from("prune_containers_from_example2");
    let container_name3 = String::from("prune_containers_from_example3");

    let parameter = PruneContainersParameter::example();

    let filter_map = parameter.filters.as_ref().unwrap().clone();
    let label_vec = filter_map.get("label");
    let label = label_vec.unwrap().first().unwrap().to_owned();

    utils::create_container_with_label(&container_name1, &label);
    utils::create_container_with_label(&container_name2, &label);
    utils::create_container_with_label(&container_name3, &label);

    let podtender_result = podman_service.containers().prune(parameter).await;

    match podtender_result {
        Ok(response) => {
            assert_eq!(3, response.len());
        }
        Err(podtender_error) => {
            utils::delete_container(&container_name1);
            utils::delete_container(&container_name2);
            utils::delete_container(&container_name3);
            utils::print_path_if_serde_error(&podtender_error);
            panic!("{:#?}", podtender_error);
        }
    }
}
