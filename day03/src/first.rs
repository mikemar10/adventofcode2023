use day03::*;
fn main() {
    let input = include_str!("../input.txt");
    let schematic = EngineSchematic::new(input);
    let sum = schematic.find_engine_parts().iter().sum::<usize>();
    println!("Answer: {}", sum);
}
