use std::collections::HashMap;
use ordered_float::OrderedFloat;

pub fn mean(data: &[f64]) -> f64 {
    let sum: f64 = data.iter().sum();
    sum / data.len() as f64
}

pub fn median(data: &mut [f64]) -> f64 {
    data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let len = data.len();
    if len % 2 == 0 {
        (data[len / 2 - 1] + data[len / 2]) / 2.0
    } else {
        data[len / 2]
    }
}

pub fn mode(data: &[f64]) -> Option<f64> {
    let mut occurrences = HashMap::new();

    for &value in data {
        *occurrences.entry(OrderedFloat(value)).or_insert(0) += 1;
    }

    let max_occurrences = occurrences.values().cloned().max().unwrap_or(0);
    
    // Return None if no number repeats
    if max_occurrences <= 1 {
        None
    } else {
        occurrences.into_iter()
            .filter(|&(_, count)| count == max_occurrences)
            .map(|(val, _)| val.into_inner())
            .next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(mean(&data), 3.0);
    }

    #[test]
    fn test_median() {
        let mut data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(median(&mut data), 3.0);

        let mut data_even = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        assert_eq!(median(&mut data_even), 3.5);
    }

    #[test]
    fn test_mode() {
        let data = vec![1.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4.0, 5.0];
        assert_eq!(mode(&data), Some(3.0));

        let data_no_mode = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(mode(&data_no_mode), None); // Correct test case for no mode
    }
}
