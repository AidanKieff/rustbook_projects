
pub mod front_of_house {
    
    pub mod hosting {
        
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}

    }


    mod serving {
        fn take_order() {} 

        fn serve_order() {}

        fn take_payment() {}

    }
 
    mod scopee {
        
        pub fn deliver_order() {}

        mod back_of_house {

            fn fix_incorrect_order() {
                cook_order();
                deliver_order();
            
            }
            
            
            fn cook_order() {}
        }
    }
    // pub fn deliver_order() {}
    

    // mod back_of_house {
        
    //     fn fix_incorrect_order() {
    //         cook_order();
    //         deliver_order();

    //     }
        
    //     fn cook_order() {}
    // }
    
}


    





pub fn eat_at_restaurant() {
    front_of_house::hosting::add_to_waitlist();
}
