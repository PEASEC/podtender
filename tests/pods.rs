mod utils;

use futures::stream::StreamExt;
use podtender::example_values_trait::ExampleValues;
use podtender::pods::parameter_types::*;

#[tokio::test]
async fn create_pod_from_example() {
    let podman_service = utils::setup();

    let parameter = CreatePodParameter::example();
    let pod_name = parameter.name.as_ref().unwrap().clone();

    let podtender_result = podman_service.pods().create(parameter).await;

    match podtender_result {
        Ok(_) => {
            utils::delete_pod(&pod_name);
        }
        Err(podtender_error) => {
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn delete_pod_from_example() {
    let podman_service = utils::setup();

    let parameter = RemovePodParameter::example();
    let pod_name = parameter.pod_name.clone();
    utils::create_pod(pod_name.as_str());

    let podtender_result = podman_service.pods().remove(parameter).await;

    if let Err(err) = &podtender_result {
        utils::delete_pod(pod_name.as_str());
        panic!("{:#?}", err);
    }

    assert!(podtender_result.is_ok(), "Container deletion failed.");
}

#[tokio::test]
async fn pod_exists_from_example() {
    let podman_service = utils::setup();

    let parameter = PodExistsParameter::example();

    let pod_name = parameter.pod_name.clone();

    utils::create_pod(&pod_name);

    let podtender_result = podman_service.pods().exists(parameter).await;

    utils::delete_pod(&pod_name);

    match podtender_result {
        Ok(_) => {}
        Err(podtender_error) => {
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn pod_does_not_exists() {
    let podman_service = utils::setup();

    let parameter = PodExistsParameter {
        pod_name: String::from("pod_does_not_exists"),
    };

    let podtender_result = podman_service.pods().exists(parameter).await;

    match podtender_result {
        Ok(_) => {
            panic!("No pod should have been found when checking whether the pod exists.");
        }
        Err(_podtender_error) => {}
    }
}

#[tokio::test]
async fn pod_inspect_from_example() {
    let podman_service = utils::setup();

    let parameter = InspectPodParameter::example();

    let pod_name = parameter.pod_name.clone();

    utils::create_pod(&pod_name);

    let podtender_result = podman_service.pods().inspect(parameter).await;
    utils::delete_pod(&pod_name);
    match podtender_result {
        Ok(inspect_result) => {
            assert_eq!(inspect_result.name.unwrap(), pod_name);
        }
        Err(podtender_error) => {
            utils::print_path_if_serde_error(&podtender_error);
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn kill_pod_from_example() {
    let podman_service = utils::setup();

    let parameter = KillPodParameter::example();

    let pod_name = parameter.pod_name.clone();
    let container_name = String::from("kill_pod_from_example");

    let pod_id = utils::create_pod(&pod_name);
    utils::create_container_with_pod(&container_name, &pod_name);
    utils::start_pod(&pod_name);

    let podtender_result = podman_service.pods().kill(parameter).await;

    utils::delete_pod(&pod_name);
    match podtender_result {
        Ok(inspect_result) => {
            assert_eq!(inspect_result.id.unwrap(), pod_id);
        }
        Err(podtender_error) => {
            utils::print_path_if_serde_error(&podtender_error);
            panic!("{:#?}", podtender_error);
        }
    }
}
//Only supported with cgroups v2
#[tokio::test]
async fn pause_pod_from_example() {
    let podman_service = utils::setup();

    let parameter = PausePodParameter::example();

    let pod_name = parameter.pod_name.clone();
    let container_name = String::from("pause_pod_from_example");

    let pod_id = utils::create_pod(&pod_name);
    utils::create_container_with_pod(&container_name, &pod_name);
    utils::start_pod(&pod_name);

    let podtender_result = podman_service.pods().pause(parameter).await;

    utils::delete_pod(&pod_name);
    match podtender_result {
        Ok(result) => {
            assert_eq!(result.id.unwrap(), pod_id);
        }
        Err(podtender_error) => {
            utils::print_path_if_serde_error(&podtender_error);
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn restart_pod_from_example() {
    let podman_service = utils::setup();

    let parameter = RestartPodParameter::example();

    let pod_name = parameter.pod_name.clone();
    let container_name = String::from("restart_pod_from_example");

    let pod_id = utils::create_pod(&pod_name);
    utils::create_container_with_pod(&container_name, &pod_name);
    utils::start_pod(&pod_name);

    let podtender_result = podman_service.pods().restart(parameter).await;

    utils::delete_pod(&pod_name);
    match podtender_result {
        Ok(result) => {
            assert_eq!(result.id.unwrap(), pod_id);
        }
        Err(podtender_error) => {
            utils::print_path_if_serde_error(&podtender_error);
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn start_pod_from_example() {
    let podman_service = utils::setup();

    let parameter = StartPodParameter::example();

    let pod_name = parameter.pod_name.clone();
    let container_name = String::from("start_pod_from_example");

    let pod_id = utils::create_pod(&pod_name);
    utils::create_container_with_pod(&container_name, &pod_name);

    let podtender_result = podman_service.pods().start(parameter).await;

    utils::delete_pod(&pod_name);
    match podtender_result {
        Ok(result) => {
            assert_eq!(result.id.unwrap(), pod_id);
        }
        Err(podtender_error) => {
            utils::print_path_if_serde_error(&podtender_error);
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn stop_pod_from_example() {
    let podman_service = utils::setup();

    let parameter = StopPodParameter::example();

    let pod_name = parameter.pod_name.clone();
    let container_name = String::from("stop_pod_from_example");

    let pod_id = utils::create_pod(&pod_name);
    utils::create_container_with_pod(&container_name, &pod_name);
    utils::start_pod(&pod_name);

    let podtender_result = podman_service.pods().stop(parameter).await;

    utils::delete_pod(&pod_name);
    match podtender_result {
        Ok(result) => {
            assert_eq!(result.id.unwrap(), pod_id);
        }
        Err(podtender_error) => {
            utils::print_path_if_serde_error(&podtender_error);
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn list_processes() {
    let podman_service = utils::setup();
    let parameter = ListPodProcessesParameter {
        pod_name: String::from("pod_list_processes"),
        delay: None,
        ps_args: None,
        stream: Some(false),
    };

    let pod_name = parameter.pod_name.clone();
    let container_name = String::from("pod_list_processes_container");

    utils::create_pod(&pod_name);
    utils::create_container_with_pod(&container_name, &pod_name);
    utils::start_pod(&pod_name);

    let podtender_result = podman_service.pods().list_processes(parameter).await;

    utils::delete_pod(&pod_name);
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
    let parameter = ListPodProcessesParameter::example();

    let pod_name = parameter.pod_name.clone();
    let container_name = String::from("list_processes_from_example");

    utils::create_pod(&pod_name);
    utils::create_container_with_pod(&container_name, &pod_name);
    utils::start_pod(&pod_name);

    let mut processes = podman_service
        .pods()
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

                utils::delete_pod(&pod_name);
                return;
            }
            Err(err) => {
                utils::delete_pod(&pod_name);
                panic!("{:#?}", err);
            }
        }
    }
    utils::delete_pod(&pod_name);
    panic!("No processes got returned");
}

#[tokio::test]
async fn unpause_pod_from_example() {
    let podman_service = utils::setup();

    let parameter = UnpausePodParameter::example();

    let pod_name = parameter.pod_name.clone();
    let container_name = String::from("unpause_pod_from_example");

    let pod_id = utils::create_pod(&pod_name);
    utils::create_container_with_pod(&container_name, &pod_name);
    utils::start_pod(&pod_name);
    utils::pause_pod(&pod_name);

    let podtender_result = podman_service.pods().unpause(parameter).await;

    utils::delete_pod(&pod_name);
    match podtender_result {
        Ok(result) => {
            assert_eq!(result.id.unwrap(), pod_id);
        }
        Err(podtender_error) => {
            utils::print_path_if_serde_error(&podtender_error);
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn list_pods_from_example() {
    let podman_service = utils::setup();

    let parameter = ListPodsParameter::example();
    let label = String::from(
        parameter
            .filters
            .as_ref()
            .unwrap()
            .get("label")
            .as_ref()
            .unwrap()
            .get(0)
            .unwrap(),
    );

    let pod_name1 = String::from("list_pods1");
    let pod_name2 = String::from("list_pods2");
    let pod_name3 = String::from("list_pods3");

    utils::create_pod_with_label(&pod_name1, &label);
    utils::create_pod_with_label(&pod_name2, &label);
    utils::create_pod_with_label(&pod_name3, &label);

    let podtender_result = podman_service.pods().list(parameter).await;

    utils::delete_pod(&pod_name1);
    utils::delete_pod(&pod_name2);
    utils::delete_pod(&pod_name3);

    match podtender_result {
        Ok(response) => {
            assert_eq!(3, response.len());
            for element in response {
                assert!(element.name.unwrap().contains("list_pods"));
            }
        }
        Err(podtender_error) => {
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn multiple_pods_stats_streaming() {
    let podman_service = utils::setup();
    let parameter = PodStatsParameter::example();

    let pod_name = String::from("multiple_pods_stats_streaming_pod");
    let container_name = String::from("multiple_pods_stats_streaming");

    let mut pod_id = utils::create_pod(&pod_name);
    utils::create_container_with_pod(&container_name, &pod_name);
    utils::start_pod(&pod_name);

    let stats = podman_service.pods().stats(parameter).await;
    if let Err(err) = stats {
        utils::delete_pod(&pod_name);
        panic!("{:#?}", err);
    }
    let mut stats = stats.unwrap();

    // shorten pod id as used in podman
    pod_id.truncate(12);

    let mut found_container = false;
    let mut found_pod = false;
    while let Some(Ok(stats_vec)) = stats.next().await {
        for stats_entry in stats_vec {
            println!("{stats_entry:?}");
            if stats_entry.name.unwrap() == "multiple_pods_stats_streaming".to_owned() {
                found_container = true;
            }
            if stats_entry.pod.unwrap() == pod_id {
                found_pod = true;
            }
            assert!(stats_entry.cid.is_some());
        }
    }
    utils::delete_pod(&pod_name);
    assert!(found_container);
    assert!(found_pod);
}
