use yew::prelude::*;

const MODELS: [&str; 11] = [
    "begonia.png",
    "deer.png",
    "plates.png",
    "tree.png",
    "blue_bird.png",
    "diorama.png",
    "pot.png",
    "woodbird.png",
    "chandelier.png",
    "mushroom.png",
    "sturgeon.png",
];
// old path
// const PATH: &str = "./media/graphics/";
const PATH: &str = "./augmented-paths/media/";
const OFFSET: u8 = 0;
const SIZE: u8 = 9;

enum Msg {
    BlueBirdStyle,
    BegoniaStyle,
    DeerStyle,
    MushroomStyle,
    TreeStyle,
    ChandelierStyle,
    DioramaStyle,
    WoodBirdStyle,
    PlatesStyle,
    SturgeonStyle,
    PotStyle,
    Team,
    About,
    Home,
}

struct App {
    bluebird_style: String,
    begonia_style: String,
    deer_style: String,
    mushroom_style: String,
    tree_style: String,
    chandelier_style: String,
    diorama_style: String,
    woodbird_style: String,
    plates_style: String,
    sturgeon_style: String,
    pot_style: String,
    team: String,
    about: String,
    home: String,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            bluebird_style: String::from("normal"),
            begonia_style: String::from("normal"),
            deer_style: String::from("normal"),
            mushroom_style: String::from("normal"),
            tree_style: String::from("normal"),
            chandelier_style: String::from("normal"),
            diorama_style: String::from("normal"),
            woodbird_style: String::from("normal"),
            plates_style: String::from("normal"),
            sturgeon_style: String::from("normal"),
            pot_style: String::from("normal"),
            team: String::from("hide"),
            about: String::from("hide"),
            home: String::from("home"),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::BlueBirdStyle => {
                if self.bluebird_style == "normal" {
                    self.bluebird_style = String::from("hover");
                } else {
                    self.bluebird_style = String::from("normal");
                }
                true
            }
            Msg::BegoniaStyle => {
                if self.begonia_style == "normal" {
                    self.begonia_style = String::from("hover");
                } else {
                    self.begonia_style = String::from("normal");
                }
                true
            }
            Msg::DeerStyle => {
                if self.deer_style == "normal" {
                    self.deer_style = String::from("hover");
                } else {
                    self.deer_style = String::from("normal");
                }
                true
            }
            Msg::MushroomStyle => {
                if self.mushroom_style == "normal" {
                    self.mushroom_style = String::from("hover");
                } else {
                    self.mushroom_style = String::from("normal");
                }
                true
            }
            Msg::TreeStyle => {
                if self.tree_style == "normal" {
                    self.tree_style = String::from("hover");
                } else {
                    self.tree_style = String::from("normal");
                }
                true
            }
            Msg::ChandelierStyle => {
                if self.chandelier_style == "normal" {
                    self.chandelier_style = String::from("hover");
                } else {
                    self.chandelier_style = String::from("normal");
                }
                true
            }
            Msg::DioramaStyle => {
                if self.diorama_style == "normal" {
                    self.diorama_style = String::from("hover");
                } else {
                    self.diorama_style = String::from("normal");
                }
                true
            }
            Msg::WoodBirdStyle => {
                if self.woodbird_style == "normal" {
                    self.woodbird_style = String::from("hover");
                } else {
                    self.woodbird_style = String::from("normal");
                }
                true
            }
            Msg::PlatesStyle => {
                if self.plates_style == "normal" {
                    self.plates_style = String::from("hover");
                } else {
                    self.plates_style = String::from("normal");
                }
                true
            }
            Msg::SturgeonStyle => {
                if self.sturgeon_style == "normal" {
                    self.sturgeon_style = String::from("hover");
                } else {
                    self.sturgeon_style = String::from("normal");
                }
                true
            }
            Msg::PotStyle => {
                if self.pot_style == "normal" {
                    self.pot_style = String::from("hover");
                } else {
                    self.pot_style = String::from("normal");
                }
                true
            }
            Msg::Team => {
                self.home = String::from("hide");
                self.team = String::from("team");
                self.about = String::from("hide");
                true
            }
            Msg::About => {
                self.home = String::from("hide");
                self.team = String::from("hide");
                self.about = String::from("about");
                true
            }
            Msg::Home => {
                self.home = String::from("home");
                self.team = String::from("hide");
                self.about = String::from("hide");
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        // render the header
        let header_html: Html = html! {
            <div>
                <h1 onclick={link.callback(|_| Msg::Home)}>
                { "Augmented Pathways" }<br/>
            <span id="subtext">
                <i>{ "Highlighting Untold Histories & Local Ecologies" }</i>
                </span>
                </h1>
                <ul class="header">
                <li class="header" id="logo">
                <a href="https://eh.bard.edu" target="_#">
                <img src="./augmented-paths/media/logo.png" height="60"/>
                </a>
                </li>
                <li class="header"><button onclick={link.callback(|_| Msg::About)}>{ "About" }</button></li>
                <li class="header">
                <a href="https://apps.apple.com/us/app/eh-augmented-pathways/id1541210629" target="_#">
                { "Download" }</a>
            </li>
                <li class="header">
                <button href="" onclick={link.callback(|_| Msg::Team)}>{ "Team" }</button>
                </li>
                <li class="header">
                <button href="" onclick={link.callback(|_| Msg::Home)}>
                { "Home "}
            </button>
                </li>
                </ul>
                </div>
        };

        /*
         * home
         */

        // render the models
        let models: Html = html! {
            <div>
                // begonia
                <img src={PATH.to_owned()+MODELS[0]} id={self.begonia_style.clone()}
            style={"left :".to_owned()+&OFFSET.to_string()+"%;"}
            onmouseenter={link.callback(|_| Msg::BegoniaStyle)}
            onmouseleave={link.callback(|_| Msg::BegoniaStyle)}/>
                // deer
                <img src={PATH.to_owned()+MODELS[1]} id={self.deer_style.clone()}
            style={"left :".to_owned()+&(OFFSET+SIZE).to_string()+"%;"}
            onmouseenter={link.callback(|_| Msg::DeerStyle)}
            onmouseleave={link.callback(|_| Msg::DeerStyle)}/>
                // plates
                <img src={PATH.to_owned()+MODELS[2]} id={self.plates_style.clone()}
            style={"left :".to_owned()+&(OFFSET+SIZE*2).to_string()+"%;"}
            onmouseenter={link.callback(|_| Msg::PlatesStyle)}
            onmouseleave={link.callback(|_| Msg::PlatesStyle)}/>
                // tree
                <img src={PATH.to_owned()+MODELS[3]} id={self.tree_style.clone()}
            style={"left :".to_owned()+&(OFFSET+SIZE*3).to_string()+"%;"}
            onmouseenter={link.callback(|_| Msg::TreeStyle)}
            onmouseleave={link.callback(|_| Msg::TreeStyle)}/>
                // blue bird
                <img src={PATH.to_owned()+MODELS[4]} id={self.bluebird_style.clone()}
            style={"left :".to_owned()+&(OFFSET+SIZE*4).to_string()+"%;"}
            onmouseenter={link.callback(|_| Msg::BlueBirdStyle)}
            onmouseleave={link.callback(|_| Msg::BlueBirdStyle)}/>
                // diorama
                <img src={PATH.to_owned()+MODELS[5]} id={self.diorama_style.clone()}
            style={"left :".to_owned()+&(OFFSET+SIZE*5).to_string()+"%;"}
            onmouseenter={link.callback(|_| Msg::DioramaStyle)}
            onmouseleave={link.callback(|_| Msg::DioramaStyle)}/>
                // pot
                <img src={PATH.to_owned()+MODELS[6]} id={self.pot_style.clone()}
            style={"left :".to_owned()+&(OFFSET+SIZE*6).to_string()+"%;"}
            onmouseenter={link.callback(|_| Msg::PotStyle)}
            onmouseleave={link.callback(|_| Msg::PotStyle)}/>
                // woodbird
                <img src={PATH.to_owned()+MODELS[7]} id={self.woodbird_style.clone()}
            style={"left :".to_owned()+&(OFFSET+SIZE*7).to_string()+"%;"}
            onmouseenter={link.callback(|_| Msg::WoodBirdStyle)}
            onmouseleave={link.callback(|_| Msg::WoodBirdStyle)}/>
                // chandelier
                <img src={PATH.to_owned()+MODELS[8]} id={self.chandelier_style.clone()}
            style={"left :".to_owned()+&(OFFSET+SIZE*8).to_string()+"%;"}
            onmouseenter={link.callback(|_| Msg::ChandelierStyle)}
            onmouseleave={link.callback(|_| Msg::ChandelierStyle)}/>
                // mushroom
                <img src={PATH.to_owned()+MODELS[9]} id={self.mushroom_style.clone()}
            style={"left :".to_owned()+&(OFFSET+SIZE*9).to_string()+"%;"}
            onmouseenter={link.callback(|_| Msg::MushroomStyle)}
            onmouseleave={link.callback(|_| Msg::MushroomStyle)}/>
                // sturgeon
                <img src={PATH.to_owned()+MODELS[10]} id={self.sturgeon_style.clone()}
            style={"left :".to_owned()+&(OFFSET+SIZE*10).to_string()+"%;"}
            onmouseenter={link.callback(|_| Msg::SturgeonStyle)}
            onmouseleave={link.callback(|_| Msg::SturgeonStyle)}/>
                </div>
        };

        // render the stars (locations)
        let stars: Html = html! {
            <div>
                <img class={self.bluebird_style.clone()} id="bluebird"
                onmouseenter={link.callback(|_| Msg::BlueBirdStyle)}
            onmouseleave={link.callback(|_| Msg::BlueBirdStyle)}/>
                <img class={self.begonia_style.clone()} id="begonia"
                onmouseenter={link.callback(|_| Msg::BegoniaStyle)}
            onmouseleave={link.callback(|_| Msg::BegoniaStyle)}/>
                <img class={self.deer_style.clone()} id="deer"
                onmouseenter={link.callback(|_| Msg::DeerStyle)}
            onmouseleave={link.callback(|_| Msg::DeerStyle)}/>
                <img class={self.mushroom_style.clone()} id="mushroom"
                onmouseenter={link.callback(|_| Msg::MushroomStyle)}
            onmouseleave={link.callback(|_| Msg::MushroomStyle)}/>
                <img class={self.tree_style.clone()} id="tree"
                onmouseenter={link.callback(|_| Msg::TreeStyle)}
            onmouseleave={link.callback(|_| Msg::TreeStyle)}/>
                <img class={self.chandelier_style.clone()} id="chandelier"
                onmouseenter={link.callback(|_| Msg::ChandelierStyle)}
            onmouseleave={link.callback(|_| Msg::ChandelierStyle)}/>
                <img class={self.diorama_style.clone()} id="diorama"
                onmouseenter={link.callback(|_| Msg::DioramaStyle)}
            onmouseleave={link.callback(|_| Msg::DioramaStyle)}/>
                <img class={self.woodbird_style.clone()} id="woodbird"
                onmouseenter={link.callback(|_| Msg::WoodBirdStyle)}
            onmouseleave={link.callback(|_| Msg::WoodBirdStyle)}/>
                <img class={self.plates_style.clone()} id="plates"
                onmouseenter={link.callback(|_| Msg::PlatesStyle)}
            onmouseleave={link.callback(|_| Msg::PlatesStyle)}/>
                <img class={self.sturgeon_style.clone()} id="sturgeon"
                onmouseenter={link.callback(|_| Msg::SturgeonStyle)}
            onmouseleave={link.callback(|_| Msg::SturgeonStyle)}/>
                <img class={self.pot_style.clone()} id="pot"
                onmouseenter={link.callback(|_| Msg::PotStyle)}
            onmouseleave={link.callback(|_| Msg::PotStyle)}/>
                </div>
        };

        /*
         * team
         */
        let team: Html = html! {
            <div>
                <h3>{ "Team" }</h3>
                <ul>
                <li>{ "Amy" }</li>
                <li>{ "Angel" }</li>
                <li>{ "Anna" }</li>
                <li>{ "Bird" }</li>
                <li>{ "Chelsea" }</li>
                <li>{ "Genesis" }</li>
                <li>{ "Nikki" }</li>
                <li>{ "Susan" }</li>
                <li>{ "Krista" }</li>
                <li>{ "Jacob" }</li>
                <li>{ "Corinna" }</li>
                <li>{ "Carol" }</li>
                <li>{ "Cornell lab for ornethology" }</li>
                <li>{ "Montgomery Place" }</li>
                <li>{ "Special thanks to B&G for installing the signs" }</li>
                </ul>
                </div>
        };

        /*
         * about
         */
        let about: Html = html! {
            <div>
                <p>{"this is some about text"}</p>
                </div>
        };

        // return all our html blocks
        html! {
            <div class="full_app">
            {header_html}
            <div class={ self.home.clone() }>
            {stars}
            {models}
            </div>
                <div class={ self.team.clone() }>
                {team}
            </div>
                <div class= { self.about.clone() }>
                {about}
            </div>
                </div>
        }
    }
}

fn main() {
    // serve our html using trunk
    yew::start_app::<App>();
}
