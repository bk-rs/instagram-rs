use num_enum::{IntoPrimitive, TryFromPrimitive};

/*
Ref https://github.com/ping/instagram_private_api/blob/1.6.0/instagram_private_api/compatpatch.py#L10-L57
*/

#[derive(IntoPrimitive, TryFromPrimitive, PartialEq, Debug, Clone, strum_macros::Display)]
#[repr(i16)]
pub enum FilterType {
    OES = -2,
    YUV = -1,
    Normal = 0,
    #[strum(serialize = "X-Pro II")]
    XProII = 1,
    #[strum(serialize = "Lo-Fi")]
    LoFi = 2,
    Earlybird = 3,
    Inkwell = 10,
    #[strum(serialize = "1977")]
    S1977 = 14,
    Nashville = 15,
    Kelvin = 16,
    Mayfair = 17,
    Sutro = 18,
    Toaster = 19,
    Walden = 20,
    Hefe = 21,
    Brannan = 22,
    Rise = 23,
    Amaro = 24,
    Valencia = 25,
    Hudson = 26,
    Sierra = 27,
    Willow = 28,
    Dogpatch = 105,
    Vesper = 106,
    Ginza = 107,
    Charmes = 108,
    Stinson = 109,
    Moon = 111,
    Clarendon = 112,
    Skyline = 113,
    Gingham = 114,
    Brooklyn = 115,
    Ashby = 116,
    Helena = 117,
    Maven = 118,
    Ludwig = 603,
    Slumber = 605,
    Perpetua = 608,
    Aden = 612,
    Juno = 613,
    Reyes = 614,
    Lark = 615,
    Crema = 616,
    BrightContrast = 640,
    CrazyColor = 642,
    SubtleColor = 643,
}
