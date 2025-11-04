use embed_manifest::{embed_manifest, new_manifest};
use std::fs;
use std::process::Command;
use swc::{Compiler, JsMinifyExtras};
use swc::config::{Options, Config, JscConfig, JsMinifyOptions};
use swc_common::{SourceMap, sync::Lrc, GLOBALS};
use swc_common::errors::Handler;

fn main() {
    // Compile TypeScript
    compile_typescript();

    // Minify JavaScript using SWC
    //minify_javascript();

    if std::env::var_os("CARGO_CFG_WINDOWS").is_some() {
        embed_manifest(new_manifest("com.home.gmail-cleaner-rs"))
            .expect("unable to embed manifest file");
    }
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=static/js/script.ts");
    println!("cargo:rerun-if-changed=tsconfig.json");
}

fn minify_javascript() {
    let js_file = "static/js/script.js";

    // Check if JavaScript file exists
    if !std::path::Path::new(js_file).exists() {
        println!("cargo:warning=JavaScript file {} not found, skipping minification", js_file);
        return;
    }

    // Check if JavaScript file is readable
    if fs::read_to_string(js_file).is_err() {
        println!("cargo:warning=Failed to read JavaScript file");
        return;
    }

    // Set up SWC compiler
    let cm = Lrc::new(SourceMap::default());
    let handler = Handler::with_emitter_writer(Box::new(std::io::stderr()), Some(cm.clone()));
    let compiler = Compiler::new(cm.clone());
    let options = JsMinifyOptions {
        keep_fnames: true,
        ..Default::default()
    };

    // Minify the JavaScript - must run inside GLOBALS.set()
    let result = GLOBALS.set(&Default::default(), || {
        compiler.minify(
            cm.load_file(std::path::Path::new(js_file)).expect("Failed to load file"),
            &handler,
            &options,
            JsMinifyExtras::default(),
        )
    });

    match result {
        Ok(output) => {
            // Write the minified code back to the file
            if let Err(e) = fs::write(js_file, output.code.as_bytes()) {
                println!("cargo:warning=Failed to write minified JavaScript: {}", e);
            } else {
                println!("cargo:warning=JavaScript minification successful with SWC");
            }
        }
        Err(e) => {
            println!("cargo:warning=SWC minification failed: {}", e);
        }
    }
}

fn compile_typescript() {
    // Run TypeScript compiler using tsconfig.json
    match Command::new("tsc")
        .args(&["--project", "."])
        .status()
    {
        Ok(status) if status.success() => {
            println!("cargo:warning=TypeScript compilation successful");
        }
        Ok(status) => {
            println!("cargo:warning=TypeScript compilation failed with exit code: {}", status);
            // Don't fail the build if TypeScript compilation fails
            // The existing .js file will be used as fallback
        }
        Err(e) => {
            println!("cargo:warning=Failed to run TypeScript compiler: {}. Make sure TypeScript is installed (npm install -g typescript)", e);
            // Don't fail the build, use existing .js file as fallback
        }
    }
}
