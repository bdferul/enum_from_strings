use enum_from_strings::roles_enum;

fn main() {
    roles_enum!(Roles);

    println!("{:?}", Roles::tyler);
}
