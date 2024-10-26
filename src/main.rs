fn main() {
    let mut databse: Vec<Movie> =Vec::new();
    register("Taxi 1".to_string(),7.2,"1998".to_string(), &mut databse )
}
#[derive(Debug,Clone)]
struct Movie {
    name:String,
    point:f32,
    year:String,
}
fn register(name:String,point:f32,year:String, database: &mut Vec<Movie>){
    let movie = Movie{
        name,
        point,
        year
    };
    database.push(movie.clone());
    println!("Filminiz basari ile kayd edilmistir{:?}",database)
}
