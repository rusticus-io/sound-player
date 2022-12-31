# Vending Machine Mod to Play Sounds

This is a small example on how to modify a vanding machine with an raspberry pi to play sounds on pulling items.

## Preparation

This is a walkthrough to install the app on your raspberry pi from an ubuntu distro.

### For development on ubuntu

Obviously, this is a Rust project, so you need to install Rust first.

```bash
$ # install dependencies
$ sudo apt install sox watchman
$ # to test the app:
$ cp .env.example .env # only one time
$ cargo run
$ # touch a file in the sounds folder
$ touch sounds/total_commitment.wav # for instance
```
This should play one of the sounds.

If you touch files in the folder, add, or remove to it a sound should be played.

### For deployment on your raspberry pi

We will do crosscompilation with the cross-rs project.
Therefore you need to have a working docker engine running on your system.

```bash
$ # install dependencies
$ sudo apt install rpi-imager
$ cargo install cross --git https://github.com/cross-rs/cross
```

Write the image to your sd card and make sure, that the user is pi and the dns name is raspberrypi.local.
(See settings in rpi-imager and remember the pi user password)

Then add your ssh pubkey to your pi for convenience.

```bash
$ cat $HOME/.ssh/id_rsa.pub | ssh pi@raspberrypi.local "mkdir -p ~/.ssh && cat >> ~/.ssh/authorized_keys"
```

Log into your pi with `ssh pi@raspberrypi.local` and perform apt updates and install dependencies.

```bash
$ sudo apt update
$ sudo apt upgrade -y 
$ sudo apt install -y sox watchman
```

After this, perform this command on your project folder in ubuntu.

```bash
./deploy.sh
```

this will cross compile and copy all necessary files to your pi.

Afterwards, switch back to your pi shell and enter the folder `$HOME/VendingMachine`.

It is important that you copy the `.env.example` to `.env` and maybe configure your settings in it.

To test this, you can now start the programm `./sound-player` and see what's happening.

When you open another shell on your pi, you can touch, add, or remove files form the sounds folder to play sounds.

#### Finishing your project

Copy the `sound-player.service` file into the folder `/lib/system.d/system` on your pi and activate the service with
`sudo systemctl start sound-player.service`.

Buy an USB Sound Adapter at your favorite electronics store and attach it to the pi with a speaker.

Now you're ready to perform the intented action:

Based on your hardware design, place a switch of your choice near the output tray and attach it between the pins `GPIO4` and 
`V5`.

When the switch closes, it will activate the service and a random sound will be played, based on the files present in the sounds folder.

## Suggestions

- The vending machine has moving parts at its output tray. You can place a magnet and a nearby reed contact switch to emulate a closed circuit.
- https://www.wavsource.com/