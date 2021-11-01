use rake::{KeywordScore, Rake, StopWords, Metric};
use assert_approx_eq::assert_approx_eq;

#[test]
fn test_score() {
    let text = "Compatibility of systems of linear constraints over the set of natural numbers. Criteria of compatibility of a system of linear Diophantine equations, strict inequations, and nonstrict inequations are considered. Upper bounds for components of a minimal set of solutions and algorithms of construction of minimal generating sets of solutions for all types of systems are given. These criteria and the corresponding algorithms for constructing a minimal supporting set of solutions can be used in solving all the considered types of systems and systems of mixed types.";
    let sw = StopWords::from_file("tests/SmartStoplist.txt").unwrap();
    let mut r = Rake::new(sw);

    let print = |keywords: &Vec<KeywordScore>| {
        keywords.iter().for_each(
            |&KeywordScore {
                 ref keyword,
                 ref score,
             }| println!("{}: {}", keyword, score),
        );
        println!();
    };

    // default DegreeToFrequencyRatio metric

    let keywords = r.run(text);

    print(&keywords);

    assert_eq!(keywords[0].keyword, "minimal generating sets");
    assert_eq!(keywords[1].keyword, "linear Diophantine equations");
    assert_eq!(keywords[2].keyword, "minimal supporting set");
    assert_eq!(keywords[3].keyword, "minimal set");
    assert_approx_eq!(keywords[0].score, 8.666666666666666);
    assert_approx_eq!(keywords[1].score, 8.5);
    assert_approx_eq!(keywords[2].score, 7.666666666666666);
    assert_approx_eq!(keywords[3].score, 4.666666666666666);

    // WordDegree metric

    r.set_metric(Metric::WordDegree);

    let keywords = r.run(text);

    print(&keywords);

    // score of items 1 and 2 is the same so the order is not guaranteed
    assert_eq!(keywords[0].keyword, "minimal supporting set");
    assert!(keywords[1].keyword == "minimal set" || keywords[1].keyword == "minimal generating sets");
    assert!(keywords[2].keyword == "minimal set" || keywords[2].keyword == "minimal generating sets");
    assert_ne!(keywords[1].keyword, keywords[2].keyword);
    assert_eq!(keywords[3].keyword, "linear Diophantine equations");
    assert_approx_eq!(keywords[0].score, 17.0);
    assert_approx_eq!(keywords[1].score, 14.0);
    assert_approx_eq!(keywords[2].score, 14.0);
    assert_approx_eq!(keywords[3].score, 11.0);

    // WordFrequency metric

    r.set_metric(Metric::WordFrequency);

    let keywords = r.run(text);

    print(&keywords);

    // score of items 2 and 3 is the same so the order is not guaranteed
    assert_eq!(keywords[0].keyword, "minimal supporting set");
    assert_eq!(keywords[1].keyword, "minimal set");
    assert!(keywords[2].keyword == "considered types" || keywords[2].keyword == "minimal generating sets");
    assert!(keywords[3].keyword == "considered types" || keywords[3].keyword == "minimal generating sets");
    assert_ne!(keywords[2].keyword, keywords[3].keyword);
    assert_approx_eq!(keywords[0].score, 7.0);
    assert_approx_eq!(keywords[1].score, 6.0);
    assert_approx_eq!(keywords[2].score, 5.0);
    assert_approx_eq!(keywords[3].score, 5.0);
}
