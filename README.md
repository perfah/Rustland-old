Rustland
========

What is it?
-----------

  Inspired feature-wise by previous tiling window managers such as [i3](https://i3wm.org/) and [bspwm](https://github.com/baskerville/bspwm), Rustland is yet another ond of them. 
  
  Unlike them however, Rustland is not an **x11** window manager, and instead more specifically a **[Wayland](https://wayland.freedesktop.org/) compositor** filling a similar but yet very distant role. 
  
  The intention is to contribute to the Wayland ecosystem by showing up as another counterpart to the typical [x11 dynamic window manager](https://en.wikipedia.org/wiki/Dynamic_window_manager). This is however a different interpretation and the aim today is somewhat diverge with pointers to: automatic window tiling, multiple workspaces, network transparency and visual sugar (window gaps, transitions, etc). 
  
  Rustland is (lo and behold) written in Rust, but **still in the alpha stages meaning that it probably won't fit your needs just yet**. 

  <img align="right" width="549" height="361" src="https://i.gyazo.com/9d8d6f9d7956d11e958c4dbd7154b497.png">

Current state 
-------------

 - On-demand like creation of window layouts 
 - Automatic circular window tiling
 - Tag system for referencing items in the layout, e.g. both '@focused' and '@firefox' references Firefox should it be the focused application in your layout
 - Network transparency (via TCP) allows for potential interaction with the compositor from various applications/platsforms. 
 - Fancy window gaps and layout transitions

Custom configuration as a development stage has been delay which means that some things are still hardcoded:
 - Layout gap of 15 pixels
 - 2x2 grid of workspaces

rlctl, a compositor interaction utility
---------------------------------------

rlctl is a proof of concept command line utility for interacting with the Rustland compositor from the outside. 
This is developed alongside the compositor and makes use of the built-in TCP functionality.
Example syntax: ``rlctl tree``, ``rlctl runapp /usr/bin/thunar``, ``rlctl @thunar moveto @root``
   
[*more information*](https://github.com/perfah/Rustland/wiki/rlctl,-a-compositor-interaction-utility)
   
Features in the near future
---------------------------

* Background wallpaper
* Window borders
* Configuration/customization 
* More structured network transparency
* Snapshots for saving and loading layouts
* Modularity (potentially)

Contribution
------------

Credits for the backbone of this project goes to the [WLC (Wayland Compositor Project)](https://github.com/Cloudef/wlc) and the [RustWLC Rust bindings project](https://github.com/Immington-Industries/rust-wlc).
