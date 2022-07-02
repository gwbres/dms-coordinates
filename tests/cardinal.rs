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
        assert_eq!(Cardinal::North.is_latitude(), true);
        assert_eq!(Cardinal::South.is_latitude(), true);
        assert_eq!(Cardinal::East.is_latitude(), false);
        assert_eq!(Cardinal::North.is_longitude(), false);
        assert_eq!(Cardinal::East.is_longitude(), true);
        assert_eq!(Cardinal::West.is_longitude(), true);
        assert_eq!(Cardinal::North.same_kind(Cardinal::South), true);
        assert_eq!(Cardinal::South.same_kind(Cardinal::SouthWest), false);
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
        assert_eq!(Cardinal::North.is_northern(), true);
        assert_eq!(Cardinal::NorthEast.is_northern(), true);
        assert_eq!(Cardinal::NorthWest.is_northern(), true);
        assert_eq!(Cardinal::South.is_northern(), false);
        assert_eq!(Cardinal::SouthEast.is_northern(), false);
        assert_eq!(Cardinal::SouthWest.is_northern(), false);
        assert_eq!(Cardinal::East.is_northern(), false);
        assert_eq!(Cardinal::West.is_northern(), false);
    }
    #[test]
    fn is_southern() {
        assert_eq!(Cardinal::North.is_southern(), false);
        assert_eq!(Cardinal::NorthEast.is_southern(), false);
        assert_eq!(Cardinal::NorthWest.is_southern(), false);
        assert_eq!(Cardinal::South.is_southern(), true);
        assert_eq!(Cardinal::SouthEast.is_southern(), true);
        assert_eq!(Cardinal::SouthWest.is_southern(), true);
        assert_eq!(Cardinal::East.is_southern(), false);
        assert_eq!(Cardinal::West.is_southern(), false);
    }
    #[test]
    fn is_eastern() {
        assert_eq!(Cardinal::North.is_eastern(), false);
        assert_eq!(Cardinal::NorthEast.is_eastern(), true);
        assert_eq!(Cardinal::NorthWest.is_eastern(), false);
        assert_eq!(Cardinal::South.is_eastern(), false);
        assert_eq!(Cardinal::SouthEast.is_eastern(), true);
        assert_eq!(Cardinal::SouthWest.is_eastern(), false);
        assert_eq!(Cardinal::East.is_eastern(), true);
        assert_eq!(Cardinal::West.is_eastern(), false);
    }
    #[test]
    fn is_western() {
        assert_eq!(Cardinal::North.is_western(), false);
        assert_eq!(Cardinal::NorthEast.is_western(), false);
        assert_eq!(Cardinal::NorthWest.is_western(), true);
        assert_eq!(Cardinal::South.is_western(), false);
        assert_eq!(Cardinal::SouthEast.is_western(), false);
        assert_eq!(Cardinal::SouthWest.is_western(), true);
        assert_eq!(Cardinal::East.is_western(), false);
        assert_eq!(Cardinal::West.is_western(), true);
    }
    #[test]
    fn is_sub_quadrant() {
        assert_eq!(Cardinal::North.is_sub_quadrant(), false);
        assert_eq!(Cardinal::West.is_sub_quadrant(), false);
        assert_eq!(Cardinal::East.is_sub_quadrant(), false);
        assert_eq!(Cardinal::South.is_sub_quadrant(), false);
        assert_eq!(Cardinal::NorthEast.is_sub_quadrant(), true);
        assert_eq!(Cardinal::NorthWest.is_sub_quadrant(), true);
        assert_eq!(Cardinal::SouthEast.is_sub_quadrant(), true);
        assert_eq!(Cardinal::SouthWest.is_sub_quadrant(), true);
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
        assert_eq!(Cardinal::North +90, Cardinal::East);
        assert_eq!(Cardinal::North +180, Cardinal::South);
        assert_eq!(Cardinal::East +180, Cardinal::West);
        assert_eq!(Cardinal::East +90, Cardinal::South);
        assert_eq!(Cardinal::NorthEast +315, Cardinal::North);
        assert_eq!(Cardinal::NorthEast +225, Cardinal::West);
        assert_eq!(Cardinal::North + 360 +180, Cardinal::South);
        assert_eq!(Cardinal::NorthEast + 360 +180, Cardinal::SouthWest);
    }
}
