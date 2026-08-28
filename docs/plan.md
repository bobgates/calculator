# Overall plan

## main

**Main** sets up the physical links to the peripherals, including the hardware interface to the display and it sets up the hardware for the keyboard, creating a *keyboard* struct. Finally it creates a *line editer* which is the code component that processes key presses when editing a new number entry.

It then goes into a loop, forever:

- delay - this is to allow time between key presses and because key presses take long compared to Pi Pico operation.0
- scan the keyboard and await
- if there are no key presses, delay and scan again.

## Line_edit

Line edit only contains:

- a mutable reference to the stack
- an enum called State that knows what state the system is in: Editing or Calculating at the moment - others may be added.
- editing: bool
- line: String<EDIT_LENGTH>

### Line_edit::process_key

*process_key* intially differentiates between two states: *editing* and *calculating. It then hands the key on to either

- process_number_key or
- process_calculate_key.

## Display

- Display owns the stack.
- It also owns the code that puts data out in the correct form for the display.
- Finally display owns Line_edit


## Stack ownership:



<!-- Trying to move hardware definition out of main. Going to hack away most of the system to achieve it.
 -->