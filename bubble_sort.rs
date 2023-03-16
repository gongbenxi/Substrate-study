 
fn bubble_sort<T: PartialOrd + Copy>(list: &mut Vec<T>) -> &Vec<T> {
    for _i in 0..list.len() {
        for x in 0..list.len() - 1 {
            //实际交换次数等于 n-1
            if list[x] > list[x + 1] {
                list.swap(x, x + 1); //元素交换位置
            }
        }
    }
    list
}

fn main() {
 
    let mut list = vec![1, 11,12,22,3,100, 65];
    bubble_sort(&mut list);
    println!("{:?}  ", list);
 
    let mut list = vec!['M', 'b', 'A', 'C', 'a', 'W'];
    bubble_sort(&mut list);
    println!("{:?}  ", list);
}
 
 