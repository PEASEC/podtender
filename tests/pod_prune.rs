mod utils;

//In seperate file to avoid collisions with other stopped/exited pods
#[tokio::test]
async fn prune_pods_from_example() {
    let podman_service = utils::setup();

    let pod_id = utils::create_pod("prune_me");
    utils::start_pod(&pod_id);
    utils::stop_pod(&pod_id);

    let podtender_result = podman_service.pods().prune().await;

    utils::delete_pod("prune_me");

    match podtender_result {
        Ok(response) => {
            if response.is_empty() {
                panic!("No pods pruned");
            }
            for entry in response {
                assert_eq!(pod_id, entry.id.unwrap());
            }
        }
        Err(podtender_error) => {
            panic!("{:#?}", podtender_error);
        }
    }
}
