nongle_standalone_debug := 'target/debug/standalone'
nongle_standalone_release := 'target/release/standalone'

build:
    cargo xtask bundle vst
install:
    rm -rf /Library/Audio/Plug-Ins/VST3/oilcake/Nongle.vst3
    mv target/bundled/Nongle.vst3 /Library/Audio/Plug-Ins/VST3/oilcake
refresh: build install

run_big:
    cargo build --release
    {{nongle_standalone_release}} --library ./Xy_samples_big/ --voices 8 --win-size 3

run_small:
    cargo build
    {{nongle_standalone_debug}} --library ./Xy_samples_small/ --voices 8 --win-size 3
