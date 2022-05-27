mod math;

fn main(){
    println!("Ciao");


    let a = 2;

    let b = 5;

    let res_sum = math::add_two_nums(a, b);
    let res_sub = math::subtract_two_nums(a, b);

    println!("{:?}", res_sum);
    println!("{:?}", res_sub);
}
