use plotters::prelude::*;
use rand::Rng;
use std::time::Instant;

#[derive(Copy, Clone)]
pub struct Matrix {
    tablero: [[i32; 8]; 8],
}

impl Matrix {
    // generar nuevo tablero con valores aleatorios
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

    // generar nuevo tablero vacio
    fn new_empty() -> Self {
        let tab = Matrix {
            tablero: [[0; 8]; 8],
        };

        tab
    }

    // obtener tablero actual
    fn get_tablero(&self) -> [[i32; 8]; 8] {
        self.tablero
    }

    // reemplazar tablero actual por uno nuevo
    fn reemplazar(&mut self, new_tablero: [[i32; 8]; 8]) {
        self.tablero = new_tablero;
    }

    // obtener el valor de la posicion en fila, columna
    fn get(&self, fila: usize, columna: usize) -> i32 {
        self.tablero[fila][columna]
    }

    // set nuevo valor en la posicion deseada
    fn set(&mut self, fila: usize, columna: usize, value: i32) { 
        self.tablero[fila][columna] = value;
    }

    // calcular aptitud en base a colisiones de las reinas horizontal, 
    // vertical y diagonalmente
    fn aptitud(&self) -> i32 { 
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

    // mutar una posicion la reina
    fn mutar(&mut self) -> [[i32; 8]; 8] { 
        let nr_x = rand::thread_rng().gen_range(0..8);
        let mut nr_y = 0;

        //buscar reina en fila
        for i in 0..8 {
            if self.get(nr_x, i) == 1 {
                nr_y = i;
                break;
            }
        }

        // si esta en el extremo derecho mover - 1
        if nr_y == 7 {
            self.set(nr_x, nr_y, 0);
            self.set(nr_x, nr_y - 1, 1);

            return self.tablero;
        }

        // si esta en el extremo izquierdo mover + 1
        if nr_y == 0 { 
            self.set(nr_x, nr_y, 0);
            self.set(nr_x, nr_y + 1, 1);
            return self.tablero;
        }

        // si esta en el medio decidir aleatoriamente si mover izq o der
        if rand::thread_rng().gen_bool(0.50) {
            self.set(nr_x, nr_y, 0);
            self.set(nr_x, nr_y + 1, 1);
        } else {
            self.set(nr_x, nr_y, 0);
            self.set(nr_x, nr_y - 1, 1);
        }

        self.tablero
    }

    fn verificar_diagonales(&self, fila: usize, columna: usize) -> i32 {
        let mut apt = 0;

        //noreste
        let mut cont_fila = fila;
        let mut cont_col = columna;
        if fila != 0 || columna != 7 {
            while cont_fila != 0 && cont_col != 7 {
                cont_fila = cont_fila - 1;
                cont_col = cont_col + 1;
                if self.get(cont_fila, cont_col) == 1 {
                    apt = apt + 1;
                    break;
                }
            }
        }

        //noroeste
        cont_fila = fila;
        cont_col = columna;
        if fila != 0 || columna != 0 {
            while cont_fila != 0 && cont_col != 0 {
                cont_fila = cont_fila - 1;
                cont_col = cont_col - 1;
                if self.get(cont_fila, cont_col) == 1 {
                    apt = apt + 1;
                    break;
                }
            }
        }

        //suroeste
        cont_fila = fila;
        cont_col = columna;
        if fila != 7 || columna != 0 {
            while cont_fila != 7 && cont_col != 0 {
                cont_fila = cont_fila + 1;
                cont_col = cont_col - 1;
                if self.get(cont_fila, cont_col) == 1 {
                    apt = apt + 1;
                    break;
                }
            }
        }

        //sureste
        cont_fila = fila;
        cont_col = columna;
        if fila != 7 || columna != 7 {
            while cont_fila != 7 && cont_col != 7 {
                cont_fila = cont_fila + 1;
                cont_col = cont_col + 1;
                if self.get(cont_fila, cont_col) == 1 {
                    apt = apt + 1;
                    break;
                }
            }
        }

        apt
    }

    fn verificar_xy(&self, fila: usize, columna: usize) -> i32 {
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

    fn _print(&self) {
        for fila in &self.tablero {
            for &valor in fila {
                print!("{} ", valor);
            }
            println!();
        }
    }
}

fn buscar_solucion_reinas(num_ejecucion: i32) -> (bool, i32) {
    //generar 100 matrices y guardar en pob
    let mut poblacion = Vec::new();
    let mut posiciones_pob = Vec::new();
    for i in 0..100 {
        posiciones_pob.push(i);
        poblacion.push(Matrix::new());
    }

    //iterar 10000 veces o hasta encontrar la solucion
    let mut es_solucion = false;
    let mut hist_aptitudes = Vec::new();
    let mut counter = 0;
    while !es_solucion && counter < 10000 {
        //calcular aptitud de cada matriz
        let mut aptitudes = Vec::new();
        for tablero in &poblacion {
            aptitudes.push(tablero.aptitud());
        }

        //obtener 5 aleatorios de la poblacion
        let mut posiciones_padres: Vec<usize> = Vec::new();
        let mut aptitudes_padres: Vec<i32> = Vec::new();
        let mut padres = Vec::new();
        for _ in 0..5 {
            let random = rand::thread_rng().gen_range(0..100);
            let tablero = &poblacion[random];
            padres.push(poblacion[random]);
            aptitudes_padres.push(tablero.aptitud());
            posiciones_padres.push(random);
        }

        // sin permutacion, utilizando vec std
        // padres.sort_by(|a, b| a.aptitud().cmp(&b.aptitud()));
        // let mut _descendiente_s = Matrix::new_empty();
        // _descendiente_s = padres[1];
        // for i in 4..8 {
        //     for j in 0..8 {
        //         let val = padres[0].get(i, j);
        //         let _ = _descendiente_s.set(i, j, val);
        //     }
        // }


        // permutacion de aptitudes hacia las posiciones para tener el mismo orden 
        // en ambas despues del sort
        let permutation = permutation::sort(&aptitudes_padres);
        posiciones_padres = permutation.apply_slice(&posiciones_padres);

        // cruzar (combinar dos padres)
        let mut _descendiente = Matrix::new_empty();
        _descendiente = poblacion[posiciones_padres[posiciones_padres.len() - 2]];
        for i in 4..8 {
            for j in 0..8 {
                let valor = poblacion[posiciones_padres[posiciones_padres.len() - 1]].get(i, j);
                let _ = _descendiente.set(i, j, valor);
            }
        }
        
        // permutar posiciones de la poblacion en base al orden de aptitudes
        let permutation_apts = permutation::sort(&aptitudes);
        let mut temp_posiciones_pob = permutation_apts.apply_slice(&posiciones_pob);
        temp_posiciones_pob.reverse();

        // reemplazar diez peores con descendientes o mutar (80%)
        for i in 0..10 {
            let mutacion = rand::thread_rng().gen_bool(0.8); //mutación del 80%
            if mutacion {
                poblacion[temp_posiciones_pob[i]].reemplazar(_descendiente.mutar());
            } else {
                poblacion[temp_posiciones_pob[i]].reemplazar(_descendiente.get_tablero());
            }
        }

        // recalcular aptitudes despues de la cruza y mutación
        aptitudes.clear();
        for tablero in &poblacion {
            let x = tablero.aptitud();
            aptitudes.push(x);
        }

        // encontrar mejor aptitud de la iteración y guardarlo en el historial de aptitudes
        let mejor_aptitud = aptitudes.iter().min();
        match mejor_aptitud {
            Some(&mejor) => {
                hist_aptitudes.push(mejor);
                //println!("#{}: {}", counter, mejor);
                if mejor == 0 {
                    print!("[{}] solucion encontrada: it#{}", num_ejecucion, counter);
                    hist_aptitudes.push(mejor);
                    es_solucion = true;
                }
            }
            None => (),
        }

        counter = counter + 1;
    }

    //encontrar e imprimir tablero con la solucion
    // for tablero in &poblacion {
    //     let x = tablero.aptitud();
    //     if x == 0 {
    //         tablero.print();
    //         println!("{}", x);
    //     }
    // }

    generar_grafico_aptitud(hist_aptitudes, counter, num_ejecucion);

    //regresar si es solución y el número en el que se encontró la solución
    (es_solucion, counter-1)
}

fn generar_grafico_aptitud(hist_aptitudes: Vec<i32>, counter: i32, num_ejecucion: i32) {
    let max_valor = hist_aptitudes.iter().max();
    let mut peor = 0;
    match max_valor {
        Some(&max) => peor = max,
        None => (),
    }
    let path_name = &format!("images/graf\'{num_ejecucion}\'.png");
    let root_area = BitMapBackend::new(path_name, (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Convergencia", ("sans-serif", 40))
        .build_cartesian_2d(0..(counter + 100), 0..peor)
        .unwrap();
    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(
        AreaSeries::new(
            (0..).zip(hist_aptitudes.iter().map(|x| *x)),
            0,
            &RED.mix(0.2),
        )
        .border_style(&RED),
    )
    .unwrap();
}

fn main() {
    let start_time_gen = Instant::now();
    let mut _ejecuciones_exitosas = 0;
    let mut evals_ejecuciones = Vec::new();

    //repetir 30 veces para generar información
    for num_ejecucion in 0..30 {
        let start_time_sin = Instant::now();
        let resultado = buscar_solucion_reinas(num_ejecucion);

        // si es solucion
        if resultado.0 {
            print!(" en: {}s", start_time_sin.elapsed().as_secs_f64());
            _ejecuciones_exitosas = _ejecuciones_exitosas + 1;
        } else {
            println!("[{}] No se pudo encontrar una solucion: {}s", num_ejecucion, start_time_sin.elapsed().as_secs_f64())
        }
        evals_ejecuciones.push(resultado.1);
        println!();
    }

    println!("Tiempo total: {}s", start_time_gen.elapsed().as_secs_f64());
    println!("Ejecuciones exitosas: {}", _ejecuciones_exitosas);

    //determinar mejor o peor ejecución según el min y max de las ejecuciones
    let mut mejor_ejecucion = 0;
    let mut peor_ejecucion = 0;
    match evals_ejecuciones.iter().min() {
        Some(&min) => mejor_ejecucion = min,
        None => (),
    }
    match evals_ejecuciones.iter().max() {
        Some(&max) => peor_ejecucion = max,
        None => (),
    }

    //descartar ejecuciones fallidas
    evals_ejecuciones.retain(|&x| x != 999);

    // calculo medidas de tendencia central
    let mut avg = 0;
    for i in &evals_ejecuciones {
        avg = avg + i;
    }
    avg = avg / (evals_ejecuciones.len() as i32);

    let mut _mediana = 0;
    evals_ejecuciones.sort();
    let mid = evals_ejecuciones.len() / 2;
    _mediana = evals_ejecuciones[mid];

    println!("Evaluaciones de la mejor ejecucion: {}", mejor_ejecucion);
    println!("Evaluaciones de la peor ejecucion: {}", peor_ejecucion);
    println!("Media de evaluaciones: {}", avg);
    println!("Mediana de evaluaciones: {}", _mediana);
}
