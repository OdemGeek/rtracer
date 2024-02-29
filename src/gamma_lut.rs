pub struct GammaLut {
    xs: Vec<f32>,
    ys: Vec<f32>,
    max_index: usize
}

impl GammaLut {
    #[inline]
    pub fn new(resolution: usize, gamma_value: f32) -> Self {
        let mut s: Vec<f32> = Vec::with_capacity(resolution);
        let mut v: Vec<f32> = Vec::with_capacity(resolution);
        
        for i in 0..resolution {
            s.push(i as f32 / ((resolution - 1) as f32))
        }

        for x in &s {
            v.push(x.powf(1.0 / gamma_value));
        }

        GammaLut {
            xs: s,
            ys: v,
            max_index: resolution - 1
        }
    }

    #[inline(always)]
    pub fn get(&self, x: f32) -> f32 {
        // Get lower value in range where we are
        let low_index = (x * self.max_index as f32) as usize;
        // If we got last value
        if low_index >= self.max_index {
            return self.ys[self.max_index];
        }
        let high_index = low_index + 1;
        // Interpolate
        let dx = self.xs[high_index] - self.xs[low_index];
        let dy = self.ys[high_index] - self.ys[low_index];
        self.ys[low_index] + (x - self.xs[low_index]) * dy / dx
    }
}
