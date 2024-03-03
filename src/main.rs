mod components;
use crate::components::*;

use rand::prelude::*;

use leptos::*;

use stylers::style_sheet;

struct Character {
    health: i8,
    strength: u8,
    agility: u8,
    endurance: u8,
    block: bool,
}

fn determine_attack(power: u8, agility: u8) -> u8 {
    let random_number = rand::thread_rng().gen::<f64>();
    let max_attack = (power + agility) as f64;
    (random_number * max_attack) as u8
}

const IMG_OPPONENTS: [&str; 11] = [
    "Tony.jpg",
    "Tony2.png",
    "Crocodile.png",
    "Richard.jpg",
    "Rasmus.png",
    "Rick.png",
    "Bat.jpg",
    "Fish.png",
    "Giraffe.png",
    "Pig.png",
    "Pantera.jpg",
];

const DEFAULT_STATS_CHARACTER: Character = Character {
    health: 100,
    strength: 20,
    agility: 10,
    endurance: 5,
    block: false,
};

#[component]
fn App() -> impl IntoView {
    let class_name = style_sheet!("./styles.css");

    let (username, set_username) = create_signal("---".to_string());
    let (game_started, set_game_started) = create_signal(false);

    let (player_stats, set_player_stats) = create_signal(DEFAULT_STATS_CHARACTER);
    let (opponent_stats, set_opponent_stats) = create_signal(DEFAULT_STATS_CHARACTER);

    let (game_message, set_game_message) = create_signal("...".to_string());

    let (round, set_round) = create_signal(1);
    let (img_opponent, set_img_opponent) = create_signal("img/Tony.jpg".to_string());

    let opponent_attack = move || {
        let opponent_attack = determine_attack(
            opponent_stats.with(|x| x.strength),
            opponent_stats.with(|x| x.agility),
        );

        if player_stats.with(|x| x.block) {
            set_player_stats.update(move |x| x.health -= (opponent_attack / x.endurance) as i8);
            if player_stats.with(|x| x.endurance) > 1 {
                set_player_stats.update(move |x| x.endurance -= 1);
            }
            set_player_stats.update(move |x| x.block = false);
        } else {
            set_player_stats.update(move |x| x.health -= opponent_attack as i8);
        }

        if player_stats.with(|x| x.health) < 0 {
            set_player_stats.update(move |x| x.health = 0);
            set_game_message.set("You lost".to_string());
        }

        set_opponent_stats.update(move |x| x.strength = x.strength - 5);
        set_opponent_stats.update(move |x| x.block = false);
    };

    let opponent_block = move || {
        set_opponent_stats.update(move |x| x.block = true);
        set_opponent_stats.update(move |x| x.strength = x.strength + 5);
    };

    let opponent_action = move || {
        if opponent_stats.with(|x| x.strength == 0) || rand::thread_rng().gen::<f64>() > 0.5 {
            set_game_message.update(move |x| *x = "Opponent blocks".to_string());
            opponent_block();
        } else {
            set_game_message.update(move |x| *x = "Opponent attacks".to_string());
            opponent_attack();
        }
    };

    let attack = move |_| {
        let player_attack = determine_attack(
            player_stats.with(|x| x.strength),
            player_stats.with(|x| x.agility),
        );

        if opponent_stats.with(|x| x.block) {
            set_opponent_stats.update(move |x| x.health -= (player_attack / x.endurance) as i8);
            if opponent_stats.with(|x| x.endurance) > 1 {
                set_opponent_stats.update(move |x| x.endurance -= 1);
            }
            set_opponent_stats.update(move |x| x.block = false);
        } else {
            set_opponent_stats.update(move |x| x.health -= player_attack as i8);
        }

        if opponent_stats.with(|x| x.health) < 0 {
            set_opponent_stats.update(move |x| x.health = 0);
            set_game_message.set("You won".to_string());
        }

        set_player_stats.update(move |x| x.strength = x.strength - 5);
        set_player_stats.update(move |x| x.block = false);

        if opponent_stats.with(|x| x.health) > 0 {
            opponent_action();
        }
    };

    let block = move |_| {
        set_player_stats.update(move |x| x.block = true);
        set_player_stats.update(move |x| x.strength = x.strength + 5);

        if opponent_stats.with(|x| x.health) > 0 {
            opponent_action();
        }
    };

    let render_game_buttons = move || {
        if player_stats.with(|x| x.health) > 0 && opponent_stats.with(|x| x.health) > 0 {
            if game_started() && player_stats.with(|x| x.strength) > 0 {
                view! { class = class_name,
                    <button id="attack-button" on:click=attack>Hit</button>
                    <button id="protect-button" on:click=block>Block</button>
                }
                .into_view()
            } else if game_started() {
                view! { class = class_name,
                    <button id="protect-button" on:click=block>Block</button>
                }
                .into_view()
            } else {
                view! {""}.into_view()
            }
        } else {
            view! {""}.into_view()
        }
    };

    let next_opponent = move |_| {
        set_player_stats.set(DEFAULT_STATS_CHARACTER);
        set_opponent_stats.set(DEFAULT_STATS_CHARACTER);
        set_opponent_stats.update(move |x| x.strength += 5 * round() as u8);

        set_round.update(move |x| *x += 1);
        set_img_opponent.update(move |x| *x = "img/".to_owned() + IMG_OPPONENTS[round() - 1]);

        set_game_message.set("...".to_string());
    };

    let restart = move |_| {
        set_round.set(1);
        set_img_opponent.update(move |x| *x = "img/".to_owned() + IMG_OPPONENTS[round() - 1]);

        set_player_stats.set(DEFAULT_STATS_CHARACTER);
        set_opponent_stats.set(DEFAULT_STATS_CHARACTER);
        set_game_message.set("...".to_string());
    };

    let between_rounds = move || {
        if opponent_stats.with(|x| x.health) == 0 {
            view! { class = class_name,
                <button id="next-opponent-button" on:click=next_opponent>Next opponent</button>
                <button id="restart-button" on:click=restart>Restart</button>
            }
            .into_view()
        } else if player_stats.with(|x| x.health) == 0 {
            view! { class = class_name,
                <button id="restart-button" on:click=restart>Restart</button>
            }
            .into_view()
        } else {
            view! {""}.into_view()
        }
    };

    view! { class = class_name,
        <div class="gameZone">
            <div class="titleGame">"MIAMI FIGHT"</div>

            <Introduction game_started=game_started
                          set_game_started=set_game_started
                          set_username=set_username
                          class_name=class_name />
            <div class="character">
                <h3><span id="playerNameEnter">{username}</span></h3>
                <img src="img/Player.jpg" alt="Your photo goes here" id="imgPlayer"/>
                <h4>Health: <span>{move || player_stats.with(|x| x.health)}</span></h4>
                <h4>Strength: <span>{move || player_stats.with(|x| x.strength)}</span></h4>
                <h4>Agility: <span>{move || player_stats.with(|x| x.agility)}</span></h4>
            </div>

            <div class="character">
                <h3>Opponent</h3>
                <img src={img_opponent} id="imgOpponent" id="imgOpponent"/>
                <h4>Health: <span>{move || opponent_stats.with(|x| x.health)}</span></h4>
                <h4>Strength: <span>{move || opponent_stats.with(|x| x.strength)}</span></h4>
                <h4>Agility: <span>{move || opponent_stats.with(|x| x.agility)}</span></h4>
            </div>

            <br/>
            <h3><span id="round">Round {round}</span></h3>

            {render_game_buttons}

            {between_rounds}
            <h3 id="game-message">{game_message}</h3>

        </div>
    }
}

fn main() {
    mount_to_body(App)
}