mod utils;
use futures::stream::StreamExt;
use podtender::error::PodtenderError;
use podtender::example_values_trait::ExampleValues;
use podtender::images::parameter_types::*;
use podtender::images::response_types::InspectImageResponse;

#[tokio::test]
async fn remove_from_example() {
    let podman_service = utils::setup();

    let parameter = RemoveImageParameter::example();

    let image_name = parameter.image_name.clone();

    utils::build_test_image_to_delete(&image_name);

    let podtender_result = podman_service.images().remove(parameter).await;

    match podtender_result {
        Ok(result) => {
            let mut found = false;
            for element in result.untagged.unwrap() {
                if element.contains(&format!("localhost/{}:latest", &image_name)) {
                    found = true;
                }
            }
            assert!(found, "Failed to find an image with the correct name");
        }
        Err(podtender_error) => {
            utils::print_path_if_serde_error(&podtender_error);
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn exists_from_example() {
    let podman_service = utils::setup();

    let parameter = ImageExistsParameter::example();
    let image_name = parameter.image_name.clone();

    utils::tag_image(utils::TESTCONTAINER_IMAGE_NAME, &image_name);

    let podtender_result = podman_service.images().exists(parameter).await;
    utils::untag_image(utils::TESTCONTAINER_IMAGE_NAME, &image_name);

    match podtender_result {
        Ok(_) => {}
        Err(podtender_error) => {
            utils::print_path_if_serde_error(&podtender_error);
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn does_not_exists_from_example() {
    let podman_service = utils::setup();

    let parameter = ImageExistsParameter {
        image_name: String::from("this_should_not_exist"),
    };

    let podtender_result = podman_service.images().exists(parameter).await;

    if podtender_result.is_ok() {
        panic!("Should not have found an image.");
    }
}

// Run separated in case of failure when running all tests.
#[tokio::test]
async fn export_from_example() {
    let podman_service = utils::setup();
    let parameter = ExportImageParameter::example();

    let image_name = parameter.image_name.clone();

    utils::tag_image(utils::TESTCONTAINER_IMAGE_NAME, &image_name);

    let export = podman_service.images().export(parameter).await;
    if let Err(err) = export {
        utils::print_path_if_serde_error(&err);
        utils::untag_image(utils::TESTCONTAINER_IMAGE_NAME, &image_name);
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
                utils::untag_image(utils::TESTCONTAINER_IMAGE_NAME, &image_name);
                panic!("{:#?}", err);
            }
        }
    }

    if !file.is_empty() {
        let check = utils::export_image(&image_name);
        let stdout = check.stdout;
        utils::untag_image(utils::TESTCONTAINER_IMAGE_NAME, &image_name);
        if file.len() == stdout.len() {
            return;
        }
    }
    utils::untag_image(utils::TESTCONTAINER_IMAGE_NAME, &image_name);
    panic!("No export got returned");
}

// Run separated in case of failure when running all tests.
#[tokio::test]
async fn inspect_from_example() {
    let podman_service = utils::setup();

    let parameter = InspectImageParameter::example();
    let image_name = parameter.image_name.clone();

    utils::tag_image(utils::TESTCONTAINER_IMAGE_NAME, &image_name);

    let podtender_result = podman_service.images().inspect(parameter).await;

    match podtender_result {
        Ok(mut result) => {
            let mut podman_command_result: Vec<InspectImageResponse> =
                serde_json::from_slice(utils::inspect_image(&image_name).stdout.as_slice())
                    .expect("failed to deserialize podman command output");
            utils::untag_image(utils::TESTCONTAINER_IMAGE_NAME, &image_name);
            let podman_command_result = podman_command_result.get_mut(0).unwrap();

            result.names_history = None;
            podman_command_result.names_history = None;
            result.repo_digests = None;
            podman_command_result.repo_digests = None;
            result.repo_tags = None;
            podman_command_result.repo_tags = None;

            assert_eq!(podman_command_result, &result);
        }
        Err(podtender_error) => {
            utils::print_path_if_serde_error(&podtender_error);
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn list_from_example() {
    let podman_service = utils::setup();
    let parameter = ListImagesParameter::example();
    let image_name = parameter
        .filters
        .as_ref()
        .unwrap()
        .get("reference")
        .unwrap()
        .get(0)
        .unwrap()
        .clone();

    utils::tag_image(utils::TESTCONTAINER_IMAGE_NAME, &image_name);

    let podtender_result = podman_service.images().list(parameter).await;

    match podtender_result {
        Ok(response) => {
            assert_eq!(1, response.len());
            for element in response {
                assert!(element
                    .names
                    .unwrap()
                    .contains(&format!("localhost/{}:latest", &image_name)));
            }
            utils::untag_image(utils::TESTCONTAINER_IMAGE_NAME, &image_name);
        }
        Err(podtender_error) => {
            utils::print_path_if_serde_error(&podtender_error);
            utils::untag_image(utils::TESTCONTAINER_IMAGE_NAME, &image_name);
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn prune_from_example() {
    let podman_service = utils::setup();
    let parameter = PruneImagesParameter::example();
    let image_name = String::from("prune_from_example");
    let label = parameter
        .filters
        .as_ref()
        .unwrap()
        .get("label")
        .as_ref()
        .unwrap()
        .get(0)
        .unwrap()
        .clone();
    utils::build_test_image_with_label(&image_name, &label);

    let podtender_result = podman_service.images().prune(parameter).await;
    match podtender_result {
        Ok(_) => {}
        Err(podtender_error) => {
            utils::print_path_if_serde_error(&podtender_error);
            utils::delete_image(&image_name);
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn pull_from_example() {
    let podman_service = utils::setup();

    let parameter = PullImagesParameter::example();
    let image_name = String::from(
        parameter
            .reference
            .as_ref()
            .unwrap()
            .clone()
            .split('/')
            .collect::<Vec<&str>>()
            .get(0)
            .unwrap()
            .to_owned(),
    );

    let podtender_result_stream = podman_service.images().pull(parameter).await;

    if let Err(err) = podtender_result_stream {
        utils::print_path_if_serde_error(&err);
        utils::untag_image(utils::TESTCONTAINER_IMAGE_NAME, &image_name);
        panic!("{:#?}", err);
    }
    let mut podtender_result_stream = podtender_result_stream.unwrap();

    while let Some(response_line) = podtender_result_stream.next().await {
        match response_line {
            Ok(line) => {
                if line.images.is_some() {
                    utils::untag_image(utils::TESTCONTAINER_IMAGE_NAME, &image_name);
                    return;
                }
            }
            Err(err) => {
                utils::untag_image(utils::TESTCONTAINER_IMAGE_NAME, &image_name);
                panic!("{:#?}", err);
            }
        }
    }
    utils::untag_image(utils::TESTCONTAINER_IMAGE_NAME, &image_name);
    panic!("No response returned");
}

#[tokio::test]
async fn search_from_example() {
    let podman_service = utils::setup();
    let parameter = SearchImagesParameter::example();

    let podtender_result = podman_service.images().search(parameter).await;
    match podtender_result {
        Ok(result) => {
            assert!(result
                .get(0)
                .as_ref()
                .unwrap()
                .name
                .as_ref()
                .unwrap()
                .contains("docker.io/library/hello-world"));
        }
        Err(podtender_error) => {
            utils::print_path_if_serde_error(&podtender_error);
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn import_from_example() {
    let podman_service = utils::setup();
    let parameter = ImportImageParameter::example();

    let podman_export = utils::export_image(utils::TESTCONTAINER_IMAGE_NAME).stdout;
    let podman_export: Vec<Result<Vec<u8>, PodtenderError>> = podman_export
        .iter()
        .map(|entry| Result::<Vec<u8>, PodtenderError>::Ok(vec![*entry]))
        .collect();
    let podman_export = futures::stream::iter(podman_export);

    let podtender_result = podman_service
        .images()
        .import(parameter, podman_export)
        .await;
    match podtender_result {
        Ok(result) => if result.id.is_some() {},
        Err(podtender_error) => {
            utils::print_path_if_serde_error(&podtender_error);
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn load() {
    let podman_service = utils::setup();

    let podman_export = utils::export_image(utils::TESTCONTAINER_IMAGE_NAME).stdout;
    let podman_export: Vec<Result<Vec<u8>, PodtenderError>> = podman_export
        .iter()
        .map(|entry| Result::<Vec<u8>, PodtenderError>::Ok(vec![*entry]))
        .collect();
    let podman_export = futures::stream::iter(podman_export);

    let podtender_result = podman_service.images().load(podman_export).await;
    match podtender_result {
        Ok(result) => {
            for name in result.names.unwrap().iter() {
                if name.contains(utils::TESTCONTAINER_IMAGE_NAME) {
                    return;
                }
            }
            panic!("could not find loaded container in returned names");
        }
        Err(podtender_error) => {
            utils::print_path_if_serde_error(&podtender_error);
            panic!("{:#?}", podtender_error);
        }
    }
}
