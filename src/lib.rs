pub mod data;
pub mod operators;
pub mod tourney;

#[cfg(test)]
mod tests {
    use crate::data::Team;
    use crate::operators::end::EndInputs;
    use crate::operators::start::StartOutputs;
    use crate::operators::Handle;
    use crate::operators::OperatorType::Start;
    use crate::tourney::Tourney;

    #[test]
    fn basic_graph() {
        let mut tourney = Tourney::new(20);
        tourney.add_edge(
            tourney.starts[0],
            StartOutputs::Output.to_handle(),
            tourney.ends[0],
            EndInputs::Input.to_handle(),
            20,
        );
        let start = &mut tourney.tourney[tourney.starts[0]];
        if let Start(ref mut so) = start.op {
            so.init_seeding(vec![
                Team { id: 0, name: None },
                Team { id: 1, name: None },
                Team { id: 2, name: None },
            ])
        }
    }
}
