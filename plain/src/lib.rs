pub use matchers::Substring;
pub use sources::Concat;

mod sources {
    pub struct Concat {}
}

mod matchers {
    use linearf::session::Vars;
    use linearf::{async_trait, New, Shared, State};
    use linearf::{matcher::*, Item};
    use std::sync::Arc;

    pub struct Substring {
        _state: Shared<State>,
    }

    impl New for Substring {
        fn new(_state: &Shared<State>) -> Self
        where
            Self: Sized,
        {
            Self {
                _state: _state.clone(),
            }
        }
    }

    impl IsMatcher for Substring {
        type Params = ();
    }

    #[async_trait]
    impl SimpleScorer<()> for Substring {
        async fn score(
            &self,
            (vars, _): (&Arc<Vars>, &Arc<Self::Params>),
            item: &Arc<Item>,
        ) -> Score {
            return if item.view_for_matcing().find(&vars.query).is_some() {
                Score::new(item.id, vec![1])
            } else {
                Score::new(item.id, vec![0])
            };
        }

        async fn reusable(
            &self,
            (prev, _): (&Arc<Vars>, &Arc<Self::Params>),
            (senario, _): (&Arc<Vars>, &Arc<Self::Params>),
        ) -> bool {
            prev.query == senario.query
        }
    }
}
