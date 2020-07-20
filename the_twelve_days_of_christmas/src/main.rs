fn main() {
    const INTRO: &str = "My true love gave to me";
    const OUTRO: &str = "A partridge in a pear tree";
    

    let sentences_of_days = ["Two turtle doves", "Three French hens", "Four calling birds", 
                             "Five gold rings", "Six geese a laying", "Seven swans a swimming", 
                             "Eight maids a milking", "Nine ladies dancing", "Ten lords a leaping",
                             "Eleven pipers piping", "12 drummers drumming"];

    println!("On the first day of Christmas");
    println!("{}", INTRO);
    println!("{}", OUTRO);
    println!("");

    let mut count: u8 = 1;
    
    for day_sentence in sentences_of_days.iter() {
        
        match count {
            1 =>  println!("On the second day of Christmas"),
            2 =>  println!("On the thirth day of Christmas"),
            3 =>  println!("On the fourth day of Christmas"),
            4 =>  println!("On the five day of Christmas"),
            5 =>  println!("On the sixth day of Christmas"),
            6 =>  println!("On the seventh day of Christmas"),
            7 =>  println!("On the eighth day of Christmas"),
            8 =>  println!("On the ninth day of Christmas"),
            9 =>  println!("On the tenth day of Christmas"),
            10 =>  println!("On the eleventh day of Christmas"),
            11 =>  println!("On the twelfth day of Christmas"),
            _ => ()
        }

        println!("{}", INTRO);
        println!("{}", day_sentence);
        
        let mut revert_count : usize = (count - 1) as usize;

        while revert_count > 1 {
            println!("{}", sentences_of_days[revert_count - 1]);
            revert_count = revert_count - 1;
        }

        println!("{}", OUTRO);
        println!("");
        
        count = count + 1;
    }                         
}
