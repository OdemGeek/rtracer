use std::default;
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
pub struct Texture<T>
where
    T: Default,
    T: Clone
    {
    width: usize,
    height: usize,
    buffer: Vec<T>,
    sampling_mode: TextureSamplingMode
}

#[allow(dead_code)]
impl<T> Texture<T>
where
    T: Default,
    T: Clone
    {
    #[inline]
    pub fn new(width: usize, height: usize, sampling_mode: TextureSamplingMode) -> Self {
        Texture {
            width,
            height,
            buffer: vec![T::default(); width * height],
            sampling_mode,
        }
    }

    #[inline]
    pub fn from_buffer(buffer: Vec<T>, width: usize, height: usize, sampling_mode: TextureSamplingMode) -> Self {
        Texture {
            width,
            height,
            buffer,
            sampling_mode,
        }
    }

    #[inline]
    pub fn sample(&self, x: f32, y: f32) -> T {
        let mut x = 1.0 - x;
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
        let i = y_ind * self.width + x_ind;
        let color_sampled = self.buffer.get(i).expect("Image out of bounds").clone();
        color_sampled
    }

    #[inline]
    pub fn set_buffer(&mut self, buffer: Vec<T>) {
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
    pub fn get_buffer_read(&self) -> &Vec<T> {
        &self.buffer
    }

    #[inline]
    pub fn get_buffer_clone(&self) -> Vec<T> {
        self.buffer.clone()
    }

    #[inline]
    pub fn get_buffer_mut(&mut self) -> &mut Vec<T> {
        &mut self.buffer
    }
}