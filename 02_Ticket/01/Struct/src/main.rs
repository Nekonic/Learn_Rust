struct User {
    username: String,
    email: String,
    sign_in_count: u32,
    active: bool,
}

struct score{
    math:u32,
    science:u32,
}

impl score {
    fn sum(&self) -> u32{
        self.math + self.science
    }
    fn set_math(&mut self, m:u32){
        self.math = m;
    }
}

fn main() {
    let user1 = User {
        username: "johndoe".into(),
        email: "johndoe@example.com".into(),
        sign_in_count: 1,
        active: true,
    };

    println!("사용자 이름: {}", user1.username);

    let mut user1 = User {
        username: String::from("johndoe"),
        email: String::from("johndoe@example.com"),
        sign_in_count: 1,
        active: true,
    };

    user1.email = String::from("john.doe@newdomain.com");  // 필드 수정
    println!("수정된 이메일: {}", user1.email);

    let mut score1 = score{
        math:87,
        science:93,
    };
    println!("sum = {}",score1.sum());
    score1.set_math(100);
    println!("sum = {}",score1.sum());
}
