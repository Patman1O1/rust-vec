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

        // ── `Vec::try_with_capacity()` Tests ────────────────────────────────
        // TODO
        mod try_with_capacity {

        }

        // ── `Vec::from_raw_parts()` Tests ───────────────────────────────────
        // TODO
        mod from_raw_parts {
            use super::*;
        }

        // ── `Vec::from_parts()` Tests ───────────────────────────────────────
        // TODO
        mod from_parts {
            use super::*;
        }
        
        // ── `Vec::from_fn()` Tests ──────────────────────────────────────────
        // TODO
        mod from_fn {
            use super::*;
        }

        // ── `Vec::with_capacity_in()` Tests ─────────────────────────────────
        // TODO
        mod with_capacity_in {
            use super::*;    
        }

        // ── `Vec::new_in()` Tests ───────────────────────────────────────────
        // TODO
        mod new_in {
            use super::*;
        }

        // ── `Vec::try_with_capacity_in()` Tests ─────────────────────────────
        // TODO
        mod try_with_capacity_in {
            use super::*;
        }

        // ── `Vec::from_raw_parts_in()` Tests ────────────────────────────────
        // TODO
        mod from_raw_parts_in {
            use super::*;
        }

        // ── `Vec::from_parts_in()` Tests ────────────────────────────────────
        // TODO
        mod from_parts_in {
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

        // ── `Vec::swap_remove()` Tests ──────────────────────────────────────
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

    // ── `Deref` Method Tests ────────────────────────────────────────────────
    mod dref_methods {
        use super::*;

        // ── `<[T]>::align_to()` Tests ───────────────────────────────────────
        // TODO
        mod align_to {
            use super::*;
        }

        // ── `<[T]>::align_to_mut()` Tests ───────────────────────────────────
        // TODO
        mod align_to_mut {
            use super::*;
        }

        // ── `<[T]>::array_windows()` Tests ──────────────────────────────────
        // TODO
        mod array_windows {
            use super::*;
        }

        // ── `<[T]>::as_array()` Tests ───────────────────────────────────────
        // TODO
        mod as_array {
            use super::*;
        }

        // ── `<[T]>::as_chunks()` Tests ──────────────────────────────────────
        // TODO
        mod as_chunks {
            use super::*;
        }

        // ── `<[T]>::as_chunks_mut()` Tests ──────────────────────────────────
        // TODO
        mod as_chunks_mut {
            use super::*;
        }

        // ── `<[T]>::as_chunks_unchecked()` Tests ────────────────────────────
        // TODO
        mod as_chunks_unchecked {
            use super::*;
        }

        // ── `<[T]>::as_chunks_unchecked_mut()` Tests ────────────────────────
        // TODO
        mod as_chunks_unchecked_mut {
            use super::*;
        }

        // ── `<[T]>::as_mut_array()` Tests ───────────────────────────────────
        // TODO
        mod as_mut_array {
            use super::*;
        }

        // ── `<[T]>::as_mut_ptr()` Tests ─────────────────────────────────────
        // TODO
        mod as_mut_ptr {
            use super::*;
        }

        // ── `<[T]>::as_mut_ptr_range()` Tests ───────────────────────────────
        // TODO
        mod as_mut_ptr_range {
            use super::*;
        }

        // ── `<[T]>::as_mut_slice()` Tests ───────────────────────────────────
        // TODO
        mod as_mut_slice {
            use super::*;
        }

        // ── `<[T]>::as_ptr()` Tests ─────────────────────────────────────────
        // TODO
        mod as_ptr {
            use super::*;
        }

        // ── `<[T]>::as_ptr_range()` Tests ───────────────────────────────────
        // TODO
        mod as_ptr_range {
            use super::*;
        }

        // ── `<[T]>::as_rchunks()` Tests ─────────────────────────────────────
        // TODO
        mod as_rchunks {
            use super::*;
        }

        // ── `<[T]>::as_rchunks_mut()` Tests ─────────────────────────────────
        // TODO
        mod as_rchunks_mut {
            use super::*;
        }

        // ── `<[T]>::as_simd()` Tests ────────────────────────────────────────
        // TODO
        mod as_simd {
            use super::*;
        }

        // ── `<[T]>::as_simd_mut()` Tests ────────────────────────────────────
        // TODO
        mod as_simd_mut {
            use super::*;
        }

        // ── `<[T]>::as_slice()` Tests ───────────────────────────────────────
        // TODO
        mod as_slice {
            use super::*;
        }

        // ── `<[T]>::binary_search()` Tests ──────────────────────────────────
        // TODO
        mod binary_search {
            use super::*;
        }

        // ── `<[T]>::binary_search_by()` Tests ───────────────────────────────
        // TODO
        mod binary_search_by {
            use super::*;
        }

        // ── `<[T]>::binary_search_by_key()` Tests ───────────────────────────
        // TODO
        mod binary_search_by_key {
            use super::*;
        }

        // ── `<[T]>::chunk_by()` Tests ───────────────────────────────────────
        // TODO
        mod chunk_by {
            use super::*;
        }

        // ── `<[T]>::chunk_by_mut()` Tests ───────────────────────────────────
        // TODO
        mod chunk_by_mut {
            use super::*;
        }

        // ── `<[T]>::chunks()` Tests ─────────────────────────────────────────
        // TODO
        mod chunks {
            use super::*;
        }

        // ── `<[T]>::chunks_exact()` Tests ───────────────────────────────────
        // TODO
        mod chunks_exact {
            use super::*;
        }

        // ── `<[T]>::chunks_exact_mut()` Tests ───────────────────────────────
        // TODO
        mod chunks_exact_mut {
            use super::*;
        }

        // ── `<[T]>::chunks_mut()` Tests ─────────────────────────────────────
        // TODO
        mod chunks_mut {
            use super::*;
        }

        // ── `<[T]>::clone_from_slice()` Tests ───────────────────────────────
        // TODO
        mod clone_from_slice {
            use super::*;
        }

        // ── `<[T]>::concat()` Tests ─────────────────────────────────────────
        // TODO
        mod concat {
            use super::*;
        }

        // ── `<[T]>::connect()` Tests ────────────────────────────────────────
        // TODO
        mod connect {
            use super::*;
        }

        // ── `<[T]>::contains()` Tests ───────────────────────────────────────
        // TODO
        mod contains {
            use super::*;
        }

        // ── `<[T]>::copy_from_slice()` Tests ────────────────────────────────
        // TODO
        mod copy_from_slice {
            use super::*;
        }

        // ── `<[T]>::copy_within()` Tests ────────────────────────────────────
        // TODO
        mod copy_within {
            use super::*;
        }

        // ── `<[T]>::element_offset()` Tests ─────────────────────────────────
        // TODO
        mod element_offset {
            use super::*;
        }

        // ── `<[T]>::ends_with()` Tests ──────────────────────────────────────
        // TODO
        mod ends_with {
            use super::*;
        }

        // ── `<[T]>::fill()` Tests ───────────────────────────────────────────
        // TODO
        mod fill {
            use super::*;
        }

        // ── `<[T]>::fill_with()` Tests ──────────────────────────────────────
        // TODO
        mod fill_with {
            use super::*;
        }

        // ── `<[T]>::first()` Tests ──────────────────────────────────────────
        // TODO
        mod first {
            use super::*;
        }

        // ── `<[T]>::first_chunk()` Tests ────────────────────────────────────
        // TODO
        mod first_chunk {
            use super::*;
        }

        // ── `<[T]>::first_chunk_mut()` Tests ────────────────────────────────
        // TODO
        mod first_chunk_mut {
            use super::*;
        }

        // ── `<[T]>::first_mut()` Tests ──────────────────────────────────────
        // TODO
        mod first_mut {
            use super::*;
        }

        // ── `<[T]>::get()` Tests ────────────────────────────────────────────
        // TODO
        mod get {
            use super::*;
        }

        // ── `<[T]>::get_disjoint_mut()` Tests ───────────────────────────────
        // TODO
        mod get_disjoint_mut {
            use super::*;
        }

        // ── `<[T]>::get_disjoint_unchecked_mut()` Tests ─────────────────────
        // TODO
        mod get_disjoint_unchecked_mut {
            use super::*;
        }

        // ── `<[T]>::get_mut()` Tests ────────────────────────────────────────
        // TODO
        mod get_mut {
            use super::*;
        }

        // ── `<[T]>::get_unchecked()` Tests ──────────────────────────────────
        // TODO
        mod get_unchecked {
            use super::*;
        }

        // ── `<[T]>::get_unchecked_mut()` Tests ──────────────────────────────
        // TODO
        mod get_unchecked_mut {
            use super::*;
        }

        // ── `<[T]>::is_empty()` Tests ───────────────────────────────────────
        // TODO
        mod is_empty {
            use super::*;
        }

        // ── `<[T]>::is_sorted()` Tests ──────────────────────────────────────
        // TODO
        mod is_sorted {
            use super::*;
        }

        // ── `<[T]>::is_sorted_by()` Tests ───────────────────────────────────
        // TODO
        mod is_sorted_by {
            use super::*;
        }

        // ── `<[T]>::is_sorted_by_key()` Tests ───────────────────────────────
        // TODO
        mod is_sorted_by_key {
            use super::*;
        }

        // ── `<[T]>::iter()` Tests ───────────────────────────────────────────
        // TODO
        mod iter {
            use super::*;
        }

        // ── `<[T]>::iter_mut()` Tests ───────────────────────────────────────
        // TODO
        mod iter_mut {
            use super::*;
        }

        // ── `<[T]>::join()` Tests ───────────────────────────────────────────
        // TODO
        mod join {
            use super::*;
        }

        // ── `<[T]>::last()` Tests ───────────────────────────────────────────
        // TODO
        mod last {
            use super::*;
        }

        // ── `<[T]>::last_chunk()` Tests ─────────────────────────────────────
        // TODO
        mod last_chunk {
            use super::*;
        }

        // ── `<[T]>::last_chunk_mut()` Tests ─────────────────────────────────
        // TODO
        mod last_chunk_mut {
            use super::*;
        }

        // ── `<[T]>::last_mut()` Tests ───────────────────────────────────────
        // TODO
        mod last_mut {
            use super::*;
        }

        // ── `<[T]>::len()` Tests ────────────────────────────────────────────
        // TODO
        mod len {
            use super::*;
        }

        // ── `<[T]>::partial_sort_unstable()` Tests ──────────────────────────
        // TODO
        mod partial_sort_unstable {
            use super::*;
        }

        // ── `<[T]>::partial_sort_unstable_by()` Tests ───────────────────────
        // TODO
        mod partial_sort_unstable_by {
            use super::*;
        }

        // ── `<[T]>::partial_sort_unstable_by_key()` Tests ───────────────────
        // TODO
        mod partial_sort_unstable_by_key {
            use super::*;
        }

        // ── `<[T]>::partition_dedup()` Tests ────────────────────────────────
        // TODO
        mod partition_dedup {
            use super::*;
        }

        // ── `<[T]>::partition_dedup_by()` Tests ─────────────────────────────
        // TODO
        mod partition_dedup_by {
            use super::*;
        }

        // ── `<[T]>::partition_dedup_by_key()` Tests ─────────────────────────
        // TODO
        mod partition_dedup_by_key {
            use super::*;
        }

        // ── `<[T]>::partition_point()` Tests ────────────────────────────────
        // TODO
        mod partition_point {
            use super::*;
        }

        // ── `<[T]>::rchunks()` Tests ────────────────────────────────────────
        // TODO
        mod rchunks {
            use super::*;
        }

        // ── `<[T]>::rchunks_exact()` Tests ──────────────────────────────────
        // TODO
        mod rchunks_exact {
            use super::*;
        }

        // ── `<[T]>::rchunks_exact_mut()` Tests ──────────────────────────────
        // TODO
        mod rchunks_exact_mut {
            use super::*;
        }

        // ── `<[T]>::rchunks_mut()` Tests ────────────────────────────────────
        // TODO
        mod rchunks_mut {
            use super::*;
        }

        // ── `<[T]>::repeat()` Tests ─────────────────────────────────────────
        // TODO
        mod repeat {
            use super::*;
        }

        // ── `<[T]>::reverse()` Tests ────────────────────────────────────────
        // TODO
        mod reverse {
            use super::*;
        }

        // ── `<[T]>::rotate_left()` Tests ────────────────────────────────────
        // TODO
        mod rotate_left {
            use super::*;
        }

        // ── `<[T]>::rotate_right()` Tests ───────────────────────────────────
        // TODO
        mod rotate_right {
            use super::*;
        }

        // ── `<[T]>::rsplit()` Tests ─────────────────────────────────────────
        // TODO
        mod rsplit {
            use super::*;
        }

        // ── `<[T]>::rsplit_mut()` Tests ─────────────────────────────────────
        // TODO
        mod rsplit_mut {
            use super::*;
        }

        // ── `<[T]>::rsplit_once()` Tests ────────────────────────────────────
        // TODO
        mod rsplit_once {
            use super::*;
        }

        // ── `<[T]>::rsplitn()` Tests ────────────────────────────────────────
        // TODO
        mod rsplitn {
            use super::*;
        }

        // ── `<[T]>::rsplitn_mut()` Tests ────────────────────────────────────
        // TODO
        mod rsplitn_mut {
            use super::*;
        }

        // ── `<[T]>::select_nth_unstable()` Tests ───────────────────────────
        // TODO
        mod select_nth_unstable {
            use super::*;
        }

        // ── `<[T]>::select_nth_unstable_by()` Tests ────────────────────────
        // TODO
        mod select_nth_unstable_by {
            use super::*;
        }

        // ── `<[T]>::select_nth_unstable_by_key()` Tests ────────────────────
        // TODO
        mod select_nth_unstable_by_key {
            use super::*;
        }

        // ── `<[T]>::shift_left()` Tests ─────────────────────────────────────
        // TODO
        mod shift_left {
            use super::*;
        }

        // ── `<[T]>::shift_right()` Tests ────────────────────────────────────
        // TODO
        mod shift_right {
            use super::*;
        }

        // ── `<[T]>::sort()` Tests ───────────────────────────────────────────
        // TODO
        mod sort {
            use super::*;
        }

        // ── `<[T]>::sort_by()` Tests ────────────────────────────────────────
        // TODO
        mod sort_by {
            use super::*;
        }

        // ── `<[T]>::sort_by_cached_key()` Tests ─────────────────────────────
        // TODO
        mod sort_by_cached_key {
            use super::*;
        }

        // ── `<[T]>::sort_by_key()` Tests ────────────────────────────────────
        // TODO
        mod sort_by_key {
            use super::*;
        }

        // ── `<[T]>::sort_unstable()` Tests ──────────────────────────────────
        // TODO
        mod sort_unstable {
            use super::*;
        }

        // ── `<[T]>::sort_unstable_by()` Tests ───────────────────────────────
        // TODO
        mod sort_unstable_by {
            use super::*;
        }

        // ── `<[T]>::sort_unstable_by_key()` Tests ───────────────────────────
        // TODO
        mod sort_unstable_by_key {
            use super::*;
        }

        // ── `<[T]>::split()` Tests ──────────────────────────────────────────
        // TODO
        mod split {
            use super::*;
        }

        // ── `<[T]>::split_at()` Tests ───────────────────────────────────────
        // TODO
        mod split_at {
            use super::*;
        }

        // ── `<[T]>::split_at_checked()` Tests ───────────────────────────────
        // TODO
        mod split_at_checked {
            use super::*;
        }

        // ── `<[T]>::split_at_mut()` Tests ───────────────────────────────────
        // TODO
        mod split_at_mut {
            use super::*;
        }

        // ── `<[T]>::split_at_mut_checked()` Tests ───────────────────────────
        // TODO
        mod split_at_mut_checked {
            use super::*;
        }

        // ── `<[T]>::split_at_mut_unchecked()` Tests ─────────────────────────
        // TODO
        mod split_at_mut_unchecked {
            use super::*;
        }

        // ── `<[T]>::split_at_unchecked()` Tests ─────────────────────────────
        // TODO
        mod split_at_unchecked {
            use super::*;
        }

        // ── `<[T]>::split_first()` Tests ────────────────────────────────────
        // TODO
        mod split_first {
            use super::*;
        }

        // ── `<[T]>::split_first_chunk()` Tests ──────────────────────────────
        // TODO
        mod split_first_chunk {
            use super::*;
        }

        // ── `<[T]>::split_first_chunk_mut()` Tests ──────────────────────────
        // TODO
        mod split_first_chunk_mut {
            use super::*;
        }

        // ── `<[T]>::split_first_mut()` Tests ────────────────────────────────
        // TODO
        mod split_first_mut {
            use super::*;
        }

        // ── `<[T]>::split_inclusive()` Tests ────────────────────────────────
        // TODO
        mod split_inclusive {
            use super::*;
        }

        // ── `<[T]>::split_inclusive_mut()` Tests ────────────────────────────
        // TODO
        mod split_inclusive_mut {
            use super::*;
        }

        // ── `<[T]>::split_last()` Tests ─────────────────────────────────────
        // TODO
        mod split_last {
            use super::*;
        }

        // ── `<[T]>::split_last_chunk()` Tests ───────────────────────────────
        // TODO
        mod split_last_chunk {
            use super::*;
        }

        // ── `<[T]>::split_last_chunk_mut()` Tests ───────────────────────────
        // TODO
        mod split_last_chunk_mut {
            use super::*;
        }

        // ── `<[T]>::split_last_mut()` Tests ─────────────────────────────────
        // TODO
        mod split_last_mut {
            use super::*;
        }

        // ── `<[T]>::split_mut()` Tests ──────────────────────────────────────
        // TODO
        mod split_mut {
            use super::*;
        }

        // ── `<[T]>::split_off()` Tests ──────────────────────────────────────
        // TODO
        mod split_off {
            use super::*;
        }

        // ── `<[T]>::split_off_first()` Tests ────────────────────────────────
        // TODO
        mod split_off_first {
            use super::*;
        }

        // ── `<[T]>::split_off_first_mut()` Tests ────────────────────────────
        // TODO
        mod split_off_first_mut {
            use super::*;
        }

        // ── `<[T]>::split_off_last()` Tests ─────────────────────────────────
        // TODO
        mod split_off_last {
            use super::*;
        }

        // ── `<[T]>::split_off_last_mut()` Tests ─────────────────────────────
        // TODO
        mod split_off_last_mut {
            use super::*;
        }

        // ── `<[T]>::split_off_mut()` Tests ──────────────────────────────────
        // TODO
        mod split_off_mut {
            use super::*;
        }

        // ── `<[T]>::split_once()` Tests ─────────────────────────────────────
        // TODO
        mod split_once {
            use super::*;
        }

        // ── `<[T]>::splitn()` Tests ─────────────────────────────────────────
        // TODO
        mod splitn {
            use super::*;
        }

        // ── `<[T]>::splitn_mut()` Tests ─────────────────────────────────────
        // TODO
        mod splitn_mut {
            use super::*;
        }

        // ── `<[T]>::starts_with()` Tests ────────────────────────────────────
        // TODO
        mod starts_with {
            use super::*;
        }

        // ── `<[T]>::strip_circumfix()` Tests ────────────────────────────────
        // TODO
        mod strip_circumfix {
            use super::*;
        }

        // ── `<[T]>::strip_prefix()` Tests ───────────────────────────────────
        // TODO
        mod strip_prefix {
            use super::*;
        }

        // ── `<[T]>::strip_suffix()` Tests ───────────────────────────────────
        // TODO
        mod strip_suffix {
            use super::*;
        }

        // ── `<[T]>::subslice_range()` Tests ─────────────────────────────────
        // TODO
        mod subslice_range {
            use super::*;
        }

        // ── `<[T]>::swap()` Tests ───────────────────────────────────────────
        // TODO
        mod swap {
            use super::*;
        }

        // ── `<[T]>::swap_unchecked()` Tests ─────────────────────────────────
        // TODO
        mod swap_unchecked {
            use super::*;
        }

        // ── `<[T]>::swap_with_slice()` Tests ────────────────────────────────
        // TODO
        mod swap_with_slice {
            use super::*;
        }

        // ── `<[T]>::to_vec()` Tests ─────────────────────────────────────────
        // TODO
        mod to_vec {
            use super::*;
        }

        // ── `<[T]>::to_vec_in()` Tests ──────────────────────────────────────
        // TODO
        mod to_vec_in {
            use super::*;
        }

        // ── `<[T]>::trim_prefix()` Tests ────────────────────────────────────
        // TODO
        mod trim_prefix {
            use super::*;
        }

        // ── `<[T]>::trim_suffix()` Tests ────────────────────────────────────
        // TODO
        mod trim_suffix {
            use super::*;
        }

        // ── `<[T]>::windows()` Tests ────────────────────────────────────────
        // TODO
        mod windows {
            use super::*;
        }
    }

    // ── Trait Implementations ───────────────────────────────────────────────
    mod trait_implementations {
        use super::*;

        // ── `AsMut<Vec<T, A>>` Tests ────────────────────────────────────────
        // TODO
        mod as_mut_vec {
            use super::*;
        }

        // ── `AsMut<[T]>` Tests ──────────────────────────────────────────────
        // TODO
        mod as_mut_slice {
            use super::*;
        }

        // ── `AsRef<Vec<T, A>>` Tests ────────────────────────────────────────
        // TODO
        mod as_ref_vec {
            use super::*;
        }

        // ── `AsRef<[T]>` Tests ──────────────────────────────────────────────
        // TODO
        mod as_ref_slice {
            use super::*;
        }

        // ── `Borrow<[T]>` Tests ─────────────────────────────────────────────
        // TODO
        mod borrow_slice {
            use super::*;
        }

        // ── `BorrowMut<[T]>` Tests ──────────────────────────────────────────
        // TODO
        mod borrow_mut_slice {
            use super::*;
        }

        // ── `Clone` Tests ───────────────────────────────────────────────────
        // TODO
        mod clone {
            use super::*;
        }

        // ── `Debug` Tests ───────────────────────────────────────────────────
        // TODO
        mod debug {
            use super::*;
        }

        // ── `Default` Tests ─────────────────────────────────────────────────
        // TODO
        mod default {
            use super::*;
        }

        // ── `Deref` Tests ───────────────────────────────────────────────────
        // TODO
        mod deref {
            use super::*;
        }

        // ── `DerefMut` Tests ────────────────────────────────────────────────
        // TODO
        mod deref_mut {
            use super::*;
        }

        // ── `DerefPure` Tests ───────────────────────────────────────────────
        // TODO
        mod deref_pure {
            use super::*;
        }

        // ── `Drop` Tests ────────────────────────────────────────────────────
        // TODO
        mod drop {
            use super::*;
        }

        // ── `Eq` Tests ──────────────────────────────────────────────────────
        // TODO
        mod eq {
            use super::*;
        }

        // ── `Extend<&'a T>` Tests ───────────────────────────────────────────
        // TODO
        mod extend_ref {
            use super::*;
        }

        // ── `Extend<T>` Tests ───────────────────────────────────────────────
        // TODO
        mod extend_val {
            use super::*;
        }

        // ── `From<&'a Vec<T>>` Tests ────────────────────────────────────────
        // TODO
        mod from_vec_ref {
            use super::*;
        }

        // ── `From<&[T; N]>` Tests ───────────────────────────────────────────
        // TODO
        mod from_array_ref {
            use super::*;
        }

        // ── `From<&[T]>` Tests ──────────────────────────────────────────────
        // TODO
        mod from_slice_ref {
            use super::*;
        }

        // ── `From<&mut [T; N]>` Tests ───────────────────────────────────────
        // TODO
        mod from_array_mut_ref {
            use super::*;
        }

        // ── `From<&mut [T]>` Tests ──────────────────────────────────────────
        // TODO
        mod from_slice_mut_ref {
            use super::*;
        }

        // ── `From<&str>` Tests ──────────────────────────────────────────────
        // TODO
        mod from_str {
            use super::*;
        }

        // ── `From<BinaryHeap<T, A>>` Tests ──────────────────────────────────
        // TODO
        mod from_binary_heap {
            use super::*;
        }

        // ── `From<Box<[T], A>>` Tests ───────────────────────────────────────
        // TODO
        mod from_boxed_slice {
            use super::*;
        }

        // ── `From<ByteString>` Tests ────────────────────────────────────────
        // TODO
        mod from_byte_string {
            use super::*;
        }

        // ── `From<CString>` Tests ───────────────────────────────────────────
        // TODO
        mod from_c_string {
            use super::*;
        }

        // ── `From<Cow<'a, [T]>>` Tests ──────────────────────────────────────
        // TODO
        mod from_cow_slice {
            use super::*;
        }

        // ── `From<String>` Tests ────────────────────────────────────────────
        // TODO
        mod from_string {
            use super::*;
        }

        // ── `From<Vec<NonZero<u8>>>` Tests ──────────────────────────────────
        // TODO
        mod from_vec_nonzero_u8 {
            use super::*;
        }

        // ── `From<Vec<T, A>>` Tests ─────────────────────────────────────────
        // TODO
        mod from_vec_alloc {
            use super::*;
        }

        // ── `From<Vec<T>>` Tests ────────────────────────────────────────────
        // TODO
        mod from_vec {
            use super::*;
        }

        // ── `From<VecDeque<T, A>>` Tests ────────────────────────────────────
        // TODO
        mod from_vec_deque {
            use super::*;
        }

        // ── `From<[T; N]>` Tests ────────────────────────────────────────────
        // TODO
        mod from_array {
            use super::*;
        }

        // ── `FromIterator<T>` Tests ─────────────────────────────────────────
        // TODO
        mod from_iterator {
            use super::*;
        }

        // ── `Hash` Tests ────────────────────────────────────────────────────
        // TODO
        mod hash {
            use super::*;
        }

        // ── `Index<I>` Tests ────────────────────────────────────────────────
        // TODO
        mod index {
            use super::*;
        }

        // ── `IndexMut<I>` Tests ─────────────────────────────────────────────
        // TODO
        mod index_mut {
            use super::*;
        }

        // ── `IntoIterator` (Value) Tests ────────────────────────────────────
        // TODO
        mod into_iterator_val {
            use super::*;
        }

        // ── `IntoIterator` (Ref) Tests ──────────────────────────────────────
        // TODO
        mod into_iterator_ref {
            use super::*;
        }

        // ── `IntoIterator` (Mut Ref) Tests ──────────────────────────────────
        // TODO
        mod into_iterator_mut_ref {
            use super::*;
        }

        // ── `Ord` Tests ─────────────────────────────────────────────────────
        // TODO
        mod ord {
            use super::*;
        }

        // ── `PartialEq<&[U; N]>` Tests ──────────────────────────────────────
        // TODO
        mod partial_eq_array_ref {
            use super::*;
        }

        // ── `PartialEq<&[U]>` Tests ─────────────────────────────────────────
        // TODO
        mod partial_eq_slice_ref {
            use super::*;
        }

        // ── `PartialEq<&mut [U]>` Tests ─────────────────────────────────────
        // TODO
        mod partial_eq_slice_mut_ref {
            use super::*;
        }

        // ── `PartialEq<ByteStr>` Tests ──────────────────────────────────────
        // TODO
        mod partial_eq_byte_str {
            use super::*;
        }

        // ── `PartialEq<ByteString>` Tests ───────────────────────────────────
        // TODO
        mod partial_eq_byte_string {
            use super::*;
        }

        // ── `PartialEq<Vec<U, A>>` Tests ────────────────────────────────────
        // TODO
        mod partial_eq_vec {
            use super::*;
        }

        // ── `PartialEq<Vec<u8>>` Tests ──────────────────────────────────────
        // TODO
        mod partial_eq_vec_u8 {
            use super::*;
        }

        // ── `PartialEq<[U; N]>` Tests ───────────────────────────────────────
        // TODO
        mod partial_eq_array {
            use super::*;
        }

        // ── `PartialEq<[U]>` Tests ──────────────────────────────────────────
        // TODO
        mod partial_eq_slice {
            use super::*;
        }

        // ── `PartialOrd<Vec<T, A2>>` Tests ──────────────────────────────────
        // TODO
        mod partial_ord {
            use super::*;
        }

        // ── `TryFrom<Vec<T, A>>` Tests ──────────────────────────────────────
        // TODO
        mod try_from_vec_alloc {
            use super::*;
        }

        // ── `TryFrom<Vec<T>>` Tests ─────────────────────────────────────────
        // TODO
        mod try_from_vec {
            use super::*;
        }

        // ── `TryFrom<Vec<u8>>` Tests ────────────────────────────────────────
        // TODO
        mod try_from_vec_u8 {
            use super::*;
        }

        // ── `Write` Tests ───────────────────────────────────────────────────
        // TODO
        mod write {
            use super::*;
        }
    }

    // ── Auto Trait Implementations ──────────────────────────────────────────
    mod auto_trait_implementations {
        use super::*;

        // ── `Freeze` Tests ──────────────────────────────────────────────────
        // TODO
        mod freeze {
            use super::*;
        }

        // ── `RefUnwindSafe` Tests ───────────────────────────────────────────
        // TODO
        mod ref_unwind_safe {
            use super::*;
        }

        // ── `Send` Tests ────────────────────────────────────────────────────
        // TODO
        mod send {
            use super::*;
        }

        // ── `Sync` Tests ────────────────────────────────────────────────────
        // TODO
        mod sync {
            use super::*;
        }

        // ── `Unpin` Tests ───────────────────────────────────────────────────
        // TODO
        mod unpin {
            use super::*;
        }

        // ── `UnsafeUnpin` Tests ─────────────────────────────────────────────
        // TODO
        mod unsafe_unpin {
            use super::*;
        }

        // ── `UnwindSafe` Tests ──────────────────────────────────────────────
        // TODO
        mod unwind_safe {
            use super::*;
        }
    }

    // ── Blanket Implementations ─────────────────────────────────────────────
    mod blanket_trait_implementations {
        use super::*;

        // ── `Any` Tests ─────────────────────────────────────────────────────
        // TODO
        mod any {
            use super::*;
        }

        // ── `Borrow<T>` Tests ───────────────────────────────────────────────
        // TODO
        mod borrow {
            use super::*;
        }

        // ── `BorrowMut<T>` Tests ────────────────────────────────────────────
        // TODO
        mod borrow_mut {
            use super::*;
        }

        // ── `CloneToUninit` Tests ───────────────────────────────────────────
        // TODO
        mod clone_to_uninit {
            use super::*;
        }

        // ── `From<T>` Tests ─────────────────────────────────────────────────
        // TODO
        mod from_t {
            use super::*;
        }

        // ── `Into<U>` Tests ─────────────────────────────────────────────────
        // TODO
        mod into_u {
            use super::*;
        }

        // ── `Receiver` Tests ────────────────────────────────────────────────
        // TODO
        mod receiver {
            use super::*;
        }

        // ── `ToOwned` Tests ─────────────────────────────────────────────────
        // TODO
        mod to_owned {
            use super::*;
        }

        // ── `TryFrom<U>` Tests ──────────────────────────────────────────────
        // TODO
        mod try_from_u {
            use super::*;
        }

        // ── `TryInto<U>` Tests ──────────────────────────────────────────────
        // TODO
        mod try_into_u {
            use super::*;
        }
    }
}
