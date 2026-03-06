use anyhow::{bail, Result};
use clap::Parser;
use rand::Rng;

#[derive(Parser)]
#[command(name = "dice", about = "NdM 形式でダイスを振る")]
struct Args {
    /// ダイス指定 (例: 2d6, 1d20, 3d8)
    dice: String,
}

fn parse_dice(s: &str) -> Result<(u32, u32)> {
    let s = s.to_lowercase();
    let parts: Vec<&str> = s.split('d').collect();
    if parts.len() != 2 {
        bail!("NdM 形式で指定してください (例: 2d6)");
    }
    let count: u32 = parts[0].parse().unwrap_or(0);
    let sides: u32 = parts[1].parse().unwrap_or(0);
    if count == 0 || sides == 0 {
        bail!("ダイスの個数と面数は1以上で指定してください");
    }
    Ok((count, sides))
}

fn roll(count: u32, sides: u32) -> Vec<u32> {
    let mut rng = rand::rng();
    (0..count).map(|_| rng.random_range(1..=sides)).collect()
}

fn main() -> Result<()> {
    let args = Args::parse();
    let (count, sides) = parse_dice(&args.dice)?;
    let results = roll(count, sides);
    let total: u32 = results.iter().sum();
    println!("🎲 {}: {:?} = {}", args.dice, results, total);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid() {
        assert_eq!(parse_dice("2d6").unwrap(), (2, 6));
        assert_eq!(parse_dice("1d20").unwrap(), (1, 20));
        assert_eq!(parse_dice("3D8").unwrap(), (3, 8));
    }

    #[test]
    fn parse_invalid() {
        assert!(parse_dice("abc").is_err());
        assert!(parse_dice("0d6").is_err());
        assert!(parse_dice("2d0").is_err());
        assert!(parse_dice("d6").is_err());
    }

    #[test]
    fn roll_range() {
        for _ in 0..100 {
            let results = roll(3, 6);
            assert_eq!(results.len(), 3);
            for &r in &results {
                assert!((1..=6).contains(&r));
            }
        }
    }
}
