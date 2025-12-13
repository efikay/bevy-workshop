use bevy::{math::VectorSpace, prelude::*};
use ndarray::prelude::*;

use crate::utils::vec2;

use super::r#trait::ChunkToGrid;

#[derive(Debug, PartialEq)]
pub struct RectExt {
    rect: Rect,
}
impl RectExt {
    pub fn new(x0: f32, y0: f32, x1: f32, y1: f32) -> Self {
        let p0 = Vec2::new(x0, y0);
        let p1 = Vec2::new(x1, y1);

        Self {
            rect: Rect::from_corners(p0, p1),
        }
    }
}

impl From<Rect> for RectExt {
    fn from(rect: Rect) -> Self {
        Self { rect }
    }
}

impl RectExt {
    fn actually_chunk(&self, chunk_size: Vec2, include_cuttings: bool) -> Array2<Rect> {
        let rect = self.rect;

        if rect.is_empty()
            || chunk_size.length() <= f32::EPSILON
            || vec2::is_negative_direction(chunk_size)
            || !include_cuttings && chunk_size.cmpgt(rect.size()).any()
        {
            return Array2::from_shape_vec((0, 0), vec![]).ok().unwrap();
        }

        let min = rect.min;
        let max = rect.max;

        // Calculate number of chunks including partial ones
        let cols = (rect.width() / chunk_size.x).ceil() as usize;
        let rows = (rect.height() / chunk_size.y).ceil() as usize;

        Array2::from_shape_fn((rows, cols), |(j, i)| {
            let x = min.x + i as f32 * chunk_size.x;
            let y = min.y + j as f32 * chunk_size.y;

            let is_last_col = i == cols - 1;
            let is_last_row = j == rows - 1;

            let chunk_width = if include_cuttings && is_last_col {
                max.x - (i as f32 * chunk_size.x)
            } else {
                chunk_size.x.min(rect.width())
            };

            let chunk_height = if include_cuttings && is_last_row {
                max.y - (j as f32 * chunk_size.y)
            } else {
                chunk_size.y.min(rect.height())
            };

            Rect::new(x, y, x + chunk_width, y + chunk_height)
        })
    }
}

impl ChunkToGrid<Rect> for RectExt {
    fn chunk_to_grid(&self, chunk_size: Vec2) -> Array2<Rect> {
        self.actually_chunk(chunk_size, false)
    }

    fn chunk_to_grid_with_cuttings(&self, chunk_size: Vec2) -> Array2<Rect> {
        self.actually_chunk(chunk_size, true)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn zero_chunk_size_returns_nothing() {
        let rect = RectExt::new(0., 0., 2., 2.);
        let chunk_size = Vec2::new(0.0, 0.0);

        let grid = rect.chunk_to_grid(chunk_size);
        assert_eq!(grid.shape(), [0, 0]);
    }

    #[test]
    fn negative_chunk_size_axis_returns_nothing() {
        let rect = RectExt::new(0., 0., 2., 2.);

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
        let rect = RectExt::new(0., 0., 0., 2.);
        let chunk_size = Vec2::new(2.0, 1.0);

        let grid = rect.chunk_to_grid(chunk_size);
        assert_eq!(grid.shape(), [0, 0]);
    }

    #[test]
    fn test_perfect_fit_chunking() {
        let rect = RectExt::from(Rect::new(0.0, 0.0, 2.0, 2.0));
        let chunk_size = Vec2::new(1.0, 1.0);
        let grid = rect.chunk_to_grid(chunk_size);

        let expected = array![
            [Rect::new(0., 0., 1., 1.), Rect::new(1., 0., 2., 1.)],
            [Rect::new(0., 1., 1., 2.), Rect::new(1., 1., 2., 2.)],
        ];

        assert_eq!(grid.shape(), [2, 2]);
        assert_eq!(grid, expected);
    }

    #[test]
    fn test_perfect_fit_chunking_rev_params() {
        let rect = RectExt::new(2., 2., 0., 0.);
        let chunk_size = Vec2::new(1.0, 1.0);
        let grid = rect.chunk_to_grid(chunk_size);

        let expected = array![
            [Rect::new(0., 0., 1., 1.), Rect::new(1., 0., 2., 1.)],
            [Rect::new(0., 1., 1., 2.), Rect::new(1., 1., 2., 2.)],
        ];

        assert_eq!(grid.shape(), [2, 2]);
        assert_eq!(grid, expected);
    }

    #[test]
    fn test_perfect_fit_chunking_with_rect_offset() {
        let rect = RectExt::new(2.0, 2.0, 4.0, 4.0);
        let chunk_size = Vec2::new(1.0, 1.0);
        let grid = rect.chunk_to_grid(chunk_size);

        let expected = array![
            [Rect::new(2., 2., 3., 3.), Rect::new(3., 2., 4., 3.)],
            [Rect::new(2., 3., 3., 4.), Rect::new(3., 3., 4., 4.)],
        ];

        assert_eq!(grid.shape(), [2, 2]);
        assert_eq!(grid, expected);
    }

    #[test]
    fn test_perfect_fit_chunking_with_rect_offset_between_minus_and_plus() {
        let rect = RectExt::from(Rect::new(-2.0, -2.0, 2.0, 2.0));
        let chunk_size = Vec2::new(2.0, 2.0);
        let grid = rect.chunk_to_grid(chunk_size);

        let expected = array![
            [Rect::new(-2., -2., 0., 0.), Rect::new(0., -2., 2., 0.)],
            [Rect::new(-2., 0., 0., 2.), Rect::new(0., 0., 2., 2.)],
        ];

        assert_eq!(grid.shape(), [2, 2]);
        assert_eq!(grid, expected);
    }

    #[test]
    fn test_partial_chunking_with_cuttings() {
        let rect = RectExt::new(0.0, 0.0, 2.5, 2.5);
        let chunk_size = Vec2::new(1.0, 1.0);
        let grid = rect.chunk_to_grid_with_cuttings(chunk_size);

        assert_eq!(grid.shape(), [3, 3]);
        assert_eq!(
            grid,
            array![
                [
                    Rect::new(0.0, 0.0, 1.0, 1.0),
                    Rect::new(1.0, 0.0, 2.0, 1.0),
                    Rect::new(2.0, 0.0, 2.5, 1.0)
                ],
                [
                    Rect::new(0.0, 1.0, 1.0, 2.0),
                    Rect::new(1.0, 1.0, 2.0, 2.0),
                    Rect::new(2.0, 1.0, 2.5, 2.0)
                ],
                [
                    Rect::new(0.0, 2.0, 1.0, 2.5),
                    Rect::new(1.0, 2.0, 2.0, 2.5),
                    Rect::new(2.0, 2.0, 2.5, 2.5)
                ],
            ]
        );
    }

    #[test]
    fn test_partial_chunking_with_cuttings_rev_rect() {
        let rect = RectExt::new(2.5, 2.5, 0.0, 0.0);
        let chunk_size = Vec2::new(1.0, 1.0);
        let grid = rect.chunk_to_grid_with_cuttings(chunk_size);

        assert_eq!(grid.shape(), [3, 3]);
        assert_eq!(
            grid,
            array![
                [
                    Rect::new(0.0, 0.0, 1.0, 1.0),
                    Rect::new(1.0, 0.0, 2.0, 1.0),
                    Rect::new(2.0, 0.0, 2.5, 1.0)
                ],
                [
                    Rect::new(0.0, 1.0, 1.0, 2.0),
                    Rect::new(1.0, 1.0, 2.0, 2.0),
                    Rect::new(2.0, 1.0, 2.5, 2.0)
                ],
                [
                    Rect::new(0.0, 2.0, 1.0, 2.5),
                    Rect::new(1.0, 2.0, 2.0, 2.5),
                    Rect::new(2.0, 2.0, 2.5, 2.5)
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
        let rect = RectExt::from(Rect::new(0.0, 0.0, 1.0, 1.0));
        let chunk_size = Vec2::new(2.0, 2.0);

        // Without cuttings - should return empty
        let grid = rect.chunk_to_grid(chunk_size);
        assert_eq!(grid.len(), 0);

        // With cuttings - should return single small chunk
        let grid = rect.chunk_to_grid_with_cuttings(chunk_size);
        assert_eq!(grid.len(), 1);
        assert_eq!(grid[[0, 0]], Rect::new(0.0, 0.0, 1.0, 1.0));
    }
}
