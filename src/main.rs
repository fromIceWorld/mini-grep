use regex::Regex;
use std::env;

#[derive(Debug)]
struct MyArgs {
    name: Option<String>,
    age: Option<String>,
    cache: Option<bool>,
    version: Option<String>
}

fn main(){
  let args: Vec<_> = env::args().skip(1).collect();
  let k_regex = Regex::new(r"^--?").unwrap();
  let mut params = MyArgs {
     name: None,
     age: None,
     cache: None,
     version: None,
  };
  let mut args_iter = args.iter().peekable();
  while let Some(arg) = args_iter.next() {
      if k_regex.is_match(arg) {
        let value = args_iter.peek()
                            .map(|s| !k_regex.is_match(s)).unwrap_or(false);
        if value {
          let v = args_iter.next().map(|s| s.to_string());
          match arg.as_str() {
            "--version" | "-v" => params.version = v,
            "--name" | "-n"=> params.name = v,
            "--age" | "-g" => params.age = v,
            _ => println!("无效的参数: {arg}")
          }
        } else {
          println!("期望参数值后跟参数: {arg}");
        }
      }else{
        match arg.as_str() {
          "cache" => params.cache = Some(true),
          _ => println!("无效的参数: {arg}"),
        }
      }
  }
  println!("{:?}", params);
}