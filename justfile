nongle_standalone_debug := 'target/debug/standalone'
nongle_standalone_release := 'target/release/standalone'

# settings
set dotenv-load := true

# vst section
build_debug:
    cargo xtask bundle vst
build_release:
    cargo xtask bundle vst --release

clean:
    rm -rf /Library/Audio/Plug-Ins/VST3/oilcake/Nongle.vst3
install:
    mv target/bundled/Nongle.vst3 /Library/Audio/Plug-Ins/VST3/oilcake

refresh_debug: build_debug clean install
refresh_release: build_release clean install

# standalone section
run_big:
    cargo build --package standalone
    {{nongle_standalone_release}} --library ./Xy_samples_big/ --voices 8 --win-size 3

run_small:
    cargo build --package standalone
    RUST_LOG=debug {{nongle_standalone_debug}} --library ./Xy_samples_small/ --voices 8 --win-size 3

# tools
start_live:
    echo "$DAW_PATH"
    echo "$PROJECT_PATH"
    "$DAW_PATH" "$PROJECT_PATH"

launch: refresh_debug start_live
