use std::io;

fn main() -> io::Result<()> {
    let mut trial_cnt = 0;
    let question: &str = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";

    loop {
        println!("{}", question);
        trial_cnt += 1;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let trimmed = input.trim();

        if trimmed == "The letter e" {
            break;
        }
    }

    println!("Number of trials: {}", trial_cnt);

    Ok(())
}
