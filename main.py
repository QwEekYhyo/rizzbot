from rizzbotv2 import RizzBot, keyboard


bot = RizzBot("logan", lines=["netflix & chill ?", "on baise ?", "wallah t bonne",
                  "quoicoubea <3", "je bench 100kg"])
keyboard.add_hotkey('escape', bot.stop)
bot.calibrate(0, (994, 817))
bot.calibrate(1, (1116, 847))
bot.calibrate(2, (970, 400))
bot.calibrate(3, (1000, 840))
bot.start(with_text=True)
