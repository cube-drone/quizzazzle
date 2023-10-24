import { h, Component, render} from 'preact';
import { useState } from 'preact/hooks'
import htm from 'htm';
import { marked } from 'marked';
import insane from 'insane';

const text = `
## The Skewered Dragon, Old Xoblob's Shop, and The Crime Scene

* **Xoblob** - Dark Gnome shopkeep, sells Trinkets and Potions. His shop has a large purple stuffed beholder in the window, and everything inside is painted purple.  He loves purple things and will overvalue them.  "Simple talk." Had a bad experience in the Underdark with some exploding bibberbang.
* * Potions: 50gp
* * SPIN THE WHEEL: 15gp
* * 1/10: healing potion!
* * 1/10: intellect potion!
* * 1/10: cheese potion!
* * 1/10: storm potion!
* * 6/10: trinket!
* * **You - musician, yes?** - "I have product... you look. " (he reaches behind the counter and pulls up a wooden pick, painted in blue and silver) "I have story. God - Tempus - make instrument. Most powerful instrument ever create. So loud heaven and earth rumble. This pick, hold in your hand, trust your heart, it help you find. Only five gold. Bargain."
* * _this is a real instrument, Calamity, but the pick is just a useless painted piece of wood_
* **Dockworkers**
* **Skewered Dragon Barkeep - Angus McTangus**
* * - he won't talk unless you pass some kind of charisma check or bribe him - drinks are 5cp, but they're 5gp if you're asking _questions_
* * There were two boys here: it's funny, one of them was dressed in fine noble's clothes - like, the sort of thing a prince would wear, but it was all felt and thin fabric and costume jewelry. The other one - he was wearing laborer's clothes but I didn't believe for a second he was a working man - his clothes were clean, pressed, crisp - imagine you asked the best tailor in town to make you look like a laborer. His hands were smooth, his skin was fair, his boots were new, I wasn't buying it.
* * When the two boys _(Renaer and Floon)_ left, there was a scuffle outside - seemed like a big fight.
* * A nasty fella crawled in after the fight, stab wounds on his arms. Now, this is a rough part of town so I'm no stranger to back-alley medicine, I managed to stitch him up some, let him stay the night.  _(sent a runner for the City Watch in the morning)_ - City Watch picked him up for questioning in the morning.
* * _(Doesn't want the Zhent to know he summoned the City Watch, might lie about it.)_
* **Three-Dragon Ante**
* **Captured Zhentarim Bandit - Jeffrey Criminal** - he'll tell you everything if you can get him away from the cops.
* * They were sent here to capture Renaer and bring him back to their warehouse on Castle Lane
* * They got Renaer, but Skiv was injured and they left him behind
* **Officer Hyustus Staget** - human **veteran**, two other cops, who are just **guards**.

### secrets
* The Police are investigating, and aren't friendly to amateur investigators
* Floon and a young princeling were both jumped by the Zhentarim
* The Zhentarim operate locally out of a warehouse on Castle Lane
* The young princeling had a fancy looking locket, maybe that's why they got jumped.

------

## Zhentarim Warehouse

* **Kenku**
* **Officer Hyustus Staget**
* **Renaer Neverember**, who looks just like Floon

### Z5 - Kenku Friend, "Kakau"
In one of the offices (Z5), a badly injured Kenku (3HP) is tied to a large pair of wooden wings.

"Oh, this one tried to free the prisoner? No stomach for torture, little one? That's okay. Nihiloor will understand, he's very reasonable. Boys - string him up - and hand me my bat."

### Let's Talk About Kenku

* **Initiative**: 4 <small> wow, bad roll</small>
* **AC**: 13
* **HP**: 13
* **Speed**: 30
* **STR**: 10 (0)
* **DEX**: 16 (3)
* **CON**: 10 (0)
* **INT**: 11 (0)
* **WIS**: 10 (0)
* **CHA**: 10 (0)
* **Deception**: +4
* **Perception**: +2
* **Stealth**: +5
* **Passive Perception**: 12
* **Ambusher**: The Kenku has advantage on attack rolls against Surprised opponents.
* **Mimicry**: Can mimic any sounds they've heard, including voices.
* **Shortsword**: **+5** to hit, **1d6+3** piercing damage.
* **Shortbow**: **+5** to hit, **1d6+3** piercing damage.

### Loot:

* secret room
* 1 crate of art, 4 paintings, (75gp each) - total 40lbs
* 1 crate of bars of silver, 15x10lbs each, each worth (50gp) - total 150lbs

### wut

Renaer is trapped in the closet in the back of the downstairs warehouse, **Z2**:

Renaer *may* admit that he's Renaer Neverember, he's clearly in a state of disarray but a bit cagey about it after the day he's had. He'll pretend to be Floon Dagmar but a Wis (Insight) check will clear that up.

If he admits he's Renaer, Yagra will be excited, but she's not sure exactly what she can get away with with a team of adventurers.

### If "Floon"

* Waiting for the Kenku to lose interest
* Jumped by Zhentarim thugs!
* Tied me up downstairs.
* Questioned Renaer upstairs. I slipped out of the rope.
* A bunch of monsters broke in. I used the distraction to hide in the closet.
* They killed the Zhentarim in here and took off with Renaer.

Yagra will offer to join the team on the hunt for Renaer.

### If Renaer:
* Waiting for the Kenku to lose interest
* Jumped by Zhentarim thugs!
* Tied me up downstairs.
* Mistook Floon for Renaer and questioned Floon upstairs. I slipped out of the rope.
* A bunch of monsters broke in. I used the distraction to hide in the closet.
* They killed the Zhentarim in here and took off with Floon.

Yagra has some health potions, which she'll offer to the party and Renaer - they're laced with Truth Serum. She'll ask the players if they're agents for any of the local factions. She'll ask Renaer leading questions about the fortune and if he knows anything about it. Once Renaer reveals that he knows nothing of value, Yagra will lose interest in the whole case - she'll pay the characters 50gp each - a small fortune- for their help finding Renaer and tell them that they've earned a favor from the Zhentarim - then leave.

If this doesn't work, Yagra will congratulate the team for finding Renaer, and offer them 50gp each. She'll "escort Renaer to safety" while the party goes to find Floon. If they turn her down, she'll offer to come along to keep everyone safe.

If the players aren't okay with this arrangement, she and Renaer with both offer to come along and help find Floon, and use this as an excuse to try and either abscond with Renaer or get the information that she needs from him organically.

One of the Crows has Renaer's possessions, which they looted off a Zhentarim: a gaudy locket of a unicorn, a purse containing 20gp, and a small black book listing a variety of interesting taverns and entertainment spots in Waterdeep. If you loot the body before finding Renaer, he'll ask for his things back.

If the players smash the locket, they'll find a Stone of Golorr and Yagra will immediately start plotting to get her hands on the stone.

If Renaer is asked for more details about why the Zhents might be looking for them (or if Yagra presses him, which she will), he'll tell them:
* his father, Lord Neverember, was Open Lord of Waterdeep for many years,
* and after he left, rumors started to circulate that he had embezzled a great deal of funds for his own personal use and left them hidden somewhere within the city.
* This is - obviously- nonsense, as I'll tell anybody who comes asking, but they were convinced I might know something.

### Renaer Statblock
_(Tiff controls Renaer)_

* **AC**: 17
* **HP**: 14
* **STR**: 12 (+1)
* **DEX**: 13 (+2)
* **CON**: 12 (+1)
* **INT**: 12 (+1)
* **WIS**: 10 (0)
* **CHA**: 10 (0)
* **Speed**: 30ft
* **Acrobatics**: +8
* **Athletics**: +5
* **Stealth**: +3
* **Knowledge (History)**: +6
* **Lightfooted**: Can dash or disengage as a bonus action
* **Attack**: normally higher, but for the sake of the game, let's say...
* **Rapier**: **+4** to hit, **1d8+4** piercing damage.

### The Watch Arrives

The police show up! They were following up on the same leads that the characters were!

* **Officer Hyustus Staget** is back
* This building was supposed to be under surveillance but apparently the patrols have been skipping it.
* **secret**: They're looking for Urstul Floxin, a high-level Zhentarim gang-boss
* if Renaer Neverember is alive and well, the players will get off scot-free and the police will be cool and chill, if not, they've got a LOT of explaining to do
* the police don't want to get deeply involved in a gang war or go into the sewers. They'll advise the characters "keep the blood off the streets, okay?".
`;

const rendered = insane(marked.parse(text));

const html = htm.bind(h);

let Rendered = () => {
    return html`<div class="hi" dangerouslySetInnerHTML=${{ __html: rendered }}></div>`;
}

let Quiz = () => {
    let [index, setIndex] = useState(0);
    let items = [];
    for(let i = 0; i < 100; i++){
        items.push(i)
    }
    let counter = 0;
    return html`<div class="card">
        <div class="content">
            <div class="everything">
            <h2>Hi!</h2>
            <${Rendered} />
            </div>
        </div>
    </div>`
}
console.log("Marts ahoy")

render(html`<${Quiz} name="World" />`, document.getElementById('app'));