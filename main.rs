//Constante global decimal (constante não aloca espaço na memória)
const PI:f32 = 3.14;

//Variável global mutável (mut não recomendado)
static mut VARIAVEL_GLOBAL:u8 = 1;

//Demonstração de uma função
fn sombra(){
    let a = 123;

    { // Demonstração de shadowing no escopo
        let b = 456;
        println!("b = {}", b);
        
        // Variável é re-declarada, não sobrepõem a anterior, cria uma nova, sendo essa uma "sombra"
        let a = 789; 
        println!("a = {}", a);
    }

    println!("a = {}", a);
}

fn escopo(){

    // mostra uma constante e o seu respectivo tamanho de memória, semelhante às ideias de C/C++
    println!("PI = {}, Tamanho de PI = {}", PI, std::mem::size_of_val(&PI));

    // quando uma variável global é definida como mutável, que não é o recomendado, temos um erro.
    // esse erro pode ser eliminado definindo um escopo como unsafe, basicamente dizendo "deixa comigo, sei o que eu tô fazendo"
    // não é recomendado esse uso, mas existem situações...
    unsafe{
        println!("VARIAVEL_GLOBAL = {}", VARIAVEL_GLOBAL); 
    }
    
    // declarações de variáveis e seus respectivos tipos, assim como o tamanho alocado das demais.
    let variavel:i32 = 300;
    println!("Variável = {}, tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));
    
    let decimal:f32 = 2.5;
    println!("Decimal = {}", decimal);

    let booleana:bool = false;
    println!("Booleana = {}, Tamanho boioleana = {}", booleana, std::mem::size_of_val(&booleana));

    let letra:char = 'C';
    println!("Char = {}, Tamanho char = {}", letra, std::mem::size_of_val(&letra));

}

// Função principal
fn main() {
    //Chamada dos escopos
    escopo(); 
    sombra(); 
}