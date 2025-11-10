/// Example showing some basic usage of the C++ library.
use clap::{crate_version, App};
use rpi_led_matrix::{args, LedFont, LedColor, LedMatrix, TextDrawOptions};

const DELAY: std::time::Duration = std::time::Duration::from_secs(5);

fn main() {
    let app = args::add_matrix_args(
        App::new("C++ Library Example")
            .about("shows basic usage of matrix arguments")
            .version(crate_version!())
    );
    let matches = app.get_matches();
    let (options, runtime_options) = args::matrix_options_from_args(&matches);

    let matrix = LedMatrix::new(Some(options), Some(runtime_options)).unwrap();
    let mut canvas = matrix.offscreen_canvas();
    let color = LedColor {
        red: 255,
        green: 255,
        blue: 255,
    };

    let font_file = std::path::Path::new("./rpi-led-matrix-sys/cpp-library/fonts/5x8.bdf");
    let font = LedFont::new(font_file).unwrap();
    let baseline = font.height().unwrap();

    let options = TextDrawOptions::new()
        .position(0, baseline)
        .color(&color);
    
    canvas.clear();
    canvas.draw_text(&font, "Halle World", &options);
    let _ = matrix.swap(canvas);

    std::thread::sleep(DELAY);
}
