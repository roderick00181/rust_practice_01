use std::collections::HashMap;
use std::io;

fn main() {
	{
		let mut targets = vec![1, 2, 4, 1];
		let results = ex1(&mut targets);
		for i in &results {
			print!("{i} ");
		}
		println!();
	}
	{
		let results = ex2(&"y̆irst".to_string());
		print!("{results}");
		println!();
	}
	{
		let mut department: Vec<String> = Vec::new();
		department.push("工程部門".to_string());
		department.push("業務部門".to_string());
		
		let mut employee = HashMap::new();
		
		println!("請選擇工作項目");
		loop {
			println!("1.新增員工");
			println!("2.選擇部門顯示員工列表");
			println!("3.顯示公司員工列表");
			println!("4.離開工作列表");
			let mut option = String::new();
			
			io::stdin()
				.read_line(&mut option)
				.expect("讀取失敗");
				
			match option.trim() {
				"1" => {
					let mut count = 1;
					for i in &department {
						println!("{count}.{i}");
						count += 1;
					}
					// println!("{count}.新增部門");
					println!("{count}.離開");
					println!("請選擇部門");
					loop {
						option = String::new();
						
						io::stdin()
							.read_line(&mut option)
							.expect("讀取失敗");
						
						let option: i32 = match option.trim().parse() {
							Ok(num) => num,
							Err(_) => {
								println!("請輸入選項");
								continue;
							}
						};	
						
						if option <= department.len().try_into().unwrap() {
							println!("請輸入員工姓名");
							let mut emp_name = String::new();
							
							io::stdin()
								.read_line(&mut emp_name)
								.expect("讀取失敗");
								
							let emp_name = emp_name.trim();
							
							let index: usize = (option - 1).try_into().unwrap();
							
							let department_name = &department[index].clone();
							
							employee.insert(String::from(emp_name), String::from(department_name));
							
							println!("將{emp_name}加入{}", department_name);
							println!();
							break;
							
						// } else if option == (department.len() + 1).try_into().unwrap() {
						// 	println!("請輸入部門名稱");
						} else if option == (department.len() + 2).try_into().unwrap() {
							break;
						} else {
							println!("請輸入選項");
							continue;
						}
					}
					
				},
				"2" => {
					let mut count = 1;
					for i in &department {
						println!("{count}.{i}");
						count += 1;
					}
					println!("{count}.離開");
					println!("請選擇部門");
					loop {
						option = String::new();
						
						io::stdin()
							.read_line(&mut option)
							.expect("讀取失敗");
						
						let option: i32 = match option.trim().parse() {
							Ok(num) => num,
							Err(_) => {
								println!("請輸入選項");
								continue;
							}
						};	
						
						if option <= department.len().try_into().unwrap() {
							let index: usize = (option - 1).try_into().unwrap();
							println!("{}", department[index]);
							for (key, value) in &employee {
								if value == &department[index] {
								    println!("{key}");
								}
							}
							println!();
							break;
						} else if option == (department.len() + 1).try_into().unwrap() {
							break;
						} else {
							println!("請輸入選項");
							continue;
						}
					}
					
				},
				"3" => {
					let mut all_employee: Vec<(String, String)> = Vec::new();
					for (key, value) in &employee {
						all_employee.push((value.clone(), key.clone()));
						println!("姓名：{key} 部門：{value}");
					}
					println!();
					all_employee.sort_unstable();
					for index in &all_employee {
						println!("姓名：{} 部門：{}", index.1, index.0);
					}
					println!();
				},
				"4" => break,
				_ => {
					println!("請輸入選項");
					continue;
				},
			}
		}
	}
}

fn ex1(nums: &mut Vec<i32>) -> Vec<f64> {
	let mut results = Vec::new();
	nums.sort();
	if nums.len() % 2 == 1 {
		let mid = (nums.len() - 1) / 2;
		results.push(nums[mid].into());
	} else {
		let mid = nums.len() / 2;
		let leftone: f64 =  nums[mid-1].into();
		let rightone: f64 =  nums[mid].into();
		results.push(((leftone + rightone) / 2.0).into());
	}
	
	let mut map: HashMap<i32, i32> = HashMap::new();
	
	for i in nums {
		let count = map.entry(*i).or_insert(0);
		*count += 1;
	}
	let mut mode = 0;
	let mut mode_count = 0;
	for (key, value) in &map {
        if *value > mode_count {
			mode = *key;
			mode_count = *value;
		}
    }
	results.push((mode).into());
	results
}

fn ex2(word: &String) -> String {
	let chars = word.chars();
    let mut results = word.clone();
	for c in chars {
		match c {
			'a' => results.push_str("-hay"),
			'e' => results.push_str("-hay"),
			'i' => results.push_str("-hay"),
			'o' => results.push_str("-hay"),
			'u' => results.push_str("-hay"),
			_ => {
				results = results[c.len_utf8()..].to_string();
				results = format!("{results}-{c}ay");
			},
		};	
		break;
	}
	results
}

fn _ex3() {

}
