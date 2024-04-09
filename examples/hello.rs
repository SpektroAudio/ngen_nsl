extern crate ngen_nsl;

use ngen_nsl::*;
fn main() {
    let mut script = NSLScript::new();
    // Add commands to the script
    script.add_command(Commands::Set(step_pitch(0), constant(36))); // Set the pitch of the first step to 36
    script.add_command(Commands::Set(step_velocity(0), constant(100))); // Set the velocity of the first step to 100
    script.add_command(Commands::End); // End the script


    println!("Commands added to the script: {}", script.commands.len());

    let code = script.code();
    println!("Converted script: {:?}", code);


}