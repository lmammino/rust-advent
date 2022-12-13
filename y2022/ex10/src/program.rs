#[derive(Debug, PartialEq)]
pub(crate) enum Cmd {
    Noop,
    AddX(i32),
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Cpu {
    pub x: i32,
    pub cycle: i32,
}

impl Cpu {
    fn new() -> Self {
        Self { x: 1, cycle: 1 }
    }
}

pub(crate) struct Program<I>
where
    I: Iterator<Item = Cmd>,
{
    cpu: Cpu,
    pending_add: Option<i32>,
    cmds: I,
}

impl<I> Program<I>
where
    I: Iterator<Item = Cmd>,
{
    pub(crate) fn new(cmds: I) -> Self {
        Self {
            cpu: Cpu::new(),
            pending_add: None,
            cmds,
        }
    }
}

impl<I> Iterator for Program<I>
where
    I: Iterator<Item = Cmd>,
{
    type Item = Cpu;

    fn next(&mut self) -> Option<Self::Item> {
        self.cpu.cycle += 1;

        if let Some(pending_add) = self.pending_add.take() {
            self.cpu.x += pending_add;
        } else {
            let cmd = self.cmds.next()?;
            if let Cmd::AddX(value) = cmd {
                self.pending_add = Some(value);
            }
        }

        Some(self.cpu.clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_example_small() {
        let cmds = vec![Cmd::Noop, Cmd::AddX(3), Cmd::AddX(-5)];
        let mut program = Program::new(cmds.into_iter());

        // after cycle 1:
        program.next();
        assert_eq!(program.cpu, Cpu { x: 1, cycle: 2 });

        // after cycle 2:
        program.next();
        assert_eq!(program.cpu, Cpu { x: 1, cycle: 3 });
        assert_eq!(program.pending_add, Some(3));

        // after cycle 3:
        program.next();
        assert_eq!(program.cpu, Cpu { x: 4, cycle: 4 });
        assert_eq!(program.pending_add, None);

        // after cycle 4:
        program.next();
        assert_eq!(program.cpu, Cpu { x: 4, cycle: 5 });
        assert_eq!(program.pending_add, Some(-5));

        // after cycle 5:
        program.next();
        assert_eq!(program.cpu, Cpu { x: -1, cycle: 6 });
        assert_eq!(program.pending_add, None);
    }
}
