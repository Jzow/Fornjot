//! Convenience trait to insert objects into their respective stores
//!
//! See [`Insert`].

use crate::{
    objects::{
        Cycle, Face, GlobalEdge, GlobalVertex, HalfEdge, Objects, Shell,
        Sketch, Solid, Surface, SurfaceVertex,
    },
    services::{Service, ServiceObjectsExt},
    storage::Handle,
    validate::Validate,
};

/// Convenience trait to insert objects into their respective stores
pub trait Insert: Sized + Validate {
    /// Insert the object into its respective store
    fn insert(self, objects: &mut Service<Objects>) -> Handle<Self>;
}

macro_rules! impl_insert {
    ($($ty:ty, $store:ident;)*) => {
        $(
            impl Insert for $ty {
                fn insert(self, objects: &mut Service<Objects>) -> Handle<Self>
                {
                    let handle = objects.$store.reserve();
                    objects.insert(handle.clone(), self);
                    handle
                }
            }
        )*
    };
}

impl_insert!(
    Cycle, cycles;
    Face, faces;
    GlobalEdge, global_edges;
    GlobalVertex, global_vertices;
    HalfEdge, half_edges;
    Shell, shells;
    Sketch, sketches;
    Solid, solids;
    SurfaceVertex, surface_vertices;
    Surface, surfaces;
);
