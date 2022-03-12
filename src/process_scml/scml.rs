pub struct ScmlObject{
    folders : Vec<Folder>,
    entities : Vec<Entity>,
    scml_version : f32,
    generator: String,
    generator_version: String,
}    

#[derive(Debug)]
pub struct Folder{
    id: String,
    name: String,
    files: Vec<File>
}

#[derive(Debug)]
pub struct File{
    id: String,
    name: String,
    pivot_x: f32,
    pivot_y: f32
    //width: f32,
    //height: f32,
}

#[derive(Debug)]
pub struct Entity{
    pub id: String,
    pub name: String,
    pub animations: Vec<Animation>
}

pub struct CharacterMap{
    id: String,
    name: String,
    maps: Vec<MapInstruction>

}

struct MapInstruction{
    folder: Folder,
    file: String,
    target_folder: Folder
}

#[derive(Debug)]
pub struct Animation{
    pub id: String,
    pub name: String,
    pub length: u32,
    pub looping: bool,

}

pub struct MainlineKey{
    id: String,
    time: u32,    
}

pub struct Ref{
    id: String,
    parent: i32,
    timeline: String,
    key: String,
}

pub struct Timeline{
    id: String,
    name: String,
    timeline_type: String, // TODO: enum : SPRITE,BONE,BOX,POINT,SOUND,ENTITY,VARIABLE

}

pub struct TimelineKey{
    id: String,
    time: u32,
    spin: u32,
    curve_type:u32,
    c1: u32,
    c2: u32,
}

pub struct BoneKey{
    x: u32,
    y: u32,
    angle: u32,
    scale_x: u32,
    scale_y: u32,
    a: u32,
}

pub struct ObjectKey{
    folder: Folder,
    file: String,
    x: u32,
    y: u32,
    angle: u32,
    scale_x: u32,
    scale_y: u32,
    pivot_x: u32,
    pivot_y: u32,
    a: u32,
}

