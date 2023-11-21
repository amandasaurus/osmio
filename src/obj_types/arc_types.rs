use std::sync::Arc;
use *;

macro_rules! func_call_inner_get {
    ($slf:ident, $name:ident) => {
        match $slf {
            ArcOSMObj::Node(x) => x.$name(),
            ArcOSMObj::Way(x) => x.$name(),
            ArcOSMObj::Relation(x) => x.$name(),
        }
    };
}

macro_rules! func_call_inner_set {
    ($slf:ident, $name:ident, $val:ident) => {
        match $slf {
            ArcOSMObj::Node(x) => x.$name($val),
            ArcOSMObj::Way(x) => x.$name($val),
            ArcOSMObj::Relation(x) => x.$name($val),
        };
    };
}

#[derive(PartialEq, Debug, Clone)]
pub struct ArcNode {
    pub(crate) _id: ObjId,
    pub(crate) _version: Option<u32>,
    pub(crate) _deleted: bool,
    pub(crate) _changeset_id: Option<u32>,
    pub(crate) _timestamp: Option<TimestampFormat>,
    pub(crate) _uid: Option<u32>,
    pub(crate) _user: Option<Arc<str>>,
    pub(crate) _tags: Option<Vec<(Arc<str>, Arc<str>)>>,

    pub(crate) _lat_lon: Option<(Lat, Lon)>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct ArcWay {
    pub(crate) _id: ObjId,
    pub(crate) _version: Option<u32>,
    pub(crate) _deleted: bool,
    pub(crate) _changeset_id: Option<u32>,
    pub(crate) _timestamp: Option<TimestampFormat>,
    pub(crate) _uid: Option<u32>,
    pub(crate) _user: Option<Arc<str>>,
    pub(crate) _tags: Vec<(Arc<str>, Arc<str>)>,

    pub(crate) _nodes: Vec<ObjId>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct ArcRelation {
    pub(crate) _id: ObjId,
    pub(crate) _version: Option<u32>,
    pub(crate) _deleted: bool,
    pub(crate) _changeset_id: Option<u32>,
    pub(crate) _timestamp: Option<TimestampFormat>,
    pub(crate) _uid: Option<u32>,
    pub(crate) _user: Option<Arc<str>>,
    pub(crate) _tags: Vec<(Arc<str>, Arc<str>)>,

    pub(crate) _members: Vec<(OSMObjectType, ObjId, Arc<str>)>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum ArcOSMObj {
    Node(ArcNode),
    Way(ArcWay),
    Relation(ArcRelation),
}

impl OSMObjBase for ArcOSMObj {
    fn id(&self) -> ObjId {
        func_call_inner_get!(self, id)
    }
    fn version(&self) -> Option<u32> {
        func_call_inner_get!(self, version)
    }
    fn deleted(&self) -> bool {
        func_call_inner_get!(self, deleted)
    }
    fn changeset_id(&self) -> Option<u32> {
        func_call_inner_get!(self, changeset_id)
    }
    fn timestamp(&self) -> &Option<TimestampFormat> {
        func_call_inner_get!(self, timestamp)
    }
    fn uid(&self) -> Option<u32> {
        func_call_inner_get!(self, uid)
    }
    fn user(&self) -> Option<&str> {
        func_call_inner_get!(self, user)
    }

    fn set_id(&mut self, val: impl Into<ObjId>) {
        func_call_inner_set!(self, set_id, val);
    }
    fn set_version(&mut self, val: impl Into<Option<u32>>) {
        func_call_inner_set!(self, set_version, val);
    }
    fn set_deleted(&mut self, val: bool) {
        func_call_inner_set!(self, set_deleted, val);
    }
    fn set_changeset_id(&mut self, val: impl Into<Option<u32>>) {
        func_call_inner_set!(self, set_changeset_id, val);
    }
    fn set_timestamp(&mut self, val: impl Into<Option<TimestampFormat>>) {
        func_call_inner_set!(self, set_timestamp, val);
    }
    fn set_uid(&mut self, val: impl Into<Option<u32>>) {
        func_call_inner_set!(self, set_uid, val);
    }
    fn set_user<'a>(&mut self, val: impl Into<Option<&'a str>>) {
        func_call_inner_set!(self, set_user, val);
    }

    fn tags<'a>(&'a self) -> Box<dyn ExactSizeIterator<Item = (&'a str, &'a str)> + 'a> {
        match self {
            ArcOSMObj::Node(x) => x.tags(),
            ArcOSMObj::Way(x) => x.tags(),
            ArcOSMObj::Relation(x) => x.tags(),
        }
    }

    fn num_tags(&self) -> usize {
        match self {
            ArcOSMObj::Node(x) => x._tags.as_ref().map_or(0, |t| t.len()),
            ArcOSMObj::Way(x) => x._tags.len(),
            ArcOSMObj::Relation(x) => x._tags.len(),
        }
    }
    fn untagged(&self) -> bool {
        match self {
            ArcOSMObj::Node(x) => x._tags.as_ref().map_or(true, |t| t.is_empty()),
            ArcOSMObj::Way(x) => x._tags.is_empty(),
            ArcOSMObj::Relation(x) => x._tags.is_empty(),
        }
    }

    fn tag(&self, key: impl AsRef<str>) -> Option<&str> {
        match self {
            ArcOSMObj::Node(x) => x.tag(key),
            ArcOSMObj::Way(x) => x.tag(key),
            ArcOSMObj::Relation(x) => x.tag(key),
        }
    }

    fn set_tag(&mut self, key: impl AsRef<str>, value: impl Into<String>) {
        match self {
            ArcOSMObj::Node(x) => x.set_tag(key, value),
            ArcOSMObj::Way(x) => x.set_tag(key, value),
            ArcOSMObj::Relation(x) => x.set_tag(key, value),
        }
    }

    fn unset_tag(&mut self, key: impl AsRef<str>) {
        match self {
            ArcOSMObj::Node(x) => x.unset_tag(key),
            ArcOSMObj::Way(x) => x.unset_tag(key),
            ArcOSMObj::Relation(x) => x.unset_tag(key),
        }
    }

    fn object_type(&self) -> OSMObjectType {
        match self {
            ArcOSMObj::Node(_) => OSMObjectType::Node,
            ArcOSMObj::Way(_) => OSMObjectType::Way,
            ArcOSMObj::Relation(_) => OSMObjectType::Relation,
        }
    }
}

impl OSMObj for ArcOSMObj {
    type Node = ArcNode;
    type Way = ArcWay;
    type Relation = ArcRelation;


    fn into_node(self) -> Option<ArcNode> {
        if let ArcOSMObj::Node(n) = self {
            Some(n)
        } else {
            None
        }
    }

    fn into_way(self) -> Option<ArcWay> {
        if let ArcOSMObj::Way(w) = self {
            Some(w)
        } else {
            None
        }
    }

    fn into_relation(self) -> Option<ArcRelation> {
        if let ArcOSMObj::Relation(r) = self {
            Some(r)
        } else {
            None
        }
    }

    fn as_node(&self) -> Option<&ArcNode> {
        if let ArcOSMObj::Node(n) = self {
            Some(n)
        } else {
            None
        }
    }

    fn as_way(&self) -> Option<&ArcWay> {
        if let ArcOSMObj::Way(w) = self {
            Some(w)
        } else {
            None
        }
    }

    fn as_relation(&self) -> Option<&ArcRelation> {
        if let ArcOSMObj::Relation(r) = self {
            Some(r)
        } else {
            None
        }
    }

    fn as_node_mut(&mut self) -> Option<&mut ArcNode> {
        if let ArcOSMObj::Node(n) = self {
            Some(n)
        } else {
            None
        }
    }

    fn as_way_mut(&mut self) -> Option<&mut ArcWay> {
        if let ArcOSMObj::Way(w) = self {
            Some(w)
        } else {
            None
        }
    }

    fn as_relation_mut(&mut self) -> Option<&mut ArcRelation> {
        if let ArcOSMObj::Relation(r) = self {
            Some(r)
        } else {
            None
        }
    }
}

impl OSMObjBase for ArcNode {
    fn id(&self) -> ObjId {
        self._id
    }
    fn version(&self) -> Option<u32> {
        self._version
    }
    fn deleted(&self) -> bool {
        self._deleted
    }
    fn changeset_id(&self) -> Option<u32> {
        self._changeset_id
    }
    fn timestamp(&self) -> &Option<TimestampFormat> {
        &self._timestamp
    }
    fn uid(&self) -> Option<u32> {
        self._uid
    }
    fn user(&self) -> Option<&str> {
        self._user.as_ref().map(|x| x as _)
    }

    fn set_id(&mut self, val: impl Into<ObjId>) {
        self._id = val.into();
    }
    fn set_version(&mut self, val: impl Into<Option<u32>>) {
        self._version = val.into();
    }
    fn set_deleted(&mut self, val: bool) {
        self._deleted = val;
    }
    fn set_changeset_id(&mut self, val: impl Into<Option<u32>>) {
        self._changeset_id = val.into();
    }
    fn set_timestamp(&mut self, val: impl Into<Option<TimestampFormat>>) {
        self._timestamp = val.into();
    }
    fn set_uid(&mut self, val: impl Into<Option<u32>>) {
        self._uid = val.into();
    }
    fn set_user<'a>(&mut self, val: impl Into<Option<&'a str>>) {
        self._user = val.into().map(Arc::from);
    }

    fn tags<'a>(&'a self) -> Box<dyn ExactSizeIterator<Item = (&'a str, &'a str)> + 'a> {
        match self._tags {
            None => Box::new(std::iter::empty()),
            Some(ref t) => Box::new(t.iter().map(|(k, v)| (k.as_ref(), v.as_ref()))),
        }
    }

    fn tag(&self, key: impl AsRef<str>) -> Option<&str> {
        let key = key.as_ref();
        self._tags.as_ref().and_then(|tags| {
            tags.iter()
                .filter_map(|(k, v)| {
                    if k.as_ref() == key {
                        Some(v.as_ref())
                    } else {
                        None
                    }
                })
                .next()
        })
    }

    fn set_tag(&mut self, key: impl AsRef<str>, value: impl Into<String>) {
        let key = key.as_ref();
        let value = value.into();
        match self._tags {
            None => {
                self._tags = Some(vec![(Arc::from(key), Arc::from(value.as_str()))]);
            }
            Some(ref mut tags) => {
                let idx = tags
                    .iter()
                    .enumerate()
                    .filter_map(|(i, (k, _))| if k.as_ref() == key { Some(i) } else { None })
                    .next();
                match idx {
                    None => tags.push((Arc::from(key), Arc::from(value.as_str()))),
                    Some(i) => tags[i] = (key.into(), Arc::from(value.as_str())),
                }
            }
        }
    }

    fn unset_tag(&mut self, key: impl AsRef<str>) {
        if let Some(ref mut tags) = self._tags {
            let key = key.as_ref();
            let idx = tags
                .iter()
                .enumerate()
                .filter_map(|(i, (k, _))| if k.as_ref() == key { Some(i) } else { None })
                .next();
            if let Some(i) = idx {
                tags.remove(i);
            }
        }
    }

    fn object_type(&self) -> OSMObjectType {
        OSMObjectType::Node
    }

}

impl Node for ArcNode {
    fn lat_lon(&self) -> Option<(Lat, Lon)> {
        self._lat_lon
    }

    fn set_lat_lon_direct(&mut self, loc: Option<(Lat, Lon)>) {
        self._lat_lon = loc;
    }

}

impl OSMObjBase for ArcWay {
    fn id(&self) -> ObjId {
        self._id
    }
    fn version(&self) -> Option<u32> {
        self._version
    }
    fn deleted(&self) -> bool {
        self._deleted
    }
    fn changeset_id(&self) -> Option<u32> {
        self._changeset_id
    }
    fn timestamp(&self) -> &Option<TimestampFormat> {
        &self._timestamp
    }
    fn uid(&self) -> Option<u32> {
        self._uid
    }
    fn user(&self) -> Option<&str> {
        self._user.as_ref().map(|x| x as _)
    }

    fn set_id(&mut self, val: impl Into<ObjId>) {
        self._id = val.into();
    }
    fn set_version(&mut self, val: impl Into<Option<u32>>) {
        self._version = val.into();
    }
    fn set_deleted(&mut self, val: bool) {
        self._deleted = val;
    }
    fn set_changeset_id(&mut self, val: impl Into<Option<u32>>) {
        self._changeset_id = val.into();
    }
    fn set_timestamp(&mut self, val: impl Into<Option<TimestampFormat>>) {
        self._timestamp = val.into();
    }
    fn set_uid(&mut self, val: impl Into<Option<u32>>) {
        self._uid = val.into();
    }
    fn set_user<'a>(&mut self, val: impl Into<Option<&'a str>>) {
        self._user = val.into().map(Arc::from);
    }

    fn tags<'a>(&'a self) -> Box<dyn ExactSizeIterator<Item = (&'a str, &'a str)> + 'a> {
        Box::new(self._tags.iter().map(|(k, v)| (k.as_ref(), v.as_ref())))
    }

    fn tag(&self, key: impl AsRef<str>) -> Option<&str> {
        let key = key.as_ref();
        self._tags
            .iter()
            .filter_map(|(k, v)| {
                if k.as_ref() == key {
                    Some(v.as_ref())
                } else {
                    None
                }
            })
            .next()
    }

    fn set_tag(&mut self, key: impl AsRef<str>, value: impl Into<String>) {
        let key = key.as_ref();
        let value = value.into();
        let idx = self
            ._tags
            .iter()
            .enumerate()
            .filter_map(|(i, (k, _))| if k.as_ref() == key { Some(i) } else { None })
            .next();
        match idx {
            None => self._tags.push((Arc::from(key), Arc::from(value.as_str()))),
            Some(i) => self._tags[i] = (key.into(), Arc::from(value.as_str())),
        }
    }

    fn unset_tag(&mut self, key: impl AsRef<str>) {
        let key = key.as_ref();
        let idx = self
            ._tags
            .iter()
            .enumerate()
            .filter_map(|(i, (k, _))| if k.as_ref() == key { Some(i) } else { None })
            .next();
        if let Some(i) = idx {
            self._tags.remove(i);
        }
    }

    fn object_type(&self) -> OSMObjectType {
        OSMObjectType::Way
    }
}

impl Way for ArcWay {
    fn nodes(&self) -> &[ObjId] {
        &self._nodes
    }

    fn num_nodes(&self) -> usize {
        self._nodes.len()
    }

    fn node(&self, idx: usize) -> Option<ObjId> {
        self._nodes.get(idx).cloned()
    }
    fn set_nodes(&mut self, nodes: impl IntoIterator<Item = impl Into<ObjId>>) {
        self._nodes.truncate(0);
        self._nodes.extend(nodes.into_iter().map(|i| i.into()));
    }
}

impl OSMObjBase for ArcRelation {
    fn id(&self) -> ObjId {
        self._id
    }
    fn version(&self) -> Option<u32> {
        self._version
    }
    fn deleted(&self) -> bool {
        self._deleted
    }
    fn changeset_id(&self) -> Option<u32> {
        self._changeset_id
    }
    fn timestamp(&self) -> &Option<TimestampFormat> {
        &self._timestamp
    }
    fn uid(&self) -> Option<u32> {
        self._uid
    }
    fn user(&self) -> Option<&str> {
        self._user.as_ref().map(|x| x as _)
    }

    fn set_id(&mut self, val: impl Into<ObjId>) {
        self._id = val.into();
    }
    fn set_version(&mut self, val: impl Into<Option<u32>>) {
        self._version = val.into();
    }
    fn set_deleted(&mut self, val: bool) {
        self._deleted = val;
    }
    fn set_changeset_id(&mut self, val: impl Into<Option<u32>>) {
        self._changeset_id = val.into();
    }
    fn set_timestamp(&mut self, val: impl Into<Option<TimestampFormat>>) {
        self._timestamp = val.into();
    }
    fn set_uid(&mut self, val: impl Into<Option<u32>>) {
        self._uid = val.into();
    }
    fn set_user<'a>(&mut self, val: impl Into<Option<&'a str>>) {
        self._user = val.into().map(Arc::from);
    }

    fn tags<'a>(&'a self) -> Box<dyn ExactSizeIterator<Item = (&'a str, &'a str)> + 'a> {
        Box::new(self._tags.iter().map(|(k, v)| (k.as_ref(), v.as_ref())))
    }

    fn tag(&self, key: impl AsRef<str>) -> Option<&str> {
        let key = key.as_ref();
        self._tags
            .iter()
            .filter_map(|(k, v)| {
                if k.as_ref() == key {
                    Some(v.as_ref())
                } else {
                    None
                }
            })
            .next()
    }

    fn set_tag(&mut self, key: impl AsRef<str>, value: impl Into<String>) {
        let key = key.as_ref();
        let value = value.into();
        let idx = self
            ._tags
            .iter()
            .enumerate()
            .filter_map(|(i, (k, _))| if k.as_ref() == key { Some(i) } else { None })
            .next();
        match idx {
            None => self._tags.push((Arc::from(key), Arc::from(value.as_str()))),
            Some(i) => self._tags[i] = (key.into(), Arc::from(value.as_str())),
        }
    }

    fn unset_tag(&mut self, key: impl AsRef<str>) {
        let key = key.as_ref();
        let idx = self
            ._tags
            .iter()
            .enumerate()
            .filter_map(|(i, (k, _))| if k.as_ref() == key { Some(i) } else { None })
            .next();
        if let Some(i) = idx {
            self._tags.remove(i);
        }
    }

    fn object_type(&self) -> OSMObjectType {
        OSMObjectType::Relation
    }

}

impl Relation for ArcRelation {
    fn members<'a>(
        &'a self,
    ) -> Box<dyn ExactSizeIterator<Item = (OSMObjectType, ObjId, &'a str)> + 'a> {
        Box::new(self._members.iter().map(|(t, o, r)| (*t, *o, r.as_ref())))
    }

    fn set_members(
        &mut self,
        members: impl IntoIterator<Item = (OSMObjectType, ObjId, impl Into<String>)>,
    ) {
        self._members.truncate(0);

        self._members.extend(
            members
                .into_iter()
                .map(|(t, i, r)| (t, i, Arc::from(r.into()))),
        );
    }
}
