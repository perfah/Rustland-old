Rustland
========

*Late update: The reason for the lack of updates has mainly been the deprecation of WLC, the underlying backend library. Rustland may still however be rewritten for another library (perhaps [wlroots](https://github.com/swaywm/wlroots)) in the future.*

A [Wayland](https://wayland.freedesktop.org/) compositor for window management written in Rust.
  
*Sources of inspirations: [sway](http://swaywm.org/), [i3](https://i3wm.org/), [bspwm](https://github.com/baskerville/bspwm)*

[<img alt="A preview on Youtube" align="center" width="827" height="481" src="https://i.imgur.com/Ek06LG8.png">](https://www.youtube.com/watch?v=C5r9Bc7rCI4)

 This project is not an attempt to clone anything you have seen before but you may recognize some features such as automatic window tiling and multiple workspaces from other projects. The intention is to create a new future-proof window manager with some of the powerful features of the past.
  
 Although Rustland is still under development you can try it today by [building it yourself](https://github.com/perfah/Rustland/wiki/Get-started#building).

What is currently in the scope of this project?
-----------------------------------------------

 - [x] On-demand like creation of window layouts 
 - [x] Automatic circular window tiling
 - [x] Background wallpapers, window gaps and layout transitions
 - [x] Command for showing an overview of the layout (the different workspaces) 
 - [x] Tag system for referencing items in the layout, e.g. both '@focused' and '@firefox' references Firefox should it be the focused application in your layout
 - [x] Some network transparency (via TCP) - allows for potential interaction with the compositor from various applications/platsforms. 
 - [x] [Basic configution and customization](https://github.com/perfah/Rustland/wiki/Configuration)
 - [X] Layout interaction: resizing windows with mouse, keyboard shortcuts..
 - [ ] Window borders
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
