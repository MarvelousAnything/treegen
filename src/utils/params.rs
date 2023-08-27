use crate::sampler::Sampler;

pub trait Parameters {
    fn generate_parameters(sampler: &dyn Sampler<SampleType=f64>) -> Self;
}
