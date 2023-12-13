use std::{fs::read_to_string, collections::HashMap};

fn eat(pattern:Vec<char>, filt:Vec<i32>, dictionary:&mut HashMap<(Vec<char>, Vec<i32>), i64>) -> i64
{
    let hash_key = (pattern.clone(), filt.clone());

    if dictionary.contains_key(&hash_key)
    {
        return *dictionary.get(&hash_key).unwrap();
    }

    if filt.len() == 0
    {
        if pattern.contains(&'#')
        {
            dictionary.insert(hash_key, 0);
            return 0;
        }
        else
        {
            dictionary.insert(hash_key, 1);
            return 1;
        }
    }

    if pattern.len() == 0 && filt.len() > 0
    {
        dictionary.insert(hash_key, 1);
        return 0;
    }

    let mut hash_count = 0;
    for i in 0..pattern.len()
    {
        if pattern[i] == '#'
        {
            hash_count += 1;
            if hash_count > *filt.first().unwrap()
            {
                dictionary.insert(hash_key, 0);
                return 0;
            }
        }
        if pattern[i] == '.'
        {
            if hash_count != 0
            {
                if hash_count != *filt.first().unwrap()
                {
                    dictionary.insert(hash_key, 0);
                    return 0;
                }
                else
                {
                    let ret = eat(pattern[i..].to_vec().clone(), filt[1..].to_vec().clone(), dictionary);
                    dictionary.insert(hash_key, ret);
                    return ret;
                }
            }
        }
        if pattern[i] == '?'
        {
            let mut new_str1 = pattern.clone();
            let mut new_str2 = pattern.clone();
            new_str1[i] = '.';
            new_str2[i] = '#';
            let ret = eat(new_str1, filt.clone(), dictionary) + eat(new_str2, filt.clone(), dictionary);
            dictionary.insert(hash_key, ret);
            return ret;
        }
    }

    if hash_count == *filt.first().unwrap() && filt.len() == 1
    {
        dictionary.insert(hash_key, 1);
        return 1;
    }

    dictionary.insert(hash_key, 0);
    return 0;
}

fn count(pattern:Vec<char>, filt:&Vec<i32>) -> i64
{
    let mut contig = 0;
    let mut contig_index = 0;

    for i in 0..pattern.len()
    {
        if pattern[i] == '?'
        {
            let mut newstr_1 = pattern.clone();
            let mut newstr_2 = pattern.clone();

            newstr_1[i] = '.';
            newstr_2[i] = '#';
            
            return count(newstr_1, filt) + count(newstr_2, filt);
        }
        else if pattern[i] == '.'
        {
            if contig != 0
            {
                if filt.len() <= contig_index
                {
                    return 0;
                }
                if filt[contig_index] != contig
                {
                    return 0;
                }
                contig_index += 1;
                contig = 0;
            }
        }
        else
        {
            contig += 1;
        }
    }

    if *pattern.last().unwrap() == '#'
    {
        if contig == 0
        {
            contig = 1;
        }

        if filt.len() <= contig_index
        {
            return 0;
        }
        if filt[contig_index] != contig
        {
            return 0;
        }
        contig_index += 1;
    }

    if contig_index != filt.len()
    {
        return 0;
    }
    return 1;
}

fn main() {

    let mut tot = 0;

    for line in read_to_string("input.txt").unwrap().lines()
    {
        let sub_line:Vec<String> = line.split(' ').map(str::to_string).collect();
        let pattern:Vec<char> = sub_line[0].chars().collect();
        let filt = sub_line[1].split(',').map(|x| x.parse::<i32>().unwrap()).collect();

        let sub_count =  count(pattern, &filt);
        tot += sub_count;
    }

    println!("Part 1: {}", tot);

    let mut dictionary = HashMap::new();

    tot = 0;
    for line in read_to_string("input.txt").unwrap().lines()
    {
        let sub_line:Vec<String> = line.split(' ').map(str::to_string).collect();
        let pattern:Vec<char> = sub_line[0].chars().collect();
        let filt:Vec<i32> = sub_line[1].split(',').map(|x| x.parse::<i32>().unwrap()).collect();

        let mut big_pattern = pattern.clone();
        
        big_pattern.push('?');
        big_pattern = big_pattern.repeat(5);
        big_pattern = big_pattern[0..big_pattern.len()-1].to_vec();

        let sub_count =  eat(big_pattern, filt.repeat(5), &mut dictionary);
        tot += sub_count;
    }

    println!("Part 2: {}", tot);
}