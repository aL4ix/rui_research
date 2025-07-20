#[repr(u32)]
#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub enum StyleEnum {
    Extra,
    ApplyTo,
    Color,
    BackgroundColor,
    Size,
    Font,
    FontSize,
}
