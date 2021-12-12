pub trait HasGenome<T: Genome>
where
{
    fn to_genome(&self) -> T;
    fn from_genome(genome: &T) -> Self;
}

pub trait Genome {
    // fn mix_genomes(p1: &G, p2: &G) -> G;
    fn mix(&self, partner: &Self) -> Self;
    // fn to_genome(&self) -> G;
}
