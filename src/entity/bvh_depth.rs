use super::bvh::Bvh;

pub struct BvhDepth<'a> {
    pub data: &'a [Bvh],
    pub depth: u32,
    pub bvhs: Vec<&'a Bvh>
}

impl BvhDepth<'_> {
    #[inline]
    pub fn new(data: &[Bvh], depth: u32) -> BvhDepth<'_> {
        BvhDepth { data, depth, bvhs: vec![] }
    }

    #[inline]
    pub fn intersect_hierarchy(&mut self) {
        self.intersect_bvh(0, 0);
    }

    #[inline]
    fn intersect_bvh(&mut self, bvh_index: usize, depth: u32) {
        let bvh = &self.data[bvh_index];

        if depth > self.depth {
            return;
        }
        if depth == self.depth {
            self.bvhs.push(bvh);
        } else if !bvh.is_leaf() {
            self.intersect_bvh(bvh.first_object, depth + 1);
            self.intersect_bvh(bvh.first_object + 1, depth + 1);
        }
    }
}
