# raudio
Rust test tool for rodio / cpal

## Sample commands

```
./raudio -d alsa:front:CARD=A,DEV=0 -f media/audiobooks/1\ -\ Benjamin\ als\ Wetterelefant.m4b
./raudio -d alsa:front:CARD=A,DEV=0 -f sample-3s.wav
```



andreas@t480s: raudio git main[!] is pkg v0.1.0 via rs v1.94.0
x  cargo run -- --file=./media/test.m4b --quick                                                                                                                                   🕰 3s222ms  | 19:17:17
Compiling raudio v0.1.0 (/home/andreas/projects/sandreas/raudio)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.58s
Running `target/debug/raudio --file=./media/test.m4b --quick`
file result: Ok(File { fd: 8, path: "/home/andreas/projects/sandreas/raudio/media/test.m4b", read: true, write: false })
audio stream error: Buffer underrun/overrun occurred.
audio stream error: A backend-specific error has occurred: unexpected number of frames written: expected 940, result 24 (this should never happen)
audio stream error: Buffer underrun/overrun occurred.



a::poll()` returned POLLERR
audio stream error: A backend-specific error has occurred: `alsa::poll()` returned POLLERR
audio stream error: A backend-specific error has occurred: `alsa::poll()` returned POLLERR

