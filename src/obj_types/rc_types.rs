use std::rc::Rc;
use ::*;

macro_rules! func_call_inner_get {
    ($slf:ident, $name:ident) => (
        match $slf {
            RcOSMObj::Node(x) => x.$name(),
            RcOSMObj::Way(x) => x.$name(),
            RcOSMObj::Relation(x) => x.$name(),
        }
    )
}

macro_rules! func_call_inner_set {
    ($slf:ident, $name:ident, $val:ident) => (
        match $slf {
            RcOSMObj::Node(x) => x.$name($val),
            RcOSMObj::Way(x) => x.$name($val),
            RcOSMObj::Relation(x) => x.$name($val),
        };
    )
}


#[derive(PartialEq, Debug)]
pub struct RcNode {
    pub(crate) _id: ObjId,
    pub(crate) _version: Option<u32>,
    pub(crate) _deleted: bool,
    pub(crate) _changeset_id: Option<u32>,
    pub(crate) _timestamp: Option<TimestampFormat>,
    pub(crate) _uid: Option<u32>,
    pub(crate) _user: Option<Rc<str>>,
    pub(crate) _tags: Vec<(Rc<str>, Rc<str>)>,

    pub(crate) _lat_lon: Option<(Lat, Lon)>,
}


#[derive(PartialEq, Debug)]
pub struct RcWay {
    pub(crate) _id: ObjId,
    pub(crate) _version: Option<u32>,
    pub(crate) _deleted: bool,
    pub(crate) _changeset_id: Option<u32>,
    pub(crate) _timestamp: Option<TimestampFormat>,
    pub(crate) _uid: Option<u32>,
    pub(crate) _user: Option<Rc<str>>,
    pub(crate) _tags: Vec<(Rc<str>, Rc<str>)>,

    pub(crate) _nodes: Vec<ObjId>,
}

#[derive(PartialEq, Debug)]
pub struct RcRelation {
    pub(crate) _id: ObjId,
    pub(crate) _version: Option<u32>,
    pub(crate) _deleted: bool,
    pub(crate) _changeset_id: Option<u32>,
    pub(crate) _timestamp: Option<TimestampFormat>,
    pub(crate) _uid: Option<u32>,
    pub(crate) _user: Option<Rc<str>>,
    pub(crate) _tags: Vec<(Rc<str>, Rc<str>)>,

    pub(crate) _members: Vec<(OSMObjectType, ObjId, Rc<str>)>,
}

#[derive(PartialEq, Debug)]
pub enum RcOSMObj {
    Node(RcNode),
    Way(RcWay),
    Relation(RcRelation),
}


impl OSMObjBase for RcOSMObj {

    fn id(&self) -> ObjId { func_call_inner_get!(self, id) } 
    fn version(&self) -> Option<u32> { func_call_inner_get!(self, version) }
    fn deleted(&self) -> bool { func_call_inner_get!(self, deleted) }
    fn changeset_id(&self) -> Option<u32> { func_call_inner_get!(self, changeset_id) }
    fn timestamp(&self) -> &Option<TimestampFormat> { func_call_inner_get!(self, timestamp) }
    fn uid(&self) -> Option<u32> { func_call_inner_get!(self, uid) }
    fn user(&self) -> Option<&str> { func_call_inner_get!(self, user) }

    fn set_id(&mut self, val: impl Into<ObjId>) { func_call_inner_set!(self, set_id, val); } 
    fn set_version(&mut self, val: impl Into<Option<u32>>) { func_call_inner_set!(self, set_version, val); }
    fn set_deleted(&mut self, val: bool) { func_call_inner_set!(self, set_deleted, val); }
    fn set_changeset_id(&mut self, val: impl Into<Option<u32>>) { func_call_inner_set!(self, set_changeset_id, val); }
    fn set_timestamp(&mut self, val: impl Into<Option<TimestampFormat>>) { func_call_inner_set!(self, set_timestamp, val); }
    fn set_uid(&mut self, val: impl Into<Option<u32>>) { func_call_inner_set!(self, set_uid, val); }
    fn set_user<'a>(&mut self, val: impl Into<Option<&'a str>>) { func_call_inner_set!(self, set_user, val); }

    fn tags<'a>(&'a self) -> Box<dyn ExactSizeIterator<Item=(&'a str, &'a str)>+'a>
    {
        match self {
            RcOSMObj::Node(x) => x.tags(),
            RcOSMObj::Way(x) => x.tags(),
            RcOSMObj::Relation(x) => x.tags(),
        }
    }

    fn num_tags(&self) -> usize {
        match self {
            RcOSMObj::Node(x) => x._tags.len(),
            RcOSMObj::Way(x) => x._tags.len(),
            RcOSMObj::Relation(x) => x._tags.len(),
        }
    }
    fn untagged(&self) -> bool {
        match self {
            RcOSMObj::Node(x) => x._tags.is_empty(),
            RcOSMObj::Way(x) => x._tags.is_empty(),
            RcOSMObj::Relation(x) => x._tags.is_empty(),
        }
    }

    fn tag(&self, key: impl AsRef<str>) -> Option<&str>
    {
        match self {
            RcOSMObj::Node(x) => x.tag(key),
            RcOSMObj::Way(x) => x.tag(key),
            RcOSMObj::Relation(x) => x.tag(key),
        }
    }

    fn set_tag(&mut self, key: impl AsRef<str>, value: impl Into<String>)
    {
        match self {
            RcOSMObj::Node(x) => x.set_tag(key, value),
            RcOSMObj::Way(x) => x.set_tag(key, value),
            RcOSMObj::Relation(x) => x.set_tag(key, value),
        }
    }

    fn unset_tag(&mut self, key: impl AsRef<str>)
    {
        match self {
            RcOSMObj::Node(x) => x.unset_tag(key),
            RcOSMObj::Way(x) => x.unset_tag(key),
            RcOSMObj::Relation(x) => x.unset_tag(key),
        }
    }
}

impl OSMObj for RcOSMObj {
    type Node = RcNode;
    type Way = RcWay;
    type Relation = RcRelation;

    fn object_type(&self) -> OSMObjectType {
        match self {
            RcOSMObj::Node(_) => OSMObjectType::Node,
            RcOSMObj::Way(_) => OSMObjectType::Way,
            RcOSMObj::Relation(_) => OSMObjectType::Relation,
        }
    }

    fn into_node(self) -> Option<RcNode> {
        if let RcOSMObj::Node(n) = self {
            Some(n)
        } else {
            None
        }
    }

    fn into_way(self) -> Option<RcWay> {
        if let RcOSMObj::Way(w) = self {
            Some(w)
        } else {
            None
        }
    }

    fn into_relation(self) -> Option<RcRelation> {
        if let RcOSMObj::Relation(r) = self {
            Some(r)
        } else {
            None
        }
    }

    fn as_node(&self) -> Option<&RcNode> {
        if let RcOSMObj::Node(n) = self {
            Some(&n)
        } else {
            None
        }
    }

    fn as_way(&self) -> Option<&RcWay> {
        if let RcOSMObj::Way(w) = self {
            Some(&w)
        } else {
            None
        }
    }

    fn as_relation(&self) -> Option<&RcRelation> {
        if let RcOSMObj::Relation(r) = self {
            Some(&r)
        } else {
            None
        }
    }

    fn as_node_mut(&mut self) -> Option<&mut RcNode> {
        if let RcOSMObj::Node(n) = self {
            Some(n)
        } else {
            None
        }
    }

    fn as_way_mut(&mut self) -> Option<&mut RcWay> {
        if let RcOSMObj::Way(w) = self {
            Some(w)
        } else {
            None
        }
    }

    fn as_relation_mut(&mut self) -> Option<&mut RcRelation> {
        if let RcOSMObj::Relation(r) = self {
            Some(r)
        } else {
            None
        }
    }



}

impl OSMObjBase for RcNode {
    fn id(&self) -> ObjId { self._id }
    fn version(&self) -> Option<u32> { self._version }
    fn deleted(&self) -> bool { self._deleted }
    fn changeset_id(&self) -> Option<u32> { self._changeset_id }
    fn timestamp(&self) -> &Option<TimestampFormat> { &self._timestamp }
    fn uid(&self) -> Option<u32> { self._uid }
    fn user(&self) -> Option<&str> {
        match self._user {
            None => None,
            Some(ref s) => {
                Some(&s)
            },
        }
    }

    fn set_id(&mut self, val: impl Into<ObjId>) { self._id = val.into(); }
    fn set_version(&mut self, val: impl Into<Option<u32>>) { self._version = val.into(); }
    fn set_deleted(&mut self, val: bool) { self._deleted = val.into(); }
    fn set_changeset_id(&mut self, val: impl Into<Option<u32>>) { self._changeset_id = val.into(); }
    fn set_timestamp(&mut self, val: impl Into<Option<TimestampFormat>>) { self._timestamp = val.into(); }
    fn set_uid(&mut self, val: impl Into<Option<u32>>) { self._uid = val.into(); }
    fn set_user<'a>(&mut self, val: impl Into<Option<&'a str>>) {
        self._user = val.into().map(|s| Rc::from(s));
    }

    fn tags<'a>(&'a self) -> Box<dyn ExactSizeIterator<Item=(&'a str, &'a str)>+'a>
    {
        Box::new(self._tags.iter().map(|(k, v)| (k.as_ref(), v.as_ref())))
    }

    fn tag(&self, key: impl AsRef<str>) -> Option<&str>
    {
        let key = key.as_ref();
        self._tags.iter().filter_map(|(k, v)| if &k.as_ref() == &key { Some(v.as_ref()) } else { None }).nth(0)
    }

    fn set_tag(&mut self, key: impl AsRef<str>, value: impl Into<String>) {
        let key = key.as_ref();
        let value = value.into();
        let idx = self._tags.iter().enumerate().filter_map(|(i, (k, _))| if &k.as_ref() == &key { Some(i) } else { None }).nth(0);
        match idx {
            None => self._tags.push((Rc::from(key), Rc::from(value.as_str()))),
            Some(i) => self._tags[i] = (key.into(), Rc::from(value.as_str())),
        }
    }

    fn unset_tag(&mut self, key: impl AsRef<str>) {
        let key = key.as_ref();
        let idx = self._tags.iter().enumerate().filter_map(|(i, (k, _))| if &k.as_ref() == &key { Some(i) } else { None }).nth(0);
        if let Some(i) = idx {
            self._tags.remove(i);
        }
    }

}

impl Node for RcNode {
    fn lat_lon(&self) -> Option<(Lat, Lon)> {
        self._lat_lon
    }

    fn set_lat_lon(&mut self, loc: impl Into<Option<(Lat, Lon)>>) {
        self._lat_lon = loc.into();
    }
}

impl OSMObjBase for RcWay {
    fn id(&self) -> ObjId { self._id }
    fn version(&self) -> Option<u32> { self._version }
    fn deleted(&self) -> bool { self._deleted }
    fn changeset_id(&self) -> Option<u32> { self._changeset_id }
    fn timestamp(&self) -> &Option<TimestampFormat> { &self._timestamp }
    fn uid(&self) -> Option<u32> { self._uid }
    fn user(&self) -> Option<&str> {
        match self._user {
            None => None,
            Some(ref s) => {
                Some(&s)
            },
        }
    }

    fn set_id(&mut self, val: impl Into<ObjId>) { self._id = val.into(); }
    fn set_version(&mut self, val: impl Into<Option<u32>>) { self._version = val.into(); }
    fn set_deleted(&mut self, val: bool) { self._deleted = val.into(); }
    fn set_changeset_id(&mut self, val: impl Into<Option<u32>>) { self._changeset_id = val.into(); }
    fn set_timestamp(&mut self, val: impl Into<Option<TimestampFormat>>) { self._timestamp = val.into(); }
    fn set_uid(&mut self, val: impl Into<Option<u32>>) { self._uid = val.into(); }
    fn set_user<'a>(&mut self, val: impl Into<Option<&'a str>>) { self._user = val.into().map(|s| Rc::from(s)); }

    fn tags<'a>(&'a self) -> Box<dyn ExactSizeIterator<Item=(&'a str, &'a str)>+'a>
    {
        Box::new(self._tags.iter().map(|(k, v)| (k.as_ref(), v.as_ref())))
    }

    fn tag(&self, key: impl AsRef<str>) -> Option<&str>
    {
        let key = key.as_ref();
        self._tags.iter().filter_map(|(k, v)| if &k.as_ref() == &key { Some(v.as_ref()) } else { None }).nth(0)
    }

    fn set_tag(&mut self, key: impl AsRef<str>, value: impl Into<String>) {
        let key = key.as_ref();
        let value = value.into();
        let idx = self._tags.iter().enumerate().filter_map(|(i, (k, _))| if &k.as_ref() == &key { Some(i) } else { None }).nth(0);
        match idx {
            None => self._tags.push((Rc::from(key), Rc::from(value.as_str()))),
            Some(i) => self._tags[i] = (key.into(), Rc::from(value.as_str())),
        }
    }

    fn unset_tag(&mut self, key: impl AsRef<str>) {
        let key = key.as_ref();
        let idx = self._tags.iter().enumerate().filter_map(|(i, (k, _))| if &k.as_ref() == &key { Some(i) } else { None }).nth(0);
        if let Some(i) = idx {
            self._tags.remove(i);
        }
    }
}

impl Way for RcWay {
    fn nodes(&self) -> &[ObjId] {
        &self._nodes
    }

    fn num_nodes(&self) -> usize {
        self._nodes.len()
    }

    fn node(&self, idx: usize) -> Option<ObjId> {
        self._nodes.get(idx).cloned()
    }
    fn set_nodes(&mut self, nodes: impl IntoIterator<Item=impl Into<ObjId>>) {
        self._nodes.truncate(0);
        self._nodes.extend(nodes.into_iter().map(|i| i.into()));
    }
}

impl OSMObjBase for RcRelation {
    fn id(&self) -> ObjId { self._id }
    fn version(&self) -> Option<u32> { self._version }
    fn deleted(&self) -> bool { self._deleted }
    fn changeset_id(&self) -> Option<u32> { self._changeset_id }
    fn timestamp(&self) -> &Option<TimestampFormat> { &self._timestamp }
    fn uid(&self) -> Option<u32> { self._uid }
    fn user(&self) -> Option<&str> {
        match self._user {
            None => None,
            Some(ref s) => {
                Some(&s)
            },
        }
    }

    fn set_id(&mut self, val: impl Into<ObjId>) { self._id = val.into(); }
    fn set_version(&mut self, val: impl Into<Option<u32>>) { self._version = val.into(); }
    fn set_deleted(&mut self, val: bool) { self._deleted = val.into(); }
    fn set_changeset_id(&mut self, val: impl Into<Option<u32>>) { self._changeset_id = val.into(); }
    fn set_timestamp(&mut self, val: impl Into<Option<TimestampFormat>>) { self._timestamp = val.into(); }
    fn set_uid(&mut self, val: impl Into<Option<u32>>) { self._uid = val.into(); }
    fn set_user<'a>(&mut self, val: impl Into<Option<&'a str>>) { self._user = val.into().map(|s| Rc::from(s)); }


    fn tags<'a>(&'a self) -> Box<dyn ExactSizeIterator<Item=(&'a str, &'a str)>+'a>
    {
        Box::new(self._tags.iter().map(|(k, v)| (k.as_ref(), v.as_ref())))
    }

    fn tag(&self, key: impl AsRef<str>) -> Option<&str>
    {
        let key = key.as_ref();
        self._tags.iter().filter_map(|(k, v)| if &k.as_ref() == &key { Some(v.as_ref()) } else { None }).nth(0)
    }

    fn set_tag(&mut self, key: impl AsRef<str>, value: impl Into<String>) {
        let key = key.as_ref();
        let value = value.into();
        let idx = self._tags.iter().enumerate().filter_map(|(i, (k, _))| if &k.as_ref() == &key { Some(i) } else { None }).nth(0);
        match idx {
            None => self._tags.push((Rc::from(key), Rc::from(value.as_str()))),
            Some(i) => self._tags[i] = (key.into(), Rc::from(value.as_str())),
        }
    }

    fn unset_tag(&mut self, key: impl AsRef<str>) {
        let key = key.as_ref();
        let idx = self._tags.iter().enumerate().filter_map(|(i, (k, _))| if &k.as_ref() == &key { Some(i) } else { None }).nth(0);
        if let Some(i) = idx {
            self._tags.remove(i);
        }
    }
}

impl Relation for RcRelation {
    fn members<'a>(&'a self) -> Box<dyn ExactSizeIterator<Item=(OSMObjectType, ObjId, &'a str)>+'a> {
        Box::new(self._members.iter().map(|(t, o, r)| (
                t.clone(),
                o.clone(),
                r.as_ref(),
                )
        ))
    }

    fn set_members(&mut self, members: impl IntoIterator<Item=(OSMObjectType, ObjId, impl Into<String>)>) {
        self._members.truncate(0);

        self._members.extend(members.into_iter().map(|(t, i, r)| (t, i, Rc::from(r.into()))));
    }

}
