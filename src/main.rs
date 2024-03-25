use rand::Rng;

pub struct Matrix {
    tablero: [[u32; 8]; 8],
}

impl Matrix {
    fn new() -> Self {
        let mut tab = Matrix {
            tablero: [[0; 8]; 8],
        };

        for fila in &mut tab.tablero {
            let nr = rand::thread_rng().gen_range(0..8);
            fila[nr] = 1;
        }

        tab
    }

    fn get(&self, fila: usize, columna: usize) -> u32 {
        self.tablero[fila][columna]
    }

    fn set(&mut self, fila: usize, columna: usize, value: u32) {
        self.tablero[fila][columna] = value;
    }

    fn aptitud(&self) -> u32 {
        let mut apt = 0;
        for i in 0..8 {
            for j in 0..8 {
                if self.get(i, j) != 0 {
                    apt += self.verificar_xy(i, j);
                    apt += self.verificar_diagonales(i, j);
                }
            }
        }

        apt
    }

    fn verificar_diagonales(&self, fila: usize, columna: usize) -> u32 {
        let mut apt = 0;

        //noreste
        let mut cf = fila;
        let mut cc = columna;
        if fila != 0 || columna != 7 {
            while cf != 0 && cc != 7 {
                cf = cf - 1;
                cc = cc + 1;
                if self.get(cf, cc) == 1 {
                    apt = apt + 1;
                    break;
                }
            }
        }

        //noroeste
        cf = fila;
        cc = columna;
        if fila != 0 || columna != 0 {
            while cf != 0 && cc != 0 {
                cf = cf - 1;
                cc = cc - 1;
                if self.get(cf, cc) == 1 {
                    apt = apt + 1;
                    break;
                }
            }
        }

        //suroeste
        cf = fila;
        cc = columna;
        if fila != 7 || columna != 0 {
            while cf != 7 && cc != 0 {
                cf = cf + 1;
                cc = cc - 1;
                if self.get(cf, cc) == 1 {
                    apt = apt + 1;
                    break;
                }
            }
        }

        //sureste
        cf = fila;
        cc = columna;
        if fila != 7 || columna != 7 {
            while cf != 7 && cc != 7 {
                cf = cf + 1;
                cc = cc + 1;
                if self.get(cf, cc) == 1 {
                    apt = apt + 1;
                    break;
                }
            }
        }

        apt
    }

    fn verificar_xy(&self, fila: usize, columna: usize) -> u32 {
        let mut apt = 0;

        //vertical
        for i in fila..8 {
            if fila == i {
                continue;
            }
            if self.get(i, columna) == 1 {
                apt = apt + 1;
                break;
            }
        }
        for i in (fila..8).rev() {
            if fila == i {
                continue;
            }
            if self.get(i, columna) == 1 {
                apt = apt + 1;
                break;
            }
        }

        //horizontal
        for i in columna..8 {
            if columna == i {
                continue;
            }
            if self.get(fila, i) == 1 {
                apt = apt + 1;
                break;
            }
        }
        for i in (columna..8).rev() {
            if columna == i {
                continue;
            }
            if self.get(fila, i) == 1 {
                apt = apt + 1;
                break;
            }
        }

        apt
    }

    fn print(&self) {
        for row in &self.tablero {
            for &value in row {
                print!("{} ", value);
            }
            println!();
        }
    }
}

fn main() {
    let mut pob = Vec::new();
    for _ in 0..100 {
        pob.push(Matrix::new());
    }

    for tab in pob {
        println!("{:?}", tab.aptitud());
    }
}
