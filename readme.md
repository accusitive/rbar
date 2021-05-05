# rbar
rbar is an x11 status bar
if you want to use this you will have to change some of the code, specifically the audio setup, I normally have 2 sinks, my speakers and my headphones, but this might not exactly be a common setup, so you will have to change that.
The code should be readable/easily changable if you know rust.

This codebase features a component system, where each component runs every X seconds, independant of eachother.
The project architecture should be pretty simple to understand.
# license
this project is licensed under MIT