struct Student{
    name: String,
    major:String,

}
impl Student{
    fn new(s:String,m:String) -> Student{
        Self{
            name : s,
            major : m,
        }
    }
    fn get_major( &self) -> &String {
        return &self.major
    }
    fn set_major(&mut self, new_major:String){
        self.major = new_major
    }
}

fn main(){
    let mut me = Student::new("Steven".to_string(), "Computer Science".to_string());
    println!("{}",me.get_major());
    me.set_major("Business".to_string());

    println!("Hello my name is {} and my major is {}",me.name, me.major);
    

    
}