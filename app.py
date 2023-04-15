import random
import time

import requests

MIN_SLEEP = 1 * 60
MAX_SLEEP = 3 * 60

URL = "https://docs.google.com/forms/d/e/1FAIpQLSfpDutphid_UDNqpn_dr61jYOdQhvIl_4bupW0IIzpt_MLlMw/formResponse"
RESPONSES = (
    "Arson 🔥",
    "War Crimes",
    "Joseph Evals Mama Thomas Abbate III",
    "I totally did not make a python script to submit these",
    "I forgor 💀",
    "What's voter fraud?",
    "💀💀💀💀💀💀💀💀💀💀💀💀💀💀💀💀💀💀💀💀",
    "FREEEEEEEEE BIRRRRRRRRRRRRRRD 🦅",
    "Hi, we are calling you about you're cars extended warrenty",
    "hiwearecallingyouaboutyourecarsextendedwarrenty",
    "I am once again asking for your financial support",
    "No Corner Crew, we will not play Free Bird",
    "🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥",
    "https://thatsa.skillissue.fyi",
    "cope seethe mald",
    "here if you want cs2 for free dont share it thats not official thats a site that exploits steam to get the cs2 beta but it works but you need prime for that and dont share it k thanks bye",
    "hewe if you want cs2 fow fwee dont s-shawe it thats n-nyot officiaw thats a-a site that e-expwoits steam t-to get the cs2 beta b-but it wowks b-but you nyeed pwime f-fow that and dont shawe it k thanks bye",
    "NOOOOO, YOU CANT JUST SCRRENSHOT AN NFT",
    "This NFT goes hard, feel free to screenshot",
    "NNOOOO, YOU CANT JUST PRINT MONEY, HAHA MONEY PRINTER GO BRRRRRRRRRRRRRRRRRRRRRRRRRRRR",
    "*Discord ping intensifies*",
    "Deez Nutz 🥜",
    "HA GOTTEEM",
    "DO IT FOR THE VINE!",
    "DO IT FOR THE BIT!",
    "freshavacado",
    "freeshavacado",
    "WHAT ARE THOSE!",
    "cum",
    "*Slack brush sweep sound intensifies*",
    "Ted Kaczynski",
    "Blueberry bagels",
    "Sesame bagles",
    "Cringle my Pringle",
    "Weggle my Eggle",
    "It would be highly illegal to say that I want to [REDACTED]",
    "Gollosano",
    "https://www.rochesterapex.com/",
    "I use Arch btw",
    "🤡🤡🤡🤡🤡🤡🤡🤡🤡🤡🤡🤡🤡🤡🤡🤡🤡🤡🤡🤡",
    "https://youtu.be/dQw4w9WgXcQ",
    "https://youtu.be/bZe5J8SVCYQ",
    "https://youtu.be/bzJDimvPW1Y"
)


def get_random_response():
    index = random.randint(0, len(RESPONSES))
    return RESPONSES[index]


def commit_fraud():
    submission = get_random_response()

    form_data = {
        "entry.492815402": "__other_option__",
        "entry.492815402.other_option_response": submission
    }

    response = requests.post(URL, data=form_data)
    print("Sent: " + submission)
    print(str(response) + "\n")


def main():
    while True:
        commit_fraud()

        sleep_time = random.randint(MIN_SLEEP, MAX_SLEEP)
        time.sleep(sleep_time)


if __name__ == "__main__":
    main()
