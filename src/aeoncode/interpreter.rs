use bevy::prelude::*;

use super::fragment::AeonFragment;
use super::matrix::AeonCodeMatrix;

/// Apply interpretation rules to decode a fragment semantically.
pub fn interpret_fragment(fragment: &AeonFragment) -> String {
    let mut decoded = fragment.encoded_data.clone();
    for rule in &fragment.interpretation_rules {
        decoded.push_str(&format!("|{}", rule.hint));
    }
    decoded
}

/// System that updates the global code matrix when new fragments appear.
pub fn update_matrix(
    mut matrix: ResMut<AeonCodeMatrix>,
    query: Query<&AeonFragment, Added<AeonFragment>>,
) {
    for frag in query.iter() {
        matrix.insert(frag.clone());
    }
}
