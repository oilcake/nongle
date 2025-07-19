## Multi-layered xylophone rompler.

Can be used as a sampler with any one-shot library. For now samples have to be named very strictly and have some information about the amplitude of audio.

It is a work in progress and does not have any fancy features yet, just the core concept that brought it to life.
Main key of this program (when that's already a huge pile of samplers out there) is its ability to utilize as many velocity layers as you have available. Traditional samplers usually hard link a layer(which is an audio file, sample) to a specific velocity value limiting you to only 127 layers. While MIDI 2.0 may address this limitation, even bigger challenge for me has always been manually setting up each layer to fit correctly within the velocity table. This sampler automates the selection of layers, ensuring that no layer is played twice in succession. As a result, even when you draw MIDI notes with a mouse, you'll still achieve a dynamic and musically expressive performance.

Aims to be in pure rust and use as little dependencies as possible.

 **Usage:**
<br>
(assuming you've got `just` installed)
<br>
##### compile and start standalone version
```bash
# it's gonna start from command line and for incoming midi messages
just run_small
```
##### compile vst and place it in your VST folder(mac only, sorry)
```bash
# debug version
just refresh_debug
# release version
just refresh_release
```
Some scripts in `justfile` may be specific to my setup and macos, so you may have to customize it.
<br>
<br>
Note that if you want to see logs from you plugin in terminal you just should start your daw from it, assuming RUST_LOG variable is set.
