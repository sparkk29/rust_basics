fn main(){
    let arr = [1, 2, 3, 4, 5, 6];
    let slice = &arr[1..5];

    borrowing(arr, slice);
}

fn borrowing(arr: [u8;6], slice: &[u8]){
    println!("{:?}", arr);
    println!("{:?}", slice);
}