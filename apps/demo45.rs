#[derive(Debug)]
struct Item {
    x: i32
}

fn main() {

    let mut items = vec![Item {x: 1}, Item {x: 2}, Item {x: 3}];
    do_processing(&mut items, |items| {
        dbg!(items);
    });
}

fn do_processing<F>(items: &mut Vec<Item>, progress_closure: F)
    where F: Fn(&Vec<Item>) {
    for i in 0..items.len() {
        let item = &mut items[i];
        item.x += 1;
        progress_closure(items);
    }
}