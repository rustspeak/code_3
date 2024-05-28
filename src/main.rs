#[derive(Clone, Debug, Copy)]


struct Objet {
    poids: f64, 
    valeur: f64, 
}

struct SacADos {
    capacite_max: f64, 
    liste_objets: Vec<Objet>, 
    valeur_totale: f64, 
}

impl Objet {
    fn nouveau_objet(poids: f64, valeur: f64) -> Objet {
        Objet { poids, valeur }
    }
}

impl SacADos {
    fn nouveau_sac_a_dos(capacite_max: f64) -> SacADos {
        SacADos {
            capacite_max,
            liste_objets: Vec::new(),
            valeur_totale: 0.0,
        }
    }

    fn ajouter_objet(&mut self, objet: &Objet) {
        if self.capacite_max >= self.liste_objets.iter().map(|objet| objet.poids).sum::<f64>() + objet.poids {
            self.liste_objets.push(objet.clone());
            self.valeur_totale += objet.valeur;
        }
    }

    fn sac_a_dos_optimal(&self) -> Vec<&Objet> {
        let mut table = vec![vec![0; self.capacite_max as usize + 1]; self.liste_objets.len() + 1];

        for (i, objet) in self.liste_objets.iter().enumerate() {
            for capacite in 0..=self.capacite_max as usize {
                table[i + 1][capacite] = table[i][capacite]; // Exclude current object

                if capacite >= objet.poids as usize {
                    table[i + 1][capacite] = std::cmp::max(
                        table[i][self.capacite_max   - objet.valeur  ] + objet.poids,
                        table[i + 1][capacite],
                    ); // Include current object
                }
            }
        }

        // Reconstruct the solution from the table
        let mut solution = Vec::new();
        let mut capacite = self.capacite_max as usize;
        for i in self.liste_objets.len() - 1..=0 {
            if table[i + 1][capacite] != table[i][capacite] {
                solution.push(&self.liste_objets[i]);
                capacite -= self.liste_objets[i].poids as usize;
            }
        }

        solution
    }
}

fn main() {
    // Create objects (poids, valeur)
    let objets = vec![
        Objet::nouveau_objet(2.0, 3.0),
        Objet::nouveau_objet(3.0, 4.0),
        Objet::nouveau_objet(4.0, 5.0),
        // ... add more objects
    ];

    // Create knapsack with maximum capacity
    let sac_a_dos = SacADos::nouveau_sac_a_dos(5.0);
    for objet in objets {
        sac_a_dos.ajouter_objet(&objet);
    }

    // Solve the knapsack problem
    let solution = sac_a_dos.sac_a_dos_optimal();

    // Print the optimal solution
    println!("Sac à dos optimal:");
    for objet in solution {
        println!("Poids: {:?}, Valeur: {:?}", objet.poids, objet.valeur);
    }

    // Print the total maximum value
    println!("Valeur totale maximale: {}", sac_a_dos.valeur_totale);
}
