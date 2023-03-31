use std::env;
use kafka::client::metadata::Broker;

fn main() {
    let args: Vec<String> = env::args().collect();
    for endpoint in &args[1..] {
        println!("arg:{}", endpoint);
        let mut client = kafka::client::KafkaClient::new(vec!(endpoint.to_owned()));
        client.load_metadata_all().unwrap();
        for topic in client.topics() {
            for partition in topic.partitions() {
                println!("{} #{} => {}", topic.name(), partition.id(),
             partition.leader()
                      .map(Broker::host)
                      .unwrap_or("no-leader!"));
            }
        }
    }
}
