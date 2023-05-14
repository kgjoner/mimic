use std::env;
use std::process;

use mimic::gateway::CliGateway;
use mimic::repository::treasure_records::TreasureRecordsRepo;

fn main() {
    let treasure_records_repo = TreasureRecordsRepo::new();

    let gateway = CliGateway::raise(treasure_records_repo);

    let result = gateway.cross(env::args()).unwrap_or_else(|err| {
        err.print();
        process::exit(1);
    });

    result.print();
}
