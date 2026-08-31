#[cfg(test)]
mod tests {
    // ── Core Aliases ────────────────────────────────────────────────────────

    // ── Standard Library Aliases ────────────────────────────────────────────

    // ── Crate Aliases ───────────────────────────────────────────────────────
    use std::vec::ExtractIf; // Test with the Standard Library's `ExtractIf` first
    //use crate::collections::vec::ExtractIf;

    // ── Method Tests ────────────────────────────────────────────────────────
    mod methods {
        use super::*;

        // ── `ExtractIf::allocator()` Tests ──────────────────────────────────
        mod allocator {
            use super::*;

            // Signature:
            // pub fn allocator(&self) -> &A
        }
    }

    // ── Trait Implementations ───────────────────────────────────────────────
    mod trait_implementations {
        use super::*;

         // ── `Debug` Tests ───────────────────────────────────────────────────
        mod debug {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `Debug::fmt()` Tests ────────────────────────────────────
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

        // ── `Drop` Tests ────────────────────────────────────────────────────
        mod drop {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `Drop::drop()` Tests ────────────────────────────────────
                mod drop {
                    use super::*;

                    // Signature:
                    // fn drop(&mut self)
                }

                // ── `Drop::pin_drop()` Tests ────────────────────────────────
                mod pin_mod {
                    use super::*;

                    // Signature:
                    // fn pin_drop(self: Pin<&mut Self>)
                }
            }
        }

        // ── `Iterator` Tests ────────────────────────────────────────────────
        mod iterator {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `Iterator::next()` Tests ────────────────────────────────
                mod next {
                    use super::*;

                    // Signature:
                    // fn next(&mut self) -> Option<Self::Item>
                }

                // ── `Iterator::size_hint()` Tests ───────────────────────────
                mod size_hint {
                    use super::*;

                    // Signature:
                    // fn size_hint(&self) -> (usize, Option<usize>)
                }

                // ── `Iterator::next_chunk()` Tests ──────────────────────────
                mod next_chunk {
                    use super::*;

                    // Signature:
                    // fn next_chunk<const N: usize>(
                    //     &mut self
                    //  ) -> Result<[Self::Item; N], IntoIter<Self::Item, N>>

                }

                // ── `Iterator::count()` Tests ───────────────────────────────
                mod count {
                    use super::*;

                    // Signature:
                    // fn count(self) -> usize
                }

                // ── `Iterator::last()` Tests ────────────────────────────────
                mod last {
                    use super::*;

                    // Signature:
                    // fn last(self) -> Option<Self::Item>
                }

                // ── `Iterator::advance_by()` Tests ──────────────────────────
                mod advance_by {
                    use super::*;

                    // Signature:
                    // fn advance_by(
                    //     &mut self, 
                    //     n: usize
                    // ) -> Result<(), NonZero<usize>>

                }

                // ── `Iterator::nth()` Tests ─────────────────────────────────
                mod nth {
                    use super::*;

                    // Signature:
                    // fn nth(&mut self, n: usize) -> Option<Self::Item>

                }

                // ── `Iterator::step_by()` Tests ──────────────────────────────
                mod step_by {
                    use super::*;

                    // Signature:
                    // fn step_by(self, step: usize) -> StepBy<Self>

                }

                // ── `Iterator::chain()` Tests ───────────────────────────────
                mod chain {
                    use super::*;

                    // Signature:
                    // fn chain<U>(self, other: U) -> Chain<Self, U::IntoIter>

                }

                // ── `Iterator::zip()` Tests ─────────────────────────────────
                mod zip {
                    use super::*;

                    // Signature:
                    // fn zip<U>(self, other: U) -> Zip<Self, U::IntoIter>

                }

                // ── `Iterator::intersperse()` Tests ─────────────────────────
                mod intersperse {
                    use super::*;

                    // Signature:
                    // fn intersperse(
                    //     self, 
                    //     separator: Self::Item
                    // ) -> Intersperse<Self>

                }

                // ── `Iterator::intersperse_with()` Tests ────────────────────
                mod intersperse_with {
                    use super::*;

                    // Signature:
                    // fn intersperse_with<G>(
                    //     self, 
                    //     separator: G
                    // ) -> IntersperseWith<Self, G>

                }

                // ── `Iterator::map()` Tests ─────────────────────────────────
                mod map {
                    use super::*;

                    // Signature:
                    // fn map<B, F>(self, f: F) -> Map<Self, F>

                }

                // ── `Iterator::enumerate()` Tests ───────────────────────────
                mod enumerate {
                    use super::*;

                    // Signature:
                    // fn enumerate(self) -> Enumerate<Self>

                }

                // ── `Iterator::peekable()` Tests ────────────────────────────
                mod peekable {
                    use super::*;

                    // Signature:
                    // fn peekable(self) -> Peekable<Self>

                }

                // ── `Iterator::skip_while()` Tests ──────────────────────────
                mod skip_while {
                    use super::*;

                    // Signature:
                    // fn skip_while<P>(
                    //     self, 
                    //     predicate: P
                    // ) -> SkipWhile<Self, P>

                }

                // ── `Iterator::map_while()` Tests ───────────────────────────
                mod map_while {
                    use super::*;

                    // Signature:
                    // fn map_while<B, P>(
                    //     self, 
                    //     predicate: P
                    // ) -> MapWhile<Self, P>

                }

                // ── `Iterator::skip()` Tests ────────────────────────────────
                mod skip {
                    use super::*;

                    // Signature:
                    // fn skip(self, n: usize) -> Skip<Self>

                }

                // ── `Iterator::take()` Tests ────────────────────────────────
                mod take {
                    use super::*;

                    // Signature:
                    // fn take(self, n: usize) -> Take<Self>

                }

                // ── `Iterator::scan()` Tests ────────────────────────────────
                mod scan {
                    use super::*;

                    // Signature:
                    // fn scan<St, B, F>(
                    //     self, 
                    //     initial_state: St,
                    //     f: F
                    // ) -> Scan<Self, St, F>

                }

                // ── `Iterator::flat_map()` Tests ────────────────────────────
                mod flat_map {
                    use super::*;

                    // Signature:
                    // fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F>

                }

                // ── `Iterator::flatten()` Tests ─────────────────────────────
                mod flatten {
                    use super::*;

                    // Signature:
                    // fn flatten(self) -> Flatten<Self>

                }

                // ── `Iterator::map_windows()` Tests ─────────────────────────
                mod map_windows {
                    use super::*;

                    // Signature:
                    // fn map_windows<F, R, const N: usize>(
                    //     self, 
                    //     f: F
                    // ) -> MapWindows<Self, F, N>

                }

                // ── `Iterator::fuse()` Tests ────────────────────────────────
                mod fuse {
                    use super::*;

                    // Signature:
                    // fn fuse(self) -> Fuse<Self>

                }

                // ── `Iterator::inspect()` Tests ─────────────────────────────
                mod inspect {
                    use super::*;

                    // Signature:
                    // fn inspect<F>(self, f: F) -> Inspect<Self, F>

                }

                // ── `Iterator::by_ref()` Tests ──────────────────────────────
                mod by_ref {
                    use super::*;

                    // Signature:
                    // fn by_ref(&mut self) -> &mut Self

                }

                // ── `Iterator::collect()` Tests ─────────────────────────────
                mod collect {
                    use super::*;

                    // Signature:
                    // fn collect<B>(self) -> B

                }

                // ── `Iterator::try_collect()` Tests ─────────────────────────
                mod try_collect {
                    use super::*;

                    // Signature:
                    // fn try_collect<B>(
                    //     &mut self
                    // ) -> ChangeOutputType<Self::Item, B>

                }

                // ── `Iterator::collect_into()` Tests ────────────────────────
                mod collect_into {
                    use super::*;

                    // Signature:
                    // fn collect_into<E>(self, collection: &mut E) -> &mut E

                }

                // ── `Iterator::partition()` Tests ───────────────────────────
                mod partition {
                    use super::*;

                    // Signature:
                    // fn partition<B, F>(self, f: F) -> (B, B)

                }

                // ── `Iterator::partition_in_place()` Tests ──────────────────
                mod partition_in_place {
                    use super::*;

                    // Signature:
                    // fn partition_in_place<'a, P>(
                    //     self, 
                    //     predicate: P
                    // ) -> usize

                }

                // ── `Iterator::is_partitioned()` Tests ──────────────────────
                mod is_partitioned {
                    use super::*;

                    // Signature:
                    // fn is_partitioned<P>(self, predicate: P) -> bool

                }

                // ── `Iterator::try_fold()` Tests ────────────────────────────
                mod try_fold {
                    use super::*;

                    // Signature:
                    // fn try_fold<B, F, R>(&mut self, init: B, f: F) -> R

                }

                // ── `Iterator::try_for_each()` Tests ────────────────────────
                mod try_for_each {
                    use super::*;

                    // Signature:
                    // fn try_for_each<F, R>(&mut self, f: F) -> R

                }

                // ── `Iterator::fold()` Tests ────────────────────────────────
                mod fold {
                    use super::*;

                    // Signature:
                    // fn fold<B, F>(self, init: B, f: F) -> B

                }

                // ── `Iterator::reduce()` Tests ──────────────────────────────
                mod reduce {
                    use super::*;

                    // Signature:
                    // fn reduce<F>(self, f: F) -> Option<Self::Item>

                }

                // ── `Iterator::try_reduce()` Tests ──────────────────────────
                mod try_reduce {
                    use super::*;

                    // Signature:
                    // fn try_reduce<F, R>(self, f: F) -> R

                }

                // ── `Iterator::all()` Tests ─────────────────────────────────
                mod all {
                    use super::*;

                    // Signature:
                    // fn all<P>(&mut self, predicate: P) -> bool

                }

                // ── `Iterator::any()` Tests ─────────────────────────────────
                mod any {
                    use super::*;

                    // Signature:
                    // fn any<P>(&mut self, predicate: P) -> bool

                }

                // ── `Iterator::find()` Tests ────────────────────────────────
                mod find {
                    use super::*;

                    // Signature:
                    // fn find<P>(
                    //     &mut self, 
                    //     predicate: P
                    // ) -> Option<Self::Item>

                }

                // ── `Iterator::find_map()` Tests ────────────────────────────
                mod find_map {
                    use super::*;

                    // Signature:
                    // fn find_map<B, F>(&mut self, f: F) -> Option<B>

                }

                // ── `Iterator::try_find()` Tests ────────────────────────────
                mod try_find {
                    use super::*;

                    // Signature:
                    // fn try_find<F, R>(
                    //     &mut self,
                    //     f: F
                    // ) -> Result<Option<Self::Item>, R::Error>

                }

                // ── `Iterator::position()` Tests ────────────────────────────
                mod position {
                    use super::*;

                    // Signature:
                    // fn position<P>(&mut self, predicate: P) -> Option<usize>

                }

                // ── `Iterator::rposition()` Tests ───────────────────────────
                mod rposition {
                    use super::*;

                    // Signature:
                    // fn rposition<P>(
                    //     &mut self, 
                    //     predicate: P
                    // ) -> Option<usize>

                }

                // ── `Iterator::max()` Tests ─────────────────────────────────
                mod max {
                    use super::*;

                    // Signature:
                    // fn max(self) -> Option<Self::Item>

                }

                // ── `Iterator::min()` Tests ─────────────────────────────────
                mod min {
                    use super::*;

                    // Signature:
                    // fn min(self) -> Option<Self::Item>

                }

                // ── `Iterator::max_by_key()` Tests ──────────────────────────
                mod max_by_key {
                    use super::*;

                    // Signature:
                    // fn max_by_key<B, F>(self, f: F) -> Option<Self::Item>

                }

                // ── `Iterator::max_by()` Tests ──────────────────────────────
                mod max_by {
                    use super::*;

                    // Signature:
                    // fn max_by<F>(self, compare: F) -> Option<Self::Item>

                }

                // ── `Iterator::min_by_key()` Tests ──────────────────────────
                mod min_by_key {
                    use super::*;

                    // Signature:
                    // fn min_by_key<B, F>(self, f: F) -> Option<Self::Item>

                }

                // ── `Iterator::min_by()` Tests ──────────────────────────────
                mod min_by {
                    use super::*;

                    // Signature:
                    // fn min_by<F>(self, compare: F) -> Option<Self::Item>

                }

                // ── `Iterator::rev()` Tests ─────────────────────────────────
                mod rev {
                    use super::*;

                    // Signature:
                    // fn rev(self) -> Rev<Self>

                }

                // ── `Iterator::unzip()` Tests ───────────────────────────────
                mod unzip {
                    use super::*;

                    // Signature:
                    // fn unzip<A, B, FromA, FromB>(self) -> (FromA, FromB)

                }

                // ── `Iterator::copied()` Tests ──────────────────────────────
                mod copied {
                    use super::*;

                    // Signature:
                    // fn copied<'a, T>(self) -> Copied<Self>

                }

                // ── `Iterator::cloned()` Tests ──────────────────────────────
                mod cloned {
                    use super::*;

                    // Signature:
                    // fn cloned<'a, T>(self) -> Cloned<Self>

                }

                // ── `Iterator::cycle()` Tests ───────────────────────────────
                mod cycle {
                    use super::*;

                    // Signature:
                    // fn cycle(self) -> Cycle<Self>

                }

                // ── `Iterator::array_chunks()` Tests ────────────────────────
                mod array_chunks {
                    use super::*;

                    // Signature:
                    // fn array_chunks<const N: usize>(
                    //     self
                    // ) -> ArrayChunks<Self, N>

                }

                // ── `Iterator::sum()` Tests ─────────────────────────────────
                mod sum {
                    use super::*;

                    // Signature:
                    // fn sum<S>(self) -> S

                }

                // ── `Iterator::product()` Tests ─────────────────────────────
                mod product {
                    use super::*;

                    // Signature:
                    // fn product<P>(self) -> P

                }

                // ── `Iterator::cmp()` Tests ─────────────────────────────────
                mod cmp {
                    use super::*;

                    // Signature:
                    // fn cmp<I>(self, other: I) -> Ordering

                }

                // ── `Iterator::cmp_by()` Tests ──────────────────────────────
                mod cmp_by {
                    use super::*;

                    // Signature:
                    // fn cmp_by<I, F>(self, other: I, cmp: F) -> Ordering

                }

                // ── `Iterator::partial_cmp()` Tests ─────────────────────────
                mod partial_cmp {
                    use super::*;

                    // Signature:
                    // fn partial_cmp<I>(self, other: I) -> Option<Ordering>

                }

                // ── `Iterator::partial_cmp_by()` Tests ──────────────────────
                mod partial_cmp_by {
                    use super::*;

                    // Signature:
                    // fn partial_cmp_by<I, F>(
                    //     self,
                    //     other: I,
                    //     partial_cmp: F
                    // ) -> Option<Ordering>

                }

                // ── `Iterator::eq()` Tests ──────────────────────────────────
                mod eq {
                    use super::*;

                    // Signature:
                    // fn eq<I>(self, other: I) -> bool

                }

                // ── `Iterator::eq_by()` Tests ───────────────────────────────
                mod eq_by {
                    use super::*;

                    // Signature:
                    // fn eq_by<I, F>(self, other: I, eq: F) -> bool

                }

                // ── `Iterator::ne()` Tests ──────────────────────────────────
                mod ne {
                    use super::*;

                    // Signature:
                    // fn ne<I>(self, other: I) -> bool

                }

                // ── `Iterator::lt()` Tests ──────────────────────────────────
                mod lt {
                    use super::*;

                    // Signature:
                    // fn lt<I>(self, other: I) -> bool

                }

                // ── `Iterator::le()` Tests ──────────────────────────────────
                mod le {
                    use super::*;

                    // Signature:
                    // fn le<I>(self, other: I) -> bool

                }

                // ── `Iterator::gt()` Tests ──────────────────────────────────
                mod gt {
                    use super::*;

                    // Signature:
                    // fn gt<I>(self, other: I) -> bool

                }

                // ── `Iterator::ge()` Tests ──────────────────────────────────
                mod ge {
                    use super::*;

                    // Signature:
                    // fn ge<I>(self, other: I) -> bool

                }

                // ── `Iterator::is_sorted()` Tests ───────────────────────────
                mod is_sorted {
                    use super::*;

                    // Signature:
                    // fn is_sorted(self) -> bool

                }

                // ── `Iterator::is_sorted_by()` Tests ────────────────────────
                mod is_sorted_by {
                    use super::*;

                    // Signature:
                    // fn is_sorted_by<F>(self, compare: F) -> bool

                }

                // ── `Iterator::is_sorted_by_key()` Tests ────────────────────
                mod is_sorted_by_key {
                    use super::*;

                    // Signature:
                    // fn is_sorted_by_key<F, K>(self, f: F) -> bool

                }
            }
        }
    }

    // ── Auto Trait Implementations ──────────────────────────────────────────
    mod auto_trait_implementations {
        use super::*;

        // ── UnwindSafe ──────────────────────────────────────────────────────
        mod unwind_safe {
            use super::*;
        }

        // ── Freeze ──────────────────────────────────────────────────────────
        mod freeze {
            use super::*;
        }

        // ── RefUnwindSafe ───────────────────────────────────────────────────
        mod ref_unwind_safe {
            use super::*;
        }

        // ── Send ────────────────────────────────────────────────────────────
        mod send {
            use super::*;
        }

        // ── Sync ────────────────────────────────────────────────────────────
        mod sync {
            use super::*;
        }

        // ── Unpin ───────────────────────────────────────────────────────────
        mod unpin {
            use super::*;
        }

        // ── UnsafeUnpin ─────────────────────────────────────────────────────
        mod unsafe_unpin {
            use super::*;
        }
    }

    // ── Blanket Implementations ─────────────────────────────────────────────
    mod blanket_trait_implementations {
        use super::*;

        // ── `Any` Tests ─────────────────────────────────────────────────────
        mod any {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `Any::type_id()` Tests ──────────────────────────────────
                mod type_id {
                    use super::*;

                    // Signature:
                    // fn type_id(&self) -> TypeId

                }
            }
        }

        // ── `Borrow<T>` Tests ───────────────────────────────────────────────
        mod borrow {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `Borrow::borrow()` Tests ────────────────────────────────
                mod borrow {
                    use super::*;

                    // Signature:
                    // fn borrow(&self) -> &Borrowed
                }
            }
        }

        // ── `BorrowMut<T>` Tests ────────────────────────────────────────────
        mod borrow_mut {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `BorrowMut::borrow_mut()` Tests ─────────────────────────
                mod borrow_mut {
                    use super::*;

                    // Signature:
                    // fn borrow_mut(&mut self) -> &mut Borrowed
                }
            }
        }

        // ── `From<T>` Tests ─────────────────────────────────────────────────
        mod from {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `From::from()` Tests ────────────────────────────────────
                mod from {
                    use super::*;

                    // Signature:
                    // fn from(t: T) -> T

                }
            }
        }

        // ── `Into<U>` Tests ─────────────────────────────────────────────────
        mod into {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `Into::into()` Tests ────────────────────────────────────
                mod into {
                    use super::*;

                    // Signature:
                    // fn into(self) -> U
                }
            }
        }

        // ── `IntoIterator` Tests ────────────────────────────────────────────
        mod into_iterator {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `IntoIterator::into_iter()` Tests ───────────────────────
                mod into_iter {
                    use super::*;

                    // Signature:
                    // fn into_iter(self) -> I
                }
            }
        }

        // ── `TryFrom<U>` Tests ──────────────────────────────────────────────
        mod try_from {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `TryFrom::try_from()` Tests ─────────────────────────────
                mod try_from {
                    use super::*;

                    // Signature:
                    // fn try_from(value: U) -> Result<T, Self::Error>
                }
            }
        }

        // ── `TryInto<U>` Tests ──────────────────────────────────────────────
        mod try_into {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `TryInto::try_into()` Tests ─────────────────────────────
                mod try_into {
                    use super::*;

                    // Signature:
                    // fn try_into(self) -> Result<U, Self::Error>
                }
            }
        }
    }
}