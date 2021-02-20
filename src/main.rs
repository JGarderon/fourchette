
use std::env; 
use std::process::Command;
use std::thread; 
use std::thread::JoinHandle; 
use std::collections::HashMap; 
use std::io::{self, Write}; 

#[derive(Debug)]
struct SousProcessus { 
  act: Action, 
  sp: BoolOrThread
} 

impl SousProcessus { 

  fn creer( sp: BoolOrThread, act: Action ) -> Self { 
    SousProcessus { 
      act: act, 
      sp: sp 
    } 
  } 

}

#[derive(Debug)]
enum BoolOrThread { 
  Bool(bool), 
  Thread(JoinHandle<bool>)
} 

#[derive(Debug)]
struct Action {
  contexte: String, 
  arguments: Vec<String>, 
  environnement: Vec<(String, String)> 
} 

impl Action {

  fn creer( contexte: String ) -> Self {
    Action { 
      contexte: contexte, 
      arguments: Vec::new(), 
      environnement: Vec::new() 
    } 
  }

  fn tester( &self ) -> bool { 
    self.arguments.len() > 0 && self.contexte != "".to_string() 
  } 

  fn superviser( &mut self, env_global: Vec<(String, String)> ) -> BoolOrThread {
    self.environnement = env_global.clone(); 
    return match self.contexte.as_str() { 
      ":print" => { 
          println!( 
              ">>> {:}", 
              self.reduction() 
          ); 
          BoolOrThread::Bool( true ) 
      }, 
      ":bash" => BoolOrThread::Thread( 
          executer_bash( 
              env_global, 
              self.reduction() 
          ) 
      ), 
      ":cmd" => BoolOrThread::Thread( 
          executer_commande( 
              env_global, 
              self.arguments.to_vec().into_iter().map( | c | { c.to_string() } ).collect()  
          ) 
      ), 
      _ => panic!( 
          "contexte '{:?}' non implémenté", 
          self.contexte 
      ),
    } 
  } 
  fn reduction( &self ) -> String { 
    return self.arguments.iter().fold( 
      String::new(), 
      | a, b | return a+b+" " 
    );  
  }
} 

fn executer_bash( environnement: Vec<(String, String)>, commande: String ) -> JoinHandle<bool> { 
  executer_commande( 
    environnement, 
    vec!( 
      "bash".to_string(), "-c".to_string(), commande 
    )
  ) 
} 

fn executer_commande( environnement: Vec<(String, String)>, args: Vec<String> ) -> JoinHandle<bool> { 
  return thread::spawn( 
    move || { 
      let mut a = args.iter(); 
      let mut sp = Command::new( a.next().unwrap() ); 
      sp.env_clear(); 
      sp.envs(
        env::vars().filter( 
          |&(ref k, _) | 
            k == "TERM" || k == "TZ" || k == "LANG" || k == "PATH"
        ).collect::<HashMap<String, String>>()
      ); 
      for (cle, valeur) in environnement {
        sp.env( cle, valeur ); 
      }
      for arg in a { 
          sp.arg( arg ); 
      } 
      match sp.output() { 
        Ok( output ) => { 
          io::stdout().write_all(&output.stdout).unwrap();
          io::stderr().write_all(&output.stderr).unwrap(); 
          output.status.success() 
        }
        Err( _ ) => false, 
      }  
    } 
  ); 
} 

fn var_env_traduire( chaine: &String ) -> (String, String) { 
  let paire = chaine.splitn( 2, '=' ).collect::<Vec<&str>>(); 
  if paire.len() == 1 { 
    return ( chaine.to_string(), "".to_string() ); 
  } else { 
    return ( paire[0].to_string(), paire[1].to_string() ); 
  } 
}

fn main() { 

  let mut environnement_global = Vec::<(String, String)>::new(); 
  let mut sous_processus = Vec::<SousProcessus>::new(); 
  let args: Vec<String> = env::args().collect(); 
  let mut action = Action::creer( "".to_string() );  
  let mut args_iter = args.iter().enumerate(); 
  loop {
    match args_iter.next() { 
      Some( (i, arg) ) => {
        if i == 0 { 
          continue; 
        } 
        match arg.chars().next() { 
          Some( ':' ) => match arg.as_str() { 
            ":env" => { 
              if action.tester() { 
                sous_processus.push( 
                  SousProcessus::creer( 
                    action.superviser( environnement_global.clone() ), 
                    action 
                  )
                ); 
              } 
              action = Action::creer( "".to_string() ); 
              match args_iter.next() {
                Some( (_, e) ) => environnement_global.push( 
                  var_env_traduire( 
                    e 
                  ) 
                ), 
                None => panic!("demande de déf d'env sans valeur") 
              } 
            } 
            _ => { 
              if action.tester() { 
                sous_processus.push( 
                  SousProcessus::creer( 
                    action.superviser( environnement_global.clone() ), 
                    action 
                  )
                ); 
              } 
              action = Action::creer( arg.to_string() ); 
            }
          }, 
          _ => action.arguments.push( arg.to_string() ) 
        } 
      }
      None => break 
    } 
  }
  if action.tester() { 
    sous_processus.push( 
      SousProcessus::creer( 
        action.superviser( environnement_global.clone() ), 
        action 
      )
    ); 
  } 
  std::process::exit(
    match sous_processus.into_iter().fold( 
      true, 
      | etat, sous_processus | {
        match sous_processus.sp { 
            BoolOrThread::Thread( t ) => { 
              match t.join() { 
                Ok( r ) => etat && r, 
                Err( _ ) => false 
              }
            }, 
            BoolOrThread::Bool( r ) => etat && r 
        } 
      }
    ) {
      true => 0, 
      false => 1 
    } 
  ); 
} 

