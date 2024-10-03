use strsermac::SerializeStruct;

pub trait SerializeStruct {
    fn serialize_struct_inner(&self) -> Vec<(String, String, String)>;
}

pub fn serialize_struct<T: SerializeStruct>(random_struct: T) -> Vec<(String, String, String)> {
    let mut our_datas: Vec<(String, String, String)> = vec![];

    for (field, value, ty) in random_struct.serialize_struct_inner() {
        let new_data = (field, value, ty);

        our_datas.push(new_data);
        //println!("{} : {} : {}", field, value, ty);
    }

    return our_datas
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        #[derive(Debug, strsermac::SerializeStruct)]
        pub struct HelloWorld {
            pub hello1: String,
            pub hello2: i32,
            pub hello3: f64,
            pub hello4: bool
        }

        let new_hello_world = HelloWorld {
            hello1: "Hello Everyone!".to_string(),
            hello2: 23,
            hello3: 23.3456,
            hello4: false
        };

        assert_eq!([("HelloWorld".to_string(), "4".to_string(), "descriptor".to_string(), ), 
                    ("hello1".to_string(), "Hello Everyone!".to_string(), "String".to_string(),), 
                    ("hello2".to_string(), "23".to_string(), "i32".to_string(),),
                    ("hello3".to_string(), "23.3456".to_string(), "f64".to_string(),),
                    ("hello4".to_string(), "false".to_string(), "bool".to_string(),),].to_vec(), 
                    serialize_struct(new_hello_world));

        //println!("{:#?}", serialize_struct(new_hello_world))
    }
}
