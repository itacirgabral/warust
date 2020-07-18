// To be a senior, a member must be at least 55 years old and have a handicap greater than 7
fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
  data.iter().map(|(years, handicap)| if years > &54 && handicap > &7 {
    "Senior".to_string()
  } else {
    "Open".to_string()
  }).collect()
}

#[test]
fn returns_expected() {
  assert_eq!(open_or_senior(vec![(45, 12), (55,21), (19, -2), (104, 20)]), vec!["Open", "Senior", "Open", "Senior"]);
  assert_eq!(open_or_senior(vec![(3, 12), (55,1), (91, -2), (54, 23)]), vec!["Open", "Open", "Open", "Open"]);
}