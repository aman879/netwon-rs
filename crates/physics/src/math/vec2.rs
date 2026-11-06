
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    pub fn magnitude_squared(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn magnitude(self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    pub fn normalized(self) -> Self {
        let magnitude = self.magnitude();

        if magnitude == 0.0 {
            return Self::zero();
        }

        Self::new(self.x / magnitude, self.y / magnitude)
    }

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn is_zero(self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl std::ops::Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {

        assert!(
            rhs.abs() > f32::EPSILON,
            "Cannot divide Vec2 by zero"
        );
        Self::new(self.x / rhs, self.y / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_addition() {
        assert_eq!(
            Vec2::new(2.0, 3.0) + Vec2::new(4.0, 5.0),
            Vec2::new(6.0, 8.0)
        );
    }

    #[test]
    fn vector_subtraction() {
        assert_eq!(
            Vec2::new(5.0, 7.0) - Vec2::new(2.0, 3.0),
            Vec2::new(3.0, 4.0)
        );
    }

    #[test]
    fn vector_magnitude() {
        assert_eq!(Vec2::new(3.0, 4.0).magnitude(), 5.0);
    }

    #[test]
    fn vector_magnitude_squared() {
        assert_eq!(Vec2::new(3.0, 4.0).magnitude_squared(), 25.0);
    }

    #[test]
    fn vector_normalization() {
        let result = Vec2::new(0.0, 5.0).normalized();

        assert!((result.x - 0.0).abs() < 0.0001);
        assert!((result.y - 1.0).abs() < 0.0001);
    }

    #[test]
    fn normalize_zero_vector() {
        assert_eq!(Vec2::zero().normalized(), Vec2::zero());
    }

    #[test]
    fn vector_dot_product() {
        assert_eq!(
            Vec2::new(2.0, 3.0).dot(Vec2::new(4.0, 5.0)),
            23.0
        );
    }

    #[test]
    fn scalar_multiplication() {
        assert_eq!(
            Vec2::new(2.0, 4.0) * 3.0,
            Vec2::new(6.0, 12.0)
        );
    }

    #[test]
    fn scalar_division() {
        assert_eq!(
            Vec2::new(6.0, 8.0) / 2.0,
            Vec2::new(3.0, 4.0)
        );
    }

    #[test]
    #[should_panic]
    fn division_by_zero() {
        let _ = Vec2::new(2.0, 4.0) / 0.0;
    }

    #[test]
    fn zero_vector_check() {
        assert!(Vec2::zero().is_zero());
        assert!(!Vec2::new(1.0, 0.0).is_zero());
    }
}