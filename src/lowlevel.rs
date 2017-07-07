#![allow(dead_code)]

// Base types of the TIFF format.
pub type BYTE      = u8;
pub type SHORT     = u16;
pub type LONG      = u32;
pub type ASCII     = String;
pub type RATIONAL  = (u32, u32);
pub type SBYTE     = i8;
pub type SSHORT    = i16;
pub type SLONG     = i32;
pub type SRATIONAL = (i32, i32);
pub type FLOAT     = f32;
pub type DOUBLE    = f64;

// Different values individual components can take.
enum_from_primitive! {
    #[repr(u16)]
    #[derive(Debug)]
    pub enum TIFFByteOrder {
        LittleEndian = 0x4949,
        BigEndian    = 0x4d4d,
    }
}

enum_from_primitive! {
    #[repr(u16)]
    #[derive(Debug,PartialEq)]
    pub enum TagType {
        ByteTag           = 1,
        ASCIITag          = 2,
        ShortTag          = 3,
        LongTag           = 4,
        RationalTag       = 5,
        SignedByteTag     = 6,
        UndefinedTag      = 7,
        SignedShortTag    = 8,
        SignedLongTag     = 9,
        SignedRationalTag = 10,
        FloatTag          = 11,
        DoubleTag         = 12,

        // No idea what those are...
        Long8             = 16,
        SLong8            = 17,
        IFD8              = 18,

        // Not part of spec
        ShortOrLongTag    = 0xfffe,
    }
}

pub fn tag_size(t: &TagType) -> u32 {
    match *t {
        TagType::ByteTag           => 1,
        TagType::ASCIITag          => 1,
        TagType::ShortTag          => 2,
        TagType::LongTag           => 4,
        TagType::RationalTag       => 8,
        TagType::SignedByteTag     => 1,
        TagType::UndefinedTag      => 1,
        TagType::SignedShortTag    => 2,
        TagType::SignedLongTag     => 2,
        TagType::SignedRationalTag => 8,
        TagType::FloatTag          => 4,
        TagType::DoubleTag         => 8,

        // No idea what those are...
        TagType::Long8             => 16,
        TagType::SLong8            => 17,
        TagType::IFD8              => 18,
        _                          => 0,
    }
}

#[derive(Debug)]
pub enum TagValue {
    ByteValue(BYTE),
    ShortValue(SHORT),
    LongValue(LONG),
    AsciiValue(ASCII),
    RationalValue(RATIONAL),
    SignedByteValue(SBYTE),
    SignedShortValue(SSHORT),
    SignedLongValue(SLONG),
    SignedRationalValue(SRATIONAL),
    FloatValue(FLOAT),
    DoubleValue(DOUBLE),
}

#[repr(u16)]
#[derive(Debug)]
pub enum PhotometricInterpretation {
    WhiteIsZero = 0,
    BlackIsZero = 1,
}

#[repr(u16)]
#[derive(Debug)]
pub enum Compression {
    None     = 1,
    Huffman  = 2,
    LZW      = 5,
    OJPEG    = 6,
    JPEG     = 7,
    PackBits = 32773,
}

#[repr(u16)]
#[derive(Debug)]
pub enum ResolutionUnit {
    None       = 1,
    Inch       = 2,
    Centimetre = 3,
}

#[repr(u16)]
#[derive(Debug)]
pub enum SampleFormat {
    UnsignedInteger             = 1,
    TwosComplementSignedInteger = 2,
    IEEEFloatingPoint           = 3,
    Undefined                   = 4,
}

#[derive(Debug)]
pub enum ImageType {
    Bilevel,
    Grayscale,
    PaletteColour,
    RGB,
    YCbCr,
}

#[repr(u16)]
#[derive(Debug)]
pub enum ImageOrientation {
    TopLeft     = 1,	// row 0 top, col 0 lhs
    TopRight    = 2,	// row 0 top, col 0 rhs
    BottomRight = 3,	// row 0 bottom, col 0 rhs
    BottomLeft  = 4,	// row 0 bottom, col 0 lhs
    LeftTop     = 5,	// row 0 lhs, col 0 top
    RightTop    = 6, 	// row 0 rhs, col 0 top
    RightBottom = 7,	// row 0 rhs, col 0 bottom
    LeftBottom  = 8,	// row 0 lhs, col 0 bottom
}


// Baseline Tags
enum_from_primitive! {
    #[repr(u16)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum TIFFTag {

        // Baseline Tags
        ArtistTag                    = 0x013b,
        BitsPerSampleTag             = 0x0102,
        CellLengthTag                = 0x0109,
        CellWidthTag                 = 0x0108,
        ColorMapTag                  = 0x0140,
        CompressionTag               = 0x0103,
        CopyrightTag                 = 0x8298,
        DateTimeTag                  = 0x0132,
        ExtraSamplesTag              = 0x0152,
        FillOrderTag                 = 0x010a,
        FreeByteCountsTag            = 0x0121,
        FreeOffsetsTag               = 0x0120,
        GrayResponseCurveTag         = 0x0123,
        GrayResponseUnitTag          = 0x0122,
        HostComputerTag              = 0x013c,
        ImageDescriptionTag          = 0x010e,
        ImageLengthTag               = 0x0101,
        ImageWidthTag                = 0x0100,
        MakeTag                      = 0x010f,
        MaxSampleValueTag            = 0x0119,
        MinSampleValueTag            = 0x0118,
        ModelTag                     = 0x0110,
        NewSubfileTypeTag            = 0x00fe,
        OrientationTag               = 0x0112,
        PhotometricInterpretationTag = 0x0106,
        PlanarConfigurationTag       = 0x011c,
        PredictorTag                 = 0x013d,
        ResolutionUnitTag            = 0x0128,
        RowsPerStripTag              = 0x0116,
        SampleFormatTag              = 0x0153,
        SamplesPerPixelTag           = 0x0115,
        SoftwareTag                  = 0x0131,
        StripByteCountsTag           = 0x0117,
        StripOffsetsTag              = 0x0111,
        SubfileTypeTag               = 0x00ff,
        ThresholdingTag              = 0x0107,
        XResolutionTag               = 0x011a,
        YResolutionTag               = 0x011b,

        // Section 20: Colorimetry
        WhitePointTag                = 0x013e,
        PrimaryChromaticities        = 0x013f,
        TransferFunction             = 0x012d,
        TransferRange                = 0x0156,
        ReferenceBlackWhite          = 0x0214,

        // Section 21: YCbCr Images
        YCbCrCoefficients            = 0x0211,
        YCbCrSubsampling             = 0x0212,
        YCbCrPositioning             = 0x0213,

        // TIFF/EP Tags
        SubIFDsTag                   = 0x014a,
        JPEGTablesTag                = 0x015b,
        CFARepeatPatternDimTag       = 0x828d,
        BatteryLevelTag              = 0x828f,
        IPTCTag                      = 0x83BB,
        InterColorProfileTag         = 0x8773,
        InterlaceTag                 = 0x8829,
        TimeZoneOffsetTag            = 0x882a,
        SelfTimerModeTag             = 0x882b,
        NoiseTag                     = 0x920d,
        ImageNumberTag               = 0x9211,
        SecurityClassificationTag    = 0x9212,
        ImageHistoryTag              = 0x9213,
        EPStandardIdTag              = 0x9216,

        // Extension TIFF Tags
        // See http://www.awaresystems.be/imaging/tiff/tifftags/extension.html
        XMPTag                       = 0x02bc,

        // Private Tags
        PhotoshopTag                 = 0x8649,
        EXIFTag                      = 0x8769,
    }
}

// Default Values
static PHOTOMETRIC_INTERPRETATION_SHORT_DEFAULT: SHORT = 1;
static PHOTOMETRIC_INTERPRETATION_LONG_DEFAULT: LONG = 1;