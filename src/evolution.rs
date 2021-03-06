use genevo::{operator::prelude::*, population::*, prelude::*, random::Rng, types::fmt::Display};

const NUM_INDIVIDUALS_PER_PARENTS: usize = 3;
const SELECTION_RATIO: f64 = 0.7;
const MUTATION_RATE: f64 = 0.05;
const MUTATION_PRECISION: u8 = 3;
const REINSERTION_RATIO: f64 = 0.7;

use super::piece::*;

/// The phenotype
use super::cube::PrintBox;

/// The genotype
type Placement = Vec<Piece>;

/// How do the genes of the genotype show up in the phenotype
trait AsPhenotype {
    fn as_printbox(&self) -> PrintBox;
}

impl AsPhenotype for Placement {
    fn as_printbox(&self) -> PrintBox {
        let mut b = PrintBox::new();
        for p in self {
            p.add_to_box(&mut b);
        }
        b
    }
}

/// The fitness function for a filled box.
#[derive(Clone, Debug)]
struct Problem;

impl FitnessFunction<Placement, usize> for Problem {
    fn fitness_of(&self, g: &Placement) -> usize {
        let b = g.as_printbox();
        b.occupied_positions()
    }

    fn average(&self, values: &[usize]) -> usize {
        (values.iter().sum::<usize>() as f32 / values.len() as f32 + 0.5).floor() as usize
    }

    fn highest_possible_fitness(&self) -> usize {
        125
    }

    fn lowest_possible_fitness(&self) -> usize {
        0
    }
}

impl BreederValueMutation for Piece {
    fn breeder_mutated(value: Self, _other: &Piece, _adjustment: f64, _sign: i8) -> Self {
        let mut p = value.clone();
        p.next_config();
        while !p.is_config() {
            p.next_config();
        }
        p
    }
}

impl RandomValueMutation for Piece {
    fn random_mutated<R>(value: Self, _min_value: &Piece, _max_value: &Piece, _rng: &mut R) -> Self
    where
        R: Rng + Sized,
    {
        let mut p = value.clone();
        p.next_config();
        while !p.is_config() {
            p.next_config();
        }
        p
    }
}

/// Generate some random cubes with placement
struct CubePacking;

impl GenomeBuilder<Placement> for CubePacking {
    fn build_genome<R>(&self, _: usize, _rng: &mut R) -> Placement
    where
        R: Rng + Sized,
    {
        (0..PIECES)
            .map(|index| {
                let mut p = Piece::new(index);
                while !p.is_config() {
                    p.next_config();
                }
                p
            })
            .collect::<Vec<Piece>>()
    }
}

pub fn solve_cube(generations: u64, population: usize) {
    let initial_population: Population<Placement> = build_population()
        .with_genome_builder(CubePacking)
        .of_size(population)
        .uniform_at_random();

    let mut pack_sim = simulate(
        genetic_algorithm()
            .with_evaluation(Problem)
            .with_selection(RouletteWheelSelector::new(
                SELECTION_RATIO,
                NUM_INDIVIDUALS_PER_PARENTS,
            ))
            .with_crossover(SinglePointCrossBreeder::new())
            .with_mutation(BreederValueMutator::new(
                MUTATION_RATE,
                Piece::new(24), // Mutation Range Genotype::DNA
                MUTATION_PRECISION,
                Piece::new(0),  // Min Value
                Piece::new(24), // Max Value
            ))
            .with_reinsertion(ElitistReinserter::new(Problem, false, REINSERTION_RATIO))
            .with_initial_population(initial_population)
            .build(),
    )
    .until(or(
        FitnessLimit::new(Problem.highest_possible_fitness()),
        GenerationLimit::new(generations),
    ))
    .build();

    loop {
        let result = pack_sim.step();
        match result {
            Ok(SimResult::Intermediate(step)) => {
                let evaluated_population = step.result.evaluated_population;
                let best_solution = step.result.best_solution;
                println!(
                    "Step: generation: {}, average_fitness: {}, \
                     best fitness: {}, duration: {}, processing_time: {}",
                    step.iteration,
                    evaluated_population.average_fitness(),
                    best_solution.solution.fitness,
                    step.duration.fmt(),
                    step.processing_time.fmt()
                );
            }
            Ok(SimResult::Final(step, processing_time, duration, stop_reason)) => {
                let best_solution = step.result.best_solution;
                println!("{}", stop_reason);
                println!(
                    "Final result after {}: generation: {}, \
                     best solution with fitness {} found in generation {}, processing_time: {}",
                    duration.fmt(),
                    step.iteration,
                    best_solution.solution.fitness,
                    best_solution.generation,
                    processing_time.fmt()
                );
                println!(
                    "Final Best: {}",
                    best_solution.solution.genome.as_printbox()
                );
                break;
            }
            Err(error) => {
                println!("{}", error);
                break;
            }
        }
    }
}
