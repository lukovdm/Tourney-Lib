pub mod data;
pub mod operators;
pub mod tourney;

#[cfg(test)]
mod tests {
    use crate::data::Team;
    use crate::operators::end::{EndInputs, EndOp};
    use crate::operators::splitter::{SplitInputs, SplitOp};
    use crate::operators::start::{StartInputs, StartOutputs};
    use crate::operators::Handle;
    use crate::operators::OperatorType::{End, Split, Start};
    use crate::tourney::{Edge, Node, Tourney};
    use petgraph::prelude::StableDiGraph;
    use serde_json::json;

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
        if let Start(ref mut so) = tourney.tourney[tourney.starts[0]].op {
            so.init_seeding(vec![
                Team { id: 0, name: None },
                Team { id: 1, name: None },
                Team { id: 2, name: None },
            ])
        }

        tourney.update();

        if let End(ref mut eo) = tourney.tourney[tourney.ends[0]].op {
            assert_eq!(
                eo.ranking,
                vec![
                    Team { id: 0, name: None },
                    Team { id: 1, name: None },
                    Team { id: 2, name: None },
                ]
            );
        }
    }

    #[test]
    fn test_split_graph() {
        let graph: StableDiGraph<Node, Edge> = serde_json::from_str(
            r#"
{
  "nodes": [
    {
      "id": 0,
      "op": {
        "Start": {
          "size": 20,
          "seeding": null
        }
      }
    },
    {
      "id": 1,
      "op": {
        "End": {
          "size": 10,
          "ranking": []
        }
      }
    },
    {
      "id": 2,
      "op": {
        "End": {
          "size": 10,
          "ranking": []
        }
      }
    },
    {
      "id": 3,
      "op": {
        "Split": {
          "at": 10
        }
      }
    }
  ],
  "node_holes": [],
  "edge_property": "directed",
  "edges": [
    [
      0,
      3,
      {
        "size": 20,
        "from": "Output",
        "to": "ToSplit",
        "data": null
      }
    ],
    [
      3,
      1,
      {
        "size": 10,
        "from": "Top",
        "to": "Input",
        "data": null
      }
    ],
    [
      3,
      2,
      {
        "size": 10,
        "from": "Bottom",
        "to": "Input",
        "data": null
      }
    ]
  ]
}
            "#,
        )
        .expect("Graph string incorrect");
        let mut tourney = Tourney::from_graph(graph);

        if let Start(ref mut op) = tourney
            .tourney
            .node_weight_mut(tourney.starts[0])
            .expect("Start does not exists")
            .op
        {
            op.init_seeding(vec![
                Team { id: 0, name: None },
                Team { id: 1, name: None },
                Team { id: 2, name: None },
                Team { id: 3, name: None },
                Team { id: 4, name: None },
                Team { id: 5, name: None },
                Team { id: 6, name: None },
                Team { id: 7, name: None },
                Team { id: 8, name: None },
                Team { id: 9, name: None },
                Team { id: 10, name: None },
                Team { id: 11, name: None },
                Team { id: 12, name: None },
                Team { id: 13, name: None },
                Team { id: 14, name: None },
                Team { id: 15, name: None },
                Team { id: 16, name: None },
                Team { id: 17, name: None },
                Team { id: 18, name: None },
                Team { id: 19, name: None },
            ])
        }

        tourney.update();

        if let End(ref eo) = tourney.tourney[tourney.ends[0]].op {
            assert_eq!(
                eo.ranking,
                vec![
                    Team { id: 0, name: None },
                    Team { id: 1, name: None },
                    Team { id: 2, name: None },
                    Team { id: 3, name: None },
                    Team { id: 4, name: None },
                    Team { id: 5, name: None },
                    Team { id: 6, name: None },
                    Team { id: 7, name: None },
                    Team { id: 8, name: None },
                    Team { id: 9, name: None },
                ]
            )
        }

        if let End(ref eo) = tourney.tourney[tourney.ends[1]].op {
            assert_eq!(
                eo.ranking,
                vec![
                    Team { id: 10, name: None },
                    Team { id: 11, name: None },
                    Team { id: 12, name: None },
                    Team { id: 13, name: None },
                    Team { id: 14, name: None },
                    Team { id: 15, name: None },
                    Team { id: 16, name: None },
                    Team { id: 17, name: None },
                    Team { id: 18, name: None },
                    Team { id: 19, name: None },
                ]
            )
        }

        todo!()
    }
}
