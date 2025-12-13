use bevy::{math::VectorSpace, prelude::*};
use ndarray::prelude::*;

use crate::utils::vec2;

use super::r#trait::ChunkToGrid;

#[derive(Debug, PartialEq)]
pub struct RectExt {
    min: (f32, f32),
    max: (f32, f32),
}
impl RectExt {
    #[inline]
    pub fn min(&self) -> Vec2 {
        Vec2::new(self.min.0, self.min.1)
    }
    #[inline]
    pub fn max(&self) -> Vec2 {
        Vec2::new(self.max.0, self.max.1)
    }
}
impl RectExt {
    pub fn new(min: (f32, f32), max: (f32, f32)) -> Self {
        let (min, max) = Self::order(min, max);

        Self { min, max }
    }
    pub fn new_unchecked(min: (f32, f32), max: (f32, f32)) -> Self {
        Self { min, max }
    }

    pub fn is_empty(&self) -> bool {
        Rect::new(self.min.0, self.min.1, self.max.0, self.max.1).is_empty()
    }

    fn order(min: (f32, f32), max: (f32, f32)) -> ((f32, f32), (f32, f32)) {
        let (min, max) = (Vec2::from(min), Vec2::from(max));

        (min.min(max).into(), max.max(min).into())
    }
}

impl From<((f32, f32), (f32, f32))> for RectExt {
    fn from((min, max): ((f32, f32), (f32, f32))) -> Self {
        Self::new(min, max)
    }
}
impl From<Rect> for RectExt {
    fn from(value: Rect) -> Self {
        Self::new_unchecked(value.min.into(), value.max.into())
    }
}

impl ChunkToGrid<RectExt> for RectExt {
    fn chunk_to_grid(&self, chunk_size: Vec2) -> Array2<RectExt> {
        if self.is_empty()
            || chunk_size.length() <= f32::EPSILON
            || vec2::is_negative_direction(chunk_size)
        {
            return Array2::from_shape_vec((0, 0), vec![]).ok().unwrap();
        }

        let ((x_min, y_min), (x_max, y_max)) = (self.min, self.max);

        // Calculate how many full chunks fit in each dimension
        let x_chunks = ((x_max - x_min) / chunk_size.x).floor() as usize;
        let y_chunks = ((y_max - y_min) / chunk_size.y).floor() as usize;

        // Create array and fill with chunks
        Array2::from_shape_fn((y_chunks, x_chunks), |(j, i)| {
            let x = x_min + i as f32 * chunk_size.x;
            let y = y_min + j as f32 * chunk_size.y;

            RectExt::from(((x, y), (x + chunk_size.x, y + chunk_size.y)))
        })
    }

    fn chunk_to_grid_with_cuttings(&self, chunk_size: Vec2) -> Array2<RectExt> {
        if self.is_empty()
            || chunk_size.length() <= f32::EPSILON
            || vec2::is_negative_direction(chunk_size)
        {
            return Array2::from_shape_vec((0, 0), vec![]).ok().unwrap();
        }

        let ((x_min, y_min), (x_max, y_max)) = (self.min, self.max);

        // Calculate number of chunks including partial ones
        let cols = (x_max / chunk_size.x).ceil() as usize;
        let rows = (y_max / chunk_size.y).ceil() as usize;

        Array2::from_shape_fn((rows, cols), |(j, i)| {
            let x = x_min + i as f32 * chunk_size.x;
            let y = y_min + j as f32 * chunk_size.y;

            // Calculate actual size of this chunk (might be partial)
            let w = if i == cols - 1 {
                x_max - (i as f32 * chunk_size.x)
            } else {
                chunk_size.x
            };

            let h = if j == rows - 1 {
                y_max - (j as f32 * chunk_size.y)
            } else {
                chunk_size.y
            };

            RectExt::from(((x, y), (x + w, y + h)))
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ndarray::array;

    // Quick rect
    fn qr(p0: (i8, i8), p1: (i8, i8)) -> RectExt {
        RectExt::from(((p0.0 as f32, p0.1 as f32), (p1.0 as f32, p1.1 as f32)))
    }

    #[test]
    fn zero_chunk_size_returns_nothing() {
        let rect = qr((0, 0), (2, 2));
        let chunk_size = Vec2::new(0.0, 0.0);

        let grid = rect.chunk_to_grid(chunk_size);
        assert_eq!(grid.shape(), [0, 0]);
    }

    #[test]
    fn negative_chunk_size_axis_returns_nothing() {
        let rect = qr((0, 0), (2, 2));

        let chunk_size = Vec2::new(0.0, -1.0);
        let grid = rect.chunk_to_grid(chunk_size);
        assert_eq!(grid.shape(), [0, 0]);

        let chunk_size = Vec2::new(-1.0, 0.0);
        let grid = rect.chunk_to_grid(chunk_size);
        assert_eq!(grid.shape(), [0, 0]);

        let chunk_size = Vec2::new(-1.0, -1.0);
        let grid = rect.chunk_to_grid(chunk_size);
        assert_eq!(grid.shape(), [0, 0]);
    }

    #[test]
    fn empty_rect_should_return_nothing() {
        let rect = qr((0, 0), (0, 2));
        let chunk_size = Vec2::new(2.0, 1.0);

        let grid = rect.chunk_to_grid(chunk_size);
        assert_eq!(grid.shape(), [0, 0]);
    }

    #[test]
    fn test_perfect_fit_chunking() {
        let rect = RectExt::from(((0.0, 0.0), (2.0, 2.0)));
        let chunk_size = Vec2::new(1.0, 1.0);
        let grid = rect.chunk_to_grid(chunk_size);

        let expected = array![
            [qr((0, 0), (1, 1)), qr((1, 0), (2, 1))],
            [qr((0, 1), (1, 2)), qr((1, 1), (2, 2))],
        ];

        assert_eq!(grid.shape(), [2, 2]);
        assert_eq!(grid, expected);
    }

    #[test]
    fn test_perfect_fit_chunking_rev_rect() {
        let rect = qr((2, 2), (0, 0));
        let chunk_size = Vec2::new(1.0, 1.0);
        let grid = rect.chunk_to_grid(chunk_size);

        let expected = array![
            [qr((0, 0), (1, 1)), qr((1, 0), (2, 1))],
            [qr((0, 1), (1, 2)), qr((1, 1), (2, 2))],
        ];

        assert_eq!(grid.shape(), [2, 2]);
        assert_eq!(grid, expected);
    }

    #[test]
    fn test_perfect_fit_chunking_with_rect_offset() {
        let rect = RectExt::new((2.0, 2.0), (4.0, 4.0));
        let chunk_size = Vec2::new(1.0, 1.0);
        let grid = rect.chunk_to_grid(chunk_size);

        let expected = array![
            [qr((2, 2), (3, 3)), qr((3, 2), (4, 3))],
            [qr((2, 3), (3, 4)), qr((3, 3), (4, 4))],
        ];

        assert_eq!(grid.shape(), [2, 2]);
        assert_eq!(grid, expected);
    }

    #[test]
    fn test_perfect_fit_chunking_with_rect_offset_between_minus_and_plus() {
        let rect = RectExt::new((-2.0, -2.0), (2.0, 2.0));
        let chunk_size = Vec2::new(2.0, 2.0);
        let grid = rect.chunk_to_grid(chunk_size);

        let expected = array![
            [qr((-2, -2), (0, 0)), qr((0, -2), (2, 0))],
            [qr((-2, 0), (0, 2)), qr((0, 0), (2, 2))],
        ];

        assert_eq!(grid.shape(), [2, 2]);
        assert_eq!(grid, expected);
    }

    #[test]
    fn test_partial_chunking_with_cuttings() {
        let rect = RectExt::new((0.0, 0.0), (2.5, 2.5));
        let chunk_size = Vec2::new(1.0, 1.0);
        let grid = rect.chunk_to_grid_with_cuttings(chunk_size);

        assert_eq!(grid.shape(), [3, 3]);
        assert_eq!(
            grid,
            array![
                [
                    RectExt::new((0.0, 0.0), (1.0, 1.0)),
                    RectExt::new((1.0, 0.0), (2.0, 1.0)),
                    RectExt::new((2.0, 0.0), (2.5, 1.0))
                ],
                [
                    RectExt::new((0.0, 1.0), (1.0, 2.0)),
                    RectExt::new((1.0, 1.0), (2.0, 2.0)),
                    RectExt::new((2.0, 1.0), (2.5, 2.0))
                ],
                [
                    RectExt::new((0.0, 2.0), (1.0, 2.5)),
                    RectExt::new((1.0, 2.0), (2.0, 2.5)),
                    RectExt::new((2.0, 2.0), (2.5, 2.5))
                ],
            ]
        );
    }

    #[test]
    fn test_partial_chunking_with_cuttings_rev_rect() {
        let rect = RectExt::new((2.5, 2.5), (0.0, 0.0));
        let chunk_size = Vec2::new(1.0, 1.0);
        let grid = rect.chunk_to_grid_with_cuttings(chunk_size);

        assert_eq!(grid.shape(), [3, 3]);
        assert_eq!(
            grid,
            array![
                [
                    RectExt::new((0.0, 0.0), (1.0, 1.0)),
                    RectExt::new((1.0, 0.0), (2.0, 1.0)),
                    RectExt::new((2.0, 0.0), (2.5, 1.0))
                ],
                [
                    RectExt::new((0.0, 1.0), (1.0, 2.0)),
                    RectExt::new((1.0, 1.0), (2.0, 2.0)),
                    RectExt::new((2.0, 1.0), (2.5, 2.0))
                ],
                [
                    RectExt::new((0.0, 2.0), (1.0, 2.5)),
                    RectExt::new((1.0, 2.0), (2.0, 2.5)),
                    RectExt::new((2.0, 2.0), (2.5, 2.5))
                ],
            ]
        );
    }

    #[test]
    fn test_smaller_than_chunk() {
        //         |
        //         |
        //         cc
        // --------rc-------
        //         |
        //         |
        //         |
        let rect = RectExt::new((0.0, 0.0), (1.0, 1.0));
        let chunk_size = Vec2::new(2.0, 2.0);

        // Without cuttings - should return empty
        let grid = rect.chunk_to_grid(chunk_size);
        assert_eq!(grid.shape(), [0, 0]);

        // With cuttings - should return single small chunk
        let grid = rect.chunk_to_grid_with_cuttings(chunk_size);
        assert_eq!(grid.shape(), [1, 1]);
        assert_eq!(grid[[0, 0]], RectExt::new((0.0, 0.0), (1.0, 1.0)));
    }
}
