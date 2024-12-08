use std::ops::Mul;

type Direction<T> = Coord<T>;

pub const NORTH: Direction<i8> = Direction { x: 0, y: -1 };
pub const SOUTH: Direction<i8> = Direction { x: 0, y: 1 };
pub const EAST: Direction<i8> = Direction { x: 1, y: 0 };
pub const WEST: Direction<i8> = Direction { x: -1, y: 0 };

pub const CARDINAL_DIRECTIONS: [Direction<i8>; 4] = [NORTH, SOUTH, EAST, WEST];

pub const NORTH_EAST: Direction<i8> = Direction { x: 1, y: -1 };
pub const NORTH_WEST: Direction<i8> = Direction { x: -1, y: -1 };
pub const SOUTH_EAST: Direction<i8> = Direction { x: 1, y: 1 };
pub const SOUTH_WEST: Direction<i8> = Direction { x: -1, y: 1 };

pub const INTER_CARDINAL_DIRECTIONS: [Direction<i8>; 4] =
    [NORTH_EAST, NORTH_WEST, SOUTH_EAST, SOUTH_WEST];

pub const ALL_DIRECTIONS: [Direction<i8>; 8] = [
    NORTH, SOUTH, EAST, WEST, NORTH_EAST, NORTH_WEST, SOUTH_EAST, SOUTH_WEST,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord<T>
where
    T: std::ops::Add + std::ops::Sub,
{
    pub x: T,
    pub y: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoundingBox<T>
where
    T: std::cmp::PartialOrd + std::ops::Sub<Output = T> + Mul<Output = T> + std::ops::Add,
{
    pub start: Coord<T>,
    pub end: Coord<T>,
}

impl<T> Coord<T>
where
    T: std::ops::Add<Output = T> + Copy + std::ops::Sub<Output = T>,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn direction_to(&self, other: &Coord<T>) -> Direction<T> {
        let x = other.x - self.x;
        let y = other.y - self.y;

        Direction { x, y }
    }

    pub fn sub(&self, other: Coord<T>) -> Coord<T> {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn add(&self, other: Coord<T>) -> Coord<T> {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub fn sub<T>(a: Coord<T>, b: Coord<T>) -> Coord<T>
where
    T: std::ops::Sub<Output = T> + std::ops::Add + Copy,
{
    Coord {
        x: a.x - b.x,
        y: a.y - b.y,
    }
}

pub fn add<T>(a: Coord<T>, b: Coord<T>) -> Coord<T>
where
    T: std::ops::Sub<Output = T> + std::ops::Add<Output = T> + Copy,
{
    Coord {
        x: a.x + b.x,
        y: a.y + b.y,
    }
}

impl<T> BoundingBox<T>
where
    T: std::cmp::PartialOrd + std::ops::Sub<Output = T> + Mul<Output = T> + std::ops::Add + Copy,
{
    pub fn new(start: Coord<T>, end: Coord<T>) -> Self {
        Self { start, end }
    }

    pub fn width(&self) -> T {
        self.end.x - self.start.x
    }

    pub fn height(&self) -> T {
        self.end.y - self.start.y
    }

    pub fn area(&self) -> T {
        self.width() * self.height()
    }

    pub fn contains(&self, coord: &Coord<T>) -> bool {
        coord.x >= self.start.x
            && coord.x <= self.end.x
            && coord.y >= self.start.y
            && coord.y <= self.end.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounding_box_contains() {
        // Arrange
        let start = Coord::new(0, 0);
        let end = Coord::new(3, 4);
        let bounding_box = BoundingBox::new(start, end);
        let coord = Coord::new(1, 2);

        // Act
        let result = bounding_box.contains(&coord);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn bounding_box_does_not_contain() {
        // Arrange
        let start = Coord::new(0, 0);
        let end = Coord::new(3, 4);
        let bounding_box = BoundingBox::new(start, end);
        let coord = Coord::new(4, 5);

        // Act
        let result = bounding_box.contains(&coord);

        // Assert
        assert_eq!(result, false);
    }
}
