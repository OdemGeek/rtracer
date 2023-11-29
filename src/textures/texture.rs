use nalgebra::Vector3;

use crate::math::extensions;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum TextureSamplingMode {
    Repeat,
    Clamp,
    // TODO:
    //Mirror,
    //MirrorOnce,
}

#[allow(dead_code)]
pub struct Texture {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
    sampling_mode: TextureSamplingMode
}

#[allow(dead_code)]
impl Texture {
    #[inline]
    pub fn new(width: usize, height: usize, sampling_mode: TextureSamplingMode) -> Self {
        Texture {
            width,
            height,
            buffer: vec![0u32; width * height],
            sampling_mode,
        }
    }

    #[inline]
    pub fn from_buffer(buffer: Vec<u32>, width: usize, height: usize, sampling_mode: TextureSamplingMode) -> Self {
        Texture {
            width,
            height,
            buffer,
            sampling_mode,
        }
    }

    #[inline]
    pub fn sample(&self, x: f32, y: f32) -> Vector3<f32> {
        let mut x = x;
        let mut y = 1.0 - y;
        match self.sampling_mode {
            TextureSamplingMode::Repeat => {
                x %= 1.0;
                y %= 1.0;
            },
            TextureSamplingMode::Clamp => {
                x = x.clamp(0.0, 1.0);
                y = y.clamp(0.0, 1.0);
            },
        }
        let x_ind = (x * (self.width - 1) as f32) as usize;
        let y_ind = (y * (self.height - 1) as f32) as usize;
        let i = (y_ind * self.width + x_ind) as usize;
        //return Vector3::new(0.0, 0.0, i as f32 / (self.width as f32 * self.height as f32));
        let color_sampled = *self.buffer.get(i).expect(&format!("i: {}, x: {}, y: {}, x_ind: {}, y_ind: {},", i, x, y, x_ind, y_ind));
        extensions::f32_vector3_from_u32(color_sampled)
    }

    #[inline]
    pub fn set_buffer(&mut self, buffer: Vec<u32>) {
        self.buffer = buffer;
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    #[inline]
    pub fn sampling_mode(&self) -> TextureSamplingMode {
        self.sampling_mode
    }

    #[inline]
    pub fn get_buffer_read(&self) -> &Vec<u32> {
        &self.buffer
    }

    #[inline]
    pub fn get_buffer_clone(&self) -> Vec<u32> {
        self.buffer.clone()
    }

    #[inline]
    pub fn get_buffer_mut(&mut self) -> &mut Vec<u32> {
        &mut self.buffer
    }
}