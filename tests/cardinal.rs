use dms_coordinates::cardinal::Cardinal;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        assert_eq!(Cardinal::default(), Cardinal::North)
    }
    #[test]
    fn test_lat_lon() {
        assert!(Cardinal::North.is_latitude());
        assert!(Cardinal::South.is_latitude());
        assert!(!Cardinal::East.is_latitude());
        assert!(!Cardinal::North.is_longitude());
        assert!(Cardinal::East.is_longitude());
        assert!(Cardinal::West.is_longitude());
        assert!(Cardinal::North.same_kind(Cardinal::South));
        assert!(!Cardinal::South.same_kind(Cardinal::SouthWest));
    }
    #[test]
    fn test_to_string() {
        assert_eq!(Cardinal::North.to_string(), "N");
        assert_eq!(Cardinal::NorthEast.to_string(), "NE");
        assert_eq!(Cardinal::East.to_string(), "E");
        assert_eq!(Cardinal::SouthEast.to_string(), "SE");
        assert_eq!(Cardinal::South.to_string(), "S");
        assert_eq!(Cardinal::SouthWest.to_string(), "SW");
        assert_eq!(Cardinal::West.to_string(), "W");
        assert_eq!(Cardinal::NorthWest.to_string(), "NW");
    }
    #[test]
    fn is_northern() {
        assert!(Cardinal::North.is_northern());
        assert!(Cardinal::NorthEast.is_northern());
        assert!(Cardinal::NorthWest.is_northern());
        assert!(!Cardinal::South.is_northern());
        assert!(!Cardinal::SouthEast.is_northern());
        assert!(!Cardinal::SouthWest.is_northern());
        assert!(!Cardinal::East.is_northern());
        assert!(!Cardinal::West.is_northern());
    }
    #[test]
    fn is_southern() {
        assert!(!Cardinal::North.is_southern());
        assert!(!Cardinal::NorthEast.is_southern());
        assert!(!Cardinal::NorthWest.is_southern());
        assert!(Cardinal::South.is_southern());
        assert!(Cardinal::SouthEast.is_southern());
        assert!(Cardinal::SouthWest.is_southern());
        assert!(!Cardinal::East.is_southern());
        assert!(!Cardinal::West.is_southern());
    }
    #[test]
    fn is_eastern() {
        assert!(!Cardinal::North.is_eastern());
        assert!(Cardinal::NorthEast.is_eastern());
        assert!(!Cardinal::NorthWest.is_eastern());
        assert!(!Cardinal::South.is_eastern());
        assert!(Cardinal::SouthEast.is_eastern());
        assert!(!Cardinal::SouthWest.is_eastern());
        assert!(Cardinal::East.is_eastern());
        assert!(!Cardinal::West.is_eastern());
    }
    #[test]
    fn is_western() {
        assert!(!Cardinal::North.is_western());
        assert!(!Cardinal::NorthEast.is_western());
        assert!(Cardinal::NorthWest.is_western());
        assert!(!Cardinal::South.is_western());
        assert!(!Cardinal::SouthEast.is_western());
        assert!(Cardinal::SouthWest.is_western());
        assert!(!Cardinal::East.is_western());
        assert!(Cardinal::West.is_western());
    }
    #[test]
    fn is_sub_quadrant() {
        assert!(!Cardinal::North.is_sub_quadrant());
        assert!(!Cardinal::West.is_sub_quadrant());
        assert!(!Cardinal::East.is_sub_quadrant());
        assert!(!Cardinal::South.is_sub_quadrant());
        assert!(Cardinal::NorthEast.is_sub_quadrant());
        assert!(Cardinal::NorthWest.is_sub_quadrant());
        assert!(Cardinal::SouthEast.is_sub_quadrant());
        assert!(Cardinal::SouthWest.is_sub_quadrant());
    }
    #[test]
    fn test_from_angle() {
        assert_eq!(Cardinal::from_angle(0), Cardinal::North);
        assert_eq!(Cardinal::from_angle(90), Cardinal::East);
        assert_eq!(Cardinal::from_angle(135), Cardinal::SouthEast);
        assert_eq!(Cardinal::from_angle(180), Cardinal::South);
        assert_eq!(Cardinal::from_angle(315), Cardinal::NorthWest);
    }
    #[test]
    fn test_to_angle() {
        assert_eq!(Cardinal::North.to_angle(), 0);
        assert_eq!(Cardinal::South.to_angle(), 180);
        assert_eq!(Cardinal::NorthEast.to_angle(), 45);
        assert_eq!(Cardinal::NorthWest.to_angle(), 315);
    }
    #[test]
    fn test_add_ops() {
        assert_eq!(Cardinal::North + 90, Cardinal::East);
        assert_eq!(Cardinal::North + 180, Cardinal::South);
        assert_eq!(Cardinal::East + 180, Cardinal::West);
        assert_eq!(Cardinal::East + 90, Cardinal::South);
        assert_eq!(Cardinal::NorthEast + 315, Cardinal::North);
        assert_eq!(Cardinal::NorthEast + 225, Cardinal::West);
        assert_eq!(Cardinal::North + 360 + 180, Cardinal::South);
        assert_eq!(Cardinal::NorthEast + 360 + 180, Cardinal::SouthWest);
    }
}
