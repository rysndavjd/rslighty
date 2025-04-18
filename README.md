## rslighty
Rslighty is basically a clone of my other tool [cpplighty](https://github.com/rysndavjd/cpplighty) which is a copy [acpilight](https://gitlab.com/wavexx/acpilight) just in RUST.
For adjusting backlight and led devices in the /sys filesystem in linux.

## Usage
```sh
Usage: rslighty [OPTIONS]

Options:
      --list             lists available backlight and led devices
      --device <device>  Device to adjust brightness of
      --get              Get current brightness of selected device in percent
      --get-steps        Get brightness steps of selected device
      --set <percent>    Set brightness in percent of selected device
      --inc <percent>    Increase brightness in percent of selected device
      --dec <percent>    Decrease brightness in percent of selected device
  -h, --help             Print help
  -V, --version          Print version
```

### Installation 
Clone this repo.
```sh
git clone https://github.com/rysndavjd/rslighty.git
cd ./rslighty
```
Then just run cargo
```sh
cargo build --release
```
After copy and set permissions for the binary.
```sh
sudo cp ./target/release/rslighty /usr/bin/rslighty
sudo chmod 755 /usr/bin/rslighty
```
It is also recommended to copy the udev rule for rslighty to allow any user in the "video" group to adjust brightness of devices.
```sh
sudo cp ./50-rslighty.rules /etc/udev/rules.d/
sudo udevadm control --reload
```
