fn main() {
  // let mut vers1 = String:: from("On the #day day of Christmas, my true love sent to me");
   struct dayOfChristmas {
      day: String,
      gift: String,
      };

   let mut  strophe:Vec<dayOfChristmas>= Vec::new();
   let mut day=dayOfChristmas{day: String:: from("first"), gift: String:: from("A partridge in a pear tree")};
   strophe.push(day);
   day=dayOfChristmas{day: String:: from("second"), gift: String:: from("Two turtle doves")};
   strophe.push(day);
   day=dayOfChristmas{day: String:: from("third"), gift: String:: from("Three French hens")};
   strophe.push(day);
   day=dayOfChristmas{day: String:: from("fourth"), gift: String:: from("Four calling birds")};
   strophe.push(day);
   day=dayOfChristmas{day: String:: from("fifth"), gift: String:: from("Five golden rings")};
   strophe.push(day);
   day=dayOfChristmas{day: String:: from("sixth"), gift: String:: from("Six geese a-laying")};
   strophe.push(day);
   day=dayOfChristmas{day: String:: from("seventh"), gift: String:: from("Seven swans a-swimming")};
   strophe.push(day);
   day=dayOfChristmas{day: String:: from("eighth"), gift: String:: from("Eight maids a-milking")};
   strophe.push(day);
   day=dayOfChristmas{day: String:: from("ninth"), gift: String:: from("Nine ladies dancing")};
   strophe.push(day);
   day=dayOfChristmas{day: String:: from("tenth"), gift: String:: from("Ten lords a-leaping")};
   strophe.push(day);
   day=dayOfChristmas{day: String:: from("eleventh"), gift: String:: from("Eleven pipers piping")};
   strophe.push(day);
   day=dayOfChristmas{day: String:: from("twelfth "), gift: String:: from("Twelve drummers drumming")};
   strophe.push(day);



   //[dayOfChristmas{month: String:: from("first"), gift: String:: from("a partridge in a pear tree")}
     //             ,{ month: String:: from("second"), gift: String:: from("a partridge in a pear tree"}];

  for i in 0..12{
       println!("On the {} day of Christmas, my true love sent to me", &strophe[i].day);
     for j in (0..i).rev(){
        println!("{}",&strophe[j].gift);
     }
   }

}
