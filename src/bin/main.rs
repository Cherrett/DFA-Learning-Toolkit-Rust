extern crate dfa_learning_toolkit;

use dfa_learning_toolkit::dfa_learning_toolkit::rpni::rpni;
use dfa_learning_toolkit::dfa_learning_toolkit::dfa;
use std::time::Instant;
use dfa_learning_toolkit::dfa_learning_toolkit::util::new_stats_tracker;
use dfa_learning_toolkit::dfa_learning_toolkit::edsm::exhaustive_edsm;
use tabwriter::TabWriter;
use std::io::Write;

fn main() {
    println!("BenchmarkMergeStates");

    // These are target dfa sizes we will test.
    let dfa_sizes = [16, 32, 64];
    // These are the training set sizes we will test.
    let training_set_sizes = [230, 607, 1521];

    for iterator in 0..dfa_sizes.len() {
        let target_size = dfa_sizes[iterator];
        let training_set_size = training_set_sizes[iterator];

        println!("-------------------------------------------------------------");
        println!("-------------------------------------------------------------");
        println!(
            "BENCHMARK {} (Target: {} states, Training: {} strings",
            iterator + 1,
            target_size,
            training_set_size
        );
        println!("-------------------------------------------------------------");
        println!("-------------------------------------------------------------");

        // Read APTA.
        let apta = dfa::dfa_from_go_json(format!("datasets/TestingAPTAs/{}.json", target_size));

        println!("APTA size: {}", apta.states.len());

        // Perform all the merges.
        let part = apta.to_state_partition();
        let mut snapshot = part.copy();
        let mut total_merges = 0;
        let mut valid_merges = 0;
        let start = Instant::now();

        for i in 0..apta.states.len() as i32 {
            for j in (i + 1)..apta.states.len() as i32 {
                total_merges += 1;
                if snapshot.merge_states(i, j) {
                    valid_merges += 1;
                }

                snapshot.rollback_changes_from(&part);
            }
        }

        let total_time = start.elapsed().as_secs_f64();
        println!("Total merges: {}", total_merges);
        println!("Valid merges: {}", valid_merges);
        println!("Time: {:.4}s", total_time);
        println!("Merges per second: {:.2}\n", total_merges as f64 / total_time);
    }

    println!("\nExhaustive EDSM");

    // Number of iterations.
    let n = 128;
    // Target size.
    let target_size = 32;

    let mut number_of_states = new_stats_tracker();
    let mut durations = new_stats_tracker();
    let mut merges_per_sec = new_stats_tracker();
    let mut merges = new_stats_tracker();
    let mut valid_merges = new_stats_tracker();

    for i in 0..n {
        println!("BENCHMARK {}/{}", i + 1, n);

        // Read APTA from file.
        let apta = dfa::dfa_from_go_json(format!("datasets/Generated Abbadingo/{}/{}.json", target_size, i));

        let (resultant_dfa, merge_data) = exhaustive_edsm(apta);

        number_of_states.add_int(resultant_dfa.states.len() as i64);
        durations.add(merge_data.duration.as_secs_f64());
        merges_per_sec.add(merge_data.attempted_merges_per_sec());
        merges.add_int(merge_data.attempted_merges_count as i64);
        valid_merges.add_int(merge_data.valid_merges_count as i64);
    }

    let mut tw = TabWriter::new(vec![]);
    write!(&mut tw, "\t{}\t{}\t{}\t{}\t\n", "Minimum", "Maximum", "Average", "Standard Dev").unwrap();
    write!(&mut tw, "\t{}\t{}\t{}\t{}\t\n", "------------", "------------", "------------", "------------").unwrap();
    write!(&mut tw, "{}\t{}\t{}\t{}\t{}\t\n", "Number of States", number_of_states.min(), number_of_states.max(), number_of_states.mean(), number_of_states.population_standard_dev()).unwrap();
    write!(&mut tw, "{}\t{:.2}\t{:.2}\t{:.2}\t{:.2}\t\n", "Duration", durations.min(), durations.max(), durations.mean(), durations.population_standard_dev()).unwrap();
    write!(&mut tw, "{}\t{}\t{}\t{}\t{}\t\n", "Merges/s", f64::round(merges_per_sec.min()) as i64, f64::round(merges_per_sec.max()) as i64, f64::round(merges_per_sec.mean()) as i64, f64::round(merges_per_sec.population_standard_dev()) as i64).unwrap();
    write!(&mut tw, "{}\t{}\t{}\t{}\t{}\t\n", "Attempted Merges", merges.min(), merges.max(), merges.mean(), merges.population_standard_dev()).unwrap();
    write!(&mut tw, "{}\t{}\t{}\t{}\t{}\t\n", "Valid Merges", valid_merges.min(), valid_merges.max(), valid_merges.mean(), valid_merges.population_standard_dev()).unwrap();
    tw.flush().unwrap();

    println!("--------------------------------------------------------------------------------------------");
    print!("{}", String::from_utf8(tw.into_inner().unwrap()).unwrap());
    println!("--------------------------------------------------------------------------------------------");

    println!("\nRPNI");

    // Number of iterations.
    let n = 128;
    // Target size.
    let target_size = 32;

    let mut number_of_states = new_stats_tracker();
    let mut durations = new_stats_tracker();
    let mut merges_per_sec = new_stats_tracker();
    let mut merges = new_stats_tracker();
    let mut valid_merges = new_stats_tracker();

    for i in 0..n {
        println!("BENCHMARK {}/{}", i + 1, n);

        // Read APTA from file.
        let apta = dfa::dfa_from_go_json(format!("datasets/Generated Abbadingo/{}/{}.json", target_size, i));

        let (resultant_dfa, merge_data) = rpni(apta);

        number_of_states.add_int(resultant_dfa.states.len() as i64);
        durations.add(merge_data.duration.as_secs_f64());
        merges_per_sec.add(merge_data.attempted_merges_per_sec());
        merges.add_int(merge_data.attempted_merges_count as i64);
        valid_merges.add_int(merge_data.valid_merges_count as i64);
    }

    let mut tw = TabWriter::new(vec![]);
    write!(&mut tw, "\t{}\t{}\t{}\t{}\t\n", "Minimum", "Maximum", "Average", "Standard Dev").unwrap();
    write!(&mut tw, "\t{}\t{}\t{}\t{}\t\n", "------------", "------------", "------------", "------------").unwrap();
    write!(&mut tw, "{}\t{}\t{}\t{}\t{}\t\n", "Number of States", number_of_states.min(), number_of_states.max(), number_of_states.mean(), number_of_states.population_standard_dev()).unwrap();
    write!(&mut tw, "{}\t{:.4}\t{:.4}\t{:.4}\t{:.4}\t\n", "Duration", durations.min(), durations.max(), durations.mean(), durations.population_standard_dev()).unwrap();
    write!(&mut tw, "{}\t{}\t{}\t{}\t{}\t\n", "Merges/s", f64::round(merges_per_sec.min()) as i64, f64::round(merges_per_sec.max()) as i64, f64::round(merges_per_sec.mean()) as i64, f64::round(merges_per_sec.population_standard_dev()) as i64).unwrap();
    write!(&mut tw, "{}\t{}\t{}\t{}\t{}\t\n", "Attempted Merges", merges.min(), merges.max(), merges.mean(), merges.population_standard_dev()).unwrap();
    write!(&mut tw, "{}\t{}\t{}\t{}\t{}\t\n", "Valid Merges", valid_merges.min(), valid_merges.max(), valid_merges.mean(), valid_merges.population_standard_dev()).unwrap();
    tw.flush().unwrap();

    println!("--------------------------------------------------------------------------------------------");
    print!("{}", String::from_utf8(tw.into_inner().unwrap()).unwrap());
    println!("--------------------------------------------------------------------------------------------");
}
