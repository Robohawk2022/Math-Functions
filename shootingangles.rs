fn main () {

	// maximum variable for shooter
	let upward = 90;

	// length of half of feild
	let field = 25;

	// should be 3 per foor (3:1)
	let degperfoot = upward / field;

	// distance traveling, should be replaced by input setting (CanSparkMax API)
	// set as 4 for example 
	let distance = 4;

	let angleset = distance * degperfoot;

	let finalreturnvalue = upward - angleset;

	println!("{}", finalreturnvalue);

}
