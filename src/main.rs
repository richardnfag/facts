mod proc;
mod publisher;
mod utils;

use std::env::var;
use std::thread::sleep;
use std::time::Duration;

use tokio::task::spawn;

use proc::cpuinfo::CPUInfo;
use proc::meminfo::MemInfo;
use publisher::Publisher;

const CPUINFO_TOPIC: &'static str = "facts-cpuinfo";
const MENINFO_TOPIC: &'static str = "facts-meminfo";

async fn cpuinfo_publishing(broker: String, update_delay: Duration) {
    let publisher = Publisher::new(&broker, CPUINFO_TOPIC);

    loop {
        let cpuinfo = CPUInfo::new().to_json();
        publisher.publish(cpuinfo);
        sleep(update_delay);
    }
}
async fn meminfo_publishing(broker: String, update_delay: Duration) {
    let publisher = Publisher::new(&broker, MENINFO_TOPIC);

    loop {
        let meminfo = MemInfo::new().to_json();
        publisher.publish(meminfo);
        sleep(update_delay);
    }
}

#[tokio::main]
async fn main() {
    let broker = var("FATCS_BROKER").unwrap_or("localhost:9092".to_owned());

    let cpuinfo_delay = Duration::from_millis(
        var("FATCS_CPUINFO_DELAY")
            .unwrap_or("5000".to_owned())
            .parse::<u64>()
            .unwrap(),
    );

    let meminfo_delay = Duration::from_millis(
        var("FATCS_MEMINFO_DELAY")
            .unwrap_or("5000".to_owned())
            .parse::<u64>()
            .unwrap(),
    );

    let result = tokio::try_join!(
        spawn(cpuinfo_publishing(broker.clone(), cpuinfo_delay)),
        spawn(meminfo_publishing(broker, meminfo_delay))
    );

    if let Err(err) = result {
        println!("processing failed; error = {}", err);
    }
}
