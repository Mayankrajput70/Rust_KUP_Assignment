/// Details structure.
struct Details {
    name: String,
    roll_no: i32,
    sub_score: i32,
    department: String,
    school: String,
}
/// Score structure.
struct Score {
    hindi: i32,
    english: i32,
    maths: i32,
    science: i32,
}
/// This method find the average of score.
///
/// #Arguments
///
/// get_average type object
///
/// #Return
///
/// Returns average of marks.
fn get_average(average: &Score) -> i32 {
    let avg: i32 = (average.hindi + average.english + average.maths + average.science) / 4;
    avg
}
/// This method compare the marks equal to 35 or not.
///
/// #Arguments
///
/// pass_student type object
///
/// #Return
///
/// Returns value
fn pass_student(marks: &Score) -> [i32; 4] {
    let mut value: [i32; 4] = [marks.hindi, marks.english, marks.maths, marks.science];

    let equal1: i32;
    let equal2: i32;
    let equal3: i32;
    let equal4: i32;
    if value[0] < 35 {
        equal1 = 35 - value[0];
        value[0] = value[0] + equal1;
    }
    if value[1] < 35 {
        equal2 = 35 - value[1];
        value[1] = value[1] + equal2;
    }
    if value[2] < 35 {
        equal3 = 35 - value[2];
        value[2] = value[2] + equal3;
    }
    if value[3] < 35 {
        equal4 = 35 - value[3];
        value[3] = value[3] + equal4;
    }
    return value;
}
/// This main method print the details, marks and average.
///
/// #Arguments
///
/// Compare student_record.
///
/// #Return
///
/// Returns print marks and average successfully.
fn main() {
    let mayank = Details {
        name: String::from("Mayank"),
        roll_no: i32::from(13),
        sub_score: i32::from(49),
        department: String::from("Computer Science"),
        school: String::from("Doon Public School"),
    };
    let shubham = Details {
        name: String::from("Shubham"),
        roll_no: i32::from(14),
        sub_score: i32::from(44),
        department: String::from("Art"),
        school: String::from("Roorkee Public School"),
    };
    println!(
        "Name:{}, roll_no:{}, sub_score:{}, department:{},school:{}",
        mayank.name, mayank.roll_no, mayank.sub_score, mayank.department, mayank.school
    );
    println!(
        "Name:{}, roll_no:{}, sub_score:{}, department:{},school:{}",
        shubham.name, shubham.roll_no, shubham.sub_score, shubham.department, shubham.school
    );

    let mut mayank_marks = Score {
        hindi: 92,
        english: 70,
        maths: 26,
        science: 10,
    };
    let value1 = [
        mayank_marks.hindi,
        mayank_marks.english,
        mayank_marks.maths,
        mayank_marks.science,
    ];

    println!("Mayank Score: {:?}", value1);
    println!("Average of marks: {}", get_average(&mayank_marks));

    let mut shubham_marks = Score {
        hindi: 25,
        english: 68,
        maths: 30,
        science: 55,
    };
    let value2 = [
        shubham_marks.hindi,
        shubham_marks.english,
        shubham_marks.maths,
        shubham_marks.science,
    ];

    println!("shubham Score: {:?}", value2);
    println!("Average of marks: {}", get_average(&shubham_marks));

    println!();
    let position: [i32; 4] = pass_student(&mayank_marks);
    mayank_marks = Score {
        hindi: position[0],
        english: position[1],
        maths: position[2],
        science: position[3],
    };
    println!("Mayank Score after increment: {:?}", position);
    println!(
        "Mayank average after increment : {}",
        get_average(&mayank_marks)
    );

    let position: [i32; 4] = pass_student(&shubham_marks);
    shubham_marks = Score {
        hindi: position[0],
        english: position[1],
        maths: position[2],
        science: position[3],
    };
    println!("Shubham Score after increment: {:?}", position);
    println!(
        "Shubham average after increment : {}",
        get_average(&shubham_marks)
    );

    println!();
    mayank_marks.compare_student(&shubham_marks);
}
/// Compare implement score both student.
impl Score {
    fn compare_student(&self, other: &Score) {
        if self.hindi >= other.hindi {
            println!(
                "Mayank has higher marks in Hindi by: {}",
                self.hindi - other.hindi
            );
        } else {
            println!(
                "Shubham has higher marks in Hindi by: {}",
                other.hindi - self.hindi
            );
        }
        if self.english >= other.english {
            println!(
                "Mayank has higher marks in English by: {}",
                self.english - other.english
            );
        } else {
            println!(
                "Shubham has higher marks in English by: {}",
                other.english - self.english
            );
        }
        if self.maths >= other.maths {
            println!(
                "Mayank has higher marks in Maths by: {}",
                self.maths - other.maths
            );
        } else {
            println!(
                "Shubham has higher marks in Maths by: {}",
                other.maths - self.maths
            );
        }
        if self.science >= other.science {
            println!(
                "Mayank has higher marks in Science by: {}",
                self.science - other.science
            );
        } else {
            println!(
                "Shubham has higher marks in Science by: {}",
                other.science - self.science
            );
        }
    }
}
