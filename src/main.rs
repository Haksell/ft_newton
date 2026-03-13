mod body;
mod matrix;
mod quaternion;
mod scene;
mod sphere;
mod vector;

use crate::scene::Scene;

fn main() {
    let scene = Scene::manual();
    println!("{scene:?}");
}
