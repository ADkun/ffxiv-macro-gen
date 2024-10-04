use std::panic;

use ffxiv_momacro_gen::app::App;
use ffxiv_momacro_gen::util;

fn main() {
    let app = App::new();
    let result = panic::catch_unwind(|| {
        app.run();
    });
    if result.is_err() {
        util::pause();
    }
}
