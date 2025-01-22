use crate::*;

#[derive(Debug)]
pub struct Animal {
    pub(crate) position: na::Point2<f32>,
    pub(crate) rotation: na::Rotation2<f32>,
    pub(crate) speed: f32,
    pub(crate) eye: Eye,
    pub(crate) brain: Brain,
    pub(crate) satiation: usize,
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = Brain::random(rng, &eye);

        Self::new(eye, brain, rng)
    }

    /// "Restores" bird from a chromosome.
    ///
    /// We have to have access to the PRNG in here, because our
    /// chromosomes encode only the brains - and while we restore the
    /// bird, we have to also randomize its position, direction, etc.
    /// (so it's stuff that wouldn't make sense to keep in the genome.)
    pub(crate) fn from_chromosome(chromosome: ga::Chromosome, rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = Brain::from_chromosome(chromosome, &eye);

        Self::new(eye, brain, rng)
    }

    pub(crate) fn as_chromosome(&self) -> ga::Chromosome {
        // We evolve only our birds' brains, but technically there's no
        // reason not to simulate e.g. physical properties such as size.
        //
        // If that was to happen, this function could be adjusted to
        // return a longer chromosome that encodes not only the brain,
        // but also, say, birdie's color.

        self.brain.as_chromosome()
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }

    fn new(eye: Eye, brain: Brain, rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
            eye,
            brain,
            satiation: 0,
        }
    }
}
