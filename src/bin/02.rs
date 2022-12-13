pub fn part_one(input: &str) -> Option<u32> {
    Some( input.lines().map(|l| -> Vec<i32> {
        l.split(" ").map(|c| -> i32 {
            match c {
                "A" => 1,
                "B" => 2,
                "C" => 3,
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                &_ => todo!()
            }
        }).collect()
    }).map(|v| -> i32 {
            let other = v[0];
            let me = v[1];
            let dif = me-other;
            
            if  dif == 0 {
                3 + me
            } else if dif == 1 || dif == -2 {
                6 + me
            } else {
                me
            }
        }).sum::<i32>() as u32)

}


pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
