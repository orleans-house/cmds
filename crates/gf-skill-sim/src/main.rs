use anyhow::{bail, Result};
use clap::Parser;

#[derive(Parser)]
#[command(
    name = "gf-skill-sim",
    about = "GuitarFreaks 単曲スキル目標達成シミュレーター"
)]
struct Args {
    /// 譜面レベル (1.00-9.99)
    #[arg(short, long)]
    level: f64,

    /// 目標スキルポイント
    #[arg(short, long)]
    target: f64,

    /// Perfect数
    #[arg(short, long)]
    perfect: u32,

    /// Great数
    #[arg(short = 'g', long)]
    great: u32,

    /// Good数
    #[arg(long)]
    good: u32,

    /// Miss数
    #[arg(short, long)]
    miss: u32,

    /// 総ノーツ数
    #[arg(short, long)]
    notes: u32,

    /// MaxCombo数
    #[arg(short, long)]
    combo: u32,

    /// 成功フレーズ数
    #[arg(long)]
    phrase_success: u32,

    /// 総フレーズ数
    #[arg(long)]
    phrase_total: u32,
}

fn calc_achievement(
    perfect: u32,
    great: u32,
    notes: u32,
    combo: u32,
    phrase_success: u32,
    phrase_total: u32,
) -> f64 {
    let perfect_rate = if notes == 0 {
        0.0
    } else {
        (perfect as f64 * 85.0 + great as f64 * 25.0) / notes as f64
    };

    let combo_rate = if notes == 0 {
        0.0
    } else {
        combo as f64 / notes as f64 * 5.0
    };

    let phrase_rate = if phrase_total == 0 {
        0.0
    } else {
        phrase_success as f64 / phrase_total as f64 * 10.0
    };

    perfect_rate + combo_rate + phrase_rate
}

fn calc_skill(level: f64, achievement: f64) -> f64 {
    // 小数点第三位以下切り捨て
    (level * 20.0 * achievement / 100.0 * 100.0).floor() / 100.0
}

fn status_mark(skill: f64, target: f64) -> &'static str {
    if skill >= target {
        "OK"
    } else {
        "--"
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    // バリデーション
    if args.level < 1.0 || args.level > 9.99 {
        bail!("譜面レベルは 1.00〜9.99 の範囲で指定してください");
    }
    let judged = args.perfect + args.great + args.good + args.miss;
    if judged > args.notes {
        bail!(
            "判定数の合計({})が総ノーツ数({})を超えています",
            judged,
            args.notes
        );
    }
    if args.combo > args.notes {
        bail!("MaxCombo数が総ノーツ数を超えています");
    }
    if args.phrase_success > args.phrase_total {
        bail!("成功フレーズ数が総フレーズ数を超えています");
    }

    // 現在の状態
    let current_achievement = calc_achievement(
        args.perfect,
        args.great,
        args.notes,
        args.combo,
        args.phrase_success,
        args.phrase_total,
    );
    let current_skill = calc_skill(args.level, current_achievement);
    let required_achievement = args.target / (args.level * 20.0) * 100.0;

    println!("=== 現在の状態 ===");
    println!("譜面レベル: {:.2}", args.level);
    let perfect_rate_val = if args.notes == 0 {
        0.0
    } else {
        (args.perfect as f64 * 85.0 + args.great as f64 * 25.0) / args.notes as f64
    };
    let combo_rate_val = if args.notes == 0 {
        0.0
    } else {
        args.combo as f64 / args.notes as f64 * 5.0
    };
    let phrase_rate_val = if args.phrase_total == 0 {
        0.0
    } else {
        args.phrase_success as f64 / args.phrase_total as f64 * 10.0
    };
    println!(
        "達成率: {:.2}% (Perfect率: {:.2}% / Combo率: {:.2}% / Phrase率: {:.2}%)",
        current_achievement, perfect_rate_val, combo_rate_val, phrase_rate_val
    );
    println!("スキルポイント: {:.2}", current_skill);
    println!();
    println!(
        "=== 目標: {:.2} (必要達成率: {:.2}%) ===",
        args.target, required_achievement
    );
    println!();

    if current_skill >= args.target {
        println!("既に目標を達成しています!");
        return Ok(());
    }

    let max_achievement = calc_achievement(
        args.perfect + args.great + args.good + args.miss,
        0,
        args.notes,
        args.notes,
        args.phrase_total,
        args.phrase_total,
    );
    let max_skill = calc_skill(args.level, max_achievement);
    if max_skill < args.target {
        println!(
            "全Perfect + フルコンボ + 全フレーズ成功でもスキル {:.2} のため、目標に届きません。",
            max_skill
        );
        return Ok(());
    }

    let mut scenarios: Vec<(String, f64, f64)> = Vec::new();

    // シナリオ1: Great→Perfect 変換
    if args.great > 0 {
        let mut needed = None;
        for i in 1..=args.great {
            let new_perfect = args.perfect + i;
            let new_great = args.great - i;
            let ach = calc_achievement(
                new_perfect,
                new_great,
                args.notes,
                args.combo,
                args.phrase_success,
                args.phrase_total,
            );
            let skill = calc_skill(args.level, ach);
            if skill >= args.target {
                needed = Some((i, ach, skill));
                break;
            }
        }
        if let Some((count, ach, skill)) = needed {
            scenarios.push((
                format!("Great->Perfectをあと{}個", count),
                ach,
                skill,
            ));
        } else {
            // Great全変換でも届かない場合
            let ach = calc_achievement(
                args.perfect + args.great,
                0,
                args.notes,
                args.combo,
                args.phrase_success,
                args.phrase_total,
            );
            let skill = calc_skill(args.level, ach);
            scenarios.push((
                format!("Great全Perfect化({}個)", args.great),
                ach,
                skill,
            ));
        }
    }

    // シナリオ2: フルコンボ
    {
        let ach = calc_achievement(
            args.perfect,
            args.great,
            args.notes,
            args.notes,
            args.phrase_success,
            args.phrase_total,
        );
        let skill = calc_skill(args.level, ach);
        if args.combo < args.notes {
            scenarios.push(("フルコンボ達成".to_string(), ach, skill));
        }
    }

    // シナリオ3: フレーズ全成功
    if args.phrase_success < args.phrase_total {
        let ach = calc_achievement(
            args.perfect,
            args.great,
            args.notes,
            args.combo,
            args.phrase_total,
            args.phrase_total,
        );
        let skill = calc_skill(args.level, ach);
        scenarios.push(("フレーズ全成功".to_string(), ach, skill));
    }

    // シナリオ4: Good/Miss→Perfect 変換
    let recoverable = args.good + args.miss;
    if recoverable > 0 {
        let mut needed = None;
        for i in 1..=recoverable {
            // Good から先に変換、次に Miss
            let new_perfect = args.perfect + i;
            let ach = calc_achievement(
                new_perfect,
                args.great,
                args.notes,
                args.combo,
                args.phrase_success,
                args.phrase_total,
            );
            let skill = calc_skill(args.level, ach);
            if skill >= args.target {
                needed = Some((i, ach, skill));
                break;
            }
        }
        if let Some((count, ach, skill)) = needed {
            scenarios.push((
                format!("Good/Miss->Perfectをあと{}個", count),
                ach,
                skill,
            ));
        }
    }

    // シナリオ5: Great→Perfect + フルコンボ
    if args.great > 0 && args.combo < args.notes {
        let mut needed = None;
        for i in 1..=args.great {
            let new_perfect = args.perfect + i;
            let new_great = args.great - i;
            let ach = calc_achievement(
                new_perfect,
                new_great,
                args.notes,
                args.notes,
                args.phrase_success,
                args.phrase_total,
            );
            let skill = calc_skill(args.level, ach);
            if skill >= args.target {
                needed = Some((i, ach, skill));
                break;
            }
        }
        if let Some((count, ach, skill)) = needed {
            scenarios.push((
                format!("Great->Perfectをあと{}個 + フルコンボ", count),
                ach,
                skill,
            ));
        }
    }

    // シナリオ6: Great→Perfect + フレーズ全成功
    if args.great > 0 && args.phrase_success < args.phrase_total {
        let mut needed = None;
        for i in 1..=args.great {
            let new_perfect = args.perfect + i;
            let new_great = args.great - i;
            let ach = calc_achievement(
                new_perfect,
                new_great,
                args.notes,
                args.combo,
                args.phrase_total,
                args.phrase_total,
            );
            let skill = calc_skill(args.level, ach);
            if skill >= args.target {
                needed = Some((i, ach, skill));
                break;
            }
        }
        if let Some((count, ach, skill)) = needed {
            scenarios.push((
                format!("Great->Perfectをあと{}個 + フレーズ全成功", count),
                ach,
                skill,
            ));
        }
    }

    // シナリオ7: フルコンボ + フレーズ全成功
    if args.combo < args.notes && args.phrase_success < args.phrase_total {
        let ach = calc_achievement(
            args.perfect,
            args.great,
            args.notes,
            args.notes,
            args.phrase_total,
            args.phrase_total,
        );
        let skill = calc_skill(args.level, ach);
        scenarios.push((
            "フルコンボ + フレーズ全成功".to_string(),
            ach,
            skill,
        ));
    }

    // 出力
    if scenarios.is_empty() {
        println!("改善シナリオが見つかりませんでした。");
    } else {
        let label_width = scenarios.iter().map(|(l, _, _)| l.len()).max().unwrap_or(0);
        for (label, ach, skill) in &scenarios {
            let mark = status_mark(*skill, args.target);
            println!(
                "  [{}] {:<width$}  達成率 {:.2}%  スキル {:.2}",
                mark,
                label,
                ach,
                skill,
                width = label_width
            );
        }
    }

    Ok(())
}
