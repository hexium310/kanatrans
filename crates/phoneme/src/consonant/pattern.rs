pub(crate) const CONSONANT_PATTERNS: [(&str, ConsonantPattern); 25] = [
    (
        "p",
        ConsonantPattern {
            with_vowel: ["パ", "ピ", "プ", "ペ", "ポ"],
            unit: "プ",
        },
    ),
    (
        "b",
        ConsonantPattern {
            with_vowel: ["バ", "ビ", "ブ", "ベ", "ボ"],
            unit: "ブ",
        },
    ),
    (
        "t",
        ConsonantPattern {
            with_vowel: ["タ", "ティ", "トゥ", "テ", "ト"],
            unit: "ト",
        },
    ),
    (
        "d",
        ConsonantPattern {
            with_vowel: ["ダ", "ディ", "ドゥ", "デ", "ド"],
            unit: "ド",
        },
    ),
    (
        "ch",
        ConsonantPattern {
            with_vowel: ["チャ", "チ", "チュ", "チェ", "チョ"],
            unit: "チ",
        },
    ),
    (
        "dh",
        ConsonantPattern {
            with_vowel: ["ジャ", "ジ", "ジュ", "ジェ", "ジョ"],
            unit: "ジ",
        },
    ),
    (
        "k",
        ConsonantPattern {
            with_vowel: ["カ", "キ", "ク", "ケ", "コ"],
            unit: "ク",
        },
    ),
    (
        "g",
        ConsonantPattern {
            with_vowel: ["ガ", "ギ", "グ", "ゲ", "ゴ"],
            unit: "グ",
        },
    ),
    (
        "f",
        ConsonantPattern {
            with_vowel: ["ファ", "フィ", "フュ", "フェ", "フォ"],
            unit: "フ",
        },
    ),
    (
        "v",
        ConsonantPattern {
            with_vowel: ["ヴァ", "ヴィ", "ヴ", "ヴェ", "ヴォ"],
            unit: "ヴ",
        },
    ),
    (
        "s",
        ConsonantPattern {
            with_vowel: ["サ", "シ", "ス", "セ", "ソ"],
            unit: "ス",
        },
    ),
    (
        "z",
        ConsonantPattern {
            with_vowel: ["ザ", "ジ", "ズ", "ゼ", "ゾ"],
            unit: "ズ",
        },
    ),
    (
        "th",
        ConsonantPattern {
            with_vowel: ["サ", "シ", "ス", "セ", "ソ"],
            unit: "ス",
        },
    ),
    (
        "dh",
        ConsonantPattern {
            with_vowel: ["ザ", "ジ", "ズ", "ゼ", "ゾ"],
            unit: "ズ",
        },
    ),
    (
        "sh",
        ConsonantPattern {
            with_vowel: ["シャ", "シ", "シュ", "シェ", "ショ"],
            unit: "シュ",
        },
    ),
    (
        "zh",
        ConsonantPattern {
            with_vowel: ["ジャ", "ジ", "ジュ", "ジェ", "ジョ"],
            unit: "ジュ",
        },
    ),
    (
        "jh",
        ConsonantPattern {
            with_vowel: ["ジャ", "ジ", "ジュ", "ジェ", "ジョ"],
            unit: "ジ",
        },
    ),
    (
        "hh",
        ConsonantPattern {
            with_vowel: ["ハ", "ヒ", "ヒュ", "ヘ", "ホ"],
            unit: "フ",
        },
    ),
    (
        "m",
        ConsonantPattern {
            with_vowel: ["マ", "ミ", "ム", "メ", "モ"],
            unit: "ム",
        },
    ),
    (
        "n",
        ConsonantPattern {
            with_vowel: ["ナ", "ニ", "ヌ", "ネ", "ノ"],
            unit: "ン",
        },
    ),
    (
        "ng",
        ConsonantPattern {
            with_vowel: ["ン", "ン", "ン", "ン", "ン"],
            unit: "ン",
        },
    ),
    (
        "l",
        ConsonantPattern {
            with_vowel: ["ラ", "リ", "ル", "レ", "ロ"],
            unit: "ル",
        },
    ),
    (
        "r",
        ConsonantPattern {
            with_vowel: ["ラ", "リ", "ル", "レ", "ロ"],
            unit: "アー",
        },
    ),
    (
        "w",
        ConsonantPattern {
            with_vowel: ["ワ", "ウィ", "ウ", "ウェ", "ウォ"],
            unit: "ウ",
        },
    ),
    (
        "y",
        ConsonantPattern {
            with_vowel: ["ヤ", "イ", "ユ", "イェ", "ヨ"],
            unit: "ユ",
        },
    ),
];

#[derive(Clone, Copy, Debug)]
pub struct ConsonantPattern {
    pub(crate) with_vowel: [&'static str; 5],
    pub(crate) unit: &'static str,
}
