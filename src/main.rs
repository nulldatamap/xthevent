extern crate rustc_serialize as serialze;
#[macro_use] extern crate nickel;
extern crate postgres;
extern crate crypto;
extern crate time;
extern crate getopts;

use nickel::{Nickel, HttpRouter};
use postgres::{Connection, SslMode};
use time::Timespec;
use getopts::{Options, Matches};

type Id = i32;

struct Event {
  id                  : Id,
  tournament          : bool,
  title               : String,
  date_time           : Timespec,
  unconfirmed_players : Vec<Id>,
  confirmed_players   : Vec<Id>,
  active              : bool
}

struct User {
  id         : Id,
  steam_id   : String,
  name       : String,
  email      : String,
  player_tag : Option<String>,
  rank       : Option<String>
}

struct Account {
  id            : Id,
  password_hash : Vec<u8>,
  password_salt : Vec<u8>,
  email         : String,
  session_token : Option<String>,
  expiration    : Timespec
}

struct Registration {
  token         : String,
  steam_id      : String,
  name          : String,
  email         : String,
  player_tag    : Option<String>,
  rank          : Option<String>,
  password_hash : Vec<u8>,
  password_salt : Vec<u8>
}


static DEFAULT_DB_ADDR : &'static str = "postgres://postgres@localhost/xth";

static DEFAULT_PORT    : &'static str = "491";
static DEFAULT_ADDRESS : &'static str = "localhost";

fn init_database( dbaddr : &str ) {
  let mut conn = Connection::connect( dbaddr, &SslMode::None).unwrap();
  conn.batch_execute( include_str!( "schema.sql" ) ).unwrap();
}

fn insert_user( conn : &mut Connection, user : User ) {
  conn.execute( "INSERT INTO \"User\"
                  (id, steam_id, name, email, player_tag, rank)
                VALUES
                  ($1, $2, $3, $4, $5, $6)"
              , &[ &user.id, &user.steam_id, &user.name
                 , &user.email, &user.player_tag, &user.rank ] ).unwrap();
}

fn print_usage( program : &str, opts : Options ) {
  let usage = format!( "Usage:\n    {} [addr:port] [options]", program );
  print!( "{}", opts.usage( &usage ) );
}

fn get_options() -> Options {
  let mut opts = Options::new();

  opts.optflag( "h", "help", "displays the help prompt" );
  opts.optflag( "", "initdb", "initializes the database" );

  opts.optopt( "a", "address", "sets the server's address", "ADDR" );
  opts.optopt( "p", "port", "sets the server's port", "PORT" );
  opts.optopt( "", "dbaddr", "sets the database's address", "ADDR" );

  opts
}

fn main() {
  let vargs : Vec<String> = std::env::args().collect();
  let program = &vargs[0];
  let args = &vargs[1..];
  let options = get_options();
  let matches = options.parse( args ).unwrap();

  if matches.opt_present( "h" ) {
    print_usage( program, options );
    return
  }

  if matches.free.len() > 1 {
    println!( "Error: only one address is allowed to be specified." );
    return
  }

  let database_addrress = matches.opt_str( "dbaddr" )
                                 .unwrap_or( DEFAULT_DB_ADDR.to_string() );

  if matches.opt_present( "initdb" ) {

    println!( "Initializing the database." );

    init_database( &database_addrress );

    println!( "Database initialized." );
  }

  let addr = if !matches.free.is_empty() {
    if matches.opt_present( "a" ) || matches.opt_present( "p" ) {
      println!( "Error: address has been specified twice." );
      return
    }

    matches.free[0].clone()
  } else {
    let server_address = matches.opt_str( "a" )
                                .unwrap_or( DEFAULT_ADDRESS.to_string() );

    let server_port    = matches.opt_str( "p" )
                                .unwrap_or( DEFAULT_PORT.to_string() );

    format!( "{}:{}", server_address, server_port )
  };

  let mut server = Nickel::new();
  
  server.get( "**", middleware! { |request|
    format!( "{:?}", request.origin.uri )
  } );
  
  server.listen( &addr[..] );
}