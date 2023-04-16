use std::thread;
use std::sync::mpsc;

pub mod uci {
    enum Move {
        // TODO: Implement
    }
    
    enum Position {
        /// A FEN string
        Fen(String),
        /// The normal chess starting position
        StartPosition,
        /// A list of moves since the start of the game
        MoveList(Vec<Move>)
    }
    
    /// Literally a whole enum for just the "go" command
    enum GoCommand {
        /// Represents subcommand "searchmoves".
        /// The engine should restrict it's search to only these moves from the current position.
        SearchMoves(Vec<Move>),
        /// Represents subcommand "ponder".
        /// The engine should start pondering what it might do next, asynchronously.
        Ponder,
        /// Represents subcommand "wtime".
        /// The number of milliseconds white has left on the clock.
        WhiteClockLeft(usize),
        /// Represents subcommand "btime".
        /// The number of milliseconds black has left on the clock.
        BlackClockLeft(usize),
        /// Represents subcommand "winc".
        /// Imma be honest, I can't figure out what this means.
        WhiteIncrement(usize),
        /// Represents subcommand "binc".
        /// Imma be honest, I can't figure out what this means.
        BlackIncrement(usize),
        /// Represents subcommand "movestogo".
        /// The number of moves until the next time control.
        MovesToGo(usize),
        /// Represents subcommand "depth".
        /// The maximum number of plies to search.
        MaxSearchDepth(usize),
        /// Represents subcommand "nodes".
        /// The maximum number of nodes to search.
        MaxSearchNodes(usize),
        /// Represents subcommand "mate".
        /// Search this many moves deep to find mate.
        Mate(usize),
        /// Represents subcommand "movetime".
        /// Try to search for exactly this many milliseconds.
        TargetSearchTime(usize),
        /// Represents subcommand "infinite".
        /// Search until told to stop searching.
        InfiniteSearch
    
    }

    /// Represents commands the GUI might send to the engine, and holds the data about the command if applicable.
    enum GUICommand {
        /// Corresponds to "uci" command
        /// Is sent once on initialization. The engine doesn't really need to do anything with this.
        UCIInit,
        /// Corresponds to the "debug" command.
        /// If true, the engine should provide extra debugging info to the GUI
        DebugMode(bool),
        /// Corresponds to the "isready" command
        /// Used to sync with the GUI. The engine should respond with "readyok" when it is ready to recieve commands
        IsReady,
        /// Corresponds to the "setoption" command. 
        /// The engine should modify it's parameters accordingly.
        SetEngineParameter {option_name: String, option_value: EngineParameter},
        /// Corresponds to the "ucinewgame" command.
        /// This indicates that the next position to be searched is not from the same game, so the engine should clear any game-local data it's kept.
        UCINewGame,
        /// Corresponds to the "position" command.
        /// Indicates the current position of the board to the engine.
        Position(Position),
        /// Corresponds to the "go" command.
        /// The engine should start searching.
        Go(Vec<GoCommand>),
        /// Corresponds to the "stop" command.
        /// The engine must stop calculating as soon as possible.
        Stop,
        /// Corresponds to the "ponderhit" command.
        /// Indicates to the engine that it's opponent played the expected move that it was told to ponder about. The engine should switch from ponder to normal search mode if it distinguishes the two.
        PonderHit,
        /// Corresponds to the "quit" command.
        /// The engine must quit as soon as possible.
        Quit
    }

    /// Represents the data of an ID command.
    enum IdCommandData {
        /// Identifies the name of the engine
        Name(String),
        /// Identifies the author of the engine
        Author(String)        
    }

    /// Data for the copyprotection command
    enum CopyprotectionCommandData 
    {
        Checking,
        Ok,
        Error
    }

    /// Data for the "score" info
    enum ScoreInfoData {
        /// Overall score of the position from the engine's point of view in centipawns
        CentiPawns(usize),
        /// Number of moves until mate. Positive means the engine wins, negative means the engine loses.
        MateInMoves(isize),
        /// Indicates that the score is a lower bound
        ScoreIsLowerBound,
        /// Indicates that the score is an upper bound
        ScoreIsUpperBound
    }

    /// Data for the Info command
    enum InfoCommandData {
        /// Represents "depth" info
        /// Indicates how many plies deep the search has gotten
        Depth(usize),
        /// Represents "seldepth" info
        /// Indicates the selective depth (I don't know what that means) of the current search in plies. Must always be accompanied by a depth info.
        SelectiveDepth(usize),
        /// Represents "time" info
        /// The number of milliseconds spent searching
        /// Should be sent along with the principle variation
        TimeSpentSearching(usize),
        /// Represents "nodes" info
        /// Should be sent regularly
        /// The number of nodes searched
        NodesSearched(usize),
        /// Represents "pv" info
        /// Contains the "Principle Variation", or the sequence of moves the engine currently thinks it likes the most.
        PrincipleVariation(Vec<Move>),
        /// Represents the "score" info.
        Score(ScoreInfoData),
        /// Represents the "currmove" info.
        /// Indicates which move the engine is currently searching
        CurrentMove(Move),
        /// Represents the "currmovenumber" info.
        /// Indicates that the engine is currently searching this move number. Starts counting at 1, not 0.
        CurrentMoveNumber(usize),
        /// Represents the "hashfull" info.
        /// Indicates how full the engine's hash table is, expressed as an integer out of 1000
        /// Should be sent regularly
        HashFullPermill(usize),
        /// Represents the "nps" info.
        /// The number of nodes per second the engine has searched.
        /// This should be sent regularly.
        NodesPerSecond(usize),
        /// Represents the "tbhits" info.
        /// Indicates how many positions searched were found in endgame table bases
        TableBaseHits(usize),
        /// Represents "sbhits" info.
        /// Indicates how many positions searched were found in shredder endgame databases
        ShredderDatabaseHits(usize),
        /// Represents "cpuload" info.
        /// Indicates how much CPU the engine is using, expressed as a fraction over 1000.
        CpuLoad(usize),
        /// Represents "string" info.
        /// There must be at most 1 string info per info command.
        /// Represents a string that will be displayed by the user.
        InfoString(String),
        /// Represents "refutation" info
        /// Should only be sent if the UCI_ShowRefutations option is enabled.
        /// Indicates that a given move is refuted by a given sequence of moves.
        Refutation {refuted_move: Move, refutation: Vec<Move>},
        /// Represents the "currline" info
        /// Should only be sent if "UCI_ShowCurrLine" is enabled.
        /// Indicates the current sequence of moves the engine is thinking about, and which CPU the engine is thinking about it on if applicable.
        CurrentMoveSequence {cpu_number: Option<usize>, sequence: Vec<Move>}
    }

    /// Represents commands the engine can pass to the GUI, including any extra data if applicable.
    enum EngineCommand {
        /// Represents the "id" command.
        /// One of each type must be sent after engine initialization and before the initial uciok command and optional parameters command.
        ID(IdCommandData),
        /// Represents the "uciok" command.
        /// Must be sent after the id and options commands. Indicates that the engine is ready to accept commands from the engine.
        EngineInitialized,
        /// Represents the "readyok" command.
        /// Must be sent after each "isready" command the engine recieves, whenever the engine is ready to accept new commands.
        EngineReady,
        /// Represents the "bestmove" command.
        /// Indicates that the engine has finished searching and found this move best. Optionally, the engine can send the move it would like to ponder about. It must not begin pondering unless told to do so.
        MoveSelected {selected_move: Move, desired_ponder: Move},
        /// Represents the "copyprotection" command.
        /// The engine should send checking first, then ok or error.
        Copyprotection(CopyprotectionCommandData),
        /// Represents the "registration" command.
        /// Functions identically to Copyprotection.
        Registration(CopyprotectionCommandData),
        /// Represents the "info" command.
        /// The engine can combine multiple info commands into one.
        /// All info will be sent simultaneously.
        Info(Vec<InfoCommandData>),
        /// Represents the "option" command.

    }

    enum EngineParameter {
        Check(bool),
        Spin {min: isize, max: isize},
        Combo(Vec<String>),
        Button(String),
        String(String)
    }
 
    trait Engine {
        
    }

    struct UCIInterface {

    }

    
}