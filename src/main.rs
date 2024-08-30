mod path_getter;
mod excel_reader;
mod process;


fn main() {
    /*
    Plan:
    1. DONE: Get xlsx path
    2. DONE: Read data
    3. TODO: Process data
                ✓ Convert xlsx to DataFrame
                - Calculate averages of every number column
                - Extract free text answers
                - Create visualizations (box plots, histograms)?
    4. TODO: Create output pdf
                - Find suitable formatting
                - Write file to specified output location (next to original path)
    */

    let path_buf = path_getter::get_path().expect("Couldn't open file.");
    let reader = excel_reader::new(path_buf);
    let df = process::start(reader).unwrap();
    println!("{:#?}", df);
}
