# nellos
An experimental next-generation operating system written purely in Rust. :sparkles:

We are currently in the RT0 (Research Target 0) phase aiming for an early prototype
of a functioning operating system.

Research Targets are meant to allow quick and loose experimentation and help me gain a better
understanding about operating system developement in general.
Note that early Research Targets will very likely be terrible and not resemble our end goals in any way.
Subcomponents and even the whole fundamental OS/Kernel architecture are merely ad hoc placeholders
and *will* be replaced with well designed code at some point.
This bears repeating: Until a component has been nicely and thoughtfully designed we will guarentee
to fully break compatibility if and how we need to in order to replace it.
Only then (and definitely then) will the component by subject to our determined robustness and compatibility guarentees.
But let's not get ahead of ourselves, we don't even have an operating system yet.

I believe this type of iterative developement is the perfect vector to a well designed end product,
as proven beautifully by the Rust project.
Just don't expect anything of quality right now...

## Note About Submodules
Right now in early developement the git submodules including kernel and bootloader will not
be updated frequently to their latest commit. When developing make sure you manually pull
their latest commits.

The reason for this workflow is that it doesn't make sense right now.
The "bound" commit of a submodule has semantic meaning (which version of
kernel code for this operating system version?) but during active proof
research development everything is in flux so much that there is little utility
in indicating what version goes with what.
