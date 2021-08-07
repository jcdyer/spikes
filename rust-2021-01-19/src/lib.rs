//!

pub struct Matrix {
    rows: Vec<Box<[u8]>>,
}

impl Matrix {
    pub fn window_2d(&self, n: usize) -> Window2D {
        Window2D {
            matrix: self,
            n,
            x: 0,
            y: 0,
        }
    }
    pub fn height(&self) -> usize {
        self.rows.len()
    }

    pub fn width(&self) -> usize {
        self.rows.get(0).map(|row| row.len()).unwrap_or(0)
    }
}

pub struct Window2D<'m> {
    matrix: &'m Matrix,
    n: usize,
    x: usize,
    y: usize,
}

impl<'m> Iterator for Window2D<'m> {
    type Item = Vec<&'m [u8]>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y == self.matrix.rows.len() {
            None
        } else {
            let val = self
                .matrix
                .rows
                .iter()
                .skip(dbg!(("skip", self.y.saturating_sub(self.n))).1)
                .take(dbg!(("take", (self.y + self.n).min(2 * self.n) + 1)).1)
                .map(|row| {
                    let from = &row[self.x.saturating_sub(self.n)
                        ..self.matrix.width().min(self.x + self.n + 1)];
                    println!("from {:?}", from);
                    from
                })
                .collect();
            self.x = (self.x + 1) % self.matrix.width();
            if self.x == 0 {
                self.y += 1;
            }
            Some(val)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn small() -> Matrix {
        Matrix {
            rows: {
                let mut v = Vec::new();
                v.push(vec![1, 2, 3].into_boxed_slice());
                v.push(vec![2, 3, 4].into_boxed_slice());
                v.push(vec![3, 4, 5].into_boxed_slice());
                v
            },
        }
    }

    #[test]
    fn outputs() {
        let m = small();
        let mut iter = m.window_2d(1);
        assert_eq!(iter.next(), Some(vec![&[1, 2][..], &[2, 3][..],]));
        assert_eq!(iter.next(), Some(vec![&[1, 2, 3][..], &[2, 3, 4][..],]));
        assert_eq!(iter.next(), Some(vec![&[2, 3][..], &[3, 4][..],]));
        assert_eq!(
            iter.next(),
            Some(vec!(&[1, 2][..], &[2, 3][..], &[3, 4][..],))
        );
        assert_eq!(
            iter.next(),
            Some(vec!(&[1, 2, 3][..], &[2, 3, 4][..], &[3, 4, 5][..],))
        );
        assert_eq!(
            iter.next(),
            Some(vec!(&[2, 3][..], &[3, 4][..], &[4, 5][..],))
        );
        assert_eq!(iter.next(), Some(vec!(&[2, 3][..], &[3, 4][..],)));
        assert_eq!(iter.next(), Some(vec!(&[2, 3, 4][..], &[3, 4, 5][..],)));
        assert_eq!(iter.next(), Some(vec!(&[3, 4][..], &[4, 5][..],)));

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn matrix() {
        let m = Matrix {
            rows: {
                let mut v = Vec::new();
                v.push(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5].into_boxed_slice());
                v.push(vec![5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16].into_boxed_slice());
                v.push(vec![5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16].into_boxed_slice());
                v.push(vec![5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16].into_boxed_slice());
                v.push(vec![5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16].into_boxed_slice());
                v.push(vec![5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16].into_boxed_slice());
                v.push(vec![5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16].into_boxed_slice());
                v.push(vec![20, 20, 20, 20, 20, 20, 11, 12, 20, 14, 15, 16].into_boxed_slice());
                v
            },
        };

        let out: Vec<_> = m
            .window_2d(1)
            .map(|window| window.into_iter().flatten().max().copied().unwrap_or(0))
            .collect();
        assert_eq!(
            out,
            vec![
                6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 16, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                16, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 16, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
                16, 16, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 16, 20, 20, 20, 20, 20, 20, 20, 20,
                20, 20, 20, 16, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 16, 20, 20, 20, 20, 20,
                20, 20, 20, 20, 20, 20, 16,
            ],
        )
    }
}
