
pub trait MoveLens<S: Copy, M, D> {
    /// Applies a move to the state, producing a new state and its delta.
    fn apply_move(&self, state: S, mv: M) -> (S, D);

    /// Applies a delta directly to a state (e.g. for replay or undo).
    fn apply_delta(&self, state: S, delta: &D) -> S;

    /// Inverts a delta, producing its reverse.
    fn invert(&self, delta: &D) -> D;

    /// Applies the inverse delta to restore the previous state.
    fn undo(&self, state: S, delta: &D) -> S {
        let inv = self.invert(delta);
        self.apply_delta(state, &inv)
    }
}
