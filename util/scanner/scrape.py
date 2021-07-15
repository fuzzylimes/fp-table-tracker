import time
from sys import exit
from os import (environ, devnull, uname)
from selenium import webdriver
from selenium.common.exceptions import TimeoutException
from selenium.webdriver.firefox.options import Options
from selenium.webdriver.common.by import By
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC
from selenium.webdriver.common.keys import Keys
import pymongo
url = "https://www.foxwoods.com/casino/choose/poker/"

# Setup our mongo connection
mongoUser = environ.get('MONGO_USER')
mongoPass = environ.get('MONGO_PASS')
mongoUrl = environ.get('MONGO_URL')
mongoDB = environ.get('MONGO_DB')
if not mongoUser or not mongoPass or not mongoUrl or not mongoDB:
    exit("mongo environment variables not found!")
mongoConnString = "mongodb+srv://{}:{}@{}/{}?retryWrites=true&w=majority".format(mongoUser, mongoPass, mongoUrl, mongoDB)
conn = pymongo.MongoClient(mongoConnString)
db = conn.foxwoods
print("MongoDB connection established")

# Prepare or db object
values = {}

# Handle virtual display on raspberry pi
display = {}
if 'arm' in uname().machine:
    from pyvirtualdisplay import Display
    display = Display(visible=0, size=(800, 600))
    display.start()

# Handle scrape via selenium
options = Options()
options.headless = True
ts = int(time.time())
driver = webdriver.Firefox(options=options, service_log_path=devnull)
driver.get(url)
assert "Poker" in driver.title
html = driver.find_element_by_tag_name('html')
html.send_keys(Keys.END)
# Handle page scan
try:
    # Wait for the table to load
    element = WebDriverWait(driver, 10).until(
        EC.presence_of_element_located(
            (By.CSS_SELECTOR, "#content > section > div:nth-child(4) > div.table-grid > div > div > table > tbody > tr.title"))
    )
    time.sleep(1)
    table = driver.find_element_by_css_selector(
        "#content > section > div:nth-child(4) > div.table-grid > div > div > table > tbody")
    data = [val.text for val in table.find_elements_by_tag_name("td")][3:]
    games = {}
    tables = 0
    # Iterate through the list of games and build our response object
    for i in range(0, len(data), 3):
        t = int(data[i][1:-1])
        tables += t
        gameName = data[i+2].replace(".", "_")
        if gameName not in games:
            games[gameName] = []
        games[gameName].append({'tableCount': t, 'blinds': data[i+1]})
    values = {"ts": ts, "games": games, "tableCount": tables}
    print("Scraping completed")
except TimeoutException as e:
    print("No active tables")
    values = {"ts": ts, "games": {}, "tableCount": 0}
finally:
    print("Writing to DB...")
    result = db.poker.insert_one(values)
    print("Created record:\n{}".format(values))
    driver.quit()
    display.stop()
    conn.close()
