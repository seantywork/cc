

// basic var

fn multiply_by_two(x: i64) -> i64{

    let mut result = 0;

    result = x * 2;

    return result;

}

fn minus_one(x: &mut i64) {

    *x -= 1;

}


fn basic_var(){


    let mut tryme = 0;

    for i in 0..11 {

        if i % 2 == 0 {

            tryme += i;

        } else {

            continue;

        }
    }

    println!("before: {}", tryme);

    tryme = multiply_by_two(tryme);

    println!("multplied: {}", tryme);

    minus_one(&mut tryme);

    println!("tryme: {}", tryme);

}


// basic slice

fn sum_el(arr: &mut [i64]) -> i64 {

    let mut ans = 0;

    let arrlen = arr.len();

    for i in 0..arrlen{

        ans += arr[i];
    }


    return ans;
}


fn change_el_at(arr: &mut [i64], i: usize) -> i32{


    let arrlen = arr.len() as i32;


    if arrlen == 0 {

        return -1;
    }

    if i as i32 >= arrlen {

        return -2;

    }

    arr[i] = arr[i] - 5;

    return 0;

}

fn sum_vec_el(vec: &mut Vec<i64>) -> i64{


    let mut ans: i64 = 0;

    let veclen = vec.len();

    for i in 0..veclen{

        ans += vec[i];
    }


    return ans;

}


fn change_vec_el_at(vec: &mut Vec<i64>, i: usize) -> i32 {



    let veclen = vec.len() as i32;


    if veclen == 0 {

        return -1;
    }

    if i as i32 >= veclen {

        return -2;

    }

    vec[i] = vec[i] - 500;

    return 0;
}


fn basic_slice(){

    let arr: &mut [i64] = &mut [5, 10, 15, 20];


    let vec = &mut Vec::<i64>::new();

    vec.push(100);
    vec.push(200);
    vec.push(300);



    let mut sum = sum_el(arr);

    println!("sum el: {}", sum);

    let mut res = change_el_at(arr, 6);

    println!("change res: {}", res);

    res = change_el_at(arr, 2);

    println!("change res: {}", res);

    sum = sum_el(arr);

    println!("changed sum: {}", sum);

    sum = sum_vec_el(vec);

    println!("vec sum el: {}", sum);

    res = change_vec_el_at(vec, 6);

    println!("change vec res: {}", res);

    res = change_vec_el_at(vec, 2);

    println!("change vec res: {}", res);

    sum = sum_vec_el(vec);

    println!("changed vec sum el: {}", sum);


}


// basic string



fn append_two_string(a: &mut String, b: &mut String) -> String{

    let mut retstr = String::new();

    retstr += a;

    retstr += b;

    return retstr;

}


fn basic_string(){


    let a = &mut "hello ".to_string();

    let b = &mut "you".to_string();

    let c = append_two_string(a, b);

    println!("basic string: {}", c);


}


// basic struct


struct Whatever<'a> {

    name: String,
    numbers: &'a mut Vec<i64>

}

impl Whatever<'_> {


    fn make<'a>(name: &'a String, numbers: &'a mut [i64]) -> Whatever<'a> {

        let mut w = Whatever{
            name: "".to_string(),
            numbers:  &mut Vec::<i64>::new()
        };

        let numlen = numbers.len();

        w.name = name.to_string();

        for i in 0..numlen{

            w.numbers.push(numbers[i]);

        }

        return w

    }

}

fn edit_whatever(x: &mut Whatever){


    x.numbers.push(1);

    x.numbers.push(2);

    x.numbers.push(3);


}

fn show_whatever(x: &mut Whatever){


    println!("name: {}", x.name);

    println!("numbers: {:?}", x.numbers);

}


fn basic_struct(){


    let mut w = Whatever::make(&"idiot".to_string(), &mut [10,20,30,40]);

    edit_whatever(&w);

    show_whatever(&w);

}


fn main(){

    // basic_var();

    // basic_slice();

    // basic_string();

    basic_struct();
}