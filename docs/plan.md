# Overall plan

## main

**Main** sets up the physical links to the peripherals, including the hardware interface to the display and it sets up the hardware for the keyboard, creating a *keyboard* struct. Finally it creates a *line editer* which is the code component that processes key presses when editing a new number entry.

It then goes into a loop, forever:

- delay - this is to allow time between key presses and because key presses take long compared to Pi Pico operation.0
- scan the keyboard and await
- if there are no key presses, delay and scan again.

- if there are key presses:
    go to *line_edit.process key*

## Line_edit
Line edit only contains two variables:
- editing: bool 
- line: String

### Line_edit::process_key
*process_key* intially differentiates between two states: *editing* and *calculating .

** I should convert this to an enum to allow for other states in the future. DONE! **


