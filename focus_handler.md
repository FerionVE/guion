# Focus Handler

Focus Handler is a struct between event/render source and context.

The struct contains the id of the current widget.

# Hover Tracker

like Focus Handler the struct holds the hovered widget id.

It's responsible for generating MouseEnter and MouseLeave events.

On every mouse move passed through it checks
- if the position left the bound of the previous hovered 
- if it hovers a new sub widget of the previous