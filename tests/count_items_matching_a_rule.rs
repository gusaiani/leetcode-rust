use leetcode::count_items_matching_a_rule::count_matches;

fn sample() -> Vec<Vec<String>> {
    vec![
        vec!["phone".into(), "blue".into(), "pixel".into()],
        vec!["computer".into(), "silver".into(), "lenovo".into()],
        vec!["phone".into(), "gold".into(), "iphone".into()],
    ]
}

#[test]
fn example_1() {
    assert_eq!(count_matches(sample(), "color".into(), "silver".into()), 1);
}

#[test]
fn example_2() {
    let items = vec![
        vec!["phone".into(), "blue".into(), "pixel".into()],
        vec!["computer".into(), "silver".into(), "phone".into()],
        vec!["phone".into(), "gold".into(), "iphone".into()],
    ];
    assert_eq!(count_matches(items, "type".into(), "phone".into()), 2);
}

#[test]
fn no_matches() {
    assert_eq!(count_matches(sample(), "name".into(), "nokia".into()), 0);
}

#[test]
fn match_by_name() {
    assert_eq!(count_matches(sample(), "name".into(), "pixel".into()), 1);
}

#[test]
fn all_match_by_type() {
    let items = vec![
        vec!["phone".into(), "blue".into(), "pixel".into()],
        vec!["phone".into(), "gold".into(), "iphone".into()],
    ];
    assert_eq!(count_matches(items, "type".into(), "phone".into()), 2);
}
