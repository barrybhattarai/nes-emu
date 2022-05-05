use serde::{Serialize, Deserialize};
use std::ops::{Add, Sub};
#[derive(Serialize, Deserialize, Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: > Point<T> {
    pub fn dist_from_origin(&self) -> f64 {
        let distance = (self.x.powi(2) + self.y.powi(2)).sqrt();
        distance
    }

    pub fn dist_from(&self, other: &Self)-> f64{
        let temp = other.clone() - self.clone();
        temp.magnitude()
    }

    pub fn magnitude(&self)-> f64{
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


impl<T: Add<Output=T>> Add for Point<T>{
    type Output = Self;
    fn add(self, other:Self)->Self::Output{
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl<T> Sub for Point<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x - rhs.x,
            y : self.y - rhs.y
        }
    }
}