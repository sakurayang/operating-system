use strum::{EnumCount, EnumIter, FromRepr, IntoEnumIterator};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumCount, EnumIter, FromRepr)]
pub enum DeviceType {
    Printer,
    DiskReader,
    Input,
    Output,
}

#[test]
fn wtf() {
    assert_eq!(DeviceType::COUNT, 4)
}
