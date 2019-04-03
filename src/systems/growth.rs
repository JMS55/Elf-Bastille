use crate::Context;

pub struct GrowthSystem;

impl GrowthSystem {
    pub fn run(&self, context: &mut Context) {
        for growth_component in context.growth_components.values_mut() {
            if growth_component.0 < 15 {
                growth_component.0 += 1;
            }
        }
    }
}
