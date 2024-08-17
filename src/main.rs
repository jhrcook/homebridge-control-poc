use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};
#[derive(Serialize, Deserialize, Debug)]
struct Secrets {
    username: String,
    password: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct HBAuth {
    access_token: String,
    token_type: String,
    expires_in: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct HBPlugin {
    name: String,
    #[serde(rename = "displayName")]
    display_name: String,
    description: String,
    #[serde(rename = "installedVersion")]
    installed_version: String,
    disabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct HBPlugins {
    plugins: Vec<HBPlugin>,
}

#[derive(Serialize, Deserialize, Debug)]
struct HBAccessory {
    uuid: String,
    #[serde(rename = "uniqueId")]
    unique_id: String,
    #[serde(rename = "type")]
    acc_type: String,
    #[serde(rename = "humanType")]
    huamn_type: String,
    #[serde(rename = "serviceName")]
    service_name: String,
    // #[serde(rename = "serviceCharacteristics")]
    // service_characteristics: Vec<HashMap<String, String>>,
    // #[serde(rename = "accessoryInformation")]
    // accessory_information: HashMap<String, String>,
    // values: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct HBAccessories {
    accessories: Vec<HBAccessory>,
}

#[tokio::main]
async fn main() {
    // Get secrets.
    let secrets_file = fs::File::open("secrets.json").unwrap();
    let secrets: Secrets = serde_json::from_reader(secrets_file).unwrap();

    // Create `reqwest` client.
    let client = reqwest::Client::new();

    // Get an access token.
    let mut map = HashMap::new();
    map.insert("username", &secrets.username);
    map.insert("password", &secrets.password);
    let res = client
        .post("http://192.168.0.213:8581/api/auth/login")
        .json(&map)
        .send()
        .await
        .unwrap();
    let parsed_auth = match res.status() {
        reqwest::StatusCode::CREATED => match res.json::<HBAuth>().await {
            Ok(parsed_auth) => parsed_auth,
            Err(e) => panic!("Error parsing auth response: {:?}", e),
        },
        other => panic!("Failed authorization: {:?}", other),
    };

    println!("Successfully parsed auth response.");
    println!("Token: '{}'", parsed_auth.access_token);

    // List all plugins.
    let res = client
        .get("http://192.168.0.213:8581/api/plugins")
        .bearer_auth(&parsed_auth.access_token)
        .send()
        .await
        .unwrap();

    let plugins = res.json::<HBPlugins>().await.unwrap();
    let _x = plugins.plugins.iter().for_each(|x| println!("{:?}", x));

    // List the accessories.
    let res = client
        .get("http://192.168.0.213:8581/api/accessories")
        .bearer_auth(&parsed_auth.access_token)
        .send()
        .await
        .unwrap();
    // println!("Accessories:\n{}", res.text().await.unwrap());
    let accesories = res.json::<HBAccessories>().await.unwrap();
    let _ = accesories
        .accessories
        .iter()
        .for_each(|x| println!("{:?}", x));

    // Get the light's value.

    // Set the light's value.
}
