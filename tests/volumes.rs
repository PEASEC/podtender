mod utils;
use podtender::error::PodtenderError;
use podtender::example_values_trait::ExampleValues;
use podtender::volumes::parameter_types::*;
use std::collections::HashMap;

#[tokio::test]
async fn create_volume() {
    let podman_service = utils::setup();
    let volume_name = String::from("create_volume");
    let mut label = HashMap::new();
    label.insert(String::from("myKey"), String::from("value"));

    let mut options = HashMap::new();
    options.insert(String::from("device"), String::from("tempfs"));

    let parameter = CreateVolumeParameter {
        driver: Some(String::from("local")),
        labels: Some(label),
        volume_name: Some(volume_name.clone()),
        options: Some(options),
    };

    let podtender_result = podman_service.volumes().create(parameter).await;
    utils::delete_volume(&volume_name);

    if let Err(err) = podtender_result {
        panic!("{:#?}", err);
    }
}

#[tokio::test]
async fn create_volume_from_example() {
    let podman_service = utils::setup();

    let parameter = CreateVolumeParameter::example();

    let volume_name = parameter.volume_name.as_ref().unwrap().clone();

    let podtender_result = podman_service.volumes().create(parameter).await;

    utils::delete_volume(&volume_name);

    if let Err(err) = podtender_result {
        panic!("{:#?}", err);
    }
}

#[tokio::test]
async fn remove_volume() {
    let podman_service = utils::setup();
    let volume_name = String::from("remove_volume");

    utils::create_volume(&volume_name);

    let parameter = RemoveVolumeParameter {
        volume_name: volume_name.clone(),
        force: Some(true),
    };
    let podtender_result = podman_service.volumes().remove(parameter).await;

    match podtender_result {
        Ok(_) => {}
        Err(podtender_error) => {
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn remove_volume_from_example() {
    let podman_service = utils::setup();

    let parameter = RemoveVolumeParameter::example();

    let volume_name = parameter.volume_name.clone();

    utils::create_volume(&volume_name);

    let podtender_result = podman_service.volumes().remove(parameter).await;

    match podtender_result {
        Ok(_) => {}
        Err(podtender_error) => {
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn volume_exists() {
    let podman_service = utils::setup();
    let volume_name = String::from("volume_exists");

    utils::create_volume(&volume_name);

    let parameter = VolumeExistsParameter {
        volume_name: volume_name.clone(),
    };

    let podtender_result = podman_service.volumes().exists(parameter).await;

    match podtender_result {
        Ok(_) => {
            utils::delete_volume(&volume_name);
        }
        Err(podtender_error) => {
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn volume_exists_from_example() {
    let podman_service = utils::setup();

    let parameter = VolumeExistsParameter::example();

    let volume_name = parameter.volume_name.clone();

    utils::create_volume(&volume_name);

    let podtender_result = podman_service.volumes().exists(parameter).await;

    match podtender_result {
        Ok(_) => {
            utils::delete_volume(&volume_name);
        }
        Err(podtender_error) => {
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn volume_does_not_exists() {
    let podman_service = utils::setup();
    let volume_name = String::from("volume_does_not_exists");

    let parameter = VolumeExistsParameter { volume_name };

    let podtender_err = podman_service
        .volumes()
        .exists(parameter)
        .await
        .err()
        .expect("Volume id/name existed or podman error.");

    if let PodtenderError::PodmanErrorResponse(err) = podtender_err {
        assert_eq!(404, err.response_code);
    } else {
        panic!(
            "Wrong error, expected RequestError got {:#?}",
            podtender_err
        );
    }
}

#[tokio::test]
async fn inspect_volume_exist() {
    let podman_service = utils::setup();
    let volume_name = String::from("inspect_volume_exists");

    utils::create_volume(&volume_name);

    let parameter = InspectVolumeParameter {
        volume_name: volume_name.clone(),
    };

    let podtender_result = podman_service.volumes().inspect(parameter).await;

    utils::delete_volume(&volume_name);

    match podtender_result {
        Ok(response) => {
            assert_eq!(response.name, volume_name.clone());
        }
        Err(podtender_error) => {
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn inspect_volume_from_example() {
    let podman_service = utils::setup();

    let parameter = InspectVolumeParameter::example();
    let volume_name = parameter.volume_name.clone();

    utils::create_volume(&volume_name);

    let podtender_result = podman_service.volumes().inspect(parameter).await;

    utils::delete_volume(&volume_name);

    match podtender_result {
        Ok(response) => {
            assert_eq!(response.name, volume_name.clone());
        }
        Err(podtender_error) => {
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn inspect_volume_does_not_exist() {
    let podman_service = utils::setup();
    let volume_name = String::from("inspect_volume_does_not_exist");

    let parameter = InspectVolumeParameter {
        volume_name: volume_name.clone(),
    };

    let podtender_err = podman_service
        .volumes()
        .inspect(parameter)
        .await
        .err()
        .expect("Volume id/name existed or podman error.");

    if let PodtenderError::PodmanErrorResponse(err) = podtender_err {
        assert_eq!(404, err.response_code);
    } else {
        panic!(
            "Wrong error, expected RequestError got {:#?}",
            podtender_err
        );
    }
}

#[tokio::test]
async fn list_volumes() {
    let podman_service = utils::setup();
    let volume_name1 = String::from("list_volumes1");
    let volume_name2 = String::from("list_volumes2");
    let volume_name3 = String::from("list_volumes3");

    utils::create_volume(&volume_name1);
    utils::create_volume(&volume_name2);
    utils::create_volume(&volume_name3);

    let mut filter_map: HashMap<String, Vec<String>> = HashMap::new();
    filter_map.insert(
        String::from("name"),
        vec![
            String::from(&volume_name1),
            String::from(&volume_name2),
            String::from(&volume_name3),
        ],
    );

    let parameter = ListVolumesParameter {
        filters: Some(filter_map),
    };

    let podtender_result = podman_service.volumes().list(parameter).await;

    utils::delete_volume(&volume_name1);
    utils::delete_volume(&volume_name2);
    utils::delete_volume(&volume_name3);

    match podtender_result {
        Ok(response) => {
            assert_eq!(3, response.len());
            for element in response {
                assert!(element.name.contains("list_volume"));
            }
        }
        Err(podtender_error) => {
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn list_volumes_from_example() {
    let podman_service = utils::setup();
    let volume_name1 = String::from("list_volumes_from_example1");
    let volume_name2 = String::from("list_volumes_from_example2");
    let volume_name3 = String::from("list_volumes_from_example3");

    let parameter = ListVolumesParameter::example();

    let filter_map = parameter.filters.as_ref().unwrap().clone();
    let label_vec = filter_map.get("label");
    let label = label_vec.unwrap().first().unwrap().to_owned();

    utils::create_volume_with_label(&volume_name1, &label);
    utils::create_volume_with_label(&volume_name2, &label);
    utils::create_volume_with_label(&volume_name3, &label);

    let podtender_result = podman_service.volumes().list(parameter).await;

    utils::delete_volume(&volume_name1);
    utils::delete_volume(&volume_name2);
    utils::delete_volume(&volume_name3);

    match podtender_result {
        Ok(response) => {
            assert_eq!(3, response.len());
            for element in response {
                assert!(element.name.contains("list_volumes_from_example"));
            }
        }
        Err(podtender_error) => {
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn prune_volumes() {
    let podman_service = utils::setup();
    let volume_name1 = String::from("prune_volumes1");
    let volume_name2 = String::from("prune_volumes2");
    let volume_name3 = String::from("prune_volumes3");
    let label = String::from("prune=yes");

    utils::create_volume_with_label(&volume_name1, &label);
    utils::create_volume_with_label(&volume_name2, &label);
    utils::create_volume_with_label(&volume_name3, &label);

    let mut filter_map: HashMap<String, Vec<String>> = HashMap::new();
    filter_map.insert(String::from("label"), vec![String::from("prune")]);

    let parameter = PruneVolumesParameter {
        filters: Some(filter_map),
    };
    let podtender_result = podman_service.volumes().prune(parameter).await;

    match podtender_result {
        Ok(response) => {
            assert_eq!(3, response.len());
        }
        Err(podtender_error) => {
            utils::delete_volume(&volume_name1);
            utils::delete_volume(&volume_name2);
            utils::delete_volume(&volume_name3);
            panic!("{:#?}", podtender_error);
        }
    }
}

#[tokio::test]
async fn prune_volumes_from_example() {
    let podman_service = utils::setup();
    let volume_name1 = String::from("prune_volumes_from_example1");
    let volume_name2 = String::from("prune_volumes_from_example2");
    let volume_name3 = String::from("prune_volumes_from_example3");

    let parameter = PruneVolumesParameter::example();

    let filter_map = parameter.filters.as_ref().unwrap().clone();
    let label_vec = filter_map.get("label");
    let label = label_vec.unwrap().first().unwrap().to_owned();

    utils::create_volume_with_label(&volume_name1, &label);
    utils::create_volume_with_label(&volume_name2, &label);
    utils::create_volume_with_label(&volume_name3, &label);

    let podtender_result = podman_service.volumes().prune(parameter).await;

    match podtender_result {
        Ok(response) => {
            assert_eq!(3, response.len());
        }
        Err(podtender_error) => {
            utils::delete_volume(&volume_name1);
            utils::delete_volume(&volume_name2);
            utils::delete_volume(&volume_name3);
            panic!("{:#?}", podtender_error);
        }
    }
}
