use crate::components::{Texture, Tree, TreeGrowthStage};
use crate::DELTA_TIME;
use specs::{Join, System, WriteStorage};
use std::time::Duration;

pub struct TreeGrowthSystem;

impl<'a> System<'a> for TreeGrowthSystem {
    type SystemData = (WriteStorage<'a, Tree>, WriteStorage<'a, Texture>);

    fn run(&mut self, (mut tree_data, mut texture_data): Self::SystemData) {
        for (tree, tree_texture) in (&mut tree_data, &mut texture_data).join() {
            tree.growth_timer += DELTA_TIME;
            if tree.growth_timer >= Duration::from_secs(5) {
                tree.growth_timer = Duration::from_secs(0);

                match tree.growth_stage {
                    TreeGrowthStage::Stage1 => {
                        tree.growth_stage = TreeGrowthStage::Stage2;
                        tree_texture.atlas_index = 3;
                    }
                    TreeGrowthStage::Stage2 => {
                        tree.growth_stage = TreeGrowthStage::Stage3;
                        tree_texture.atlas_index = 4;
                    }
                    TreeGrowthStage::Stage3 => {
                        tree.growth_stage = TreeGrowthStage::Stage4;
                        tree_texture.atlas_index = 5;
                    }
                    TreeGrowthStage::Stage4 => {
                        tree.growth_stage = TreeGrowthStage::Stage5;
                        tree_texture.atlas_index = 6;
                    }
                    TreeGrowthStage::Stage5 => {}
                }
            }
        }
    }
}
