use fj_math::Transform;

use crate::{
    objects::{GlobalVertex, Objects, SurfaceVertex},
    services::Service,
};

use super::{TransformCache, TransformObject};

impl TransformObject for SurfaceVertex {
    fn transform_with_cache(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
        cache: &mut TransformCache,
    ) -> Self {
        // Don't need to transform position, as that is defined in surface
        // coordinates and thus transforming the surface takes care of it.
        let position = self.position();

        let global_form = self
            .global_form()
            .clone()
            .transform_with_cache(transform, objects, cache);

        Self::new(position, global_form)
    }
}

impl TransformObject for GlobalVertex {
    fn transform_with_cache(
        self,
        transform: &Transform,
        _: &mut Service<Objects>,
        _: &mut TransformCache,
    ) -> Self {
        let position = transform.transform_point(&self.position());
        Self::new(position)
    }
}
