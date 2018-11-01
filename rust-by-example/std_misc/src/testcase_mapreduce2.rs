/*
cargo run -p std_misc --bin testcase_mapreduce2
*/
use std::thread;


// This is the `main` thread
fn main() {

    // This is our data to process.
    // We will calculate the sum of all digits via a threaded  map-reduce algorithm.
    // Each whitespace separated chunk will be handled in a different thread.
    //
    // TODO: see what happens to the output if you insert spaces!
    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";


    static NTHREADS: u32 = 10;



    /*************************************************************************
     * "Map" phase
     *
     * Divide our data into segments, and apply initial processing
     ************************************************************************/

    // split our data into segments for individual calculation
    // each chunk will be a reference (&str) into the actual data
    let chunked_data = data.split_whitespace();


// collect each thread's intermediate results into a new Vec

    let mut final_result: u32 = 0;

    // Iterate over the data segments.
    // .enumerate() adds the current loop index to whatever is iterated
    // the resulting tuple "(index, element)" is then immediately
    // "destructured" into two variables, "i" and "data_segment" with a
    // "destructuring assignment"

    let mut intermediate_sums = vec![];
    let mut children = vec![];

    let mut current_index: u32;
    let mut iterator = chunked_data.enumerate();
    let chank_len = iterator.count() as u32;
    iterator = data.split_whitespace().enumerate();
    if chank_len < NTHREADS {
        for (i, data_segment) in iterator {
            println!("data segment {} is \"{}\"", i, data_segment);
            children.push(thread::spawn(move
                || -> u32 {
                // Calculate the intermediate sum of this segment:
                let result:u32 = data_segment
                    // iterate over the characters of our segment..
                    .chars()
                    // .. convert text-characters to their number value..
                    .map(|c| c.to_digit(10).expect("should be a digit"))
                    // .. and sum the resulting iterator of numbers
                    .sum();

                // println! locks stdout, so no text-interleaving occurs
                println!("processed segment {}, result={}", i, result);

                // "return" not needed, because Rust is an "expression language", the
                // last evaluated expression in each block is automatically its value.
                result
            }));
        }
        for child in children {
            // collect each child thread's return-value
            let intermediate_sum = child.join().unwrap();
            intermediate_sums.push(intermediate_sum);
        }
        final_result += intermediate_sums.iter().sum::<u32>();
    } else {
        for (i, data_segment) in iterator {
            println!("data segment {} is \"{}\"", i, data_segment);
            current_index = i as u32;

            if current_index == NTHREADS {
                for child in children {
                    // collect each child thread's return-value
                    let intermediate_sum = child.join().unwrap();
                    intermediate_sums.push(intermediate_sum);
                }
                children = vec![];
                final_result += intermediate_sums.iter().sum::<u32>();
                intermediate_sums = vec![];
            }

            children.push(thread::spawn(move
                || -> u32 {
                // Calculate the intermediate sum of this segment:
                let result = data_segment
                    // iterate over the characters of our segment..
                    .chars()
                    // .. convert text-characters to their number value..
                    .map(|c| c.to_digit(10).expect("should be a digit"))
                    // .. and sum the resulting iterator of numbers
                    .sum();

                // println! locks stdout, so no text-interleaving occurs
                println!("processed segment {}, result={}", i, result);

                // "return" not needed, because Rust is an "expression language", the
                // last evaluated expression in each block is automatically its value.
                result
            }));
        }// for

        for child in children {
            // collect each child thread's return-value
            let intermediate_sum = child.join().unwrap();
            intermediate_sums.push(intermediate_sum);
        }

        final_result += intermediate_sums.iter().sum::<u32>();
    }

    println!("Final sum result: {}", final_result);
}//main

