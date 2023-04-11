from tkinter import *              # I will clean these imports later
from tkinter.ttk import *

class  RizzBotGUI(Tk):

    def __init__(self):
        super().__init__()
        self.initializeUI()

    def initializeUI(self):
        self.title("RizzBot")
        self.minsize(300, 200)  # width, height
        self.geometry("400x350")
        self.setupWindow()

    def setupWindow(self):
        """ Set up the widgets."""
        title = Label(self, text="RizzBot", border=10)
        title.pack()

        line = Separator(self, orient=HORIZONTAL)
        line.pack(fill='x')

        main_frm = Frame(self, padding=10)
        Label(main_frm, text="This is the main frame").grid(column=0, row=0)
        Button(main_frm, text="Click me pls").grid(column=1, row=0)
        main_frm.pack()

### MAIN ###

gui = RizzBotGUI()
gui.mainloop()
