# Parenthesis
A text editor written in Rust using Vulkan and imgui, because that makes total sense.
## Runtime Dependencies
On Windows, requires the Visual C/C++ and Vulkan Runtimes.

On Linux, requires Vulkan and GTK.
## Instructions
Window->New editor: creates a window to edit a single file in.

Window->New shell (unimplemented): Creates a window to run commands in. Uses pwsh by default.

Window->Close \[\#\]: closes the window with the given number.

File->New: clears the window.

File->Open: opens a file to edit.

File->Save As: saves the file with a newly selected name.

File->Save: saves the file.