pub fn test() {
  let reply = rprompt::prompt_reply_stdout("Password: ").unwrap();
  println!("Your reply is {}", reply);
}
