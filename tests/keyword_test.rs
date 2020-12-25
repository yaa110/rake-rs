use rake::{KeywordScore, KeywordSort};

#[test]
fn test_sort() {
    let mut keywords = vec![
        KeywordScore {
            keyword: "k1".to_string(),
            score: 0.3,
        },
        KeywordScore {
            keyword: "k2".to_string(),
            score: 0.7,
        },
        KeywordScore {
            keyword: "k3".to_string(),
            score: 0.4,
        },
        KeywordScore {
            keyword: "k4".to_string(),
            score: 0.5,
        },
    ];

    keywords.sort();

    assert_eq!(keywords[0].keyword, "k1");
    assert_eq!(keywords[1].keyword, "k3");
    assert_eq!(keywords[2].keyword, "k4");
    assert_eq!(keywords[3].keyword, "k2");

    keywords.sort_by_score();

    assert_eq!(keywords[0].keyword, "k2");
    assert_eq!(keywords[1].keyword, "k4");
    assert_eq!(keywords[2].keyword, "k3");
    assert_eq!(keywords[3].keyword, "k1");
}

#[test]
fn test_cmp() {
    let k1 = KeywordScore {
        keyword: "k1".to_string(),
        score: 0.3,
    };

    let k2 = KeywordScore {
        keyword: "k2".to_string(),
        score: 0.5,
    };

    let k3 = KeywordScore {
        keyword: "k3".to_string(),
        score: 0.3,
    };

    assert!(k3 == k1);
    assert!(k2 > k1);
    assert!(k3 < k2);
    assert!(k3 != k2);
}
