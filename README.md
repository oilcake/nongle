## Multi-layered xylophone rompler.

Can be used as a sampler with any one-shot library. For now samples have to be named very strictly and have some information about the amplitude of audio.

It is a work in progress and does not have any fancy features yet, just the basic idea that made it live.
Main key of this program when that's already a huge pile of samplers out there is that it can use as many velocity layers as you have. Traditional samplers usually hard link a layer(which is an audio file, sample) to the velocity value with which you only can have 127 layers. It may change with midi 2.0, but for me even bigger problem always was a manual setup of each layer to fall in the right place in velocity table. This sampler tries to choose layer automatically and never play the same layer twice, so even when you drawing your midi with mouse you will still have dynamic and musical performance.

Aims to be in pure rust and use as little dependencies as possible.

Usage is simple `cargo run` when you have properly named sample library.
