use std::collections::HashMap;

#[derive(Debug)]
enum Grade {
    Pass,
    Fail,
}

struct Student {
    name: String,
    scores: Vec<u32>,
}

/*
 *

name the type of s, avg, grade, and the return value
s = &Student
avg = u32
grade = Grade
return here is the hashmap with <name, Grade>

why s.name.clone()?
you gotta close s because
     ├╴  cannot move out of `s.name` which is behind a shared reference
if u do s.name, it means you are moving that name value into result, but the original student Student  is still referencing it. So you gotta clone it else Student will have no name

why take &[Student] instead of Vec<Student>?
If it's Vec it means u need to move the whole value in, unless ur ready to lose the whole vec
at the callsite!

where can this panic?
i guess when there are no scores, you'll be dividing by 0
 *
 * */

fn report(students: &[Student]) -> HashMap<String, Grade> {
    let mut result = HashMap::new();
    for s in students {
        let avg = s.scores.iter().sum::<u32>() / s.scores.len() as u32;
        let grade = if avg >= 50 { Grade::Pass } else { Grade::Fail };
        result.insert(s.name.clone(), grade);
    }
    result
}

pub fn exercise_3() {
    let students = vec![Student {
        name: String::from("Zach"),
        scores: vec![32, 90, 64],
    }];
    println!("{:?}", report(&students));
}
