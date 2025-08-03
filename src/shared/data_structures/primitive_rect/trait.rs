// Lint does not recognize that trait actually being used
#![allow(dead_code)]

use bevy::prelude::*;
use ndarray::Array2;

pub trait ChunkToGrid<T> {
    /// Return new grid which chunks onto pieces. <br />
    /// Does not modify source.
    ///
    /// ## Simple explanation:
    /// 1. Imagine a square-formed cake
    /// 2. We call the method with desired `chunk_size`
    /// 3. We get grid with all exact-sized chunks we can make from it
    /// 4. If there any cuttings left – we ignore them
    ///
    /// ## Abstract Cake Example:
    /// ```
    /// struct Cake {
    ///     pub width: u8,
    ///     pub height: u8,
    /// }
    /// impl ChunkToGrid<Cake> for Cake {
    ///   // ...
    /// }
    ///
    /// let cake = Cake { width: 2.5, height: 2.5 };
    ///
    /// let small_cakes = cake.chunk_to_grid(cake, Vec2::new(1.0, 1.0));
    ///
    /// assert_eq!(
    ///     Vec::from(small_cakes),
    ///     vec![
    ///         Cake { width: 1.0, height: 1.0 },
    ///         Cake { width: 1.0, height: 1.0 },
    ///         Cake { width: 1.0, height: 1.0 },
    ///         Cake { width: 1.0, height: 1.0 },
    ///     ]
    /// );
    /// ```
    ///
    /// ### Note
    /// <br />
    ///
    /// ### Considering the fact we discard cuttings, you may ask question: <br />
    /// "In case this abstract cake has real coordinates, position of new cakes may wary" <br />
    /// (there is option to discard top or bottom cuttings, left or right. Coordinates will differ) <br />
    ///
    /// ### Answer: <br />
    /// It defined by implementer's doc. Basic recommendation and supposed default is: <br />
    /// Cuttings are on right, on bottom
    ///
    fn chunk_to_grid(&self, chunk_size: Vec2) -> Array2<T>;

    /// Same as [`ChunkToGrid::chunk_to_grid`] but takes into account cuttings.
    /// Does not modify source.
    ///
    /// ## Abstract Cake Example:
    /// ```
    /// struct Cake {
    ///     pub width: u8,
    ///     pub height: u8,
    /// }
    /// impl ChunkToGrid<Cake> for Cake {
    ///   // ...
    /// }
    ///
    /// let cake = Cake { width: 2.5, height: 2.5 };
    ///
    /// let small_cakes = cake.chunk_to_grid(cake, Vec2::new(1.0, 1.0));
    ///
    /// assert_eq!(
    ///     Vec::from(small_cakes),
    ///     vec![
    ///         // first row
    ///         Cake { width: 1.0, height: 1.0 },
    ///         Cake { width: 1.0, height: 1.0 },
    ///         Cake { width: 0.5, height: 1.0 }, // cutting
    ///         // second row
    ///         Cake { width: 1.0, height: 1.0 },
    ///         Cake { width: 1.0, height: 1.0 },
    ///         Cake { width: 0.5, height: 1.0 }, // cutting
    ///         // Extra row of cuttings
    ///         Cake { width: 1.0, height: 0.5 },
    ///         Cake { width: 1.0, height: 0.5 },
    ///         Cake { width: 0.5, height: 0.5 }, // cut-cut-cut!
    ///     ]
    /// );
    /// ```
    fn chunk_to_grid_with_cuttings(&self, chunk_size: Vec2) -> Array2<T>;
}
