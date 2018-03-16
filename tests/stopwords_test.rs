extern crate rake;

use rake::StopWords;

#[test]
fn test_deref() {
    let mut sw = StopWords::new();
    sw.insert("What".to_string());
    sw.insert("which".to_string());
    sw.insert("Who".to_string());
    sw.insert("tree".to_string());

    sw.remove("tree");

    assert_eq!(sw.contains("what"), true);
    assert_eq!(sw.contains("which"), true);
    assert_eq!(sw.contains("who"), true);
    assert_eq!(sw.contains("tree"), false);
}

#[test]
fn test_file() {
    let sw = StopWords::from_file("tests/SmartStoplist.txt").unwrap();

    assert_eq!(sw.contains("what"), true);
    assert_eq!(sw.contains("which"), true);
    assert_eq!(sw.contains("who"), true);
    assert_eq!(sw.contains("tree"), false);
}
