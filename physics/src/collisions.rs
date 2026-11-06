use crate::objects::Object;
use crate::vector;

// Returns the first colliding object, or None if there is no collision.
// also returns a direction in which the object should move.
pub fn check_collisions(objects: &[Object], id: usize) -> Option<(&Object, vector::Vec2)> {
    let selected_object = objects.get(id)?;
    if selected_object.name != 'c' {
        return None;
    }

    for (index, object) in objects.iter().enumerate() {
        if id == index || object.name != 'c' {
            continue;
        }

        let delta = selected_object.position - object.position;
        let distance = vector::distance(selected_object.position, object.position);
        let combined = selected_object.width / 2.0 + object.width / 2.0;

        if distance <= combined {
            return Some((object, delta.normalize()));
        }
    }

    None
}

