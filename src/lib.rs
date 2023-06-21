pub mod data;
pub mod operators;
pub mod tourney;

#[cfg(test)]
mod tests {
    use crate::data::Team;
    use crate::operators::OperatorType::{End, Start};
    use crate::tourney::{Edge, Node, Tourney};
    use petgraph::prelude::StableDiGraph;

    #[test]
    fn basic_graph() {
        let mut tourney = Tourney::new();
        tourney.add_edge(tourney.starts[0], "Output", tourney.ends[0], "Input");
        if let Start(ref mut so) = tourney.tourney[tourney.starts[0]].op {
            so.set_seeding(vec![
                Team { id: 0, name: None },
                Team { id: 1, name: None },
                Team { id: 2, name: None },
            ])
        }

        tourney.update();

        // println!(
        //     "{}",
        //     serde_json::to_string(&tourney.tourney).expect("Serialization failed")
        // );

        if let End(ref mut eo) = tourney.tourney[tourney.ends[0]].op {
            assert_eq!(
                eo.ranking(),
                Ok(vec![
                    Team { id: 0, name: None },
                    Team { id: 1, name: None },
                    Team { id: 2, name: None },
                ])
            );
        }
    }

    #[test]
    fn test_split_graph() {
        let graph: StableDiGraph<Node, Edge> =
            serde_json::from_str(
                r#"{"nodes":[{"id":0,"op":{"Start":{"seeding":null}}},{"id":1,"op":{"End":{"ranking":null,"input":null}}},{"id":2,"op":{"End":{"ranking":null,"input":null}}},{"id":3,"op":{"Split":{"at":2,"input":null,"output":null}}}],"node_holes":[],"edge_property":"directed","edges":[[0,3,{"from":"Output","to":"Input","data":null}],[3,1,{"from":"Top","to":"Input","data":null}],[3,2,{"from":"Bottom","to":"Input","data":null}]]}"#).expect("Graph string incorrect");
        let mut tourney = Tourney::from_graph(graph);

        if let Start(ref mut op) = tourney
            .tourney
            .node_weight_mut(tourney.starts[0])
            .expect("Start does not exists")
            .op
        {
            op.set_seeding(vec![
                Team { id: 0, name: None },
                Team { id: 1, name: None },
                Team { id: 2, name: None },
                Team { id: 3, name: None },
            ])
        }

        tourney.update();

        if let End(ref eo) = tourney.tourney[tourney.ends[0]].op {
            assert_eq!(
                eo.ranking(),
                Ok(vec![Team { id: 0, name: None }, Team { id: 1, name: None },])
            )
        }

        if let End(ref eo) = tourney.tourney[tourney.ends[1]].op {
            assert_eq!(
                eo.ranking(),
                Ok(vec![Team { id: 2, name: None }, Team { id: 3, name: None },])
            )
        }
    }

    #[test]
    fn test_reset() {
        let graph: StableDiGraph<Node, Edge> =
            serde_json::from_str(
                r#"{"nodes":[{"id":0,"op":{"Start":{"seeding":null}}},{"id":1,"op":{"End":{"ranking":null,"input":null}}},{"id":2,"op":{"End":{"ranking":null,"input":null}}},{"id":3,"op":{"Split":{"at":2,"input":null,"output":null}}}],"node_holes":[],"edge_property":"directed","edges":[[0,3,{"from":"Output","to":"Input","data":null}],[3,1,{"from":"Top","to":"Input","data":null}],[3,2,{"from":"Bottom","to":"Input","data":null}]]}"#).expect("Graph string incorrect");
        let mut tourney = Tourney::from_graph(graph);

        if let Start(ref mut op) = tourney
            .tourney
            .node_weight_mut(tourney.starts[0])
            .expect("Start does not exists")
            .op
        {
            op.set_seeding(vec![
                Team { id: 0, name: None },
                Team { id: 1, name: None },
                Team { id: 2, name: None },
                Team { id: 3, name: None },
            ])
        }

        tourney.update();

        tourney.reset();

        if let End(ref eo) = tourney.tourney[tourney.ends[0]].op {
            assert_eq!(eo.ranking(), Err("No end ranking found".to_string()))
        }

        if let End(ref eo) = tourney.tourney[tourney.ends[1]].op {
            assert_eq!(eo.ranking(), Err("No end ranking found".to_string()))
        }
    }
}
