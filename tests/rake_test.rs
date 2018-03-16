extern crate rake;

use rake::{KeywordScore, Rake, StopWords};

#[test]
fn test_score() {
    let text = "Compatibility of systems of linear constraints over the set of natural numbers. Criteria of compatibility of a system of linear Diophantine equations, strict inequations, and nonstrict inequations are considered. Upper bounds for components of a minimal set of solutions and algorithms of construction of minimal generating sets of solutions for all types of systems are given. These criteria and the corresponding algorithms for constructing a minimal supporting set of solutions can be used in solving all the considered types of systems and systems of mixed types.";
    let sw = StopWords::from_file("tests/SmartStoplist.txt").unwrap();
    let r = Rake::new(sw);
    let keywords = r.run(text);

    keywords.iter().for_each(
        |&KeywordScore {
             ref keyword,
             ref score,
         }| println!("{}: {}", keyword, score),
    );

    assert_eq!(keywords[0].keyword, "minimal generating sets");
    assert_eq!(keywords[1].keyword, "linear Diophantine equations");
    assert_eq!(keywords[2].keyword, "minimal supporting set");
    assert_eq!(keywords[3].keyword, "minimal set");
}
