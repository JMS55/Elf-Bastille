use crate::components::{Tree, TreeGrowthStage, WorldLocation};
use specs::{Join, System, WriteStorage};

pub struct TreeGrowthSystem;

impl<'a> System<'a> for TreeGrowthSystem {
    type SystemData = (WriteStorage<'a, Tree>, WriteStorage<'a, WorldLocation>);

    fn run(&mut self, (mut tree_data, mut location_data): Self::SystemData) {
        for (tree, tree_location) in (&mut tree_data, &mut location_data).join() {
            if tree.growth_timer.triggered() {
                match tree.growth_stage {
                    TreeGrowthStage::Stage1 => {
                        tree.growth_stage = TreeGrowthStage::Stage2;
                        tree_location.texture_atlas_index = 3;
                    }
                    TreeGrowthStage::Stage2 => {
                        tree.growth_stage = TreeGrowthStage::Stage3;
                        tree_location.texture_atlas_index = 4;
                    }
                    TreeGrowthStage::Stage3 => {
                        tree.growth_stage = TreeGrowthStage::Stage4;
                        tree_location.texture_atlas_index = 5;
                    }
                    TreeGrowthStage::Stage4 => {
                        tree.growth_stage = TreeGrowthStage::Stage5;
                        tree_location.texture_atlas_index = 6;
                    }
                    TreeGrowthStage::Stage5 => {}
                }
            }
        }
    }
}
