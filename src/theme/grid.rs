use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<GridAlignment>();
}

// TODO: Workaround for <https://github.com/bevyengine/bevy/issues/19272>.
/// Row and column alignment for a [`Display::Grid`] node.
///
/// NOTE: This assumes that the grid has [`rows.len()`](Self::rows) rows and
/// [`columns.len()`](Self::columns) columns, and none of the grid items have
/// a custom [`GridPlacement`].
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct GridAlignment {
    pub rows: Vec<AlignSelf>,
    pub columns: Vec<JustifySelf>,
}

impl Configure for GridAlignment {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, apply_grid_alignment.in_set(UpdateSystems::SyncLate));
    }
}

impl GridAlignment {
    pub fn rows(rows: impl Into<Vec<AlignSelf>>) -> Self {
        Self {
            rows: rows.into(),
            columns: vec![],
        }
    }

    pub fn columns(columns: impl Into<Vec<JustifySelf>>) -> Self {
        Self {
            rows: vec![],
            columns: columns.into(),
        }
    }
}

#[cfg_attr(feature = "native_dev", hot)]
fn apply_grid_alignment(
    grid_query: Query<(Entity, &GridAlignment, &Children)>,
    mut item_query: Query<&mut Node>,
) {
    for (entity, alignment, children) in &grid_query {
        let flow = c!(item_query.get(entity)).grid_auto_flow;
        let rows = alignment.rows.len();
        let cols = alignment.columns.len();

        for (i, child) in children.iter().enumerate() {
            let mut item = c!(item_query.get_mut(child));

            let (row, col) = match flow {
                GridAutoFlow::Row | GridAutoFlow::RowDense => (i / cols, i % cols),
                GridAutoFlow::Column | GridAutoFlow::ColumnDense => (i % rows, i / rows),
            };
            if let Some(&x) = alignment.rows.get(row) {
                item.align_self = x;
            }
            if let Some(&x) = alignment.columns.get(col) {
                item.justify_self = x;
            }
        }
    }
}
