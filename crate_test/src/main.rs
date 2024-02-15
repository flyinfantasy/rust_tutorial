fn main() {
    let v = vec![1, 2, 3, 4, 5];

    //let does_not_exist = &v[100];
    let does_not_exist_2 = v.get(100);

    //println!("第100個元素是 {does_not_exist}");
    match does_not_exist_2 {
        Some(does_not_exist_2) => println!("2第100個元素是 {does_not_exist_2}"),
        None => println!("2第100個元素並不存在。"),
    }
    

    let third: &i32 = &v[2];
    println!("第三個元素是 {third}");



    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("第三個元素是 {third}"),
        None => println!("第三個元素並不存在。"),
    }
}