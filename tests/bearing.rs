use dms_coordinates::Bearing;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_northern() {
        assert_eq!(Bearing::North.is_northern(), true);
        assert_eq!(Bearing::NorthEast.is_northern(), true);
        assert_eq!(Bearing::NorthWest.is_northern(), true);
        assert_eq!(Bearing::South.is_northern(), false);
        assert_eq!(Bearing::SouthEast.is_northern(), false);
        assert_eq!(Bearing::SouthWest.is_northern(), false);
        assert_eq!(Bearing::East.is_northern(), false);
        assert_eq!(Bearing::West.is_northern(), false);
    }
    #[test]
    fn is_southern() {
        assert_eq!(Bearing::North.is_southern(), false);
        assert_eq!(Bearing::NorthEast.is_southern(), false);
        assert_eq!(Bearing::NorthWest.is_southern(), false);
        assert_eq!(Bearing::South.is_southern(), true);
        assert_eq!(Bearing::SouthEast.is_southern(), true);
        assert_eq!(Bearing::SouthWest.is_southern(), true);
        assert_eq!(Bearing::East.is_southern(), false);
        assert_eq!(Bearing::West.is_southern(), false);
    }
    #[test]
    fn is_eastern() {
        assert_eq!(Bearing::North.is_eastern(), false);
        assert_eq!(Bearing::NorthEast.is_eastern(), true);
        assert_eq!(Bearing::NorthWest.is_eastern(), false);
        assert_eq!(Bearing::South.is_eastern(), false);
        assert_eq!(Bearing::SouthEast.is_eastern(), true);
        assert_eq!(Bearing::SouthWest.is_eastern(), false);
        assert_eq!(Bearing::East.is_eastern(), true);
        assert_eq!(Bearing::West.is_eastern(), false);
    }
    #[test]
    fn is_western() {
        assert_eq!(Bearing::North.is_western(), false);
        assert_eq!(Bearing::NorthEast.is_western(), false);
        assert_eq!(Bearing::NorthWest.is_western(), true);
        assert_eq!(Bearing::South.is_western(), false);
        assert_eq!(Bearing::SouthEast.is_western(), false);
        assert_eq!(Bearing::SouthWest.is_western(), true);
        assert_eq!(Bearing::East.is_western(), false);
        assert_eq!(Bearing::West.is_western(), true);
    }
    #[test]
    fn is_sub_quadrant() {
        assert_eq!(Bearing::NorthEast.is_sub_quadrant(), true);
        assert_eq!(Bearing::NorthWest.is_sub_quadrant(), true);
        assert_eq!(Bearing::SouthEast.is_sub_quadrant(), true);
        assert_eq!(Bearing::SouthWest.is_sub_quadrant(), true);
        assert_eq!(Bearing::North.is_sub_quadrant(), false);
        assert_eq!(Bearing::West.is_sub_quadrant(), false);
        assert_eq!(Bearing::East.is_sub_quadrant(), false);
        assert_eq!(Bearing::South.is_sub_quadrant(), false);
    }
}
