import keyboard
import webbrowser
import pyautogui
import time
import csv

# most jank shit i've ever written lmfao
# works on MacOS at my scaling, i'm using firefox

with open('domestic_list.csv', 'r') as file:
    reader = csv.DictReader(file)
    for row in reader:

        name = row['First Name'] + " " + row['Last Name']
        phone_number = row['Phone Number']
        address = row['Address (line 1)']
        city = row['City']
        url = "https://www.amazon.com/gp/product/B08PZBPXLZ"
        firefox_path = 'open -a /Applications/Firefox.app %s'

        webbrowser.get(firefox_path).open(url)

        pyautogui.moveTo(1900, 830)
        time.sleep(2)
        pyautogui.click()
        pyautogui.moveTo(920, 1140)
        time.sleep(5)
        pyautogui.click()

        for i in range(3):
            keyboard.press_and_release('tab')
            time.sleep(0.2)


        keyboard.write(name)
        time.sleep(0.5)
        keyboard.press_and_release('tab')
        time.sleep(0.5)
        keyboard.write(phone_number)
        time.sleep(0.5)
        keyboard.press_and_release('tab')
        time.sleep(0.5)
        keyboard.write(address)
        time.sleep(0.5)
        keyboard.write(", ")
        time.sleep(0.5)
        keyboard.write(city)
        time.sleep(2)
        keyboard.press_and_release('down_arrow')
        time.sleep(0.5)
        keyboard.press_and_release('enter')

        for i in range(5):
            keyboard.press_and_release('tab')

        keyboard.press_and_release('down_arrow')

        for i in range(2):
            keyboard.press_and_release('tab')

        keyboard.press_and_release('enter')

        time.sleep(7)
        for i in range (5):
                jump = i * 25
                pyautogui.moveTo(860, 1250 + jump)
                time.sleep(1)
                pyautogui.click()
                time.sleep(0.5)
        time.sleep(5)