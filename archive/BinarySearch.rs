macro_rules! read {
    ($out:ident as i32) => {
        let mut input_text = String::new();
        std::io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");
        let $out = input_text
            .trim()
            .parse::<i32>()
            .expect("Value inputed it not a integer");
    };
}

fn main() {
    binarysort();
}
fn binarysort() {
    const N:usize = 5 ;
    let mut array: [i32; N] = [0; N];
    println!("Enter array elements:");
    for i in 0..N {
        read!(x as i32);
        array[i] = x;
    }
    println!("Array complete");
    let (mut ll, mut ul) = (0, array.len() as i32);
    let mut mid = (ll + ul) / 2;
    read!(qu as i32);
    let mut flag = true;
    for _i in 0..N {
        if qu == array[mid as usize] {
            println!("element found at index {mid}");
            flag = false;
            break;
        } else if qu > array[mid as usize] {
            ll = mid as i32;
        } else if qu < array[mid as usize] {
            ul = mid as i32;
        }
        mid = (ll + ul) / 2;
    }
    if flag {
        println!("No Element Found");
    }
}
