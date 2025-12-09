#![feature(const_default, const_trait_impl)]

pub struct MultiCursor<const W: usize, const H: usize, T: Copy> {
    data: [[T; W]; H],
    cursor: usize,
    default: T,
}

impl<const W: usize, const H: usize, T: Copy> MultiCursor<W, H, T> {
    pub const fn new(data: [[T; W]; H]) -> Self
    where
        T: [const] Default,
    {
        Self {
            data,
            cursor: 0,
            default: T::default(),
        }
    }

    pub fn next(&mut self) -> Option<[T; H]> {
        if self.cursor < W {
            let mut res = [self.default; H];

            for i in 0..H {
                res[i] = self.data[i][self.cursor];
            }

            self.cursor += 1;

            Some(res)
        } else {
            None
        }
    }
}

pub trait Squared {
    type Output;

    fn sqr(&self) -> Self::Output;
}

impl Squared for f32 {
    type Output = f32;

    #[inline(always)]
    fn sqr(&self) -> Self::Output {
        self * self
    }
}
