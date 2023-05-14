#[derive(Debug, Clone)]
pub struct Record {
    pub lo: i64,
    pub value: i64,
    pub weight: i64,
    pub take: i8,
}

#[derive(Debug)]
pub enum InstanceType {
    Uncorrelated,
    WeaklyCorrelated,
    StronglyCorrelated,
    InverseStronglyCorrelated,
    AlmostStronglyCorrelated,
    SubsetSum,
    SimilarWeights,
}

#[derive(Debug)]
pub struct DataSet {
    pub title: String,
    pub instance_type: InstanceType,
    pub items_count: i64,
    pub capacity: i64,
    pub optimal_value: i64,
    pub records: Vec<Record>,
}

pub fn parse_entry(entry_data: &str) -> Option<DataSet> {
    let mut lines = entry_data.lines().filter(|line| !line.is_empty());
    let first_line = lines.next();
    if first_line.is_none() {
        return None;
    }
    let title = first_line.unwrap().to_owned();

    let instance_symbol = title
        .split("_")
        .nth(1)
        .unwrap_or("1")
        .parse::<i32>()
        .unwrap_or(1);
    let items_count = lines
        .next()
        .unwrap()
        .split("n")
        .last()
        .unwrap()
        .trim()
        .parse::<i64>()
        .unwrap();
    let capacity = lines
        .next()
        .unwrap()
        .split("c")
        .last()
        .unwrap()
        .trim()
        .parse::<i64>()
        .unwrap();
    let optimal_value = lines
        .next()
        .unwrap()
        .split("z")
        .last()
        .unwrap()
        .trim()
        .parse::<i64>()
        .unwrap();
    let time = lines.next().unwrap();

    let data = lines.collect::<Vec<&str>>();
    let records = data
        .iter()
        .map(|&record| parse_record(record))
        .collect::<Vec<Record>>();

    let data_set = DataSet {
        title,
        records,
        items_count,
        optimal_value,
        capacity,
        instance_type: parse_instance_type(instance_symbol),
    };
    Some(data_set)
}

fn parse_instance_type(instance_symbol: i32) -> InstanceType {
    match instance_symbol {
        1 => InstanceType::Uncorrelated,
        2 => InstanceType::WeaklyCorrelated,
        3 => InstanceType::StronglyCorrelated,
        4 => InstanceType::InverseStronglyCorrelated,
        5 => InstanceType::AlmostStronglyCorrelated,
        6 => InstanceType::SubsetSum,
        9 => InstanceType::SimilarWeights,
        _ => InstanceType::Uncorrelated,
    }
}

fn parse_record(record_data: &str) -> Record {
    let mut content = record_data.split(",").collect::<Vec<&str>>();
    let lo = content[0].parse::<i64>().unwrap();
    let value = content[1].parse::<i64>().unwrap();
    let weight = content[2].parse::<i64>().unwrap();
    let take = content[3].parse::<i8>().unwrap();
    let record = Record {
        lo,
        value,
        weight,
        take,
    };
    record
}
