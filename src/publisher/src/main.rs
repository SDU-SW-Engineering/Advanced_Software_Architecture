use paho_mqtt as mqtt;
use rand::Rng;
use std::{thread, time::Duration};

fn main() {
    let server_uri = "tcp://mosquitto:1883";
    let client_id = "rust_publisher";

    match run_mqtt_client(server_uri, client_id) {
        Ok(_) => println!("Client terminated normally"),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}

fn run_mqtt_client(server_uri: &str, client_id: &str) -> mqtt::Result<()> {
    let mut cli = create_client(server_uri, client_id)?;
    connect_client(&mut cli)?;
    publish_in_loop(&mut cli)
}

fn create_client(server_uri: &str, client_id: &str) -> mqtt::Result<mqtt::Client> {
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(server_uri)
        .client_id(client_id)
        .finalize();
    mqtt::Client::new(create_opts).map_err(From::from)
}

fn connect_client(cli: &mut mqtt::Client) -> mqtt::Result<()> {
    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .finalize();
    cli.connect(conn_opts).map(|_response| ())
}

fn publish_in_loop(cli: &mut mqtt::Client) -> mqtt::Result<()> {
    loop {
        let payload = generate_payload();
        let msg = mqtt::MessageBuilder::new()
            .topic("data/random-numbers")
            .payload(payload)
            .qos(paho_mqtt::types::QOS_2)
            .finalize();

        cli.publish(msg)?;
        thread::sleep(Duration::from_secs(1));
    }
}

fn generate_payload() -> String {
    let random_number = rand::thread_rng().gen_range(0..=9);
    random_number.to_string()
}
