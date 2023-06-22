use enum_from_strings::roles_enum;

fn main() {
    roles_enum!(Roler);

    println!("{:?}", Roler::mariachi_music);
}
