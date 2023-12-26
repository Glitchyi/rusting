#[derive(Debug)]


#[derive(Clone,Copy)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle{

    fn get_dimensions(&self) -> (u32,u32){
        (self.width, self.height)
    }

    fn area(&self) -> u32{
        self.width * self.height
    }

    fn perimeter(&self) -> u32{
        2*(self.height + self.width)
    }

    fn can_fit(self,other:Self) -> bool{
        self.width > other.width && self.height >other.height
    }
    
    fn create_rectangle(l :u32, b :u32) -> Self {
        Rectangle { width: l, height: b }
    }
    fn create_sq(a:u32) -> Self {
        Rectangle{width: a, height: a }
    }

    fn diag_len(&self) -> u32{
        ((self.width * self.width + self.height * self.height) as f64).sqrt() as u32
    }
    #[allow(dead_code)]
    fn is_square(&self) -> bool {
        self.width == self.height
    }

    fn bigger_rect(&self, other: Self) -> Self {
        
            if self.area() >= other.area(){
                 *self
            }else{
                other
            }

    }

}


fn main(){
    println!("Implementation of Shapes");
    dbg!("Hello console");
    let recta = Rectangle::create_rectangle(7, 8);
    println!("the rectangle has dimensions :  {} x {}",recta.get_dimensions().0, recta.get_dimensions().1);
    println!("The rectangle has an area of {}",recta.area());
    println!("The rectangle has a perimeter of  {}",recta.perimeter());
    let rectb = Rectangle::create_sq(5);
    println!("Rectangle {:#?} is bigger",recta.bigger_rect(rectb));
    println!("Rectangle fits {:#?}", recta.can_fit(rectb));
    println!("Diagonal length is {}",recta.diag_len());


}