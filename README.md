Rustland
========

What is Rustland?
-----------------

  A **work-in-progress** window manager, or more specifically a
  compositor for the Wayland protocol, with features such as automatic window tiling and interactivity over tcp. Rustland is written in Rust (lo and behold) and is aiming for flexibility and powerful customization.
  
  Rustland is currently in the alpha stage and will probably not fit your needs just yet.

  <img align="right" width="549" height="361" src="https://i.gyazo.com/9d8d6f9d7956d11e958c4dbd7154b497.png">

Current state 
-------------

 - On-demand like creation of window layouts 
 - Automatic circular window tiling
 - Layout gap of 15 pixels
 - Workspaces
 - Tag system for referencing items in the layout, e.g. both '@focused' and '@firefox' would reference Firefox given that it is the active application
 - Network transparency (TCP) allows for potential interaction with the WM from various applications/platsforms. Rustland natively supports a set of commands which currently are the following (in [rlctl](#rlctl---utility) syntax):


rlctl, a compositor interaction utility
---------------------------------------

rlctl is a proof of concept command line utility for interacting with the Rustland compositor from the outside. 
This is developed alongside the compositor and makes use of the built-in TCP functionality.
Example syntax: ``rlctl tree``, ``rlctl runapp /usr/bin/thunar``, ``rlctl @thunar moveto @root``
   
[*more information*](https://github.com/perfah/Rustland/wiki/rlctl,-a-compositor-interaction-utility)
   
Potential features in the future
--------------------------------

* Customization / modularity
* Snapshots for saving and loading layouts
* Configuration

Contribution
------------

Credits for the backbone of this project goes to the [WLC (Wayland Compositor Project)](https://github.com/Cloudef/wlc) and the [RustWLC Rust bindings project](https://github.com/Immington-Industries/rust-wlc).
