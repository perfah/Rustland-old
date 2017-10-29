Rustland
========

What is it?
-----------

  Inspired feature-wise by [dynamic window managers](https://en.wikipedia.org/wiki/Dynamic_window_manager) like [i3](https://i3wm.org/) and [bspwm](https://github.com/baskerville/bspwm), this project is aiming for the the same category. 
  
  Unlike them however, Rustland is technically a **[Wayland](https://wayland.freedesktop.org/) compositor** rather than a traditional x11 window manager. 
  
 The intention is to create a future-proof window manager carrying some of the more powerful features of older projects in the category. It will probably not be exactly like what you have seen before, but automatic window tiling, multiple workspaces and flexible customization are some of the goals.
  
  Rustland is (lo and behold) written in Rust, but **still in the alpha stages so it probably won't fit your needs just yet**. 

  <img align="right" width="549" height="361" src="https://i.imgur.com/CITMr8c.gif">

Current state 
-------------

 - On-demand like creation of window layouts 
 - Automatic circular window tiling
 - Tag system for referencing items in the layout, e.g. both '@focused' and '@firefox' references Firefox should it be the focused application in your layout
 - Network transparency (via TCP) allows for potential interaction with the compositor from various applications/platsforms. 
 - Fancy window gaps and layout transitions

rlctl, a compositor interaction utility
---------------------------------------

rlctl is a proof of concept command line utility for interacting with the Rustland compositor from the outside. 
This is developed alongside the compositor and makes use of the built-in TCP functionality.
Example syntax: ``rlctl tree``, ``rlctl runapp /usr/bin/thunar``, ``rlctl @thunar moveto @root``
   
[*more information*](https://github.com/perfah/Rustland/wiki/rlctl,-a-compositor-interaction-utility)
   
   
Near goals
----------

* Background wallpaper
* Window borders
* Configuration/customization 
* More structured network transparency
* Snapshots for saving and loading layouts
* Modularity (potentially)

Contribution
------------

Credits for the backbone of this project goes to the [WLC (Wayland Compositor Project)](https://github.com/Cloudef/wlc) and the [RustWLC Rust bindings project](https://github.com/Immington-Industries/rust-wlc).
