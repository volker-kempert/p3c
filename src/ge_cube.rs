use genevo::{operator::prelude::*, prelude::*, random::Rng, types::fmt::Display};

const NUMBER_OF_QUEENS: i16 = 16;
const NUM_ROWS: i16 = NUMBER_OF_QUEENS;
const NUM_COLS: i16 = NUMBER_OF_QUEENS;

const NUM_INDIVIDUALS_PER_PARENTS: usize = 3;
const SELECTION_RATIO: f64 = 0.7;
const MUTATION_RATE: f64 = 0.05;
const REINSERTION_RATIO: f64 = 0.7;

/// The phenotype
use super::cube::PrintBox;

/// The genotype
use super::piece::*;

pub type Placement = [Piece; 25];


/// How do the genes of the genotype show up in the phenotype
trait AsPhenotype {
    fn place_pieces(&self) -> PrintBox;
}

impl AsPhenotype for PrintBox {
    fn place_pieces(&self) -> PrintBox {
        let mut b = PrintBox::new();
        for i in 0..25 {
            self[i].add_to_box(&b);
        }
        b
    }
}

/// The fitness function for a filled box.
#[derive(Clone, Debug)]
struct FitnessCalc;

impl FitnessFunction<PrintBox, usize> for FitnessCalc {
    fn fitness_of(&self, b: &PrintBox) -> usize {
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
    fn breeder_mutated(value: Self, other: &Piece, adjustment: f64, sign: i8) -> Self {
        value
    }
}

impl RandomValueMutation for Piece {
    fn random_mutated<R>(value: Self, min_value: &Piece, max_value: &Piece, rng: &mut R) -> Self
    where
        R: Rng + Sized,
    {
        value
    }
}

/// Generate some random cubes with placement
struct CubePacking;

impl GenomeBuilder<Placement> for CubePacking {
    fn build_genome<R>(&self, _: usize, rng: &mut R) -> Placement
    where
        R: Rng + Sized,
    {
        (0..PIECES)
            .map(|name| { Piece::new(name); })
            .collect()
    }
}

pub fn solve_cube(generations: u64, population: usize) {
    let initial_population: Population<CubePacking> = build_population()
        .with_genome_builder(CubePacking)
        .of_size(population)
        .uniform_at_random();

    let mut queens_sim = simulate(
        genetic_algorithm()
            .with_evaluation(FitnessCalc)
            .with_selection(RouletteWheelSelector::new(
                SELECTION_RATIO,
                NUM_INDIVIDUALS_PER_PARENTS,
            ))
            .with_crossover(UniformCrossBreeder::new())
            // .with_mutation(BreederValueMutator::new(
            //     MUTATION_RATE,
            //     Pos { x: 0, y: 1 },
            //     3,
            //     Pos { x: 0, y: 0 },
            //     Pos {
            //         x: NUM_ROWS,
            //         y: NUM_COLS,
            //     },
            // ))
            .with_reinsertion(ElitistReinserter::new(
                FitnessCalc,
                false,
                REINSERTION_RATIO,
            ))
            .with_initial_population(initial_population)
            .build(),
    )
    .until(or(
        FitnessLimit::new(FitnessCalc.highest_possible_fitness()),
        GenerationLimit::new(generations),
    ))
    .build();

    loop {
        let result = queens_sim.step();
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
                for row in best_solution.solution.genome.as_board() {
                    println!("      {:?}", row);
                }
            },
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
                for row in best_solution.solution.genome.as_board() {
                    println!("      {:?}", row);
                }
                break;
            },
            Err(error) => {
                println!("{}", error);
                break;
            },
        }
    }
}
