Rustland - Wayland Compositor 
=============================

What is Rustland?
-----------------

  A tiling Wayland compositor written in Rust with the aim of flexibility and powerful customizability.  
  Rustland is currently in the alpha stage so it will probably not fit your needs just yet.

Current state
-------------

 - On-demand like creation of window layouting 
 - Very basic automatic window tiling
 - Workspaces
 - Tag system for referencing items in the layout
 - Interaction with the WM using the TCP-protocol. Rustland natively supports a set of commands which currently are the following:
   - tree: Sends back a list of elements in the window layout in a tree like format.
   - runapp: Executes an application to start in the focused position of the layout.
   - moveto: Moves an element in the layout to another place. 

rlctl - utility
---------------

   rlctl is a proof of concept command line utility for controlling the rustland compositor from the outside. 
   This is developed alongside the compositor and makes use of the TCP functionality.  
   Example syntax:
    - ``rlctl tree``
    - ``rlctl runapp /usr/bin/thunar``
    - ``rlctl @thunar moveto @root``

Potential features in the future
--------------------------------

* Customization / modularity
* Snapshots for saving and loading layouts
* Configuration

Contribution
------------

Credits for the backbone of this project goes to the [WLC (Wayland Compositor Project)](https://github.com/Cloudef/wlc) and the [RustWLC Rust bindings project](https://github.com/Immington-Industries/rust-wlc).
