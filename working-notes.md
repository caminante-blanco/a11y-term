# OSC 4117 Protocol Summary

The `OSC 4117` protocol utilizes a state-based Start/End tag model to embed accessibility information within terminal streams. This design addresses potential buffering issues by ensuring semantic data is consistently applied to text segments.

## General Syntax

- **Initiator:** `\x1b]4117;`
- **Terminator:** `\x07` (BEL)

The protocol operates by setting an active accessibility state upon encountering a `...-START` command. Subsequent text printed to the terminal inherits the semantics defined by this active state. An `ARIA-END` command is used to clear the active accessibility state.

## Commands

The protocol defines four primary commands:

1.  **`WIDGET-START`**: This command is used to apply semantic roles and states to visual text that is already appropriate for display. It allows for tagging elements such as buttons, links, or menu items.
    - Syntax: `WIDGET-START;role={role};id={id};state={state}`

2.  **`LABEL-START`**: This command provides an explicit override for visual text that may not be screen-reader friendly. It enables replacing the displayed text with a more descriptive label for assistive technologies.
    - Syntax: `LABEL-START;label={text};role={role};id={id}`

3.  **`IMG-START`**: Designed for ASCII art, this command groups a block of characters and treats them as a single image, providing an `alt_text` for screen readers while hiding the raw characters.
    - Syntax: `IMG-START;label={alt_text};id={id}`

4.  **`LIVE`**: This command is used for event-based announcements, often referred to as "live regions." Unlike the other commands, it does not follow the Start/End tag model for grid-based state; instead, its payload is handled immediately via a callback mechanism within the `a11y-vt100` component.
    - Syntax: `LIVE;urgency={polite|assertive};text={message}`

## State Machine Operation

Within the `a11y-vt100` parser, an `active_a11y_data` state is maintained. Upon parsing a `...-START` command, this state is populated with the relevant accessibility data. Any characters subsequently printed to the terminal grid will have a clone of this `active_a11y_data` associated with their respective cells. The `ARIA-END` command is responsible for resetting the `active_a11y_data` state to `None`.
