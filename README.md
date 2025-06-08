## Description

I've always found the battery icon in the windows tray hard to read. Below around 30% it's easy to think it's completely empty when in reality you could easily have another hour of battery life.

windows-tray-battery-percent is a simple tray icon which shows your battery level very clearly as digits. Written in Rust and only updating the icon when needed, it's lightweight on your system resources.

![](screenshot.png)

## Appearance

Clear digits showing your battery percentage remaining  
![](doc-images/icon_p78_dr0_c0.png)

A green highlight indicates that you are plugged in  
![](doc-images/icon_p78_dr0_c1.png)

A red highlight shows battery drain rate  
![](doc-images/icon_p78_dr20_c0.png) 20%/hour

More red = faster drain  
The height of the red overlay indicates the percentage battery loss per hour  
![](doc-images/icon_p78_dr50_c0.png) 50%/hour  
![](doc-images/icon_p78_dr100_c0.png) 100%/hour  

Sometimes when you use USB-C power delivery connected to an underpowered charger, you can be both receiving power yet draining the battery at the same time as your laptop is drawing more than the power supply can deliver. When this happens:
1. The green charging overlay becomes lighter to denote insufficient power
2. The red discharge overlay is visible at the same time
![](doc-images/icon_p78_dr30_c1.png)

This can look intense if your charger is weak but the laptop is drawing lots of power  
![](doc-images/icon_p78_dr100_c1.png)

## More Examples

Icons in the same row represents laptops dieing in the same amount of time
| Meaning                                         |                                        |                                      |                                      |                                      |
| ----------------------------------------------- | -------------------------------------- | ------------------------------------ | ------------------------------------ | ------------------------------------ |
| Laptop will hit 0% in 30 mins:                  | ![](doc-images/icon_p50_dr100_c0.png)  | ![](doc-images/icon_p25_dr50_c0.png) |                                      |                                      |
| Laptop will hit 0% in 1 hour:                   | ![](doc-images/icon_p100_dr100_c0.png) | ![](doc-images/icon_p50_dr50_c0.png) | ![](doc-images/icon_p25_dr25_c0.png) | ![](doc-images/icon_p10_dr10_c0.png) |
| Laptop will hit 0% in 2 hours:                  | ![](doc-images/icon_p100_dr50_c0.png)  | ![](doc-images/icon_p50_dr25_c0.png) | ![](doc-images/icon_p25_dr12_c0.png) |                                      |
| Laptop will hit 0% in 2 hours despite charging: | ![](doc-images/icon_p100_dr50_c1.png)  | ![](doc-images/icon_p50_dr25_c1.png) | ![](doc-images/icon_p25_dr12_c1.png) |                                      |
