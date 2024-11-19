use crate::ProbeEnergy;

pub fn transpose(measurements: &Vec<ProbeEnergy>) -> Vec<Vec<f32>> {
    let mut iter_probes: Vec<_> = measurements.iter()
        .map(|probe_energies| probe_energies.values().cloned())
        .collect();

    let n = measurements[0].len();
    (0..n).map(|_| {
        iter_probes.iter_mut()
            .map(|probe_energies| probe_energies.next().unwrap())
            .collect()
    }).collect()
}

pub trait Statistics: Sized {
    fn mean(xs: &Vec<Self>) -> Self;

    fn std(xs: &Vec<Self>) -> Self;
}

impl Statistics for f32 {
    fn mean(xs: &Vec<f32>) -> f32 {
        let n = xs.len() as f32;
        xs.iter().sum::<f32>() / n
    }

    fn std(xs: &Vec<f32>) -> f32 {
        let μ = f32::mean(xs);
        let n = xs.len() as f32;
        (variance(xs, μ) / n).sqrt()
    }
}

impl Statistics for Vec<f32> {
    fn mean(xs: &Vec<Vec<f32>>) -> Vec<f32> {
        xs.iter().map(f32::mean).collect()
    }

    fn std(xs: &Vec<Vec<f32>>) -> Self {
        xs.iter().map(f32::std).collect()
    }
}

fn variance(xs: &Vec<f32>, μ: f32) -> f32 {
    xs.iter().map(|x| (x - μ) * (x - μ)).sum()
}
