## Multi-layered xylophone rompler.

Can be used as a sampler with any one-shot library. For now samples have to be named very strictly and have some information about the amplitude of audio.

It is a work in progress and does not have any fancy features yet, just the core concept that brought it to life.
Main key of this program (when that's already a huge pile of samplers out there) is is its ability to utilize as many velocity layers as you have available. Traditional samplers usually hard link a layer(which is an audio file, sample) to a specific velocity value limiting you to only 127 layers. While MIDI 2.0 may address this limitation, even bigger challenge for me has always been manually setting up each layer to fit correctly within the velocity table. This sampler aims to automate the selection of layers, ensuring that no layer is played twice in succession. As a result, even when you draw MIDI notes with a mouse, you'll still achieve a dynamic and musically expressive performance.

Aims to be in pure rust and use as little dependencies as possible.

Usage is simple `cargo run` when you have properly named sample library.
