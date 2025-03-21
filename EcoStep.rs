use std::io;

// STRUCT DEFINITION TO STORE CARBON EMISSION FACTORS FOR DIFFERENT CATEGORIES
struct EmissionFactors {
    electronics: f64, // kg CO2 per hour
    vehicles: f64,    // kg CO2 per liter of fuel
    household: f64,   // kg CO2 per kWh of electricity
}

// FUNCTION TO GET USER INPUT
fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("FAILED TO READ INPUT");
    input.trim().to_string()
}

fn main() {
    // Define emission factors (example values)
    let factors = EmissionFactors {
        electronics: 0.05, // 0.05 kg CO2 per hour of device usage
        vehicles: 2.31,    // 2.31 kg CO2 per liter of fuel
        household: 0.45,   // 0.45 kg CO2 per kWh of electricity
    };

    println!("============================================");
    println!("Welcome to EcoStep - Your Personal Carbon Footprint Calculator");
    println!("============================================");
    println!("EcoStep helps you track and reduce your carbon footprint by estimating your CO2 emissions ");
    println!("from daily activities like electronics usage, vehicle fuel consumption and household energy use.");
    println!("============================================");
    println!("LET'S TAKE A STEP TOWARDS A GREENER FUTURE!");

    let choice = get_input("Enter your choice by typing a letter (E - electronics / V - vehicles / H - household): ").to_uppercase();

    match choice.as_str() {
        "E" => calculate_electronics(factors.electronics),
        "V" => calculate_vehicles(factors.vehicles),
        "H" => calculate_household(factors.household),
        _ => println!("Invalid choice. Please enter E, V or H."),
    }
}

// FUNCTION TO CALCULATE ELECTRONICS CARBON FOOTPRINT
fn calculate_electronics(emission_factor: f64) {
    println!("\nEcoStep will now calculate your electronics carbon footprint!");
    let hours: f64 = get_input("Enter the total hours of device usage per day:")
        .parse()
        .expect("INVALID INPUT. Please enter a valid number.");

    let daily_footprint = hours * emission_factor;
    let weekly_footprint = daily_footprint * 7.0;

    println!("Your estimated carbon footprint from electronics usage is {:.2} kg CO2 per day.", daily_footprint);
    println!("Your estimated carbon footprint from electronics usage is {:.2} kg CO2 per week.", weekly_footprint);

    // CLASSIFY FOOTPRINT LEVEL BASED ON WEEKLY EMISSIONS
    if weekly_footprint <= 0.7 {
        println!("Your electronics carbon footprint is LOW.");
    } else if weekly_footprint <= 2.1 {
        println!("Your electronics carbon footprint is MODERATE.");
    } else if weekly_footprint <= 4.2 {
        println!("Your electronics carbon footprint is HIGH.");
    } else {
        println!("Your electronics carbon footprint is VERY HIGH.");
    }

    display_mitigation_strategies_electronics();
}

// FUNCTION TO CALCULATE VEHICLE CARBON FOOTPRINT
fn calculate_vehicles(emission_factor: f64) {
    println!("\nEcoStep will now calculate your vehicle's carbon footprint!");
    let liters: f64 = get_input("Enter the liters of fuel consumed per day:")
        .parse()
        .expect("INVALID INPUT. Please enter a valid number.");

    let daily_footprint = liters * emission_factor;
    let weekly_footprint = daily_footprint * 7.0;

    println!("Your estimated carbon footprint from vehicle fuel consumption is {:.2} kg CO2 per day.", daily_footprint);
    println!("Your estimated carbon footprint from vehicle fuel consumption is {:.2} kg CO2 per week.", weekly_footprint);

    // CLASSIFY FOOTPRINT LEVEL BASED ON WEEKLY EMISSIONS
    if weekly_footprint <= 32.2 {
        println!("Your vehicle carbon footprint is LOW.");
    } else if weekly_footprint <= 80.5 {
        println!("Your vehicle carbon footprint is MODERATE.");
    } else if weekly_footprint <= 161.7 {
        println!("Your vehicle carbon footprint is HIGH.");
    } else {
        println!("Your vehicle carbon footprint is VERY HIGH.");
    }

    display_mitigation_strategies_vehicles();
}

// FUNCTION TO CALCULATE HOUSEHOLD CARBON FOOTPRINT
fn calculate_household(emission_factor: f64) {
    println!("\nEcoStep will now calculate your household's carbon footprint!");
    let kwh: f64 = get_input("Enter the total kWh of electricity used per day:")
        .parse()
        .expect("INVALID INPUT. Please enter a valid number.");

    let daily_footprint = kwh * emission_factor;
    let weekly_footprint = daily_footprint * 7.0;

    println!("Your estimated carbon footprint from household energy consumption is {:.2} kg CO2 per day.", daily_footprint);
    println!("Your estimated carbon footprint from household energy consumption is {:.2} kg CO2 per week.", weekly_footprint);

    // CLASSIFY FOOTPRINT LEVEL BASED ON WEEKLY EMISSIONS
    if weekly_footprint <= 15.75 {
        println!("Your household carbon footprint is LOW.");
    } else if weekly_footprint <= 31.5 {
        println!("Your household carbon footprint is MODERATE.");
    } else if weekly_footprint <= 63.0 {
        println!("Your household carbon footprint is HIGH.");
    } else {
        println!("Your household carbon footprint is VERY HIGH.");
    }

    display_mitigation_strategies_household();
}

// FUNCTION TO DISPLAY MITIGATION STRATEGIES FOR ELECTRONICS
fn display_mitigation_strategies_electronics() {
    let strategies = [
        "Invest in energy-efficient appliances",
        "Minimize screen time and reduce power usage",
        "Unplug devices when not in use",
        "Use energy-efficient devices",
        "Turn off devices instead of leaving them on standby",
    ];
    println!("\nWays to Reduce Your Electronics Carbon Footprint:");
    for strategy in &strategies {
        println!("{}", strategy);
    }
}

// FUNCTION TO DISPLAY MITIGATION STRATEGIES FOR VEHICLES
fn display_mitigation_strategies_vehicles() {
    let strategies = [
        "Use public transportation, carpooling, biking or walking",
        "Drive a fuel-efficient vehicle",
        "Maintain your vehicle regularly",
        "Avoid idling your vehicle",
        "Combine errands to reduce trips",
        "Switch to electric or hybrid vehicles",
    ];
    println!("\nWays to Reduce Your Vehicle Carbon Footprint:");
    for strategy in &strategies {
        println!("{}", strategy);
    }
}

// FUNCTION TO DISPLAY MITIGATION STRATEGIES FOR HOUSEHOLD
fn display_mitigation_strategies_household() {
    let strategies = [
        "Switch to energy-efficient lighting like solar, LED or wind",
        "Use natural light and ventilation",
        "Use LED light bulbs and energy-efficient appliances",
        "Turn off lights when not in use",
        "Reduce heating and cooling usage",
        "Unplug devices when not in use",
        "Install solar panels or other renewable energy sources",
    ];
    println!("\nWays to Reduce Your Household Carbon Footprint:");
    for strategy in &strategies {
        println!("{}", strategy);
    }
}
