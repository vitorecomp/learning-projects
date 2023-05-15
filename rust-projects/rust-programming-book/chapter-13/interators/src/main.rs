#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn sum_comsumition() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
    // the iterator was consumed so it is not available anymore    
    // assert_eq!(v1_iter.next(), None);
}

fn main() {
    let v1 = vec![1, 2, 3];
    //this is a lazy iterator it will not do anything until it is called
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    print!("====================\n");
    //in case of a map or filter the iterator is consumed bu another iterator
    //is produced allowing for the iteration over the new values
    let v1_map_inter = v1.iter().map(|x| x + 1);
    for val in v1_map_inter {
        println!("Got: {}", val);
    }

    //this also allow for the the usage of the both types together producing the
    //behavior that some times is expected
    let v2 : Vec<i32> = v1.iter().map(|x| x + 1).filter(|x| x % 2 == 0).collect();

    print!("====================\n");
    for val in v2.iter() {
        println!("Got: {}", val);
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}