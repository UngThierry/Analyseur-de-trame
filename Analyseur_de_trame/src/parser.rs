#[derive(Debug, Clone, Copy)]
pub enum ParserError {
    EOF,
    EOFWhileParsingTrame,
    EOL,
    UnknownChar,
    OffsetNotMatching,
    TooManyChars,
    NotEnoughChars,
}

use ParserError::*;

pub type Result<T> = std::result::Result<T, ParserError>;


pub struct TrameBuffer {
    //L'iterateur sur les lignes du fichiers
    iterateur: Box<dyn Iterator<Item = String>>,


    //Reserve de bits
    accumulateur: u8,

    //Nombre de bits dans la reserve
    bits_number: u8,

    
    //La ligne actuelle
    current_string: String,

    //L'offset actuel de la ligne
    current_line_offset: Option<usize>,


    //L'offset de l'octet par rapport au début de la trame
    offset: usize,

    //Si on doit passer tous les espaces (Utile pour skip les espaces entre l'offset et les octets au début de chaque ligne)
    skip_space: bool,

    //Le numero de la ligne (utile pour donner des erreurs plus précises)
    line: usize,
}


impl TrameBuffer {
    
    pub fn new(lines: Box<dyn Iterator<Item = String>>) -> TrameBuffer {
        TrameBuffer {
            current_string: "".to_string(),
            current_line_offset: None,
            iterateur: lines,
            accumulateur: 0,
            bits_number: 0,
            offset: 0,
            skip_space: false,
            line: 0,
        }
    }

    //retourne la e numéro de la ligne courante
    pub fn get_line(&self) -> usize {
        self.line
    }

    //retourne l'offset de la ligne courante (prochains characteres à parser)
    pub fn get_line_offset(&self) -> usize {
        match self.current_line_offset {
            Some(offset) => offset,
            None => 0,
        }
    }

    //Ordonne de passer à la trame suivante (on lui dit de sauter au moins une ligne, et on met l'offset à 0)
    pub fn reset_offset(&mut self) {
        self.offset = 0;
        self.current_line_offset = None;
    }

    //Lit le prochain octet de la ligne courante
    fn read_hex(line: &str, limit: Option<usize>, skip_space: bool) -> Result<(usize, usize)> {
        let mut number = 0; // Le nombre qu'on lit

        let mut tmp_ind = 0; // Le nombre de characteres lus

        for (offset, c) in line
            .chars() //On itère sur les characteres de la ligne
            .enumerate() //On enumerate les characteres
            .skip_while(|&(_, c)| c == ' ' && skip_space) //Si skip_space est true, alors, on skip tous les espaces avant le premier charactere
        {
            //pour chaque charactere après les premiers espaces :

            number = (number << 4)
                + match c {
                    //si on a un charactère hexadecimal, on l'ajoute à number
                    '0'..='9' => c as u8 - b'0',
                    'a'..='f' => c as u8 - b'a' + 10,
                    'A'..='F' => c as u8 - b'A' + 10,

                    //Si c'est un espace :
                    ' ' => {
                        return match limit {

                            //si on a lu aucun charactere, on renvoie l'erreur FinDeLigne
                            _ if tmp_ind == 0 => Err(EOL),
                            
                            //si on a atteint la limite de charactere à lire, on renvoie le nomber
                            Some(limit) if limit == tmp_ind => Ok((number, offset + 1)),


                            Some(limit) if limit > tmp_ind => Err(NotEnoughChars),
                            Some(limit) if limit < tmp_ind => Err(TooManyChars),
                            
                            //S'il n'y avait pas de limite, on renvoie aussi le nombre
                            None => Ok((number, offset + 1)),

                            _ => unreachable!(),
                        }
                    }

                    //Si ce n'est pas un charactere accepte, on renvoie l'erreur Charactere Inconnu
                    _ => return Err(UnknownChar),
                } as usize;

            tmp_ind += 1; //On incrémente le nombre de charactere lus
        }

        //Après le for
        if limit == None {
            //s'il n'y a pas de limite
            match tmp_ind {
                //si on n'a lu aucun charactere, on renvoie l'erreur FinDeLigne
                0 => Err(EOL), 

                //sinon, on renvoie le nombre lu et la longueur de la ligne (on est sorti du for = on a lu toute la ligne)
                _ => Ok((number, line.len())), 
            }
        } else {
            //s'il y a une limite (= nombre de charactere demandé)
            match tmp_ind {
                //si on a lu aucun charactere, on renvoie l'erreur FinDeLigne
                0 => Err(EOL),

                //si on a lu moins de charactères que demandé, on renvoie l'erreur PasAssezDeCharacteres
                ind if ind < limit.unwrap() => Err(NotEnoughChars),

                //si on a lu le bon nombre de charactere, on renvoie le nombre hexadecimal lu et la longueur de la ligne (on est sorti du for = on a lu toute la ligne)
                ind if ind == limit.unwrap() => Ok((number, line.len())),

                //si on a lu plus de charactères que demandé, on renvoie l'erreur TropDeCharacteres
                _ => Err(TooManyChars),
            }
        }
    }

    fn next(&mut self) -> Result<u8> {
        //On regarde l'offset de la ligne en cours
        match self.current_line_offset {
            //S'il n'y en a plus : 
            None => {
                //on passe à la ligne suivante
                self.line += 1; //on incrémente le numéro de la ligne
                self.current_string = self.iterateur.next().ok_or(EOF)?; //s'il n'y en a pas, on renvoie FinDeFichier


                //On essaie de lire l'offset des octets (par rapport au début de la trame) de la ligne
                //On gère donc les différentes valeurs de retour de read_hex
                match TrameBuffer::read_hex(&self.current_string, None, self.skip_space) {
                    //Si tout a bien fonctionné : 
                    Ok((offset, ind)) => {
                        if offset != self.offset {
                            //si l'offset de la ligne est invalide, on passe la ligne 
                            //(self.current_line_offset est toujours None, donc le prochain appel à next sautera encore une ligne)
                            self.next()
                        } else {
                            //Si l'offset est valide
                            //On met skip_space à true pour passer les espaces après l'offset au début de la ligne
                            self.skip_space = true;
                            //On place l'offset courant de la ligne après les premiers characteres hexadecimaux
                            self.current_line_offset = Some(ind);
                            //Et on renvoie le prochain octet lu
                            self.next()
                        }
                    }

                    //Si on a une erreur pendant la lecture de l'offset, on ne cherche pas à comprendre, on saute la ligne
                    //(probablement du texte entre les lignes de la trame)
                    Err(_) => self.next(),
                }
            }

            //Si on a un offset
            Some(offset) => {
                //On lit le prochain octet de la ligne
                match TrameBuffer::read_hex(
                    &self.current_string[offset..], //on ne donne que la partie intéressante de la ligne
                    Some(2),    //on demande 2 characteres
                    self.skip_space,
                ) {
                    //si ça a bien fonctionné
                    Ok((number, ind)) => {
                        //alors skip_space doit être faux (ne sert que pour le premier octet (celui après l'offset))
                        self.skip_space = false;
                        
                        //on augmente l'offset du nombre de charactere
                        self.offset += 1;

                        //on augmente l'offset de la ligne courante par le nombre de characteres lu
                        self.current_line_offset = Some(offset + ind);

                        //on renvoie le nombre hexadecimal lu
                        Ok(number as u8)
                    }

                    //si ça nous renvoie une erreur 
                    Err(EOL) => {
                        //On saute la ligne courante
                        self.current_line_offset = None;
                        self.next()
                    }

                    //si on a une autre erreur, on la renvoie directement
                    Err(err) => Err(err),
                }
            }
        }
    }

    pub fn get_bits(&mut self, mut bits_number: u8) -> Result<u128> {
        if bits_number <= self.bits_number {
            //si on a déjà assez de bits en reserve

            //on decremente le nombre de bits en reserve
            self.bits_number -= bits_number;

            //On copie tous les bits sauf ceux qu'on garde en reserve
            let res = self.accumulateur >> self.bits_number;

            //On les enleves de notre reserve
            self.accumulateur -= res << self.bits_number;

            //On renvoie les bits
            Ok(res as u128)

        } else {
            //sinon, il va falloir lire des octets

            //on prend la réserve
            let mut res = self.accumulateur as u128;

            //on decremente le nombre de bits à récupérer
            bits_number -= self.bits_number;

            while bits_number > 8 {
                //tant que l'on veut plus qu'un octet entier
                //on lit les octets et on les place dans le nombre que l'on va renvoyer
                res = (res << 8) + self.next()? as u128;

                //sans oublier de decrementer le nombre de bits à prendre
                bits_number -= 8;
            }

            //Si l'on veut 8 bits ou moins :
            //on calcule le nombre de bits qu'il nous restera en reserve
            self.bits_number = 8 - bits_number;

            //on lit le prochain octet
            self.accumulateur = self.next()?;

            //On copie les bits qu'on ne garde pas
            let bits_needed = self.accumulateur >> self.bits_number;

            //et on les enleve de la reserve
            self.accumulateur -= bits_needed << self.bits_number;

            //on renvoie alors le resulat auquel on ajoute les bits en question
            Ok((res << bits_number) + bits_needed as u128)
        }
    }
}
