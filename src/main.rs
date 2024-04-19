use std::{collections::HashMap, ops::RangeInclusive, thread::sleep, time::Duration};

use rand::prelude::*;
use reqwest::Client;

const MIN_SLEEP: u64 = 1 * 60; // 1 minute
const MAX_SLEEP: u64 = 3 * 60; // 3 minutes

const URL: &str =    "https://docs.google.com/forms/d/e/1FAIpQLSeoXhwL7aF7yLj-wR37r8JZtOlzmmJchTarlLcl1LicYCNuMg/formResponse"        ;
const ROBOT_RANGE: RangeInclusive<u16> = 0..=1000;
const RESPONSES: [&str; 70] = [
    "Arson 🔥",
    "War Crimes",
    "Joseph Evals Mama Thomas Abbate III",
    "I totally did not make a python script to submit these",
    "I forgor 💀",
    "What's voter fraud?",
    "💀💀💀💀💀💀💀💀💀💀💀💀💀💀💀💀💀💀💀💀",
    "FREEEEEEEEE BIRRRRRRRRRRRRRRD 🇺🇸🦅",
    "Hi, we are calling you about you're cars extended warrenty",
    "hiwearecallingyouaboutyourecarsextendedwarrenty",
    "I am once again asking for your financial support",
    "No Corner Crew, we will not play Free Bird",
    "🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥",
    "https://thatsa.skillissue.fyi",
    "https://cringe.fyi",
    "https://youre.cringe.fyi",
    "https://thats.cringe.fyi",
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
    "I rewrote this script in Rust 🦀🦀🦀🦀🦀",
    "Joe Bidome",
    "Obama Prism",
    "Donald Trumpet",
    "George Bush",
    "SOPT PUTTING CHEMICALS IN THE WATER, ITS TURNING THE FRGOS GAY",
    "You're netflex account is under review due to suspicious activity, please make a one time donation of 5 bitcoin to verify youre account: oggfghhjgbdaasdlkjfhweqrhlgfyusegresayfurqogiurgqol",
    "Hey Corner Crew, what do we do now?",
    "Crazy? I was crazy once.....",
    "I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB I LOVE MATLAB",
    "I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH I LOVE DESCRETE MATH",
    "The only way to find out is to fuck around so fuck around as much as you can",
    "I was thinking about why so many in the radical left participate in \"speedrunning\" The reason is the left's lack of work ethic ('go fast' rather than 'do it right') and, in a Petersonian sense, to elevate alternative sexual archetypes in the marketplace ('fastest mario') Obviously, there are exceptions to this and some people more in the center or right also \"speedrun\". However, they more than sufficient to prove the rule, rather than contrast it. Consider how woke GDQ has been, almost since the very beginning. Your eyes will start to open. Returning to the topic of the work ethic... A \"speedrunner\" may well spend hours a day at their craft, but this is ultimately a meaningless exercise, since they will ultimately accomplish exactly that which is done in less collective time by a casual player. This is thus a waste of effort on the behalf of the \"speedrunner\". Put more simply, they are spending their work effort on something that someone else has already done (and done in a way deemed 'correct' by the creator of the artwork). Why do they do this? The answer is quite obvious if you think about it. The goal is the illusion of speed and the desire (SUBCONSCIOUS) to promote radical leftist, borderline Communist ideals of how easy work is. Everyone always says that \"speedruns\" look easy. That is part of the aesthetic. Think about the phrase \"fully automated luxury Communism\" in the context of \"speedrunning\" and I strongly suspect that things will start to 'click' in your mind. What happens to the individual in this? Individual accomplishment in \"speedrunning\" is simply waiting for another person to steal your techniques in order to defeat you. Where is something like \"intellectual property\" or \"patent\" in this necessarily communitarian process? Now, as to the sexual archetype model and 'speedrunning' generally... If you have any passing familiarity with Jordan Peterson's broader oeuvre and of Jungian psychology, you likely already know where I am going with this. However, I will say more for the uninitiated. Keep this passage from Maps of Meaning (91) in mind: \"The Archetypal Son... continually reconstructs defined territory, as a consequence of the 'assimilation' of the unknown [as a consequence of 'incestuous' (that is, 'sexual' - read creative) union with the Great Mother]\" In other words, there is a connection between 'sexuality' and creativity that we see throughout time (as Peterson points out with Tiamat and other examples). In the sexual marketplace, which archetypes are simultaneously deemed the most creative and valued the highest? The answer is obviously entrepreneurs like Elon Musk and others. Given that we evolved and each thing we do must have an evolutionary purpose (OR CAUSE), what archetype is the 'speedrunner' engaged in, who is accomplishing nothing new? They are aiming to make a new sexual archetype, based upon 'speed' rather than 'doing things right' and refuse ownership of what few innovations they can provide to their own scene, denying creativity within their very own sexual archetype. This is necessarily leftist. The obvious protest to this would be the 'glitchless 100% run', which in many ways does aim to play the game 'as intended' but seems to simply add the element of 'speed' to the equation. This objection is ultimately meaningless when one considers how long a game is intended to be played, in net, by the creators, even when under '100%' conditions. There is still time and effort wasted for no reason other than the ones I proposed above. By now, I am sure that I have bothered a number of you and rustled quite a few of your feathers. I am not saying that 'speedrunning' is bad, but rather that, thinking about the topic philosophically, there are dangerous elements within it. That is all.",
    "Did you ever hear the tragedy of Darth Plagueis The Wise? I thought not. It's not a story the Jedi would tell you. It's a Sith legend. Darth Plagueis was a Dark Lord of the Sith, so powerful and so wise he could use the Force to influence the midichlorians to create life… He had such a knowledge of the dark side that he could even keep the ones he cared about from dying. The dark side of the Force is a pathway to many abilities some consider to be unnatural. He became so powerful… the only thing he was afraid of was losing his power, which eventually, of course, he did. Unfortunately, he taught his apprentice everything he knew, then his apprentice killed him in his sleep. Ironic. He could save others from death, but not himself.",
    "My name is Walter Hartwell White. I live at 308 Negra Arroyo Lane, Albuquerque, New Mexico, 87104. This is my confession. If you're watching this tape, I'm probably dead- murdered by my brother-in-law, Hank Schrader. Hank has been building a meth empire for over a year now, and using me as his chemist. Shortly after my 50th birthday, he asked that I use my chemistry knowledge to cook methamphetamine, which he would then sell using connections that he made through his career with the DEA. I was... astounded. I... I always thought Hank was a very moral man, and I was particularly vulnerable at the time - something he knew and took advantage of. I was reeling from a cancer diagnosis that was poised to bankrupt my family. Hank took me in on a ride-along and showed me just how much money even a small meth operation could make. And I was weak. I didn't want my family to go into financial ruin, so I agreed. Hank had a partner, a businessman named Gustavo Fring. Hank sold me into servitude to this man. And when I tried to quit, Fring threatened my family. I didn't know where to turn. Eventually, Hank and Fring had a falling-out. Things escalated. Fring was able to arrange - uh, I guess... I guess you call it a \"hit\" - on Hank, and failed, but Hank was seriously injured. And I wound up paying his medical bills, which amounted to a little over $177,000. Upon recovery, Hank was bent on revenge. Working with a man named Hector Salamanca, he plotted to kill Fring. The bomb that he used was built by me, and he gave me no option in it. I have often contemplated suicide, but I'm a coward. I wanted to go to the police, but I was frightened. Hank had risen to become the head of the Albuquerque DEA. To keep me in line, he took my children. For three months, he kept them. My wife had no idea of my criminal activities, and was horrified to learn what I had done. I was in hell. I hated myself for what I had brought upon my family. Recently, I tried once again to quit, and in response, he gave me this. [Walt points to the bruise on his face left by Hank in \"Blood Money.\"] I can't take this anymore. I live in fear every day that Hank will kill me, or worse, hurt my family. All I could think to do was to make this video and hope that the world will finally see this man for what he really is.",
    "The phrase \"it's just a game\" is such a weak mindset. You are ok with what happened, losing, imperfection of a craft. When you stop getting angry after losing, you've lost twice. There's always something to learn, and always room for improvement, never settle.",
    "What the fuck did you just fucking say about me, you little bitch? I'll have you know I'm at the top of my class, and I've been involved in numerous leadership roles in my school, and I have over 300 college recommendations. I am trained in writing essays and I'm the top essayist in the entire US. You are nothing to me but just another anti-Intellectual. I will wipe your chances of ever being on the top of your class mark my fucking words. You think you can get away with saying that shit to me over the Internet? Think again, fucker. As we speak I am contacting my AP teachers and my network of college principals across the US and your IP is being traced right now so you better prepare for the storm, maggot. The storm that wipes out the pathetic little thing you call your education. You're fucking uneducated, kid. I can be anywhere, anytime, and I can ruin your chances of getting into Princeton in over seven hundred ways, and that's just with my bare hands. Not only am I extensively trained in memorizing SAT words, but I have access to the entire application process of Harvard University and I will use it to its full extent to wipe your little miserable education off the face of the continent, you little shit. If only you could have known what unholy retribution your little \"clever\" comment was about to bring down upon you, maybe you could have held your fucking tongue. But you couldn't, you didn't, and now you're paying the price, you goddamn idiot. I will shit fury all over you and you will drown in it. You're fucking uneducated, kiddo.",
    "Today when I walked into my economics class I saw something I dread every time I close my eyes. Someone had brought their new gaming laptop to class. The Forklift he used to bring it was still running idle at the back. I started sweating as I sat down and gazed over at the 700lb beast that was his laptop. He had already reinforced his desk with steel support beams and was in the process of finding an outlet for a power cable thicker than Amy Schumer's thigh. I start shaking. I keep telling myself I'm going to be alright and that there's nothing to worry about. He somehow finds a fucking outlet. Tears are running down my cheeks as I send my last texts to my family saying I love them. The teacher starts the lecture, and the student turns his laptop on. The colored lights on his RGB Backlit keyboard flare to life like a nuclear flash, and a deep humming fills my ears and shakes my very soul. The entire city power grid goes dark. The classroom begins to shake as the massive fans begin to spin. In mere seconds my world has gone from vibrant life, to a dark, earth shattering void where my body is getting torn apart by the 150mph gale force winds and the 500 decibel groan of the cooling fans. As my body finally surrenders, I weep, as my school and my city go under. I fucking hate gaming laptops.",
    "Wowwwww, you meow like a cat! That means you are one, right? Shut the fuck up. If you really want to be put on a leash and treated like a domestic animal then that's called a fetish, not \"quirky\" or \"cute\". What part of you seriously thinks that any part of acting like a feline establishes a reputation of appreciation? Is it your lack of any defining aspect of personality that urges you to resort to shitty representations of cats to create an illusion of meaning in your worthless life? Wearing \"cat ears\" in the shape of headbands further notes the complete absence of human attribution to your false sense of personality, such as intelligence or charisma in any form or shape. Where do you think this mindset's gonna lead you? You think you're funny, random, quirky even? What makes you think that acting like a fucking cat will make a goddamn hyena laugh? I, personally, feel extremely sympathetic towards you as your only escape from the worthless thing you call your existence is to pretend to be an animal. But it's not a worthy choice to assert this horrifying fact as a dominant trait, mainly because personality traits require an initial personality to lay their foundation on. You're not worthy of anybody's time, so go fuck off, \"cat-girl\".",
    "So I (74M) was recently hit by a car (2014 Honda) and died. My wife (5F) organized me a funeral (cost $2747) without asking me (74M) at all. I (74M) was unable to make it because I (74M) was dead (17 days). At the funeral I heard my dad (15M) and other family members talking about how they wish I could be there and now I feel bad for not showing up. AITA?",
    "\"YoU tOucHEd ThAt BlOcK So yOu HavE tO PuLl tHat OnE Out\" Go fuck yourself, that's not how the fucking game is played, you dumb, the fuck, asshole. Quoted from the official Jenga rules: \"Players may tap a block to find a loose one. Any blocks moved but not played should be replaced, unless doing so would make the tower fall.\" You've never even fucking read the rules have you, you shithead idiot. What, is the game over in 3 seconds, if you just so happen to touch a load bearing block first?FUCKING NO DUMBASS. Learn to read you illiterate fuck.",
    "I sexually Identify as the \"I sexually identify as an attack helicopter\" joke. Ever since I was a child, I've dreamed of flippantly dismissing any concepts or discussions regarding gender that don't fit in with what I learned in 8th grade bio. People say to me that this joke hasn't been funny since 2014 and please at least come up with a new one, but I don't care, I'm hilarious. I'm having a plastic surgeon install Ctrl, C, and V keys on my body. From now on I want you guys to call me \"epic kek dank meme trannies owned with facts and logic\" and respect my right to shit up social media. If you can't accept me you're a memeophobe and need to check your ability-to-critically-think privilege. Thank you for being so understanding."
];

fn get_random_reponse(random_source: &mut dyn RngCore) -> &'static str {
    RESPONSES.choose(random_source).unwrap()
}

async fn commit_fraud(random_source: &mut dyn RngCore) {
    let mut form_submission: HashMap<&str, &str> = HashMap::new();

    let name = format!("Robot {}", random_source.gen_range(ROBOT_RANGE).to_string());
    let response = get_random_reponse(random_source);
    let best_bird = format!("Option {}", random_source.gen_range(1..=4).to_string());

    form_submission.insert("entry.746961800", &name);
    form_submission.insert("entry.1385752702", "__other_option__");
    form_submission.insert("entry.1385752702.other_option_response", &response);
    form_submission.insert("entry.41364184", &best_bird);

    println!(
        "Sending:\n\tName: {}\n\tResponse: {}\n\tBest Bird: {}",
        &name, &response, &best_bird
    );

    match Client::new().post(URL).form(&form_submission).send().await {
        Ok(response) => println!("Status: {:?}\n", response.status()),
        Err(err) => println!("Status: {:?}\n{}\n", err.status(), err),
    };
}

#[tokio::main]
async fn main() {
    let mut random_source = thread_rng();

    loop {
        commit_fraud(&mut random_source).await;

        let sleep_duration = random_source.gen_range(MIN_SLEEP..=MAX_SLEEP);
        sleep(Duration::from_secs(sleep_duration));
    }
}
