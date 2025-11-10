use libc::c_int;
use std::ffi::CString;

use crate::ffi;
use crate::{LedColor, LedFont};

/// The Rust handle for the matrix canvas to draw on.
///
/// ```
/// use rpi_led_matrix::{LedMatrix, LedColor};
/// let matrix = LedMatrix::new(None, None).unwrap();
/// let mut canvas = matrix.canvas();
/// canvas.fill(&LedColor { red: 128, green: 128, blue: 128 });
/// ```
pub struct LedCanvas {
    pub(crate) handle: *mut ffi::CLedCanvas,
}

/// Layout options for rendering text on the canvas
pub enum TextLayout {
    /// Draw text horizontally
    Horizontal,
    /// Draw text vertically
    Vertical,
    /// Draw text with optimal line wrapping using an algorithm that
    /// minimizes raggedness and gaps at the ends of lines.
    Wrapped{
        /// Maximum line width
        line_width: i32
    }, 
}

/// Options for rendering text on the canvas
pub struct TextDrawOptions<'a> {
    x: i32,
    y: i32,
    color: &'a LedColor,
    layout: TextLayout,
    kerning_offset: i32,
    leading: i32,
}

/// Implements both the [`Send`] and [`Sync`] traits for [`LedCanvas`].
///
/// The underlying handle referenced by this FFI is [heap-allocated],
/// allowing safe ownership transfer between threads. Additionally,
/// references to this handle can be safely shared across thread boundaries.
///
/// [heap-allocated]: https://github.com/hzeller/rpi-rgb-led-matrix/blob/0ff6a6973f95d14e3206bcef1201237097fa8edd/lib/led-matrix.cc#L501
unsafe impl Send for LedCanvas {}
unsafe impl Sync for LedCanvas {}

impl LedCanvas {
    /// Retrieves the width & height of the canvas
    #[must_use]
    pub fn canvas_size(&self) -> (i32, i32) {
        let (mut width, mut height): (c_int, c_int) = (0, 0);
        unsafe {
            ffi::led_canvas_get_size(
                self.handle,
                std::ptr::addr_of_mut!(width),
                std::ptr::addr_of_mut!(height),
            );
        }
        (width as i32, height as i32)
    }

    /// Sets the pixel at the given coordinate to the given color.
    pub fn set(&mut self, x: i32, y: i32, color: &LedColor) {
        unsafe {
            ffi::led_canvas_set_pixel(
                self.handle,
                x as c_int,
                y as c_int,
                color.red,
                color.green,
                color.blue,
            );
        }
    }

    /// Clears the canvas.
    pub fn clear(&mut self) {
        unsafe {
            ffi::led_canvas_clear(self.handle);
        }
    }

    /// Fills the canvas with the given color.
    pub fn fill(&mut self, color: &LedColor) {
        unsafe {
            ffi::led_canvas_fill(self.handle, color.red, color.green, color.blue);
        }
    }

    /// Draws a straight, one pixel wide line using the C++ library.
    ///
    /// Consider using embedded-graphics for more drawing features.
    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: &LedColor) {
        unsafe {
            ffi::draw_line(
                self.handle,
                x0,
                y0,
                x1,
                y1,
                color.red,
                color.green,
                color.blue,
            );
        }
    }

    /// Draws a one pixel wide circle using the C++ library.
    ///
    /// Consider using embedded-graphics for more drawing features.
    pub fn draw_circle(&mut self, x: i32, y: i32, radius: u32, color: &LedColor) {
        unsafe {
            ffi::draw_circle(
                self.handle,
                x as c_int,
                y as c_int,
                radius as c_int,
                color.red,
                color.green,
                color.blue,
            );
        }
    }

    #[allow(clippy::too_many_arguments)]
    /// Renders text using the C++ library.
    ///
    /// # Panics
    /// If the given `text` fails to convert to a `CString`. This can
    /// occur when there is a null character mid way in the string.
    pub fn draw_text(&mut self, font: &LedFont, text: &str, options: &TextDrawOptions) -> i32 {
        let text = CString::new(text).expect("given string failed to convert into a CString");
        let x = options.x as c_int;
        let y = options.y as c_int;
        let r = options.color.red;
        let g = options.color.green;
        let b = options.color.blue;
        let text = text.as_ptr();
        let kerning_offset = options.kerning_offset as c_int;
        let leading = options.leading as c_int;

        match options.layout {
            TextLayout::Horizontal => {
                println!("draw_text");
                unsafe {
                    ffi::draw_text(
                        self.handle, font.handle, x, y, r, g, b, text, kerning_offset
                    ) as i32
                }
            }
            TextLayout::Vertical => {
                println!("vertical_draw_text");
                unsafe {
                    ffi::vertical_draw_text(
                        self.handle, font.handle, x, y, r, g, b, text, kerning_offset
                    ) as i32
                }
            }
            TextLayout::Wrapped { line_width } => {
                println!("draw_text_wrapped");
                unsafe {
                    ffi::draw_text_wrapped(
                        self.handle, font.handle, x, y, line_width ,r, g, b, text, kerning_offset, leading
                    ) as i32
                }
            }
        }
    }
}

impl<'a> TextDrawOptions<'a> {
    /// Creates the options for rendering text on the canvas with the default values
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            color: &LedColor {
                red: 255,
                green: 255,
                blue: 255,
            },
            layout: TextLayout::Horizontal,
            kerning_offset: 0,
            leading: 0,
        }
    }

    /// Sets the position ("x", "y") where the text is drawn
    pub fn position(mut self, x: i32, y: i32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Sets the color used to draw the text
    pub fn color(mut self, color: &'a LedColor) -> Self {
        self.color = color;
        self
    }

    /// Sets the way the text is drawn
    pub fn layout(mut self, layout: TextLayout) -> Self {
        self.layout = layout;
        self
    }

    /// Sets the value for additional horizontal spacing between characters
    pub fn kerning_offset(mut self, offset: i32) -> Self {
        self.kerning_offset = offset;
        self
    }

    /// Sets the value for additional vertical spacing between lines
    pub fn leading(mut self, leading: i32) -> Self {
        self.leading = leading;
        self
    }
}

impl Default for TextDrawOptions<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{LedMatrix, LedMatrixOptions, LedRuntimeOptions};
    use std::f64::consts::PI;
    use std::{thread, time};

    fn led_matrix() -> LedMatrix {
        let mut options = LedMatrixOptions::new();
        let mut rt_options = LedRuntimeOptions::new();
        options.set_hardware_mapping("adafruit-hat-pwm");
        options.set_chain_length(2);
        options.set_hardware_pulsing(false);
        options.set_refresh_rate(true);
        options.set_brightness(10).unwrap();
        rt_options.set_gpio_slowdown(2);
        LedMatrix::new(Some(options), Some(rt_options)).unwrap()
    }

    #[test]
    #[serial_test::serial]
    fn size() {
        let matrix = led_matrix();
        let canvas = matrix.canvas();
        assert_eq!(canvas.canvas_size(), (64, 32));
    }

    #[test]
    #[serial_test::serial]
    fn draw_line() {
        let matrix = led_matrix();
        let mut canvas = matrix.canvas();
        let (width, height) = canvas.canvas_size();
        let mut color = LedColor {
            red: 127,
            green: 0,
            blue: 0,
        };

        canvas.clear();
        for x in 0..width {
            color.blue = 255 - 3 * x as u8;
            canvas.draw_line(x, 0, width - 1 - x, height - 1, &color);
            thread::sleep(time::Duration::new(0, 10000000));
        }
    }

    #[test]
    #[serial_test::serial]
    fn draw_circle() {
        let matrix = led_matrix();
        let mut canvas = matrix.canvas();
        let (width, height) = canvas.canvas_size();
        let mut color = LedColor {
            red: 127,
            green: 0,
            blue: 0,
        };
        let (x, y) = (width / 2, height / 2);

        canvas.clear();
        for r in 0..(width / 2) {
            color.green = color.red;
            color.red = color.blue;
            color.blue = (r * r) as u8;
            canvas.draw_circle(x, y, r as u32, &color);
            thread::sleep(time::Duration::new(0, 100000000));
        }
    }

    #[test]
    #[serial_test::serial]
    fn gradient() {
        let matrix = led_matrix();
        let mut canvas = matrix.canvas();
        let mut color = LedColor {
            red: 0,
            green: 0,
            blue: 0,
        };
        let period = 400;
        let duration = time::Duration::new(3, 0);
        let sleep_duration = duration / period;

        for t in 0..period {
            let t = t as f64;
            color.red = ((PI * t / period as f64).sin() * 255.) as u8;
            color.green = ((2. * PI * t / period as f64).cos() * 255.) as u8;
            color.blue = ((3. * PI * t / period as f64 + 0.3).cos() * 255.) as u8;
            canvas.fill(&color);
            thread::sleep(sleep_duration);
        }
    }

    #[test]
    #[serial_test::serial]
    fn canvas_swap() {
        let matrix = led_matrix();
        let mut canvas = matrix.canvas();
        let mut color = LedColor {
            red: 127,
            green: 127,
            blue: 0,
        };

        canvas.fill(&color);
        canvas = matrix.offscreen_canvas();
        color.blue = 127;
        canvas.fill(&color);
        thread::sleep(time::Duration::new(0, 500000000));
        canvas = matrix.swap(canvas);
        color.red = 0;
        canvas.fill(&color);
        thread::sleep(time::Duration::new(0, 500000000));
        let _ = matrix.swap(canvas);
        thread::sleep(time::Duration::new(0, 500000000));
    }
}
