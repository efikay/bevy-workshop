#![allow(dead_code)]

use core::hash::Hash;
use std::{collections::HashMap, ops::Range};

use bevy::reflect::Reflect;

type GridFn<KOuter, KInner> = fn(outer_key: &KOuter) -> fn(inner_fn: &KInner) -> Range<u8>;

#[derive(Reflect, Debug, Clone, Copy)]
pub struct RangeDoubleMapper<KOuter, KInner> {
    grid_fn: GridFn<KOuter, KInner>,

    outer_key: KOuter,
    inner_key: KInner,
    cursor: u8,
}
impl<KOuter, KInner> RangeDoubleMapper<KOuter, KInner>
where
    KOuter: PartialEq + Default + Copy + Clone,
    KInner: PartialEq + Default + Copy + Clone,
{
    pub fn new(grid_fn: GridFn<KOuter, KInner>) -> Self {
        let outer_key = KOuter::default();
        let inner_key = KInner::default();

        let cursor = grid_fn(&outer_key)(&inner_key).start;

        Self {
            grid_fn,
            outer_key,
            inner_key,
            cursor,
        }
    }

    #[inline]
    pub fn cursor(&self) -> u8 {
        self.cursor
    }
    #[inline]
    pub fn outer_key(&self) -> KOuter {
        self.outer_key
    }
    #[inline]
    pub fn inner_key(&self) -> KInner {
        self.inner_key
    }

    pub fn to_next(&mut self) -> u8 {
        let next = self.peek_next();

        self.cursor = next;

        next
    }

    pub fn update_outer(&mut self, outer_key: KOuter) {
        if self.outer_key != outer_key {
            self.outer_key = outer_key;

            self.reset_cursor();
        }
    }

    pub fn update_inner(&mut self, inner_key: KInner) {
        if self.inner_key != inner_key {
            self.inner_key = inner_key;

            self.reset_cursor();
        }
    }

    fn peek_next(&self) -> u8 {
        let next_item_guess = self.cursor + 1;

        if self.range().contains(&next_item_guess) {
            next_item_guess
        } else {
            self.range().start
        }
    }

    #[inline]
    fn reset_cursor(&mut self) {
        self.cursor = self.range().start;
    }

    #[inline]
    fn range(&self) -> Range<u8> {
        let grid_fn = self.grid_fn;

        grid_fn(&self.outer_key)(&self.inner_key)
    }
}
