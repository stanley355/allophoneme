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
    //     ["i", "v1"]
    //  ["ɪ", "v2"]
    //  ["e", "v3"]
    //  ["ɛ", "v4"]
    //  ["æ", "v5"]
    //  ["ʌ", "v6"]
    //  ["ə", "v7"]
    //  ["ɚ", "v8"]
    //  ["u", "v9"]
    //  ["ʊ", "v10"]
    //  ["o", "v11"]
    //  ["ɔ", "v12"]
    //  ["ɑ", "v13"]
    //  ["ɑɪ", "v14"]
    //  ["ɑʊ", "v15"]
    //  ["ɔɪ", "v16"]
    //  ["p", "c1"]
    //  ["b", "c2"]
    //  ["d", "c3"]
    //  ["t", "c4"]
    //  ["k", "c5"]
    //  ["g", "c6"]
    //  ["f", "c7"]
    //  ["v", "c8"]
    //  ["ɵ", "c9"]
    //  ["ð", "c10"]
    //  ["s", "c11"]
    //  ["z", "c12"]
    //  ["ʃ", "c13"]
    //  ["ʒ", "c14"]
    //  ["h", "c15"]
    //  ["ʧ", "c16"]
    //  ["ʤ", "c17"]
    //  ["m", "c18"]
    //  ["n", "c19"]
    //  ["ŋ", "c20"]
    //  ["l", "c21"]
    //  ["r", "c22"]
    //  ["w", "c23"]
    //  ["j", "c24"]
    IpaEncoding::new("i", "v1"),
    IpaEncoding::new("ɪ", "v2"),
    IpaEncoding::new("e", "v3"),
    IpaEncoding::new("", encoding),
];
