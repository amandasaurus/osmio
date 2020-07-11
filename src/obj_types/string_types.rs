use ::*;

macro_rules! func_call_inner_get {
    ($slf:ident, $name:ident) => (
        match $slf {
            StringOSMObj::Node(x) => x.$name(),
            StringOSMObj::Way(x) => x.$name(),
            StringOSMObj::Relation(x) => x.$name(),
        }
    )
}

macro_rules! func_call_inner_set {
    ($slf:ident, $name:ident, $val:ident) => (
        match $slf {
            StringOSMObj::Node(x) => x.$name($val),
            StringOSMObj::Way(x) => x.$name($val),
            StringOSMObj::Relation(x) => x.$name($val),
        };
    )
}

#[derive(PartialEq, Debug, Builder)]
pub struct StringNode {
    pub(crate) _id: ObjId,
    pub(crate) _version: Option<u32>,
    pub(crate) _deleted: bool,
    pub(crate) _changeset_id: Option<u32>,
    pub(crate) _timestamp: Option<TimestampFormat>,
    pub(crate) _uid: Option<u32>,
    pub(crate) _user: Option<String>,
    pub(crate) _tags: HashMap<String, String>,

    pub(crate) _lat_lon: Option<(Lat, Lon)>,
}


#[derive(PartialEq, Debug)]
pub struct StringWay {
    pub(crate) _id: ObjId,
    pub(crate) _version: Option<u32>,
    pub(crate) _deleted: bool,
    pub(crate) _changeset_id: Option<u32>,
    pub(crate) _timestamp: Option<TimestampFormat>,
    pub(crate) _uid: Option<u32>,
    pub(crate) _user: Option<String>,
    pub(crate) _tags: HashMap<String, String>,

    pub(crate) _nodes: Vec<ObjId>,
}

#[derive(PartialEq, Debug)]
pub struct StringRelation {
    pub(crate) _id: ObjId,
    pub(crate) _version: Option<u32>,
    pub(crate) _deleted: bool,
    pub(crate) _changeset_id: Option<u32>,
    pub(crate) _timestamp: Option<TimestampFormat>,
    pub(crate) _uid: Option<u32>,
    pub(crate) _user: Option<String>,
    pub(crate) _tags: HashMap<String, String>,

    pub(crate) _members: Vec<(char, ObjId, String)>,
}

#[derive(PartialEq, Debug)]
pub enum StringOSMObj {
    Node(StringNode),
    Way(StringWay),
    Relation(StringRelation),
}


impl OSMObjBase for StringOSMObj {

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
    fn set_user(&mut self, val: impl Into<Option<String>>) { func_call_inner_set!(self, set_user, val); }


    fn tags<'a>(&'a self) -> Box<dyn ExactSizeIterator<Item=(&'a str, &'a str)>+'a>
    {
        func_call_inner_get!(self, tags)
    }

    fn tag(&self, key: impl AsRef<str>) -> Option<&str>
    {
        match self {
            StringOSMObj::Node(x) => x.tag(key),
            StringOSMObj::Way(x) => x.tag(key),
            StringOSMObj::Relation(x) => x.tag(key),
        }
    }

    fn set_tag(&mut self, key: impl AsRef<str>, value: impl Into<String>)
    {
        match self {
            StringOSMObj::Node(x) => x.set_tag(key, value),
            StringOSMObj::Way(x) => x.set_tag(key, value),
            StringOSMObj::Relation(x) => x.set_tag(key, value),
        }
    }

    fn unset_tag(&mut self, key: impl AsRef<str>)
    {
        match self {
            StringOSMObj::Node(x) => x.unset_tag(key),
            StringOSMObj::Way(x) => x.unset_tag(key),
            StringOSMObj::Relation(x) => x.unset_tag(key),
        }
    }
}

impl OSMObj for StringOSMObj {
    type Node = StringNode;
    type Way = StringWay;
    type Relation = StringRelation;

    fn object_type(&self) -> OSMObjectType {
        match self {
            StringOSMObj::Node(_) => OSMObjectType::Node,
            StringOSMObj::Way(_) => OSMObjectType::Way,
            StringOSMObj::Relation(_) => OSMObjectType::Relation,
        }
    }

    fn into_node(self) -> Option<StringNode> {
        if let StringOSMObj::Node(n) = self {
            Some(n)
        } else {
            None
        }
    }

    fn into_way(self) -> Option<StringWay> {
        if let StringOSMObj::Way(w) = self {
            Some(w)
        } else {
            None
        }
    }

    fn into_relation(self) -> Option<StringRelation> {
        if let StringOSMObj::Relation(r) = self {
            Some(r)
        } else {
            None
        }
    }

    fn as_node(&self) -> Option<&StringNode> {
        if let StringOSMObj::Node(n) = self {
            Some(&n)
        } else {
            None
        }
    }

    fn as_way(&self) -> Option<&StringWay> {
        if let StringOSMObj::Way(w) = self {
            Some(&w)
        } else {
            None
        }
    }

    fn as_relation(&self) -> Option<&StringRelation> {
        if let StringOSMObj::Relation(r) = self {
            Some(&r)
        } else {
            None
        }
    }


}

impl OSMObjBase for StringNode {
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
    fn set_user(&mut self, val: impl Into<Option<String>>) { self._user = val.into(); }


    fn tags<'a>(&'a self) -> Box<dyn ExactSizeIterator<Item=(&'a str, &'a str)>+'a>
    {
        Box::new(self._tags.iter().map(|(k, v)| (k.as_ref(), v.as_ref())))
    }

    fn tag(&self, key: impl AsRef<str>) -> Option<&str>
    {
        self._tags.get(key.as_ref()).map(|s| s.as_ref())
    }

    fn set_tag(&mut self, key: impl AsRef<str>, value: impl Into<String>) {
        self._tags.insert(key.as_ref().into(), value.into() );
    }

    fn unset_tag(&mut self, key: impl AsRef<str>) {
        self._tags.remove(key.as_ref());
    }
}

impl Node for StringNode {
    fn lat_lon(&self) -> Option<(Lat, Lon)> {
        self._lat_lon
    }

    fn set_lat_lon(&mut self, loc: impl Into<Option<(Lat, Lon)>>) {
        self._lat_lon = loc.into();
    }
}

impl OSMObjBase for StringWay {
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
    fn set_user(&mut self, val: impl Into<Option<String>>) { self._user = val.into(); }


    fn tags<'a>(&'a self) -> Box<dyn ExactSizeIterator<Item=(&'a str, &'a str)>+'a>
    {
        Box::new(self._tags.iter().map(|(k, v)| (k.as_ref(), v.as_ref())))
    }

    fn tag(&self, key: impl AsRef<str>) -> Option<&str>
    {
        self._tags.get(key.as_ref()).map(|s| s.as_ref())
    }

    fn set_tag(&mut self, key: impl AsRef<str>, value: impl Into<String>) {
        self._tags.insert(key.as_ref().into(), value.into() );
    }

    fn unset_tag(&mut self, key: impl AsRef<str>) {
        self._tags.remove(key.as_ref());
    }
}

impl Way for StringWay {
    fn num_nodes(&self) -> usize {
        self._nodes.len()
    }
    fn nodes(&self) -> &[ObjId] {
        self._nodes.as_ref()
    }
    fn node(&self, idx: usize) -> Option<ObjId> {
        self._nodes.get(idx).cloned()
    }
}

impl OSMObjBase for StringRelation {
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
    fn set_user(&mut self, val: impl Into<Option<String>>) { self._user = val.into(); }


    fn tags<'a>(&'a self) -> Box<dyn ExactSizeIterator<Item=(&'a str, &'a str)>+'a>
    {
        Box::new(self._tags.iter().map(|(k, v)| (k.as_ref(), v.as_ref())))
    }

    fn tag(&self, key: impl AsRef<str>) -> Option<&str>
    {
        self._tags.get(key.as_ref()).map(|s| s.as_ref())
    }

    fn set_tag(&mut self, key: impl AsRef<str>, value: impl Into<String>) {
        self._tags.insert(key.as_ref().into(), value.into() );
    }

    fn unset_tag(&mut self, key: impl AsRef<str>) {
        self._tags.remove(key.as_ref());
    }
}


impl Relation for StringRelation {
    fn members<'a>(&'a self) -> Box<dyn ExactSizeIterator<Item=(OSMObjectType, ObjId, &'a str)>+'a> {
        Box::new(self._members.iter().map(|(c, o, r)| (
                match c { 'n'=> OSMObjectType::Node, 'w'=>OSMObjectType::Way, 'r'=>OSMObjectType::Relation, _ => unreachable!() },
                o.clone(),
                r.as_ref(),
                )
        ))
    }
}
