use abm::{simple_grid_2d::SimpleGrid2D};



// Represents food pheromones. Higher f64 = more concentrated pheromone
pub struct ToFoodGrid {
	pub grid: SimpleGrid2D<f64>
}

impl ToFoodGrid {
	pub fn new(width: i64, height: i64) -> ToFoodGrid {
		ToFoodGrid {
			grid: SimpleGrid2D::new(width, height)
		}
	}
}