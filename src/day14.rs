use std::collections::HashMap;

pub fn run_day14(inputs: &String) {
    let day14_1 = day14_1(&inputs);
    println!("Day 1-1: {day14_1}");

    let day14_2 = day14_2(&inputs);
    println!("Day 1-2: {day14_2}");
}


fn day14_1(inputs: &String) -> usize {
    const MAX_STEPS: usize = 10;
    let (template, rules) = parse_input(inputs);

    return get_min_max_difference(MAX_STEPS, &rules, &template);
}

fn day14_2(inputs: &String) -> usize {
    const MAX_STEPS: usize = 40;
    let (template, rules) = parse_input(inputs);

    return get_min_max_difference(MAX_STEPS, &rules, &template);
}

fn parse_input(inputs: &String) -> (String, HashMap<String,(String,String)>) {
    let mut lines = inputs.lines().into_iter();
    let polymer_template = lines.next().unwrap();
    lines.next();
    //Each rule will expand into 2 of the other rules. We add both here as a () to avoid needless string parsing later
    let mut rules: HashMap<String, (String,String)> = HashMap::new();

    for line in lines {
        let split: Vec<&str> = line.split(" -> ").collect();
        let k = split[0].to_string();
        let mut v = split[1].to_string();
        let mut v2 = v.clone();
        
        let k_chars: Vec<char> = k.chars().collect();
        v.insert(0, k_chars[0]);
        v2.push(k_chars[1]);
        rules.insert(k, (v,v2));
    }
    return (polymer_template.to_string(),rules);
}

fn get_min_max_difference(steps: usize, rules: &HashMap<String,(String, String)>, template: &String) -> usize {

    let mut pair_count: HashMap<String, usize> = HashMap::with_capacity(rules.len());
    //get inital pairs
    let mut index = 0;
    while index < template.len() - 1 {
        let pair = &template[index..index+2];
        *pair_count.entry(pair.to_string()).or_insert(0) += 1;
        index += 1;
    }

    //get counts for n steps of each pair
    for _step in 1..=steps {
        let mut new_pairs: HashMap<String,usize> = pair_count.clone();
        //get counts of each new pair
        pair_count.iter()
            .for_each(|x| {
                let pairs = rules.get(x.0).unwrap();               

                //Because maps aren't ordered, the pairs are not processed in the same order each time
                //This only evals to true if a pair has added an amount to a previous pair. 
                //Then we subtract the previous value
                //get rid of old pair
                let current = new_pairs.get(&x.0.to_string()).unwrap();
                if current >= x.1 {
                    *new_pairs.entry(x.0.to_string()).or_default() = *current - x.1;
                }

                //add the new pairs
                *new_pairs.entry(pairs.0.to_string()).or_insert(0) += x.1;
                *new_pairs.entry(pairs.1.to_string()).or_insert(0) += x.1;                
            });
        
        //add new pair counts to total counts
        pair_count = new_pairs;
    }
    
    //get counts of each letter, we only care about the 1st letter of each pair + the last letter of the template
    let mut letter_count: HashMap<char, usize> = HashMap::with_capacity((rules.len() as f64).sqrt() as usize);
    pair_count.iter()
        .for_each(|x| {
            let c = x.0.char_indices().next().unwrap().1;
            *letter_count.entry(c).or_insert(0) += x.1;
        });

    //take last letter of the of the template and add 1 to letter count
    let c = template.char_indices().nth(template.len()-1).unwrap().1;
    *letter_count.entry(c).or_default() += 1;

    //get min/max value of each letter
    let min = letter_count.iter().min_by(|a,b| a.1.cmp(&b.1)).map(|(_k,v)| v).unwrap();
    let max = letter_count.iter().max_by(|a,b| a.1.cmp(&b.1)).map(|(_k,v)| v).unwrap();

    return max - min;
}
