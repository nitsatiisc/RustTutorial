pub mod points;
pub mod largest;
pub mod lifetimes;
pub mod iterator;

#[cfg(test)]
mod tests {
    use crate::largest::{largest, MyVector};
    use crate::iterator::{sum_row, Matrix, RefIterator};
    use crate::lifetimes::bigger_num;
    use crate::points::{Point, SimplePoint};
    use super::*;

    #[test]
    fn test_simple_point() {
        let mut p1 = SimplePoint::new(2,5);
        p1.double();
        println!("p1 = {}", p1);
    }

    #[test]
    fn add_simple_points() {
        let p1: SimplePoint = SimplePoint::new(2,3);
        let p2: SimplePoint = SimplePoint::new(5,7);
        let p3 = p1 + p2;

        println!("p1 = {:?}, p2 = {:?}, p3 = {:?}", p1, p2, p3);
        println!("p1={}, p2={}, p3={}", p1, p2, p3);
    }

    #[test]
    fn test_generic_point() {
        let mut p1: Point<f32> = Point { x: 5.6, y: 2.2 };
        p1.double();
        println!("p1 = {}", p1);

        let mut p1: Point<SimplePoint> = Point { x: SimplePoint::new(10, 20), y: SimplePoint::new(5, 11) };
        p1.double();
        println!("p1 = {}", p1);
    }

    #[test]
    fn add_generic_points_i32() {
        let p1: Point<f32> = Point {x: 2.0f32, y: 3.0f32};
        let p2: Point<f32> = Point {x: 3.0f32, y: 5.0f32};
        let p3 = p1 + p2;

        println!("p1 = {:?}, p2 = {:?}, p3 = {:?}", p1, p2, p3);
        println!("p1={}, p2={}, p3={}", p1, p2, p3);
    }

    #[test]
    fn test_generic_iterator() {
        let mut my_vector: MyVector<u32> = MyVector::new(&vec![5,6,9,2,11]);
        let x = largest(&mut my_vector);
        println!("Largest value in the vector is {}",x.expect("The vector was empty"));
    }

    #[test]
    fn test_ref_iterator() {
        let mut my_matrix = Matrix::new(&vec![vec![0,1,2],vec![3,4,5],vec![6,7,8]]);
        let row1 = my_matrix.next().unwrap();
        println!("Sum of row = {}", sum_row(row1));
        let row2 = my_matrix.next().unwrap();
        println!("Sum or row = {}", sum_row(row2));
        while let Some(row) = my_matrix.next() {
            println!("Sum of row = {}", sum_row(row));
        }
    }

    #[test]
    fn test_lifetime() {
        let c;
        let a = 10u32;
        {
            let b = 20u32;
            c = bigger_num(&a, &b);
        }
        println!("The bigger number is {}",c);
    }
}
