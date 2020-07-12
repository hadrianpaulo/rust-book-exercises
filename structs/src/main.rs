fn main() {
    let user1 = User {
        email: String::from("person@example.com"),
        username: String::from("someperson"),
        active: true,
        sign_in_count: 100,
    };

    let mut user2 = User {
        email: String::from("person2@example.com"),
        username: String::from("anotherperson"),
        active: true,
        sign_in_count: 1000,
    };

    user2.email = String::from("anotherperson@example.com");

    let user3 = User {
        email: String::from("someother@example.com"),
        username: String::from("someother"),
        active: user1.active,
        sign_in_count: user2.sign_in_count,
    };

    let user4 = User {
        email: String::from("etc@example.com"),
        username: String::from("etc"),
        ..user3
    };

    let user5 = User {
        username: user4.username,
        email: user3.email,
        ..user1
    };

    let User {
        username: username_user5,
        email: email_user5,
        ..
    } = user5;

    println!("{}, {}", email_user5, username_user5);

    // We can't do this anymore.
    // let User {
    //     username: username_user,
    //     email: email_user,
    //     ..
    // } = user5;
    // Nor this.
    // let username = user5.username;

    let _user6 = build_user(username_user5, email_user5);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
        color: black,
        upper_left: origin,
    };

    println!("Area is {}", rect1.area());
    println!("rect1 coordinates are {:#?}", rect1.calculate_coordinates());
    println!("rect1 is {:#?}", rect1);

    let rect2 = Rectangle {
        width: 20,
        height: 10,
        color: Color(255, 255, 255),
        upper_left: Point(35, 60),
    };
    println!("Can rect1 contain rect2? {:?}", rect1.can_hold(&rect2));
    println!("Does rect1 and rect2 overlap? {:?}", rect1.overlaps(&rect2));
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Color(u8, u8, u8);
#[derive(Debug, Copy, Clone)]
struct Point(i32, i32);

// Structs can't store references without using lifetimes.
// struct UserRef {
//     username: &str,
//     email: &str
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
    color: Color,
    upper_left: Point,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn calculate_coordinates(&self) -> (Point, Point, Point, Point) {
        let x1 = self.upper_left.0;
        let y1 = self.upper_left.1;
        let width = self.width as i32;
        let height = self.height as i32;

        let upper_left = self.upper_left.clone();
        let lower_left = Point(x1, y1 + height);
        let upper_right = Point(x1 + width, y1);
        let lower_right = Point(x1 + width, y1 + height);

        return (upper_left, upper_right, lower_left, lower_right);
    }

    fn overlaps(&self, other: &Rectangle) -> bool {
        let a_width = self.width as i32;
        let a_height = self.height as i32;
        let b_width = other.width as i32;
        let b_height = other.height as i32;

        let (a_x1, a_y1) = (self.upper_left.0, self.upper_left.1);
        let (a_x2, a_y2) = (a_x1 + a_width, a_y1 + a_height);
        let (b_x1, b_y1) = (other.upper_left.0, other.upper_left.1);
        let (b_x2, b_y2) = (b_x1 + b_width, b_y1 + b_height);

        a_x1 < b_x2 && a_x2 > b_x1 && a_y1 < b_y2 && a_y2 > b_y1
    }
}
