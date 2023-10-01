use crate::component::Component;
use crate::entity_id::EntityId;
use crate::seal::Sealed;
use crate::sparse_set::SparseSet;
use crate::track::{InsertionAndModification, InsertionConst, ModificationConst};
use crate::tracking::{
    InsertionTracking, ModificationTracking, Track, Tracking, TrackingTimestamp,
};

impl Sealed for Track<InsertionAndModification> {}

impl Tracking for Track<InsertionAndModification> {
    fn as_const() -> u32 {
        InsertionConst + ModificationConst
    }

    #[inline]
    fn is_inserted<T: Component>(
        sparse_set: &SparseSet<T>,
        entity: EntityId,
        last: TrackingTimestamp,
        current: TrackingTimestamp,
    ) -> bool {
        if let Some(dense) = sparse_set.index_of(entity) {
            sparse_set.insertion_data[dense].is_within(last, current)
        } else {
            false
        }
    }

    fn is_modified<T: Component>(
        sparse_set: &SparseSet<T>,
        entity: EntityId,
        last: TrackingTimestamp,
        current: TrackingTimestamp,
    ) -> bool {
        if let Some(dense) = sparse_set.index_of(entity) {
            sparse_set.modification_data[dense].is_within(last, current)
        } else {
            false
        }
    }
}

impl InsertionTracking for Track<InsertionAndModification> {}
impl ModificationTracking for Track<InsertionAndModification> {}
