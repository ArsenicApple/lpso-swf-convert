use swf_types::Tag;
use swf_types::Movie;
use swf_types::tags;

mod scml;

pub fn process_swf(movie : Movie){
    let mut folders : Vec<scml::Folder> = Vec::new();
    let mut entities : Vec<scml::Entity> = Vec::new();  
    read_movie_tags(movie.tags, &mut folders, &mut entities);
    println!("{:#?}",entities);
}

// read the tags inside a movie
fn read_movie_tags(tags : Vec<Tag>, folders : &mut Vec<scml::Folder>, entities : &mut Vec<scml::Entity>){
    for tag in tags {
        match tag{
            // the first sprite is often the main entity
            Tag::DefineSprite(sprite_tag)=>{
                let entity : scml::Entity = read_define_sprite_entity(sprite_tag, folders, entities);
                entities.push(entity);
            }
            _=>{}
        }
    }
}

fn read_sprite_tags(tags : Vec<Tag>, folders : &mut Vec<scml::Folder>, entity : &mut scml::Entity){
    for tag in tags {
        match tag{
            // the first sprite is often the main entity
            Tag::DefineSprite(sprite_tag)=>{

            }
            _=>{}
        }
    }
}

const FPS : u32 = (1.00/60.00*1000.00) as u32;
fn read_define_sprite_entity(sprite_tag : tags::DefineSprite, folders : &mut Vec<scml::Folder>, entities : &mut Vec<scml::Entity>) -> scml::Entity {    
    let time : u32 = (sprite_tag.frame_count as u32)*FPS;
    let id : String =  sprite_tag.id.to_string();
    let id_anim : String = format!("{}_{}",id,"anim");

    let mut entity : scml::Entity = scml::Entity{id: id.clone(), name: id.clone(), animations: Vec::new()};
    let animation = scml::Animation{id: id_anim.clone(), name: id_anim.clone(), length: time, looping: true};
    entity.animations.push(animation);

    read_sprite_tags(sprite_tag.tags, folders, &mut entity);   
    return entity
}

fn read_define_shape(shape_tag : tags::DefineShape){
}