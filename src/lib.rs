pub fn add_two(a:i32)->i32{
      internal_adder(a,2)
}

fn internal_adder(a:i32,b:i32)->i32{
    a+b
}

#[cfg(test)]
mod tests {
   /* #[test]
    fn it_works2() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works()->Result<(),String> {
        if 2+2==4 {
            Ok(())
        }
        else {
            Err(String::from("2+2 is not equal to 4"))
        }
    }*/

   /* use crate::add_two;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4,add_two(2))
    }
    #[test]
    fn add_two_and_three() {
        assert_eq!(5,add_two(3))
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102,add_two(100))
    }*/
 /*   #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    #[ignore]
    fn it_ignored() {
        //let's assume that this test is taking 1 hour to complete that's why we want to ignore it
    }*/

    use crate::internal_adder;

    #[test]
    fn internal(){
        assert_eq!(4,internal_adder(2,2))
    }
}
