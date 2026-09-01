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

        // ── `Vec::allocator()` Tests ────────────────────────────────────────
        // TODO
        mod allocator {
            use super::*;
        }


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

        // ── `Vec::as_non_null()` Tests ──────────────────────────────────────
        // TODO
        mod as_non_null {
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

        // ── `Vec::const_make_global()` Tests ────────────────────────────────
        // TODO
        mod const_make_global {
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

        // ── `Vec::into_array()` Tests ───────────────────────────────────────
        // TODO
        mod into_array {
            use super::*;
        }

        // ── `Vec::into_boxed_slice()` Tests ─────────────────────────────────
        // TODO
        mod into_boxed_slice {
            use super::*;
        }

        // ── `Vec::into_chunks()` Tests ──────────────────────────────────────
        // TODO
        mod into_chunks {
            use super::*;
        }

        // ── `Vec::into_flattened()` Tests ───────────────────────────────────
        // TODO
        mod into_flattened {
            use super::*;
        }

        // ── `Vec::into_parts()` Tests ───────────────────────────────────────
        // TODO
        mod into_parts {
            use super::*;
        }

        // ── `Vec::into_parts_with_alloc()` Tests ────────────────────────────
        // TODO
        mod into_parts_with_alloc {
            use super::*;
        }

        // ── `Vec::into_raw_parts()` Tests ───────────────────────────────────
        // TODO
        mod into_raw_parts {
            use super::*;
        }

        // ── `Vec::into_raw_parts_with_alloc()` Tests ────────────────────────
        // TODO
        mod into_raw_parts_with_alloc {
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

        // ── `Vec::peek_mut()` Tests ─────────────────────────────────────────
        // TODO
        mod peek_mut {
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

        // ── `Vec::push_with_capacity()` Tests ───────────────────────────────
        // TODO
        mod push_with_capacity {
            use super::*;
        }

        // ── `Vec::recycle()` Tests ──────────────────────────────────────────
        // TODO
        mod recycle {
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

        // ── `Vec::split_at_spare_mut()` Tests ───────────────────────────────
        // TODO
        mod split_at_spare_mut {
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

        // ── `Vec::try_remove()` Tests ───────────────────────────────────────
        // TODO
        mod try_remove {
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

        // ── `Vec::try_shrink_to()` Tests ────────────────────────────────────
        // TODO
        mod try_shrink_to {
            use super::*;
        }

        // ── `Vec::try_shrink_to_fit()` Tests ────────────────────────────────
        // TODO
        mod try_shrink_to_fit {
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
        mod as_mut_vec {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `AsMut<Vec<T, A>>::as_mut()` Tests ──────────────────────
                // TODO
                mod as_mut {
                    use super::*;

                    // Signature:
                    // fn as_mut(&mut self) -> &mut Vec<T, A>

                }
            }
        }

        // ── `AsMut<[T]>` Tests ──────────────────────────────────────────────
        mod as_mut_slice {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `AsMut<[T]>::as_mut()` Tests ────────────────────────────
                // TODO
                mod as_mut {
                    use super::*;

                    // Signature:
                    // fn as_mut(&mut self) -> &mut [T]

                }
            }
        }

        // ── `AsRef<Vec<T, A>>` Tests ────────────────────────────────────────
        mod as_ref_vec {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `AsRef<Vec<T, A>>::as_ref()` Tests ──────────────────────
                // TODO
                mod as_ref {
                    use super::*;

                    // Signature:
                    // fn as_ref(&self) -> &Vec<T, A>

                }
            }
        }

        // ── `AsRef<[T]>` Tests ──────────────────────────────────────────────
        mod as_ref_slice {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `AsRef<[T]>::as_ref()` Tests ────────────────────────────
                // TODO
                mod as_ref {
                    use super::*;

                    // Signature:
                    // fn as_ref(&self) -> &[T]

                }
            }
        }

        // ── `Borrow<[T]>` Tests ─────────────────────────────────────────────
        mod borrow_slice {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `Borrow<[T]>::borrow()` Tests ───────────────────────────
                // TODO
                mod borrow {
                    use super::*;

                    // Signature:
                    // fn borrow(&self) -> &[T]

                }
            }
        }

        // ── `BorrowMut<[T]>` Tests ──────────────────────────────────────────
        mod borrow_mut_slice {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `BorrowMut<[T]>::borrow_mut()` Tests ────────────────────
                // TODO
                mod borrow_mut {
                    use super::*;

                    // Signature:
                    // fn borrow_mut(&mut self) -> &mut [T]

                }
            }
        }

        // ── `Clone` Tests ───────────────────────────────────────────────────
        mod clone {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `Clone::clone()` Tests ──────────────────────────────────
                // TODO
                mod clone {
                    use super::*;

                    // Signature:
                    // fn clone(&self) -> Vec<T, A>
                }

                // ── `Clone::clone_from()` Tests ─────────────────────────────
                // TODO
                mod clone_from {
                    use super::*;

                    // Signature:
                    // fn clone_from(&mut self, source: &Vec<T, A>)

                }
            }
        }

        // ── `Debug` Tests ───────────────────────────────────────────────────
        mod debug {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `Debug::fmt()` Tests ────────────────────────────────────
                // TODO
                mod fmt {
                    use super::*;

                    // Signature:
                    // fn fmt(
                    //      &self,
                    //      f: &mut Formatter<'_>
                    // ) -> Result<(), Error>

                }
            }
        }

        // ── `Default` Tests ─────────────────────────────────────────────────
        mod default {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `Default::default()` Tests ──────────────────────────────
                // TODO
                mod default {
                    use super::*;

                    // Signature:
                    // fn default() -> Vec<T>

                }
            }
        }

        // ── `Deref` Tests ───────────────────────────────────────────────────
        mod deref {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `Deref::deref()` Tests ──────────────────────────────────
                // TODO
                mod deref {
                    use super::*;

                    // Signature:
                    // fn deref(&self) -> &[T]

                }
            }
        }

        // ── `DerefMut` Tests ────────────────────────────────────────────────
        mod deref_mut {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `DerefMut::deref_mut()` Tests ───────────────────────────
                // TODO
                mod deref_mut {
                    use super::*;

                    // Signature:
                    // fn deref_mut(&mut self) -> &mut [T]

                }
            }
        }

        // ── `DerefPure` Tests ───────────────────────────────────────────────
        // TODO
        mod deref_pure {
            use super::*;
        }

        // ── `Drop` Tests ────────────────────────────────────────────────────
        mod drop {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `Drop::drop()` Tests ────────────────────────────────────
                // TODO
                mod drop {
                    use super::*;

                    // Signature:
                    // fn drop(&mut self)

                }
            }
        }

        // ── `Eq` Tests ──────────────────────────────────────────────────────
        // TODO
        mod eq {
            use super::*;
        }

        // ── `Extend<&'a T>` Tests ───────────────────────────────────────────
        mod extend_ref {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `Extend<&'a T>::extend()` Tests ─────────────────────────
                // TODO
                mod extend {
                    use super::*;

                    // Signature:
                    // fn extend<I: IntoIterator<Item = &'a T>>(
                    //      &mut self, iter: I
                    // )

                }
            }
        }

        // ── `Extend<T>` Tests ───────────────────────────────────────────────
        mod extend_val {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `Extend<T>::extend()` Tests ─────────────────────────────
                // TODO
                mod extend {
                    use super::*;

                    // Signature:
                    // fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I)

                }
            }
        }

        // ── `From<&'a Vec<T>>` Tests ────────────────────────────────────────
        mod from_vec_ref {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `From<&'a Vec<T>>::from()` Tests ────────────────────────
                // TODO
                mod from {
                    use super::*;

                    // Signature:
                    // fn from(v: &'a Vec<T>) -> Vec<T>

                }
            }
        }

        // ── `From<&[T; N]>` Tests ───────────────────────────────────────────
        mod from_array_ref {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `From<&[T; N]>::from()` Tests ───────────────────────────
                // TODO
                mod from {
                    use super::*;

                    // Signature:
                    // fn from(s: &[T; N]) -> Vec<T>

                }
            }
        }

        // ── `From<&[T]>` Tests ──────────────────────────────────────────────
        mod from_slice_ref {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `From<&[T]>::from()` Tests ──────────────────────────────
                // TODO
                mod from {
                    use super::*;

                    // Signature:
                    // fn from(s: &[T]) -> Vec<T>

                }
            }
        }

        // ── `From<&mut [T; N]>` Tests ───────────────────────────────────────
        mod from_array_mut_ref {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `From<&mut [T; N]>::from()` Tests ───────────────────────
                // TODO
                mod from {
                    use super::*;

                    // Signature:
                    // fn from(s: &mut [T; N]) -> Vec<T>

                }
            }
        }

        // ── `From<&mut [T]>` Tests ──────────────────────────────────────────
        mod from_slice_mut_ref {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `From<&mut [T]>::from()` Tests ──────────────────────────
                // TODO
                mod from {
                    use super::*;

                    // Signature:
                    // fn from(s: &mut [T]) -> Vec<T>

                }
            }
        }

        // ── `From<&str>` Tests ──────────────────────────────────────────────
        mod from_str {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `From<&str>::from()` Tests ──────────────────────────────
                // TODO
                mod from {
                    use super::*;

                    // Signature:
                    // fn from(s: &str) -> Vec<u8>

                }
            }
        }

        // ── `From<BinaryHeap<T, A>>` Tests ──────────────────────────────────
        mod from_binary_heap {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `From<BinaryHeap<T, A>>::from()` Tests ──────────────────
                // TODO
                mod from {
                    use super::*;

                    // Signature:
                    // fn from(heap: BinaryHeap<T, A>) -> Vec<T, A>

                }
            }
        }

        // ── `From<Box<[T], A>>` Tests ───────────────────────────────────────
        mod from_boxed_slice {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `From<Box<[T], A>>::from()` Tests ───────────────────────
                // TODO
                mod from {
                    use super::*;

                    // Signature:
                    // fn from(s: Box<[T], A>) -> Vec<T, A>

                }
            }
        }

        // ── `From<ByteString>` Tests ────────────────────────────────────────
        mod from_byte_string {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `From<ByteString>::from()` Tests ────────────────────────
                // TODO
                mod from {
                    use super::*;

                    // Signature:
                    // fn from(s: ByteString) -> Vec<u8>

                }
            }
        }

        // ── `From<CString>` Tests ───────────────────────────────────────────
        mod from_c_string {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `From<CString>::from()` Tests ───────────────────────────
                // TODO
                mod from {
                    use super::*;

                    // Signature:
                    // fn from(s: CString) -> Vec<u8>

                }
            }
        }

        // ── `From<Cow<'a, [T]>>` Tests ──────────────────────────────────────
        mod from_cow_slice {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `From<Cow<'a, [T]>>::from()` Tests ──────────────────────
                // TODO
                mod from {
                    use super::*;

                    // Signature:
                    // fn from(s: Cow<'a, [T]>) -> Vec<T>

                }
            }
        }

        // ── `From<String>` Tests ────────────────────────────────────────────
        mod from_string {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `From<String>::from()` Tests ────────────────────────────
                // TODO
                mod from {
                    use super::*;

                    // Signature:
                    // fn from(s: String) -> Vec<u8>

                }
            }
        }

        // ── `From<Vec<NonZero<u8>>>` Tests ──────────────────────────────────
        mod from_vec_nonzero_u8 {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `From<Vec<NonZero<u8>>>::from()` Tests ──────────────────
                // TODO
                mod from {
                    use super::*;

                    // Signature:
                    // fn from(v: Vec<NonZero<u8>>) -> Vec<u8>

                }
            }
        }

        // ── `From<Vec<T, A>>` Tests ─────────────────────────────────────────
        mod from_vec_alloc {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `From<Vec<T, A>>::from()` Tests ─────────────────────────
                // TODO
                mod from {
                    use super::*;

                    // Signature:
                    // fn from(v: Vec<T, A>) -> Self

                }
            }
        }

        // ── `From<Vec<T>>` Tests ────────────────────────────────────────────
        mod from_vec {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `From<Vec<T>>::from()` Tests ────────────────────────────
                // TODO
                mod from {
                    use super::*;

                    // Signature:
                    // fn from(v: Vec<T>) -> Self

                }
            }
        }

        // ── `From<VecDeque<T, A>>` Tests ────────────────────────────────────
        mod from_vec_deque {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `From<VecDeque<T, A>>::from()` Tests ────────────────────
                // TODO
                mod from {
                    use super::*;

                    // Signature:
                    // fn from(v: VecDeque<T, A>) -> Vec<T, A>

                }
            }
        }

        // ── `From<[T; N]>` Tests ────────────────────────────────────────────
        mod from_array {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `From<[T; N]>::from()` Tests ────────────────────────────
                // TODO
                mod from {
                    use super::*;

                    // Signature:
                    // fn from(s: [T; N]) -> Vec<T>

                }
            }
        }

        // ── `FromIterator<T>` Tests ─────────────────────────────────────────
        mod from_iterator {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `FromIterator<T>::from_iter()` Tests ────────────────────
                // TODO
                mod from_iter {
                    use super::*;

                    // Signature:
                    // fn from_iter<I: IntoIterator<Item = T>>(
                    //      iter: I
                    // ) -> Vec<T>

                }
            }
        }

        // ── `Hash` Tests ────────────────────────────────────────────────────
        mod hash {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `Hash::hash()` Tests ────────────────────────────────────
                // TODO
                mod hash {
                    use super::*;

                    // Signature:
                    // fn hash<H: Hasher>(&self, state: &mut H)

                }
            }
        }

        // ── `Index<I>` Tests ────────────────────────────────────────────────
        mod index {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `Index<I>::index()` Tests ───────────────────────────────
                // TODO
                mod index {
                    use super::*;

                    // Signature:
                    // fn index(&self, index: I) -> &Self::Output

                }
            }
        }

        // ── `IndexMut<I>` Tests ─────────────────────────────────────────────
        mod index_mut {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `IndexMut<I>::index_mut()` Tests ────────────────────────
                // TODO
                mod index_mut {
                    use super::*;

                    // Signature:
                    // fn index_mut(&mut self, index: I) -> &mut Self::Output

                }
            }
        }

        // ── `IntoIterator` (Value) Tests ────────────────────────────────────
        mod into_iterator_val {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `IntoIterator::into_iter()` (Value) Tests ───────────────
                // TODO
                mod into_iter {
                    use super::*;

                    // Signature:
                    // fn into_iter(self) -> IntoIter<T, A>

                }
            }
        }

        // ── `IntoIterator` (Ref) Tests ──────────────────────────────────────
        mod into_iterator_ref {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `IntoIterator::into_iter()` (Ref) Tests ─────────────────
                // TODO
                mod into_iter {
                    use super::*;

                    // Signature:
                    // fn into_iter(self) -> Iter<'a, T>

                }
            }
        }

        // ── `IntoIterator` (Mut Ref) Tests ──────────────────────────────────
        mod into_iterator_mut_ref {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `IntoIterator::into_iter()` (Mut Ref) Tests ─────────────
                // TODO
                mod into_iter {
                    use super::*;

                    // Signature:
                    // fn into_iter(self) -> IterMut<'a, T>

                }
            }
        }

        // ── `Ord` Tests ─────────────────────────────────────────────────────
        mod ord {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `Ord::cmp()` Tests ──────────────────────────────────────
                // TODO
                mod cmp {
                    use super::*;

                    // Signature:
                    // fn cmp(&self, other: &Self) -> Ordering

                }
            }
        }

        // ── `PartialEq<&[U; N]>` Tests ──────────────────────────────────────
        mod partial_eq_array_ref {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `PartialEq<&[U; N]>::eq()` Tests ────────────────────────
                // TODO
                mod eq {
                    use super::*;

                    // Signature:
                    // fn eq(&self, other: &&[U; N]) -> bool

                }
            }
        }

        // ── `PartialEq<&[U]>` Tests ─────────────────────────────────────────
        mod partial_eq_slice_ref {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `PartialEq<&[U]>::eq()` Tests ───────────────────────────
                // TODO
                mod eq {
                    use super::*;

                    // Signature:
                    // fn eq(&self, other: &&[U]) -> bool

                }
            }
        }

        // ── `PartialEq<&mut [U]>` Tests ─────────────────────────────────────
        mod partial_eq_slice_mut_ref {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `PartialEq<&mut [U]>::eq()` Tests ───────────────────────
                // TODO
                mod eq {
                    use super::*;

                    // Signature:
                    // fn eq(&self, other: &&mut [U]) -> bool

                }
            }
        }

        // ── `PartialEq<ByteStr>` Tests ──────────────────────────────────────
        mod partial_eq_byte_str {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `PartialEq<ByteStr>::eq()` Tests ────────────────────────
                // TODO
                mod eq {
                    use super::*;

                    // Signature:
                    // fn eq(&self, other: &ByteStr) -> bool

                }
            }
        }

        // ── `PartialEq<ByteString>` Tests ───────────────────────────────────
        mod partial_eq_byte_string {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `PartialEq<ByteString>::eq()` Tests ─────────────────────
                // TODO
                mod eq {
                    use super::*;

                    // Signature:
                    // fn eq(&self, other: &ByteString) -> bool

                }
            }
        }

        // ── `PartialEq<Vec<U, A>>` Tests ────────────────────────────────────
        mod partial_eq_vec {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `PartialEq<Vec<U, A>>::eq()` Tests ──────────────────────
                // TODO
                mod eq {
                    use super::*;

                    // Signature:
                    // fn eq(&self, other: &Vec<U, A>) -> bool

                }
            }
        }

        // ── `PartialEq<Vec<u8>>` Tests ──────────────────────────────────────
        mod partial_eq_vec_u8 {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `PartialEq<Vec<u8>>::eq()` Tests ────────────────────────
                // TODO
                mod eq {
                    use super::*;

                    // Signature:
                    // fn eq(&self, other: &Vec<u8>) -> bool

                }
            }
        }

        // ── `PartialEq<[U; N]>` Tests ───────────────────────────────────────
        mod partial_eq_array {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `PartialEq<[U; N]>::eq()` Tests ─────────────────────────
                // TODO
                mod eq {
                    use super::*;

                    // Signature:
                    // fn eq(&self, other: &[U; N]) -> bool

                }
            }
        }

        // ── `PartialEq<[U]>` Tests ──────────────────────────────────────────
        mod partial_eq_slice {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `PartialEq<[U]>::eq()` Tests ────────────────────────────
                // TODO
                mod eq {
                    use super::*;

                    // Signature:
                    // fn eq(&self, other: &[U]) -> bool

                }
            }
        }

        // ── `PartialOrd<Vec<T, A2>>` Tests ──────────────────────────────────
        mod partial_ord {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `PartialOrd<Vec<T, A2>>::partial_cmp()` Tests ───────────
                // TODO
                mod partial_cmp {
                    use super::*;

                    // Signature:
                    // fn partial_cmp(
                    //      &self, other: &Vec<T, A2>
                    // ) -> Option<Ordering>

                }
            }
        }

        // ── `TryFrom<Vec<T, A>>` Tests ──────────────────────────────────────
        mod try_from_vec_alloc {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `TryFrom<Vec<T, A>>::try_from()` Tests ──────────────────
                // TODO
                mod try_from {
                    use super::*;

                    // Signature:
                    // fn try_from(vec: Vec<T, A>) -> Result<Self, Self::Error>

                }
            }
        }

        // ── `TryFrom<Vec<T>>` Tests ─────────────────────────────────────────
        mod try_from_vec {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `TryFrom<Vec<T>>::try_from()` Tests ─────────────────────
                // TODO
                mod try_from {
                    use super::*;

                    // Signature:
                    // fn try_from(vec: Vec<T>) -> Result<Self, Self::Error>

                }
            }
        }

        // ── `TryFrom<Vec<u8>>` Tests ────────────────────────────────────────
        mod try_from_vec_u8 {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `TryFrom<Vec<u8>>::try_from()` Tests ────────────────────
                // TODO
                mod try_from {
                    use super::*;

                    // Signature:
                    // fn try_from(vec: Vec<u8>) -> Result<Self, Self::Error>

                }
            }
        }

        // ── `Write` Tests ───────────────────────────────────────────────────
        mod write {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `Write::write()` Tests ──────────────────────────────────
                // TODO
                mod write {
                    use super::*;

                    // Signature:
                    // fn write(&mut self, buf: &[u8]) -> Result<usize>

                }

                // ── `Write::flush()` Tests ──────────────────────────────────
                // TODO
                mod flush {
                    use super::*;

                    // Signature:
                    // fn flush(&mut self) -> Result<()>

                }

                // ── `Write::write_vectored()` Tests ─────────────────────────
                // TODO
                mod write_vectored {
                    use super::*;

                    // Signature:
                    // fn write_vectored(
                    //      &mut self, bufs: &[IoSlice<'_>]
                    // ) -> Result<usize>

                }

                // ── `Write::write_all()` Tests ──────────────────────────────
                // TODO
                mod write_all {
                    use super::*;

                    // Signature:
                    // fn write_all(&mut self, buf: &[u8]) -> Result<()>

                }

                // ── `Write::write_fmt()` Tests ──────────────────────────────
                // TODO
                mod write_fmt {
                    use super::*;

                    // Signature:
                    // fn write_fmt(
                    //      &mut self, fmt: Arguments<'_>
                    // ) -> Result<()>

                }
            }
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
