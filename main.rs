struct Funcion1elemento {
    variable: char,
    coeficiente: i64,
    exponente: i64,
}
impl Funcion1elemento {
    fn comprobar_derivada(&self) -> bool {
        self.variable > 'k'
    }
}

struct Funcion2elementos {
    variable: char,
    coeficiente: i64,
    exponente: i64,
    coeficiente2: i64,
    exponente_general: i64
}

struct Derivada(i64,char,i64);

fn main() {
    let cambio: (i64,char,i64);
    let user1 = Funcion1elemento {
        variable: 'x',
        coeficiente: 5,
        exponente: 10,
    };
    let user2 = Funcion2elementos {
        variable: 'x',
        coeficiente: 5,
        exponente: 10,
        coeficiente2: 4,
        exponente_general: 8,
    };

     if user1.comprobar_derivada() {
         cambio = derivada_algebraica(&user1);
         let derivada = Derivada(cambio.0,cambio.1,cambio.2);
         println!("La derivada es {}{}^{}",derivada.0,derivada.1,derivada.2);
         derivada_raiz_cuadrada(&user1,&derivada);
         derivada_compuesta(&user2,&derivada);
     }
     else {
         println!("La derivada es 0");
     }
}

fn derivada_algebraica(funcion: &Funcion1elemento) -> (i64,char,i64){
    (funcion.coeficiente*funcion.exponente,funcion.variable, funcion.exponente -1)
}

fn derivada_raiz_cuadrada(funcion: &Funcion1elemento, derivada: &Derivada) {
    println!("\nLa derivada es:\n {}{}^{}\n--------\n2√{}{}^{}",derivada.0,derivada.1,derivada.2,
    funcion.coeficiente,funcion.variable,funcion.exponente);
}

fn derivada_compuesta(funcion: &Funcion2elementos, derivada: &Derivada){
    println!("\nLa funcion es: ({}{}^{}+{})^{}",funcion.coeficiente,funcion.variable,funcion.exponente,funcion.coeficiente2,
    funcion.exponente_general);
    println!("\nLa derivada es: {}{}^{}({}{}^{}+{})^{}",funcion.exponente_general*derivada.0,funcion.variable,funcion.exponente-1,
    funcion.coeficiente,funcion.variable,funcion.exponente,funcion.coeficiente2,
    funcion.exponente_general-1);
}
