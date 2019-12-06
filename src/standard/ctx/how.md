events are generally redirected to widget's event handlers

States:

- PressedState: pressed mouse buttons, stores for each key
    - key id
    - start WidgetID
    - start pos
- HoverState
    - hovered WidgetID or None
- DragState

Fns:
- WindowMouseLeave
    - if DragState
        - try serialize DragData to clipboard
    - Reset HoverState
    - Reset DragState
    - set mouse off to None
- WindowMouseMove
    - if mouse off is none (so the mouse just entered window (again))
        - Try Read Serialized Drag from Clipboard
    - send MouseMove into root widgets
    - the last set WidgetID is the hover candidate
    - if key 0 is down (PressedState) and DragDistance is reached
        - set DragState to current WidgetID
    - if DragState
        - probe if currently hovered Widget supports Drop
- MouseMove
    - Set current WidgetID as hovered in HoverState
    - Call WidgetEventHandler

- WindowMouseDown
    - set key and pos in PressedState
    - send MouseDown into root widgets
    - set pressed WidgetID as selected in SelectedState
- MouseDown
    - Set current WidgetID in PressedState at current key
    - Call WidgetEventHandler

- WindowMouseUp

    - if mouse key 0
        - reset DragState



SelectionIterFwd: