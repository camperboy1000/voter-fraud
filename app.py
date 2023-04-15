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
    "https://youtu.be/bzJDimvPW1Y",
    "What the fuck did you just fucking say about me, you little bitch? I'll have you know I graduated top of my class in the Navy Seals, and I've been involved in numerous secret raids on Al-Quaeda, and I have over 300 confirmed kills. I am trained in gorilla warfare and I'm the top sniper in the entire US armed forces. You are nothing to me but just another target. I will wipe you the fuck out with precision the likes of which has never been seen before on this Earth, mark my fucking words. You think you can get away with saying that shit to me over the Internet? Think again, fucker. As we speak I am contacting my secret network of spies across the USA and your IP is being traced right now so you better prepare for the storm, maggot. The storm that wipes out the pathetic little thing you call your life. You're fucking dead, kid. I can be anywhere, anytime, and I can kill you in over seven hundred ways, and that's just with my bare hands. Not only am I extensively trained in unarmed combat, but I have access to the entire arsenal of the United States Marine Corps and I will use it to its full extent to wipe your miserable ass off the face of the continent, you little shit. If only you could have known what unholy retribution your little \"clever\" comment was about to bring down upon you, maybe you would have held your fucking tongue. But you couldn't, you didn't, and now you're paying the price, you goddamn idiot. I will shit fury all over you and you will drown in it. You're fucking dead, kiddo.",
    "Just rewrite it in Rust 🦀🦀🦀🦀🦀",
    "Joe Bidome",
    "Obama Prism",
    "Donald Trumpet",
    "George Bush",
    "SOPT PUTTING CHEMICALS IN THE WATER, ITS TURNING THE FRGOS GAY",
    "You're netflex account is under review due to suspicious activity, please make a one time donation of 5 bitcoin to verify youre account: oggfghhjgbdaasdlkjfhweqrhlgfyusegresayfurqogiurgqol",
    "Hey Corner Crew, what do we do now?",
    "I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB",
    "I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH",
    "The only way to find out is to fuck around so fuck around as much as you can",
    "I was thinking about why so many in the radical left participate in \"speedrunning\" The reason is the left's lack of work ethic ('go fast' rather than 'do it right') and, in a Petersonian sense, to elevate alternative sexual archetypes in the marketplace ('fastest mario') Obviously, there are exceptions to this and some people more in the center or right also \"speedrun\". However, they more than sufficient to prove the rule, rather than contrast it. Consider how woke GDQ has been, almost since the very beginning. Your eyes will start to open. Returning to the topic of the work ethic... A \"speedrunner\" may well spend hours a day at their craft, but this is ultimately a meaningless exercise, since they will ultimately accomplish exactly that which is done in less collective time by a casual player. This is thus a waste of effort on the behalf of the \"speedrunner\". Put more simply, they are spending their work effort on something that someone else has already done (and done in a way deemed 'correct' by the creator of the artwork). Why do they do this? The answer is quite obvious if you think about it. The goal is the illusion of speed and the desire (SUBCONSCIOUS) to promote radical leftist, borderline Communist ideals of how easy work is. Everyone always says that \"speedruns\" look easy. That is part of the aesthetic. Think about the phrase \"fully automated luxury Communism\" in the context of \"speedrunning\" and I strongly suspect that things will start to 'click' in your mind. What happens to the individual in this? Individual accomplishment in \"speedrunning\" is simply waiting for another person to steal your techniques in order to defeat you. Where is something like \"intellectual property\" or \"patent\" in this necessarily communitarian process? Now, as to the sexual archetype model and 'speedrunning' generally... If you have any passing familiarity with Jordan Peterson's broader oeuvre and of Jungian psychology, you likely already know where I am going with this. However, I will say more for the uninitiated. Keep this passage from Maps of Meaning (91) in mind: \"The Archetypal Son... continually reconstructs defined territory, as a consequence of the 'assimilation' of the unknown [as a consequence of 'incestuous' (that is, 'sexual' – read creative) union with the Great Mother]\" In other words, there is a connection between 'sexuality' and creativity that we see throughout time (as Peterson points out with Tiamat and other examples). In the sexual marketplace, which archetypes are simultaneously deemed the most creative and valued the highest? The answer is obviously entrepreneurs like Elon Musk and others. Given that we evolved and each thing we do must have an evolutionary purpose (OR CAUSE), what archetype is the 'speedrunner' engaged in, who is accomplishing nothing new? They are aiming to make a new sexual archetype, based upon 'speed' rather than 'doing things right' and refuse ownership of what few innovations they can provide to their own scene, denying creativity within their very own sexual archetype. This is necessarily leftist. The obvious protest to this would be the 'glitchless 100% run', which in many ways does aim to play the game 'as intended' but seems to simply add the element of 'speed' to the equation. This objection is ultimately meaningless when one considers how long a game is intended to be played, in net, by the creators, even when under '100%' conditions. There is still time and effort wasted for no reason other than the ones I proposed above. By now, I am sure that I have bothered a number of you and rustled quite a few of your feathers. I am not saying that 'speedrunning' is bad, but rather that, thinking about the topic philosophically, there are dangerous elements within it. That is all."
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
