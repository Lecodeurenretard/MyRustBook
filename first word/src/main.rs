fn first_word(txt : &String) -> String{
	let mut buffer : String = String::from("");
	for letter in txt.chars(){
		if letter == ' '{
			break;		}
		buffer.push_str(&String::from(letter));
	}
	buffer
}
fn first_word_ref(txt : &String) -> &str{
	for (i, &item) in txt.as_bytes().iter().enumerate() {
		if item == b' ' {
			return &txt[..i];
		}
	}
	&txt[..]
}

fn unit_test(s : &str){	
	println!("'{}'", first_word_ref(&String::from(s)));
}

fn main(){
	unit_test("My words");
	unit_test("I am okay");
	unit_test("single(s) words");
	unit_test("Jojo&a");
	unit_test("I am the son of GOD! I was SENT DOWN by my father...")
}