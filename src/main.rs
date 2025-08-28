/*
CONCENTRA
---------

This application is designed to create timers for concentration, or flow timers.

It has it's own configuration files that can define custom sessions. These sessions
ore defined in `.concentra` files. These files are essentially a mini programming
language. In the concentra language, a `session` is basically a function that returns
an object. Based on the object's properties, the runtime creates a flow session that
is custom to the user.

At it's core, it is not only super simple.
*/
use directories::ProjectDirs;
use 

fn main() {
 
    if let Some(proj_dirs) = ProjectDirs::from("io", "christianstout",  "concentra") {
        proj_dirs.config_dir();
        // Lin: /home/alice/.config/barapp
        // Win: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App\config
        // Mac: /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App
        println!("{}", proj_dirs.config_dir().display());
    }
    println!("Hello, world!");
}
