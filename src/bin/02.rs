use std::cmp::Ordering;

advent_of_code::solution!(2);

fn strip_from_report(report: &str, n: i32) -> String {
    if n < 0 {
        return String::new();
    }

    let parts: Vec<&str> = report.split_whitespace().collect();

    let filtered: Vec<&str> = parts
        .iter()
        .enumerate()
        .filter(|&(i, _)| i != (n as usize))
        .map(|(_, &item)| item)
        .collect();

    filtered.join(" ")
}

fn is_report_safe(report: &str, dampen: bool) -> bool {
    let mut iterator = report
        .split_whitespace()
        .filter_map(|s| s.parse::<u32>().ok())
        .enumerate();

    let (_, mut prev) = match iterator.next() {
        Some(x) => x,
        None => return false,
    };

    let mut is_increasing = false;

    for (i, curr) in iterator {
        match curr.cmp(&prev) {
            Ordering::Greater => {
                if i == 1 {
                    is_increasing = true
                }
                if curr - prev > 3 || !is_increasing {
                    if dampen {
                        return is_report_safe(&strip_from_report(report, i as i32), false)
                            || is_report_safe(&strip_from_report(report, i as i32 - 1), false)
                            || is_report_safe(&strip_from_report(report, i as i32 - 2), false);
                    } else {
                        return false;
                    }
                }
            }
            Ordering::Less => {
                if prev - curr > 3 || is_increasing {
                    if dampen {
                        return is_report_safe(&strip_from_report(report, i as i32), false)
                            || is_report_safe(&strip_from_report(report, i as i32 - 1), false)
                            || is_report_safe(&strip_from_report(report, i as i32 - 2), false);
                    } else {
                        return false;
                    }
                }
            }
            Ordering::Equal => {
                if dampen {
                    return is_report_safe(&strip_from_report(report, i as i32), false)
                        || is_report_safe(&strip_from_report(report, i as i32 - 1), false)
                        || is_report_safe(&strip_from_report(report, i as i32 - 2), false);
                } else {
                    return false;
                }
            }
        }

        prev = curr;
    }

    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe_reports: u32 = 0;

    for report in input.split("\n") {
        if is_report_safe(report, false) {
            safe_reports += 1;
        }
    }

    Some(safe_reports)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe_reports: u32 = 0;

    for report in input.split("\n") {
        if is_report_safe(report, true) {
            safe_reports += 1;
        }
    }

    Some(safe_reports)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
