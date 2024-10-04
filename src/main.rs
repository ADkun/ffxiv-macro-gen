use std::panic;

use ffxiv_macro_gen::app::App;
use ffxiv_macro_gen::util;

fn main() {
    let app = App::new();
    let result = panic::catch_unwind(|| {
        app.run();
    });
    if result.is_err() {
        util::pause();
    }
}
