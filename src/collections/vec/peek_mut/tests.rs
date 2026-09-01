#[cfg(test)]
mod tests {
    // ── Core Aliases ────────────────────────────────────────────────────────
    
    // ── Standard Library Aliases ────────────────────────────────────────────

    // ── Crate Aliases ───────────────────────────────────────────────────────
    use std::vec::PeekMut; // Test with the Standard Library's `PeekMut` first
    //use crate::collections::vec::PeekMut;

    // ── Function Tests ──────────────────────────────────────────────────────
    mod functions {
        use super::*;

        // ── `peek_mut::pop()` Tests ─────────────────────────────────────────
        // TODO
        mod pop {
            use super::*;

            // Signature:
            // pub fn pop(this: PeekMut<'a, T, A>) -> T
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

         // ── `Deref` Tests ──────────────────────────────────────────────────
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
                    // fn fmt(
                    //      &self, 
                    //      f: &mut Formatter<'_>
                    // ) -> Result<(), Error>
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
                    // fn deref_mut(
                    //      &mut self
                    // ) -> &mut <PeekMut<'a, T, A> as Deref>::Target
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
                // TODO
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
                // TODO
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
                // TODO
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
                // TODO
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
                // TODO
                mod into {
                    use super::*;

                    // Signature:
                    // fn into(self) -> U
                }
            }
        }

        // ── `Receiver` Tests ────────────────────────────────────────────────
        mod receiver {
            use super::*;
        }

        // ── `TryFrom<U>` Tests ──────────────────────────────────────────────
        mod try_from {
            use super::*;

            // ── Method Tests ────────────────────────────────────────────────
            mod methods {
                use super::*;

                // ── `TryFrom::try_from()` Tests ─────────────────────────────
                // TODO
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
                // TODO
                mod try_into {
                    use super::*;

                    // Signature:
                    // fn try_into(self) -> Result<U, Self::Error>
                }
            }
        }
    }
}
