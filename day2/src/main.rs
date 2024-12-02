#[derive(Debug)]
struct Report {
    entries: Vec<i32>,
}

impl Report {
    fn is_safe(&self) -> bool {
        let is_increasing = self.entries[0] < self.entries[1];

        for i in 0..(self.entries.len() - 1) {
            if (self.entries[i] < self.entries[i + 1]) != is_increasing {
                return false;
            }

            let diff = (self.entries[i] - self.entries[i + 1]).abs();

            if diff < 1 || diff > 3 {
                return false;
            }
        }

        true
    }

    fn without(&self, index: usize) -> Report {
        let entries = self.entries.iter()
            .enumerate()
            .filter(|(i, _)| *i != index)
            .map(|(_, entry)| *entry)
            .collect();

        Report { entries }
    }
}

fn main() {
    let file = include_str!("input.txt");

    let reports = file.lines()
        .map(|line| {
            let entries = line.split_whitespace()
                .map(|entry| entry.parse().unwrap())
                .collect();
            Report { entries }
        })
        .collect::<Vec<_>>();

    let safe_reports = reports.iter()
        .filter(|report| report.is_safe())
        .count();

    println!("PART 1: {}", safe_reports);

    let safe_reports_tolerant = reports.iter()
        .filter(| report| {
            if report.is_safe() {
                return true;
            }

            for i in 0..report.entries.len() {
                let new_report = report.without(i);

                if new_report.is_safe() {
                    return true;
                }
            }

            false
        })
        .count();

    println!("PART 2: {}", safe_reports_tolerant);
}
