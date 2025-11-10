/// Example showing some basic usage of the C++ library.
use clap::{crate_version, App};
use rpi_led_matrix::{args, LedFont, LedColor, LedMatrix, TextDrawOptions, TextLayout};

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
    let (width, _height) = canvas.canvas_size();
    let color = LedColor {
        red: 255,
        green: 255,
        blue: 255,
    };

    let font_file = std::path::Path::new("./rpi-led-matrix-sys/cpp-library/fonts/Grand9K-Pixel.bdf");
    let font = LedFont::new(font_file).unwrap();
    let text = "To be, or not to be: that is the question";

    let options = TextDrawOptions::new()
        .color(&color)
        .layout(TextLayout::Wrapped{line_width: width});
    
    canvas.clear();
    canvas.draw_text(&font, text, &options);
    let _ = matrix.swap(canvas);

    std::thread::sleep(DELAY);
}
