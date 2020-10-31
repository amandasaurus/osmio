mod arc_types;
mod rc_types;
mod string_types;

use {Node, OSMObjBase, Relation, Way};

pub use self::arc_types::*;
pub use self::rc_types::*;
pub use self::string_types::*;

impl From<RcNode> for StringNode {
    fn from(obj: RcNode) -> Self {
        let mut n = StringNodeBuilder::default()._id(obj.id()).build().unwrap();

        n.set_version(obj.version());
        n.set_deleted(obj.deleted());
        n.set_changeset_id(obj.changeset_id());
        n.set_timestamp(obj.timestamp().clone());
        n.set_uid(obj.uid());
        n.set_user(obj.user());
        n.set_lat_lon(obj.lat_lon());

        for (k, v) in obj.tags() {
            n.set_tag(k, v);
        }

        n
    }
}

impl From<RcWay> for StringWay {
    fn from(obj: RcWay) -> Self {
        let mut w = StringWayBuilder::default()._id(obj.id()).build().unwrap();

        w.set_version(obj.version());
        w.set_deleted(obj.deleted());
        w.set_changeset_id(obj.changeset_id());
        w.set_timestamp(obj.timestamp().clone());
        w.set_uid(obj.uid());
        w.set_user(obj.user());

        for (k, v) in obj.tags() {
            w.set_tag(k, v);
        }

        w.set_nodes(obj.nodes().iter().map(|i| i.clone()));

        w
    }
}

impl From<RcRelation> for StringRelation {
    fn from(obj: RcRelation) -> Self {
        let mut r = StringRelationBuilder::default()
            ._id(obj.id())
            .build()
            .unwrap();

        r.set_version(obj.version());
        r.set_deleted(obj.deleted());
        r.set_changeset_id(obj.changeset_id());
        r.set_timestamp(obj.timestamp().clone());
        r.set_uid(obj.uid());
        r.set_user(obj.user());

        for (k, v) in obj.tags() {
            r.set_tag(k, v);
        }

        r.set_members(obj.members());

        r
    }
}

impl From<RcOSMObj> for StringOSMObj {
    fn from(obj: RcOSMObj) -> Self {
        match obj {
            RcOSMObj::Node(n) => StringOSMObj::Node(n.into()),
            RcOSMObj::Way(w) => StringOSMObj::Way(w.into()),
            RcOSMObj::Relation(r) => StringOSMObj::Relation(r.into()),
        }
    }
}
