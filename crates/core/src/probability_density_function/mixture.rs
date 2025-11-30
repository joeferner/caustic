use std::sync::Arc;

use crate::{ProbabilityDensityFunction, RenderContext, Vector3};

pub struct MixturePdf {
    pdf0: Arc<dyn ProbabilityDensityFunction>,
    pdf1: Arc<dyn ProbabilityDensityFunction>,
}

impl MixturePdf {
    pub fn new(
        pdf0: Arc<dyn ProbabilityDensityFunction>,
        pdf1: Arc<dyn ProbabilityDensityFunction>,
    ) -> Self {
        Self { pdf0, pdf1 }
    }
}

impl ProbabilityDensityFunction for MixturePdf {
    fn value(&self, ctx: &RenderContext, direction: &Vector3) -> f64 {
        let v0 = 0.5 * self.pdf0.value(ctx, direction);
        let v1 = 0.5 * self.pdf1.value(ctx, direction);
        v0 + v1
    }

    fn generate(&self, ctx: &RenderContext) -> Vector3 {
        if ctx.random.rand() < 0.5 {
            self.pdf0.generate(ctx)
        } else {
            self.pdf1.generate(ctx)
        }
    }
}
