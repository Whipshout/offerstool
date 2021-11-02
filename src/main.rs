use offerstool::config::get_args;
use offerstool::run;

fn main() {
    let config = get_args();

    run(config).expect("Something failed, unlucky");
}
