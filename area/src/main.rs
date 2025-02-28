#[derive(Debug)]
struct Rect {
	width  : f32,
	height : f32,
}

impl Rect {
	fn area(&self) -> f32{
		self.width * self.height
	}
	fn can_hold(&self, rect : &Rect) -> bool{
		self.width > rect.width && self.height > rect.height
	}

	fn new(width : f32, height : f32) -> Self{
		Self {
			width,
			height,
		}
	}
	fn square(length : f32) -> Self {
		Self::new(length, length)	//calls the ::new() constructor
	}
}

fn main() {
    let rect1 = Rect::new(30.0, 50.0);
    let rect2 = Rect::new(10.0, 40.0);
    let rect3 = Rect::square(60.0);

    println!(
		"Can rect1 hold rect2? {}", 
		rect1.can_hold(&rect2)		//-> true
	);
    println!(
		"Can rect1 hold rect3? {}",
		rect1.can_hold(&rect3)		//-> false
	);
}