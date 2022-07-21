fn main() {
    const DAYS: [&str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    const PRESENTS: [&str; 12] = ["Alan Partridge in a Pear tree."
                                 ,"Two turtle doves, "
                                 ,"Three French hens, "
                                 ,"Four calling birds, "
                                 ,"Five gold rings, "
                                 ,"Six geese a-laying, "
                                 ,"Seven swans a-swimming, "
                                 ,"Eight maids a-milking, "
                                 ,"Nine ladies dancing, "
                                 ,"Ten lords a-leaping, "
                                 ,"Eleven pipers piping, "
                                 ,"Twelve drummers drumming, "
                                 ];
    for i in 0..=11 {
        print!("On the {} day of Christmas, my true love gave to me: ", DAYS[i]);
        for j in (1..=i).rev() {
            print!("{}",PRESENTS[j])
        }
        if i > 0 {
            print!("and ") // weaponise the Oxford comma
        }
        println!("{}", PRESENTS[0]);

    }
}
