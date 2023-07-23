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
    width: u16,
    height: u16,
    buffer: Vec<u32>,
    sampling_mode: TextureSamplingMode
}

#[allow(dead_code)]
impl Texture {
    #[inline]
    pub fn new(width: u16, height: u16, sampling_mode: TextureSamplingMode) -> Self {
        Texture {
            width,
            height,
            buffer: vec![0u32; (width * height) as usize],
            sampling_mode,
        }
    }

    #[inline]
    pub fn from_buffer(buffer: Vec<u32>, width: u16, height: u16, sampling_mode: TextureSamplingMode) -> Self {
        Texture {
            width,
            height,
            buffer,
            sampling_mode,
        }
    }

    #[inline]
    pub fn set_buffer(&mut self, buffer: Vec<u32>) {
        self.buffer = buffer;
    }

    #[inline]
    pub fn width(&self) -> u16 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> u16 {
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

    #[inline]
    pub fn sample(&self, mut x: f32, mut y: f32) -> (u8, u8, u8) {
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
        let i = (y * self.width as f32 + x) as usize;
        let color_sampled = self.buffer[i];
        extensions::u8_from_u32(color_sampled)
    }
}