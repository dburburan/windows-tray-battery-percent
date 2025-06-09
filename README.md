## Description
I've always found the battery icon in the Windows tray hard to read. Below about 30%, it often looks completely empty, even though you could easily have an hour or more of battery life remaining.

*windows-tray-battery-percent* is a simple tray icon which shows your battery level very clearly as digits. It also uses colour to show powered status and discharge rate. Written in Rust and only updating the icon when needed, it's lightweight on your system resources.

Example screenshot:  
![](screenshot.png)

## Build and Install
```
cargo build --release
```
Then just run `windows-tray-battery-percent.exe` or copy it into the following folder to run at startup.
```
%APPDATA%\Roaming\Microsoft\Windows\Start Menu\Programs\Startup
```

## Appearance
Simple, easy to read digits.  
![](doc-images/icon_p78_dr0_c0.png)

## Charging Status
**Green Overlay:** Indicates that the device is plugged in and charging.  
![](doc-images/icon_p78_dr0_c1.png)

## Battery Discharge Rate
**Red Overlay:** Shows the rate of battery discharge.  
- The height of the red overlay represents the percentage of battery loss per hour.
- More red indicates a faster discharge rate.

| Icon                                  | Discharge Rate |
| ------------------------------------- | -------------- |
| ![](doc-images/icon_p78_dr20_c0.png)  | 20%/hour       |
| ![](doc-images/icon_p78_dr50_c0.png)  | 50%/hour       |
| ![](doc-images/icon_p78_dr100_c0.png) | 100%/hour      |

## Discharging While Plugged In
When using USB-C power delivery with an underpowered charger, the laptop may draw more power than the charger provides. This causes battery drain even while plugged in. In this case:
- **Lighter Green Overlay:** The lighter shade indicates insufficient power.
- **Red Discharge Overlay:** Appears together with the green overlay to indicate battery drain.

| Icon                                  | Discharge Rate |
| ------------------------------------- | -------------- |
| ![](doc-images/icon_p78_dr30_c1.png)  | 30%/hour       |
| ![](doc-images/icon_p78_dr100_c1.png) | 100%/hour      |

## Battery Remaining
With the combined information, estimating the battery remaining is possible with just a glance.
- The following table demonstrates battery icon examples grouped by the time until 0%.
- Icons on the same row represent different scenarios with the same time to 0% battery.

| Time to 0% Battery       |                                        |                                      |                                      |                                      |
|------------------------- | -------------------------------------- | ------------------------------------ | ------------------------------------ | ------------------------------------ |
| 30 mins                  | ![](doc-images/icon_p50_dr100_c0.png)  | ![](doc-images/icon_p25_dr50_c0.png) |                                      |                                      |
| 1 hour                   | ![](doc-images/icon_p100_dr100_c0.png) | ![](doc-images/icon_p50_dr50_c0.png) | ![](doc-images/icon_p25_dr25_c0.png) | ![](doc-images/icon_p10_dr10_c0.png) |
| 2 hours                  | ![](doc-images/icon_p100_dr50_c0.png)  | ![](doc-images/icon_p50_dr25_c0.png) | ![](doc-images/icon_p25_dr12_c0.png) |                                      |
| 2 hours despite charging | ![](doc-images/icon_p100_dr50_c1.png)  | ![](doc-images/icon_p50_dr25_c1.png) | ![](doc-images/icon_p25_dr12_c1.png) |                                      |

## Building Reduced Binary Size (Optional)
If you would like a smaller binary, you can build from rust-src. But the size reduction wasn't amazing.
- Size reduction: ~357KB -> ~277KB
#### Prerequisites
- rust nightly
- rust-src
```
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly
```
Then run
```
build.bat
```

