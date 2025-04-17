use tera::Tera;

pub fn init_template() -> Tera{
    match Tera::new("templates/**/*"){
        Ok(t)=> t,
        Err(e) =>{
            println!("Ошибка парсинга шаблона: {}",e);
            ::std::process::exit(1);
        }
    }
}