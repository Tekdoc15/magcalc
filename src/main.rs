use std::f64::consts::PI; // gives us pi as a 64bit floar for higher precision
use std::io::{self, Write};
use std::env::args;

fn prompt(label: &str) -> f64 {
    print!("{}: ", label);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().expect("Expected a number")
}

fn get_or_prompt(args: &[String], index: usize, label: &str) -> f64 {
    if let Some(val) = args.get(index) {
        val.parse().expect("Expected a numeric argument")
    } else {
        prompt(label)
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    // Check for --help before anything else
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("Saturation calculator for ungapped core");
        println!("DO NOT USE IF YOUR CORE HAS AN AIRGAP");
        println!("Usage: {} [B_sat] [l_e] [N] [mu_r]", args[0]);
        println!();
        println!("Arguments (positional, all optional — omitted args will be prompted):");
        println!("  B_sat   Saturation flux density     (T)");
        println!("  l_e     Effective magnetic path length  (m)");
        println!("  N       Number of turns             (dimensionless)");
        println!("  mu_r    Initial relative permeability");
        println!();
        println!("Example:");
        println!("  {} 0.35 0.072 30 2000", args[0]);
        println!();
        println!("Output:");
        println!("  I_sat   Saturation current          (A)");
        return;
    }

    let mu_o = 4.0 * PI * 1e-7; // mu_o is constant and in units of H/m

    let b_sat   = get_or_prompt(&args, 1, "Saturation Flux Density (T)");
    let l_e = get_or_prompt(&args, 2, "Effective Length (m)");
    let n   = get_or_prompt(&args, 3, "Number of turns");
    let mu_r   = get_or_prompt(&args, 4, "Initial permeability");
    
    // Calculate I_sat
    let i_sat = (b_sat * l_e) / (n * mu_o * mu_r);


    println!("I_sat in A is: {:.3}", i_sat);
}
