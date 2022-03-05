# Widgets for Bevy UI

A collection of widgets that are built on top of bevy_ui and fully embrace the ECS pattern.

## Current State

This was started recently and is still under development. This is still just an experiment and it's unclear if and when a first release will be made available. I will be iterating over different API's and implementations to find what works and what doesn't.

## Design Goals
 - **Modular**: Widgets should be extensible, replaceable and customizable
 - **Bevy-native**: Built on top of Bevy with no third-party UI library
 - **Bevy-ready**: Follows rougly the same design goals as Bevy as if it was an official Bevy crate

## Widgets

### Tooltip

A tooltip is a box that "floats" over other UI elements and contains some text.

Tooltips are useful to display additional information to users when they mouse-over or select something on the screen.
