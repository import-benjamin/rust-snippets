/* This file contain a collection of rust example from the official documentation */
/* If, else if, else statement example */
fn if_else_if_and_if() {
	let number = 6;

	if number % 4 == 0 {
		println!("number is divisible by 4");
	} else if number % 3 == 0 {
		println!("number is divisible by 3");
	} else if number % 2 == 0 {
		println!("number is divisible by 2");
	} else {
		println!("number is not divisible by 4, 3, or 2");
	}
}

/* inline statement */
fn inline_condition_statement() {
	let condition = true;
	let number = if condition { 5 } else { 6 };

	println!("The value of number is: {}", number);
}

/* infinite loop */
fn infinite_loop() {
	loop {
		println!("again!");
	}
}

/* break from loop with a return value */
fn finite_loop_with_return_value() {
	let mut counter = 0;

	let result = loop {
		counter += 1;

		if counter == 10 {
			break counter * 2;
		}
	};

	println!("The result is {}", result);
}

/* conditional loop with while */
fn conditional_loop_with_while() {
	let mut number = 3;

	while number != 0 {
		println!("{}!", number);

		number -= 1;
	}

	println!("LIFTOFF!!!");
}

/* for loop with while */
fn for_loop_with_while() {
	let a = [10, 20, 30, 40, 50];
	let mut index = 0;

	while index < 5 {
		println!("the value is: {}", a[index]);

		index += 1;
	}
}

/* for loop */
fn for_loop() {
	let a = [10, 20, 30, 40, 50];

	for element in a.iter() {
		println!("the value is: {}", element);
	}
}
