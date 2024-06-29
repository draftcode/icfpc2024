use crate::compiler::parser::parse;

pub fn compile(prog: String) -> anyhow::Result<()> {
    let mut program = parse(input.chars())?;

    while let Some(p) = update(program) {
        program = p;
    }

    println!("{:?}", program);

    Ok(())
}

fn update(program: Program) -> Option<Program> {
    todo!()
}
