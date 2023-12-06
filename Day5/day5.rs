use std::{fs::read_to_string};

fn convert(val:i64, map:&Vec<(i64,i64,i64)>) -> i64
{
    for i in map
    {
        if val >= i.1 && val <= i.1 + i.2
        {
            return (val - i.1) + i.0;
        }
    }
    return val;
}

fn convert_ranges(val:&Vec<(i64, i64)>, map:&Vec<(i64, i64, i64)>) -> Vec<(i64, i64)>
{
    let mut start_i = 0;

    let mut start:Vec<(i64, i64)> = val.clone();
    let mut out:Vec<(i64, i64)> = vec![];

    for m in map
    {
        let m_start = m.1;
        let m_end = m.1 + m.2;

        loop
        {
            if start_i >= start.len()
            {
                start_i = 0;
                break;
            }

            if start[start_i].0 >= m_start && start[start_i].1 <= m_end
            {
                let tmp = start.remove(start_i);
                out.push((convert(tmp.0, map), convert(tmp.1, map)));
                start_i = 0;
                continue;
            }
            else if start[start_i].0 < m_start && start[start_i].1 > m_end
            {

                let tmp = start.remove(start_i);
                start.insert(start_i, (tmp.0, m_start-1));
                out.push((convert(m_start, map), convert(m_end, map)));
                start.insert(start_i+1, (m_end+1,tmp.1));
                start_i = 0;
                continue;
            }
            else if start[start_i].1 < m_start || start[start_i].0 > m_end
            {
                start_i += 1;
                continue;
            }
            else if start[start_i].0 < m_start && start[start_i].1 >= m_start && start[start_i].1 < m_end
            {
                let tmp = start.remove(start_i);
                start.insert(start_i, (tmp.0, m_start-1));
                out.push((convert(m_start, map), convert(tmp.1, map)));
                start_i = 0;
                continue;
            }
            else if start[start_i].0 > m_start && start[start_i].0 <= m_end && start[start_i].1 > m_end
            {
                let tmp = start.remove(start_i);
                out.push((convert(tmp.0, map), convert(m_end, map)));
                start.insert(start_i, (m_end+1, tmp.1));
                start_i = 0;
                continue;
            }
            else
            {
                println!("I don't know how to deal with this");
            }
        }
    }
    for i in start
    {
        out.push(i);
    }
    return out;
}

fn main() {

    let mut seeds:Vec<i64> = vec![];
    let mut seed_to_soil:Vec<(i64, i64, i64)> = vec![];
    let mut soil_to_fertilizer:Vec<(i64, i64, i64)> = vec![];
    let mut fertilizer_to_water:Vec<(i64, i64, i64)> = vec![];
    let mut water_to_light:Vec<(i64, i64, i64)> = vec![];
    let mut light_to_temperature:Vec<(i64, i64, i64)> = vec![];
    let mut temperature_to_humidity:Vec<(i64, i64, i64)> = vec![];
    let mut humidity_to_location:Vec<(i64, i64, i64)> = vec![];

    let mut state = 0;

    let mut seed_line:String = "".to_string();
    
    for line in read_to_string("input.txt").unwrap().lines()
    {
        if state == 0
        {
            seed_line = line.to_string();
            let s:Vec<String> = line.split(' ').map(str::to_string).collect();
            for i in 1..s.len()
            {
                seeds.push(s[i].parse::<i64>().unwrap());
            }
            state += 1;
        }
        else if line.starts_with(|x:char| x.is_ascii_digit())
        {
            let digits:Vec<String> = line.split(' ').map(str::to_string).collect();
            let start = digits[0].parse::<i64>().unwrap();
            let end = digits[1].parse::<i64>().unwrap();
            let span = digits[2].parse::<i64>().unwrap();

            match state {
            2=>seed_to_soil.push((start, end, span)),
            3=>soil_to_fertilizer.push((start, end, span)),
            4=>fertilizer_to_water.push((start, end, span)),
            5=>water_to_light.push((start, end, span)),
            6=>light_to_temperature.push((start, end, span)),
            7=>temperature_to_humidity.push((start, end, span)),
            8=>humidity_to_location.push((start, end, span)),
            _=>println!("Error, too many states!")
            }
        }
        else if line.len() == 0
        {
            continue;
        }
        else 
        {
            state += 1;
        }
    }

    let mut min_location = -1;

    for seed in seeds
    {
        let soil = convert(seed, &seed_to_soil);
        let fertilizer = convert(soil, &soil_to_fertilizer);
        let water = convert(fertilizer, &fertilizer_to_water);
        let light = convert(water, &water_to_light);
        let temperature = convert(light, &light_to_temperature);
        let humidity = convert(temperature, &temperature_to_humidity);
        let location = convert(humidity, &humidity_to_location);
        if min_location == -1 || min_location > location
        {
            min_location = location;
        }
    }

    println!("Part 1: {}", min_location);

    let mut seed_list:Vec<(i64, i64)> = vec![];
    let seed_line_split:Vec<String> = seed_line.split(' ').map(str::to_string).collect();
    state = 0;
    let mut st = 0;
    let mut sp;
    for val in seed_line_split
    {
        match state {
            0=>state += 1,
            1=>{st = val.parse::<i64>().unwrap(); state += 1},
            2=>{
                sp = val.parse::<i64>().unwrap() + st; 
                seed_list.push((st, sp));
                state -= 1;
            },
            _=>println!("I have no idea how you got here"),
        }
    }
    
    min_location = -1;

    let soil_list = convert_ranges(&seed_list, &seed_to_soil);
    let fertilizer_list: Vec<(i64, i64)> = convert_ranges(&soil_list, &soil_to_fertilizer);
    let water_list = convert_ranges(&fertilizer_list, &fertilizer_to_water);
    let light_list = convert_ranges(&water_list, &water_to_light);
    let temperature_list = convert_ranges(&light_list, &light_to_temperature);
    let humidity_list = convert_ranges(&temperature_list, &temperature_to_humidity);
    let location_list = convert_ranges(&humidity_list, &humidity_to_location);
    
    for loc in location_list
    {
        if loc.0 < min_location || min_location == -1
        {
            min_location = loc.0;
        }
    }

    println!("Part 2: {}", min_location);

}
