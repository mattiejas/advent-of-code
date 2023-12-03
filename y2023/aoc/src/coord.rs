#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoundingBox {
    pub start: Coord,
    pub end: Coord,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn manhattan_distance(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl BoundingBox {
    pub fn new(start: Coord, end: Coord) -> Self {
        Self { start, end }
    }

    pub fn width(&self) -> i32 {
        self.end.x - self.start.x
    }

    pub fn height(&self) -> i32 {
        self.end.y - self.start.y
    }

    pub fn area(&self) -> i32 {
        self.width() * self.height()
    }

    pub fn contains(&self, coord: &Coord) -> bool {
        coord.x >= self.start.x && coord.x <= self.end.x
            && coord.y >= self.start.y && coord.y <= self.end.y
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

        // Act
        let result = bounding_box.contains(Coord::new(1, 2));

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn bounding_box_does_not_contain() {
        // Arrange
        let start = Coord::new(0, 0);
        let end = Coord::new(3, 4);
        let bounding_box = BoundingBox::new(start, end);

        // Act
        let result = bounding_box.contains(Coord::new(4, 5));

        // Assert
        assert_eq!(result, false);
    }
}