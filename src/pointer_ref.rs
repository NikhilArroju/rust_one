//Reference pointers point to a resource in memory
pub fn run() {
    //primitive array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("{:?}", (arr1, arr2));
    //With non primitive data if you assign a variable to another data
    //then the firstone will no longer hold the value

    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;

    println!("{:?}", (&vec1, vec2));
}
