use std::collections::HashMap;

pub fn run_day14(inputs: &String) {
    let day14_1 = day14_1(&inputs);
    println!("Day 1-1: {day14_1}");

    let day14_2 = day14_2(&inputs);
    println!("Day 1-2: {day14_2}");
}


fn day14_1(inputs: &String) -> usize {
    const MAX_STEPS: usize = 10;
    let (mut template, rules) = parse_input(inputs);
    println!("template: {}", template);
    println!("rules: {:?}", rules);
    let mut step = 0;
    while step < MAX_STEPS {
        template = process_step(&template, &rules);
        step += 1;
    }
    let counts = get_counts(&template);

    let max = counts
        .iter()
        .max_by(|a,b| a.1.cmp(&b.1))
        .map(|(_k,v)| v)
        .unwrap();
    let min = counts
        .iter()
        .min_by(|a,b| a.1.cmp(&b.1))
        .map(|(_k,v)| v)
        .unwrap();
    println!("min: {:?}, max: {:?}", min, max);
    return max - min;
}

fn day14_2(inputs: &String) -> usize {
    const MAX_STEPS: usize = 10;
    let (mut template, rules) = parse_input(inputs);
    let mut step = 0;
    while step < MAX_STEPS {
        println!("step: {}, len: {}", step, template.len());
        template = process_step(&template, &rules);
        step += 1;
    }
    let counts = get_counts(&template);

    let max = counts
        .iter()
        .max_by(|a,b| a.1.cmp(&b.1))
        .map(|(_k,v)| v)
        .unwrap();
    let min = counts
        .iter()
        .min_by(|a,b| a.1.cmp(&b.1))
        .map(|(_k,v)| v)
        .unwrap();
    println!("min: {:?}, max: {:?}", min, max);
    return max - min;
}

fn parse_input(inputs: &String) -> (String, HashMap<&str,String>) {
    let mut lines = inputs.lines().into_iter();
    let polymer_template = lines.next().unwrap();
    lines.next();
    let mut rules: HashMap<&str, String> = HashMap::new();

    for line in lines {
        let split: Vec<&str> = line.split(" -> ").collect();
        let mut v: Vec<char> = split[1].chars().collect();
        let c = split[0].char_indices().next().unwrap();
        v.insert(0, c.1);
        let v = v.iter().map(|c| *c).collect::<String>();
        rules.insert(split[0], v);
    }
    return (polymer_template.to_string(),rules);
}

//WAY TO SLOW...
fn process_step(template: &String, rules: &HashMap<&str,String>) -> String {
    let mut new_template = String::with_capacity(template.len());
    let mut index = 0;

    while index < template.len() - 1 {
        let pair: &str = &template[index..index+2];
        let to_insert = rules.get(pair).unwrap();
        
        new_template.push_str(to_insert.as_str());

        index += 1;
    }
    new_template.push_str(template.get(template.len() -1..).unwrap());
    return new_template.to_string();
}

fn get_counts(template: &String) -> HashMap<char, usize> {
    let mut counts: HashMap<char, usize> = HashMap::new();

    template.chars().into_iter().for_each(|c| *counts.entry(c).or_insert(0) += 1);
    println!("counts: {:?}", counts);
    return counts;
}