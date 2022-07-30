pub struct RealmFlags;

impl RealmFlags {
    pub const NONE: u32 = 0x00;
    pub const VERSION_MISMATCH: u32 = 0x01;
    pub const OFFLINE: u32 = 0x02;
    pub const SPECIFY_BUILD: u32 = 0x04;
    pub const UNK1: u32 = 0x08;
    pub const UNK2: u32 = 0x10;
    pub const RECOMMENDED: u32 = 0x20;
    pub const NEW: u32 = 0x40;
    pub const FULL: u32 = 0x80;
}

pub enum RealmType {
    Normal = 0,
    PVP = 1,
    Normal2 = 4,
    RP = 6,
    RPPVP = 8,
    MaxType = 14,
    FFAPVP = 16,
}

pub enum RealmZones {
    // Any Language
    Unknown = 0,
    // Any Language
    Development = 1,
    // Extended-Latin
    UnitedStates = 2,
    // Extended-Latin
    Oceanic = 3,
    // Extended-Latin
    LatinAmerica = 4,
    // Basic-Latin At Create, Any At Login
    Tournament5 = 5,
    // East-Asian
    Korea = 6,
    // Basic-Latin At Create, Any At Login
    Tournament7 = 7,
    // Extended-Latin
    English = 8,
    // Extended-Latin
    German = 9,
    // Extended-Latin
    French = 10,
    // Extended-Latin
    Spanish = 11,
    // Cyrillic
    Russian = 12,
    // Basic-Latin At Create, Any At Login
    Tournament13 = 13,
    // East-Asian
    Taiwan = 14,
    // Basic-Latin At Create, Any At Login
    Tournament15 = 15,
    // East-Asian
    China = 16,
    // Basic-Latin At Create, Any At Login
    Cn1 = 17,
    // Basic-Latin At Create, Any At Login
    Cn2 = 18,
    // Basic-Latin At Create, Any At Login
    Cn3 = 19,
    // Basic-Latin At Create, Any At Login
    Cn4 = 20,
    // Basic-Latin At Create, Any At Login
    Cn5 = 21,
    // Basic-Latin At Create, Any At Login
    Cn6 = 22,
    // Basic-Latin At Create, Any At Login
    Cn7 = 23,
    // Basic-Latin At Create, Any At Login
    Cn8 = 24,
    // Basic-Latin At Create, Any At Login
    Tournament25 = 25,
    // Any Language
    TestServer = 26,
    // Basic-Latin At Create, Any At Login
    Tournament27 = 27,
    // Any Language
    QaServer = 28,
    // Basic-Latin At Create, Any At Login
    Cn9 = 29,
    // Any Language
    TestServer2 = 30,
    // Basic-Latin At Create, Any At Login
    Cn10 = 31,
    // Basic-Latin At Create, Any At Login
    Ctc = 32,
    // Basic-Latin At Create, Any At Login
    Cnc = 33,
    // Basic-Latin At Create, Any At Login
    Cn14 = 34,
    // Basic-Latin At Create, Any At Login
    Cn269 = 35,
    // Basic-Latin At Create, Any At Login
    Cn37 = 36,
    // Basic-Latin At Create, Any At Login
    Cn58 = 37,
}