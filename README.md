Rustland
========

  Wayland compositor written in Rust.

<img align="center" width="900" height="422" src="https://i.imgur.com/UaQbarC.gif">

Inspired feature-wise by [dynamic window managers](https://en.wikipedia.org/wiki/Dynamic_window_manager) like [i3](https://i3wm.org/) and [bspwm](https://github.com/baskerville/bspwm), this project is aiming for the the same category. Unlike them however, Rustland is technically a **[Wayland](https://wayland.freedesktop.org/) compositor** rather than a traditional x11 window manager. 
  
 The intention is to create a future-proof window manager carrying some of the more powerful features of older projects in the category. This project is probably not aiming for exactly what you have seen before, but automatic window tiling, multiple workspaces and flexible customization are some of the goals.
  
  Rustland is **still in the alpha stages so it probably won't fit your needs just yet**. 

What is currently in the scope of this project?
-----------------------------------------------

 - [x] On-demand like creation of window layouts 
 - [x] Automatic circular window tiling
 - [x] Background wallpapers, window gaps and layout transitions
 - [x] Command for showing an overview of the layout (the different workspaces) 
 - [x] Tag system for referencing items in the layout, e.g. both '@focused' and '@firefox' references Firefox should it be the focused application in your layout
 - [x] Some network transparency (via TCP) - allows for potential interaction with the compositor from various applications/platsforms. 
 - [x] [Basic configution and customization](https://github.com/perfah/Rustland/wiki/Configuration)
 - [ ] Layout interaction: resizing windows with mouse, keyboard shortcuts..
 - [ ] Touchscreen support
 - [ ] "Snapshots" for saving and loading layouts (maybe)
 - [ ] Floating windows (maybe)
 - [ ] Dbus integration (maybe) 

What is not in the scope of this project?
-----------------------------------------

* A panel/bar is not in the scope of this project since one can be implemented as a third party application. Implementing the APIs necessary for that to work will be the priority instead.
* Screen capturing is at least not a primary concern

rlctl, a compositor interaction utility
---------------------------------------

rlctl is a proof of concept command line utility for interacting with the Rustland compositor from the outside. 
This is developed alongside the compositor and makes use of the built-in TCP functionality.
Example syntax: ``rlctl tree``, ``rlctl runapp /usr/bin/thunar``, ``rlctl @thunar moveto @root``
   
[*more information*](https://github.com/perfah/Rustland/wiki/rlctl,-a-compositor-interaction-utility)

Contribution
------------

Credits for the backbone of this project goes to the [WLC (Wayland Compositor Project)](https://github.com/Cloudef/wlc) and the [wlc.rs bindings project](https://github.com/Drakulix/wlc.rs).
