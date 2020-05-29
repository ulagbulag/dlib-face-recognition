use std::ops::Deref;

use image::{ImageBuffer, Rgb};

cpp_class!(
    /// A wrapper around a `matrix<rgb_pixel>`, dlibs own image class.
    pub unsafe struct ImageMatrix as "dlib::matrix<dlib::rgb_pixel>"
);

impl ImageMatrix {
    /// # Safety
    ///
    /// Create a new matrix from rgb channel values (r, g, b, r, g, b).
    ///
    /// Unsafe because we can't check that width * height * 3 <= number of channels
    pub unsafe fn new(width: usize, height: usize, ptr: *const u8) -> Self {
        cpp!([width as "size_t", height as "size_t", ptr as "uint8_t*"] -> ImageMatrix as "dlib::matrix<dlib::rgb_pixel>" {
            dlib::matrix<dlib::rgb_pixel> image = dlib::matrix<dlib::rgb_pixel>(height, width);

            size_t offset = 0;

            for (size_t y = 0; y < height; y++) {
                for (size_t x = 0; x < width; x++) {
                    uint8_t red = *(ptr + offset);
                    uint8_t green = *(ptr + offset + 1);
                    uint8_t blue = *(ptr + offset + 2);

                    image(y, x) = dlib::rgb_pixel(red, green, blue);
                    offset += 3;
                }
            }

            return image;
        })
    }

    /// Copy a matrix from an rgb image
    pub fn from_image<C: Deref<Target = [u8]>>(image: &ImageBuffer<Rgb<u8>, C>) -> Self {
        let width = image.width() as usize;
        let height = image.height() as usize;
        let ptr = image.as_ptr();

        unsafe { Self::new(width, height, ptr) }
    }
}

impl ImageMatrix {
    pub fn resize(&self, width: usize, height: usize) -> Self {
        unsafe {
            cpp!([self as "const dlib::matrix<dlib::rgb_pixel>*", width as "size_t", height as "size_t"] -> ImageMatrix as "dlib::matrix<dlib::rgb_pixel>" {
                dlib::matrix<dlib::rgb_pixel> out(height, width);
                dlib::resize_image(*self, out);
                return out;
            })
        }
    }
}
