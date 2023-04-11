# requirements : mouse, keyboard, clipboard

import mouse
import keyboard
import random
import clipboard
from time import time

class RizzBot:
    def __init__(self, target, posing_time = 0, lines = []):
        self.target = target
        self.lines = lines
        self.movements = [0, 0, 0, 0]
        self.working = False
        self.posing_time = posing_time

    def change_target(self, new_target):
        self.target = new_target

    def change_lines(self, new_lines):
        self.lines = new_lines

    def set_posing_time(self, new_time):
        self.posing_time = new_time

    def calibrate(self, index, coord):
        self.movements[index] = mouse.MoveEvent(coord[0], coord[1], 10)

    def is_calibrated(self):
        return all([isinstance(self.movements[i], mouse.MoveEvent) for i in
             range(len(self.movements))])

    def snap(self):
        if self.is_calibrated():
            wait(self.posing_time)
            click_to(self.movements[0:2])
            keyboard.write(self.target)
            wait(0.4)
            click_to(self.movements[2:])

    def snap_wtext(self):
        if self.is_calibrated():
            wait(self.posing_time)
            text_pos = mouse.MoveEvent(self.movements[0].x,
                                       random.randint(self.movements[2].y - 100,
                                                      self.movements[1].y - 40), 10)
            click_to([self.movements[0], text_pos])
            rdm = random.choice(self.lines)
            clipboard.copy(rdm)
            keyboard.send("ctrl+v")
            wait(0.5)
            click_to([self.movements[1]])
            keyboard.write(self.target)
            click_to(self.movements[2:])

    def start(self, with_text = False):
        self.working = True
        while self.working:
            if with_text:
                self.snap_wtext()
            else:
                self.snap()

    def stop(self):
        self.working = False


click = lambda: mouse.click(button='left')

def click_to(arr):
  for event in arr:
    mouse.play([event])
    click()
    wait(0.2)

def wait(seconds):
  start = time()
  delta = 0
  while (delta < seconds):
    delta = time() - start
