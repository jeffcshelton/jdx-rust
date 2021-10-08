mod bindings;

pub mod jdx {
    use crate::bindings;

    pub type Version = bindings::JDXVersion;
    pub type ColorType = bindings::JDXColorType;
    
    #[derive(Clone)]
    pub struct Image {
        pub data: Box<[u8]>,

        pub width: i16,
        pub height: i16,
        pub color_type: ColorType,
    }

    pub type Label = i16;

    #[derive(Clone, Copy)]
    pub struct Header {
        pub version: Version,
        pub color_type: ColorType,

        pub image_width: i16,
        pub image_height: i16,
        pub item_count: usize,
    }

    #[derive(Clone)]
    pub struct Dataset {
        pub header: Header,

        pub images: Vec<Image>,
        pub labels: Vec<Label>,
    }
}
