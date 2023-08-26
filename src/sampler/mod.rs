use rand_distr::{Distribution, Normal};

pub trait Sampler {
    type SampleType;
    fn sample(&self) -> Self::SampleType;
}

pub trait ParameterizedSampler {
    type Params;
    type SampleType;
    fn sample_with_params(&self, params: &Self::Params) -> Self::SampleType;
}

pub struct UniformSampler<T> where T : Copy {
    value: T,
}

impl<T> Sampler for UniformSampler<T> where T : Copy {
    type SampleType = T;

    fn sample(&self) -> Self::SampleType {
        self.value
    }
}

pub struct GaussianSampler {
    mean: f64,
    std_dev: f64,
}

impl Sampler for GaussianSampler {
    type SampleType = f64;

    fn sample(&self) -> Self::SampleType {
        let normal = Normal::new(self.mean, self.std_dev).unwrap();
        let mut rng = rand::thread_rng();
        normal.sample(&mut rng)
    }
}
