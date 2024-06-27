use crate::consonant::pattern::ConsonantPattern;

pub(crate) static CONSONANT_CLUSTER_PATTERNS: [(&str, ConsonantPattern); 13] = [
    (
        "ts",
        ConsonantPattern {
            with_vowel: ["タ", "ティ", "ツ", "テ", "ト"],
            unit: "ツ",
        },
    ),
    (
        "ds",
        ConsonantPattern {
            with_vowel: ["ダ", "ディ", "ズ", "デ", "ド"],
            unit: "ズ",
        },
    ),
    (
        "mb",
        ConsonantPattern {
            with_vowel: ["ンバ", "ンビ", "ンビュ", "ンベ", "ンボ"],
            unit: "ンブ",
        },
    ),
    (
        "mm",
        ConsonantPattern {
            with_vowel: ["ンマ", "ンミ", "ンム", "ンメ", "ンモ"],
            unit: "ンム",
        },
    ),
    (
        "mp",
        ConsonantPattern {
            with_vowel: ["ンパ", "ンピ", "ンピュ", "ンペ", "ンポ"],
            unit: "ンプ",
        },
    ),
    (
        "by",
        ConsonantPattern {
            with_vowel: ["ビャ", "ビ", "ビュ", "ビェ", "ビョ"],
            unit: "ビュ",
        },
    ),
    (
        "fy",
        ConsonantPattern {
            with_vowel: ["ヒャ", "ヒ", "フュ", "ヒェ", "ヒョ"],
            unit: "フュ",
        },
    ),
    (
        "gy",
        ConsonantPattern {
            with_vowel: ["ギャ", "ギ", "ギュ", "ギェ", "ギョ"],
            unit: "ギュ",
        },
    ),
    (
        "hhy",
        ConsonantPattern {
            with_vowel: ["ヒャ", "ヒ", "ヒュ", "ヒェ", "ヒョ"],
            unit: "ヒュ",
        },
    ),
    (
        "ky",
        ConsonantPattern {
            with_vowel: ["キャ", "キ", "キュ", "キェ", "キョ"],
            unit: "キュ",
        },
    ),
    (
        "ly",
        ConsonantPattern {
            with_vowel: ["リャ", "リ", "リュ", "リェ", "リョ"],
            unit: "リュ",
        },
    ),
    (
        "my",
        ConsonantPattern {
            with_vowel: ["ミャ", "ミ", "ミュ", "ミェ", "ミョ"],
            unit: "ミュ",
        },
    ),
    (
        "py",
        ConsonantPattern {
            with_vowel: ["ピャ", "ピ", "ピュ", "ピェ", "ピョ"],
            unit: "ピュ",
        },
    ),
];
