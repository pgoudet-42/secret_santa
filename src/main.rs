use rand::Rng;
use rand::rngs::ThreadRng;
use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;
use std::collections::HashMap;
use std::fs;
use std::process::exit;

struct  Datas<'a> {
    user: &'a str,
    password: &'a str,
    references: &'a mut Vec<&'a str>,
    correspondances: &'a mut HashMap<&'a str, &'a str>
}

fn check_map(paires: &HashMap<&str, &str>)-> bool {
    if paires.len() == 0 { return false; }
    for (key, value) in paires {
        if key == value { return false; }
    }
    return true;
}

fn gen_test_map<'a>(size: usize, mut the_random: ThreadRng, reference: &'a Vec<&'a str>) -> HashMap<&'a str, &'a str> {
    let mut targets = reference.clone();
    let mut candidates = reference.clone();
    let mut paires: HashMap<&str, &str> = HashMap::new();

    for i in 0..size {
        let mut rand_targets;
        let mut rand_candidates;
        loop {
            rand_targets = the_random.gen_range(0..8 - i);
            rand_candidates = the_random.gen_range(0..8 - i);
            if targets[rand_targets] != candidates[rand_candidates] || targets.len() <= 1 { break; }
        }
        paires.insert(targets[rand_targets], candidates[rand_candidates]);
        targets.remove(rand_targets); 
        candidates.remove(rand_candidates);
    }
    return paires
}

fn gen_paires<'a>(reference: &'a Vec<&'a str>) -> HashMap<&'a str, &'a str> {
    let size = reference.len();
    let mut paires = HashMap::new();
    let mut check = false;
    
    while check == false {
        let the_random = rand::thread_rng();
        paires = gen_test_map(size, the_random, reference);
        check = check_map(&paires);
    }
    return paires
}

#[warn(dead_code)]
fn send_mail(to: &str, subject: &str, text: &str, user:&str, pass: &str) {
    let email = EmailBuilder::new()
        .to(to)
        .from(user)
        .subject(subject)
        .text(text)
        .build()
        .unwrap();
    let mut mailer = SmtpClient::new_simple("mail.gandi.net")
        .unwrap()
        .credentials(Credentials::new(user.into(), pass.into()))
        .transport();
    let result = mailer.send(email.into());
    println!("res = {:?}", result);
}

fn format_text(key: &str, correspondances: &HashMap<&str, &str>) -> String {
    let text: String;
    let name;
    name = correspondances.get(key);
    text =  format!("Voici la personne à qui tu dois offrir un cadeau pour le Blind Xmas: {}", name.unwrap().to_string());
    return text;
}

fn set_mail(paires: &HashMap<&str, &str>, correspondances: &HashMap<&str, &str>, user: &str, pass: &str) {
    const SUBJECT: &str = "SEEEEEEECREEETTTTT";
    let mut receiver: &str;
    let mut text: &str;
    let mut tmp:String;

    for (value_1, value_2) in paires {
        receiver = value_1;
        tmp = format_text(value_2, correspondances);
        text = &tmp[..];
        send_mail(receiver, SUBJECT, text, user, pass)
    }
}

fn check_size(splt: std::str::Split<'_, &str>) {
    if splt.count() <= 1 { 
        println!("Error: not enought candidates!");
        exit(1);
    }
}

fn main() {
    let mut data: Datas = Datas {
        user: "****",
        password: "******",
        references: &mut Vec::new(),
        correspondances: &mut HashMap::new(),
    };
    let paires: HashMap<&str, &str>;
    let contenu = fs::read_to_string("./src/.conf")
    .expect("Quelque chose s'est mal passé lors de la lecture du fichier");
    let splt = contenu.split("\n");
    check_size(splt.clone());
    for (i,el) in splt.enumerate() {
        if i == 0 { data.user = el.trim(); }
        else if i == 1 { data.password =  el.trim(); }
        else {
            let mut paire = el.trim().split(", ");
            let tmp: &str = paire.next().unwrap();
            data.references.push(tmp);
            data.correspondances.insert(tmp, paire.next().unwrap()); 
        }
    }

    paires = gen_paires(&data.references);
    set_mail(&paires, &data.correspondances, &data.user, &data.password);
    println!("finish");
}
