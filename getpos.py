import mouse
import keyboard

def on_esc():
  global working
  working = False

def save_pos():
  positions.append(mouse.get_position())
  print("position saved")

keyboard.add_hotkey('escape', on_esc)
keyboard.add_hotkey('k', save_pos)

positions = []

working = True
while (working):
  print(mouse.get_position())
print(positions)
