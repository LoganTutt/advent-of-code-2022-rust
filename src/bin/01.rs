pub fn part_one(input: &str) -> Option<u32> {
    let vec : Vec<Option<u32>> = input.lines().map(|l| -> Option<u32>{
        match l.parse::<u32>() {
            Ok(n) => Some(n),
            Err(_e) => None
        }
    }).collect();
    let splits : Vec<_> = vec.split(|&x| x.is_none()).collect();
    
    splits.iter().map(|&v| -> u32 {
        v.iter().map(|&n| n.unwrap()).sum()
    }).max()
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    //advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
