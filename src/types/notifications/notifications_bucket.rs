use std::collections::{hash_map::Entry, HashMap};

use chrono::{DateTime, Utc};
use derivative::Derivative;
use serde::{Deserialize, Serialize};

use crate::{
    runtime::Env,
    types::{
        notifications::NotificationItem,
        profile::UID,
        resource::{MetaItemId, VideoId},
    },
};

#[derive(Default, Derivative, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct NotificationsBucket {
    pub uid: UID,
    /// Notifications
    pub items: HashMap<MetaItemId, HashMap<VideoId, NotificationItem>>,
    #[derivative(Default(value = "Utc::now()"))]
    pub created: DateTime<Utc>,
}

impl NotificationsBucket {
    pub fn new<E: Env + 'static>(uid: UID, items: Vec<NotificationItem>) -> Self {
        NotificationsBucket {
            uid,
            items: items.into_iter().fold(HashMap::new(), |mut acc, item| {
                let meta_notifs: &mut HashMap<_, _> = acc.entry(item.meta_id.clone()).or_default();

                let notif_entry = meta_notifs.entry(item.video_id.clone());

                // for now just skip same videos that already exist
                // leave the first one found in the Vec.
                if let Entry::Vacant(new) = notif_entry {
                    new.insert(item);
                }

                acc
            }),
            created: E::now(),
        }
    }
}
