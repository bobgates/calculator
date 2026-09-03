# Overall plan

## main

**Main** sets up the physical links to the peripherals, including the hardware interface to the *display* and it sets up the hardware for the keyboard, creating a *keyboard* struct.  It creates a *line editer* which is the code component that processes key presses when editing a new number entry. **Main** also owns the stack.

It then goes into a loop, forever:

- delay - this is to allow time between key presses and because key presses take long compared to Pi Pico operation.0
- scan the keyboard and await
- if there are no key presses, delay and scan again.
- Inside main is the state machine. It contains two states at the moment: *Entry* and *Calculating*. 

## Line_edit

Line edit only contains:

- a mutable reference to the stack
- an enum called State that knows what state the system is in: Entry or Calculating at the moment - others may be added.
- editing: bool
- line: String<EDIT_LENGTH>

### Line_edit::process_key

*process_key* intially differentiates between two states: *editing* and *calculating*. It then hands the key on to either

- process_number_key or
- process_calculate_key.

## Display

- Display owns the stack.
- It also owns the code that puts data out in the correct form for the display.
- Finally display owns Line_edit


## Stack ownership:
The intent is for the stack to live in main. It can then be set to display when required, and potentially just update bits of the display?

