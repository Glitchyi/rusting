//Stacks
const MAX_SISE:i32 =10;
fn main() {
    let mut x:[i32;100]=[0;100];
    let mut top=-1;
    push(&mut x, &mut top, 1);
    push(&mut x, &mut top, 2);
    push(&mut x, &mut top, 3);
    push(&mut x, &mut top, 4);
    push(&mut x, &mut top, 5);
    pop(x, &mut top);
    traverse(x, top);
    pop(x, &mut top);
    traverse(x, top);
    pop(x, &mut top);
    traverse(x, top);
    pop(x, &mut top);
    traverse(x, top);
}
fn push(i: &mut[i32;100],top:&mut i32,value:i32){
    if *top==MAX_SISE {
        return;
    }else{
        *top+=1;
        i[*top as usize]=value;
        println!("Pushed value {value}");
        traverse(*i, *top);
    }
}
fn pop(i:[i32;100],top: &mut i32){
    if *top>=0{
        println!("Poped value {}",i[*top as usize]);
        *top-=1;
    }
}
fn traverse(i:[i32;100],top:i32){
    for j in 0..top+1{
        print!("{}\t",i[j as usize]);
    }
    print!("\n")
}