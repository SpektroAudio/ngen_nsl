# ngen_nsl

## About ngen_nsl

ngen_nsl is Rust library for generating NSL (NGEN Scripting Language) scripts for the [Spektro Audio NGEN](https://spektroaudio.com/ngen).

For more information about NGEN and NSL, visit [https://spektroaudio.com/ngen](https://spektroaudio.com/ngen).

## Examples 

```rust
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

    let path = "output/path/test.nsl";
    script.export_hex(path);

}


```

To run the included examples (after cloning the repository), use the following command:

```cargo run --example example_name``` (where ```example_name``` is the name of the example you want to run).

--- 

**Spektro Audio**  
@spektroaudio  
spektroaudio.com

