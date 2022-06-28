pub struct BoundedAngle<T> {
    /// Actual angle value in °
    pub value: T, 
    /// Min tolerated value in °
    pub min: T,
    /// Max value tolerated in °
    pub max: T,
}

impl<T: std::fmt::Display> std::fmt::Display for BoundedAngle<T> {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T: PartialOrd + PartialEq + Clone> BoundedAngle<T> {
    pub fn new (min: T, max: T) -> Self {
        Self {
            value: min.clone(),
            min: min.clone(),
            max: max,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_angle() {
        let a : BoundedAngle<u8> = BoundedAngle::new(0_u8, 90_u8);
        assert_eq!(a.min, 0);
        assert_eq!(a.max, 90);
        assert_eq!(a.value, 0);
        assert_eq!(format!("{}", a), "0");
        
        let a : BoundedAngle<i8> = BoundedAngle::new(-90_i8, 90_i8);
        assert_eq!(a.min, -90);
        assert_eq!(a.max, 90);
        assert_eq!(a.value, -90);
        assert_eq!(format!("{}", a), "-90");
    }
}
