#[cfg(test)]
mod tests {
    // ── Core Aliases ────────────────────────────────────────────────────────
    use core::{
        marker::{
            PhantomData
        }
    };

    // ── Standard Library Aliases ────────────────────────────────────────────
    use std::{
        alloc::{
            System
        }
    };

    // ── Collection Aliases ──────────────────────────────────────────────────
    use std::vec::Vec; // Test with the Standard Library's `Vec` first
    //use collections::vec::Vec;

    // ── Function Tests ──────────────────────────────────────────────────────
    mod functions {
        use super::*;

        // ── `Vec::new()` Tests ──────────────────────────────────────────────
        // TODO
        mod new {
            use super::*;
        }

        // ── `Vec::with_capacity()` Tests ────────────────────────────────────
        // TODO
        mod with_capacity {
            use super::*;
        }

        // ── `Vec::from_raw_parts()` Tests ───────────────────────────────────
        // TODO
        mod from_raw_parts {
            use super::*;
        }
    }

    // ── Method Tests ────────────────────────────────────────────────────────
    mod methods {
        use super::*;

        // ── `Vec::append()` Tests ───────────────────────────────────────────
        // TODO
        mod append {
            use super::*;
        }

        // ── `Vec::as_mut_ptr()` Tests ───────────────────────────────────────
        // TODO
        mod as_mut_ptr {
            use super::*;
        }

        // ── `Vec::as_mut_slice()` Tests ─────────────────────────────────────
        // TODO
        mod as_mut_slice {
            use super::*;
        }

        // ── `Vec::as_ptr()` Tests ───────────────────────────────────────────
        // TODO
        mod as_ptr {
            use super::*;
        }

        // ── `Vec::as_slice()` Tests ─────────────────────────────────────────
        // TODO
        mod as_slice {
            use super::*;
        }

        // ── `Vec::capacity()` Tests ─────────────────────────────────────────
        // TODO
        mod capacity {
            use super::*;
        }

        // ── `Vec::clear()` Tests ────────────────────────────────────────────
        // TODO
        mod clear {
            use super::*;
        }

        // ── `Vec::dedup()` Tests ────────────────────────────────────────────
        // TODO
        mod dedup {
            use super::*;
        }

        // ── `Vec::dedup_by()` Tests ─────────────────────────────────────────
        // TODO
        mod dedup_by {
            use super::*;
        }

        // ── `Vec::dedup_by_key()` Tests ─────────────────────────────────────
        // TODO
        mod dedup_by_key {
            use super::*;
        }

        // ── `Vec::drain()` Tests ────────────────────────────────────────────
        // TODO
        mod drain {
            use super::*;
        }

        // ── `Vec::extend_from_slice()` Tests ────────────────────────────────
        // TODO
        mod extend_from_slice {
            use super::*;
        }

        // ── `Vec::extend_from_within()` Tests ───────────────────────────────
        // TODO
        mod extend_from_within {
            use super::*;
        }

        // ── `Vec::extract_if()` Tests ───────────────────────────────────────
        // TODO
        mod extract_if {
            use super::*;
        }

        // ── `Vec::insert()` Tests ───────────────────────────────────────────
        // TODO
        mod insert {
            use super::*;
        }

        // ── `Vec::insert_mut()` Tests ───────────────────────────────────────
        // TODO
        mod insert_mut {
            use super::*;
        }

        // ── `Vec::into_boxed_slice()` Tests ─────────────────────────────────
        // TODO
        mod into_boxed_slice {
            use super::*;
        }

        // ── `Vec::into_flattened()` Tests ───────────────────────────────────
        // TODO
        mod into_flattened {
            use super::*;
        }

        // ── `Vec::into_raw_parts()` Tests ───────────────────────────────────
        // TODO
        mod into_raw_parts {
            use super::*;
        }

        // ── `Vec::is_empty()` Tests ─────────────────────────────────────────
        // TODO
        mod is_empty {
            use super::*;
        }

        // ── `Vec::leak()` Tests ─────────────────────────────────────────────
        // TODO
        mod leak {
            use super::*;
        }

        // ── `Vec::len()` Tests ──────────────────────────────────────────────
        // TODO
        mod len {
            use super::*;
        }

        // ── `Vec::pop()` Tests ──────────────────────────────────────────────
        // TODO
        mod pop {
            use super::*;
        }

        // ── `Vec::pop_if()` Tests ───────────────────────────────────────────
        // TODO
        mod pop_if {
            use super::*;
        }

        // ── `Vec::push()` Tests ─────────────────────────────────────────────
        // TODO
        mod push {
            use super::*;
        }

        // ── `Vec::push_mut()` Tests ─────────────────────────────────────────
        // TODO
        mod push_mut {
            use super::*;
        }

        // ── `Vec::remove()` Tests ───────────────────────────────────────────
        // TODO
        mod remove {
            use super::*;
        }

        // ── `Vec::reserve()` Tests ──────────────────────────────────────────
        // TODO
        mod reserve {
            use super::*;
        }

        // ── `Vec::reserve_exact()` Tests ────────────────────────────────────
        // TODO
        mod reserve_exact {
            use super::*;
        }

        // ── `Vec::resize()` Tests ───────────────────────────────────────────
        // TODO
        mod resize {
            use super::*;
        }

        // ── `Vec::resize_with()` Tests ──────────────────────────────────────
        // TODO
        mod resize_with {
            use super::*;
        }

        // ── `Vec::retain()` Tests ───────────────────────────────────────────
        // TODO
        mod retain {
            use super::*;
        }

        // ── `Vec::retain_mut()` Tests ───────────────────────────────────────
        // TODO
        mod retain_mut {
            use super::*;
        }

        // ── `Vec::set_len()` Tests ──────────────────────────────────────────
        // TODO
        mod set_len {
            use super::*;
        }

        // ── `Vec::shrink_to()` Tests ────────────────────────────────────────
        // TODO
        mod shrink_to {
            use super::*;
        }

        // ── `Vec::shrink_to_fit()` Tests ────────────────────────────────────
        // TODO
        mod shrink_to_fit {
            use super::*;
        }

        // ── `Vec::spare_capacity_mut()` Tests ───────────────────────────────
        // TODO
        mod spare_capacity_mut {
            use super::*;
        }

        // ── `Vec::splice()` Tests ───────────────────────────────────────────
        // TODO
        mod splice {
            use super::*;
        }

        // ── `Vec::split_off()` Tests ────────────────────────────────────────
        // TODO
        mod split_off {
            use super::*;
        }

        // ── `Vec::swap_remove()` Tests ─────────────────────────────────────
        // TODO
        mod swap_remove {
            use super::*;
        }

        // ── `Vec::truncate()` Tests ─────────────────────────────────────────
        // TODO
        mod truncate {
            use super::*;
        }

        // ── `Vec::try_reserve()` Tests ──────────────────────────────────────
        // TODO
        mod try_reserve {
            use super::*;
        }

        // ── `Vec::try_reserve_exact()` Tests ────────────────────────────────
        // TODO
        mod try_reserve_exact {
            use super::*;
        }
    }
}
