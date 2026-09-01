



pub fn plusminus(mut string: String)->String{

    // let mut string: String = "123E56".to_string();/
    println!("\n ");
    println!("\n\n{}", string);
    
    if let Some(index) = string.find('E'){
        println!("Has E at {} and string length of {}", index, string.len());
        if string.len()>(index+1){
            if string.as_bytes()[index+1]==b'-'{
                string.remove(index+1);
            } else {
                if string.len()==(index+1){
                    string.insert(index+1, '-');
                }
            }
            println!("index: {}", index+1); 
        } else {
            if string.len()==(index+1){
                string.push_str("-");
                println!("index+1 === stringlength, so just append a '-'");
            }
        }
    } else {
        if string.chars().nth(0) == Some('-'){
         let a: String = string.chars().skip(1).collect();
            println!("a = {}",a);
        } else {
            string.insert(0,'-');
        }
        println!("{}", string);
    }
    string
}

fn main() {
    let a =        ["123E56",  "123E", "123E-", "124E-5","1234.56",  "0.001",  "5.0",  "26.29E3" ];
    let expected = ["123E-56", "123E-","123E",  "124E5" ,"-1234.56", "-0.001", "-5.0", "26.29E-3"];
    for i in 0..(a.len()-1) {
        // println!("-------Num: {}", n);
        let output = plusminus(a[i].to_string());
        println!("In: {}, expected: {}  out: {} ", a[i], expected[i], output);
    }
}
    