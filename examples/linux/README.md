# Camsense-X1 - Linux Example

![Screenshot of rerun viewer showing Camsense-X1 data](rerun_viewer_screenshot.png)

## Quick Start

- Add user to `dialout` group in order to be able to access serial port:

  ```shell
  sudo usermod -a -G dialout $USER
  ```
  
  You have to logout and login again for the change to take effect.

- Install [Rerun](https://rerun.io/) viewer:

  ```shell
  cargo binstall rerun-cli@0.33.0
  ```

- Build and run binary:

  ```shell
  cargo run --release
  ```

  This may take some time to build...

  Once it's finished and if everything works,
  it will open the rerun viewer and show the LiDAR scans in real-time.
