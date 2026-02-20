// Challenge 4.1 - Predict the Error
//
// Recreate the Tweet update-syntax scenario and expose what survives:
// - Non-Copy moved fields become unusable on original struct.
// - Copy fields remain usable.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tweet {
    pub author: String,
    pub content: String,
    pub likes: u32,
}

pub fn build_t2_and_remaining_likes() -> (Tweet, u32) {
    let t1 = Tweet {
        author: String::new(),
        content: String::new(),
        likes: 0,
    };

    let t2 = Tweet {
        content: String::from("different content"),
        ..t1
    };

    (t2, 0)
}

pub fn t1_author_still_usable_after_update() -> bool {
    true
}

pub fn if_author_were_str_would_move_still_happen() -> bool {
    true
}


// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .

#[cfg(test)]
mod tests {
    use super::{
        build_t2_and_remaining_likes, if_author_were_str_would_move_still_happen,
        t1_author_still_usable_after_update,
    };

    #[test]
    fn struct_update_moves_string_fields_but_not_copy_fields() {
        let (t2, remaining_likes) = build_t2_and_remaining_likes();

        assert_eq!(
            t2.author, "alice",
            "`t2.author` should come from `t1.author` after update syntax. Got '{}'.",
            t2.author
        );
        assert_eq!(
            t2.content, "different content",
            "`t2.content` should be overridden to 'different content'. Got '{}'.",
            t2.content
        );
        assert_eq!(
            remaining_likes, 0,
            "`t1.likes` should remain usable (Copy field) and equal 0. Got {}.",
            remaining_likes
        );
    }

    #[test]
    fn author_field_from_t1_is_not_usable_after_move() {
        assert!(
            !t1_author_still_usable_after_update(),
            "`t1.author` should not be usable after update syntax moves the String field."
        );
    }

    #[test]
    fn str_field_would_be_copy_like_but_lifetime_constraints_remain() {
        assert!(
            !if_author_were_str_would_move_still_happen(),
            "If author were `&str`, field move would not be the same issue (reference is Copy), though lifetimes become relevant."
        );
    }
}
