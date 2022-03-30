use std::io;
use rand::Rng;

fn main() {
    println!("\n\tDICESPA GAME");
    println!("
    ------------------------------------------------------
    |    ....       .       :::.       .::::     .::::.  |
    | .:   .:.  --.   - :+:.  ...  :-:.  .:-  -:.  .:-.  |
    |       =:+-     *.:-       ::.       -+:        +.  |
    |  :=: -%+    :+-           . .      .*   :+:   +=   |
    | :=-   .+-  :==.         ..  -+      =+ .++.    +-  |
    |-=.    -== .-.           .*  -+     -+.:*:      +-  |
    |*     := ::      -.      +-  :=    ==  -=      ---  |
    |.:   ::   ...  .:.     .+:   :-  .=:    :.   .:.  . |
    |                             :-                     |
    |                             :-                     |
    |                             :-                     |
    |____________________________________________________|");
    let dice: u8 = rand::thread_rng().gen_range(1..7);

    println!("\n\tPlease pulse enter to play!");
    let mut play = String::new();

    io::stdin()
            .read_line(&mut play)
            .expect("Failed to read line");

    println!("
        -----
        |   |
        | o |
        |   |
        ----- "
    );
    println!("\n\tResultado:");
    match dice {
        2 => add_symbol(2),
        5 => add_cover(5),
        _other => random(1),
    };
}

fn add_symbol(_x: u8) {
    println!("
        -----
        |o  |
        |   |
        |  o|
        ----- " );
    let symbol: u8 = rand::thread_rng().gen_range(1..5);
    match symbol {
        1 => {
            println!("\n\tWinter");
            println!("
             ==
            -%%:
           .%@@#.
    .------#@@@@*--::-:.
     -*%%@@@@@@@@@@%#*-
       .=#%@@@@@@%#=.
        .*%%%@@%%%+
        +#%%####%%#-
       =*#*=:  :+*#*.
       +=:       .-++ ");
        },
        2 => {
            println!("\n\tKarina");
            println!("
      .===-.    .-=+=.
     -#####*=..=*#####-
    :*#*****#++#*****#*:
    .=++++++++++++++++=.
     .-==============-.
      .:------------:.
        ::---------:.
         .:------:.
           .:--:.
             :: ");
        },
        3 => {
            println!("\n\tGiselle");
            println!("
              .:=+-.
                 -#%+.
                  :%@#=
                   #@@#-
                  .#@@%*
    -             *%@@%#
    -+:         -#%@@%#-
     =##+=---=*#%%%@%#=
      .+#%%%%%%%%%%#+:
        .-+*####*+-. "
            );
    },
        _other => {
            println!("\n\tNingning");
            println!("
        ..            ..
        -+*+:      .=*+=
         =***=    =***+.
         =****=  -****=
         .+***+..=***+.
         :+***+:.+***+:
         .=+++-  -++++.
          .-=-.   -==: ");
        },
    };
}

fn add_cover(_x: u8) {
    println!("
        -----
        |o o|
        | o |
        |o o|
        -----");
    println!("
    @@@@@@@@@@@@@@@@%%%%%%%%@%%%%%%%%%%%%%%%%%%%%%%%%%
    @@@@@@@@@@@@@@@@@%@%@@@%%%@@@@@@@%%@%%%%%%%%%%%%%%
    @@@@@@@@@@@@@@@@@@%@#=-=*##*+==-::.=@%%%%%%%%%%%%%
    @@@@@@@@@@@@@@@@@%@*    :=.....::. -@%%%%%%%%%%%%%
    @@@@@@@@@@@@@@%%%%@-      .-*#%%-: *@%%%%%%%%%%%%%
    @@@@@@@@@@@%%%%%@@@+ -=-.  :@@@=: :%%%%%%%%%%%%%%%
    @@@@@@@@%%%%%@@@%*=. :--   .%#=::.%%%%%%%%%%%%%%%%
    @@@%%%%%%%%@@%*-.  :: ==.:... .: +%%%%%%%%%%%%%%%%
    %%%%%%%%%%@*:...-*#%%#*=.. .  . -%%%%%%%%%%%%%%%%%
    %%%%%%%%%%@*:.::---::.         -%%%%%%%%%%%%%%%%%%
    %%%%%%%%%%%@@##+=--.          .#%%%%%%%%%%%%%%%%%%
    %%%%%%%%%%%%%%%%%%%*:         =%%%%%%%%%%%%%%%%%%%
    %%%%%%%%%%%%%%%%%%%%+         *%%%%%%%%%%%%%%#%%%%
    %%%%%%%%%%%%%%%%%%%%-        =%#%#################
    %%%%%%%%%%%%%%%%%%%+        :%####################
    %%%%%%%%%%%%%%%%%%-         -%####################
    %%%%%%%%%%%%%%%%%:          .#####################
    %%%%%%%%%%%%%##%-            *####################
    %%%%%%%%%%####%=             =%###################
    %%%%%%%%#####%=              :####################
    %%%%%%########*+..            *###################
    %%%%%#########%#:... ::       +###################
    %%%%############:::..*#*- ..:=+###################
    %%%############%-:-::*###=.:===###################
    %%%%%%%%%%%%###%=.---####%=.:--+%#################
    %%%%%%%%%%%%%%%%+.--=%%%##%=.:--+%%###############
    %%%%%%%%%%%%%%%#:.::+%%%%%%%+.:--+#%%%%%%%%%%%%%%%
    %%%%%%%%%%%%%%%*    *%%%%%%%%*:..  *%%%%%%%%%%%%%%
    %%%%%%%%%%%%%%%*   .#%%%%%%%#%#:   .*%###%%%%%%%%%
    %%%%%%%%######%+   :############.   .#############
    ###############=   -##########*#*.   -#***********
    ###############=   +###########*#*.   *#**********
    ###############+  .*#************#*.  -#**********
    ###############+  :#****************.  +**********
    ########******#+  :#****************=  .**********
    **************+.  :******************.  =*********
    *************-     +********+*******+   :*********
    ***********++:    :+************++++=   .+********
    ************+=:::-=+++++++++*******+=:::-+++++++**
    ************+++====++++++++++++=======++++******++
    ");
}

fn random(_x: u8) {
    let song_number: u8 = rand::thread_rng().gen_range(1..5);
    match song_number {
        1 => {
            println!("
        -----
        |   |
        | o |
        |   |
        -----" );
            println!("\n\tSong: Next Level");
        }
        2 => {
            println!("
        -----
        |o  |
        | o |
        |  o|
        -----" );
            println!("\n\tSong: Yeppi Yeppi");
        }
        3 => {
            println!("
        -----
        |o o|
        |   |
        |o o|
        -----" );
            println!("\n\tSong: Black Mamba");
        }
        4 => {
            println!("
        -----
        |o o|
        |o o|
        |o o|
        -----" );
            println!("\n\tSong: Savage");
        }
        _other => (),
    }
}