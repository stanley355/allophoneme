#[derive(Debug, Clone)]
pub struct IpaEncoding {
    pub ipa: String,
    pub encoding: String,
}

impl IpaEncoding {
    pub fn new(ipa: &str, encoding: &str) -> Self {
        Self {
            ipa: ipa.to_string(),
            encoding: encoding.to_string(),
        }
    }
}

pub const IPA_ENCODING_LIST: [IpaEncoding] = [
    IpaEncoding::new("i", "v1"),
    IpaEncoding::new("ɪ", "v2"),
    IpaEncoding::new("e", "v3"),
    IpaEncoding::new("ɛ", "v4"),
    IpaEncoding::new("æ", "v5"),
    IpaEncoding::new("ʌ", "v6"),
    IpaEncoding::new("ə", "v7"),
    IpaEncoding::new("ɚ", "v8"),
    IpaEncoding::new("u", "v9"),
    IpaEncoding::new("ʊ", "v10"),
    IpaEncoding::new("o", "v11"),
    IpaEncoding::new("ɔ", "v12"),
    IpaEncoding::new("ɑ", "v13"),
    IpaEncoding::new("ɑɪ", "v14"),
    IpaEncoding::new("ɑʊ", "v15"),
    IpaEncoding::new("ɔɪ", "v16"),
    IpaEncoding::new("p", "c1"),
    IpaEncoding::new("b", "c2"),
    IpaEncoding::new("d", "c3"),
    IpaEncoding::new("t", "c4"),
    IpaEncoding::new("k", "c5"),
    IpaEncoding::new("g", "c6"),
    IpaEncoding::new("f", "c7"),
    IpaEncoding::new("v", "c8"),
    IpaEncoding::new("ɵ", "c9"),
    IpaEncoding::new("ð", "c10"),
    IpaEncoding::new("s", "c11"),
    IpaEncoding::new("z", "c12"),
    IpaEncoding::new("ʃ", "c13"),
    IpaEncoding::new("ʒ", "c14"),
    IpaEncoding::new("h", "c15"),
    IpaEncoding::new("ʧ", "c16"),
    IpaEncoding::new("ʤ", "c17"),
    IpaEncoding::new("m", "c18"),
    IpaEncoding::new("n", "c19"),
    IpaEncoding::new("ŋ", "c20"),
    IpaEncoding::new("l", "c21"),
    IpaEncoding::new("r", "c22"),
    IpaEncoding::new("w", "c23"),
    IpaEncoding::new("j", "c24"),
];
