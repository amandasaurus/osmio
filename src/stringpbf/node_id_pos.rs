use osmio::{ObjId, Lat, Lon};

/// Reads a PBF file and returns just (nodeid, pos)
pub struct PBFNodePositionReader<R: Read> {
    filereader: FileReader<R>,
    _buffer: Vec<(ObjId, (Lat, Lon))>,
}

impl Iterator for PBFNodePositionReader {
    type Item = (ObjId, (Lat, Lon));

    fn next(&mut self) -> Option<StringOSMObj> {
        while self._buffer.is_empty() {
            // get the next file block and fill up our buffer
            // FIXME make this parallel

            // get the next block
            let mut blob = self.filereader.next()?;

            let blob_data = blob_raw_data(&mut blob).unwrap();
            let block: osmformat::PrimitiveBlock = protobuf::parse_from_bytes(&blob_data).unwrap();

            // Turn a block into OSM objects
            let mut objs = decode_block_to_objs(block);

            // we reverse the Vec so that we can .pop from the buffer, rather than .remove(0)
            // IME pop'ing is faster, since it means less memory moving
            objs.reverse();

            self._buffer = objs;
        }

        self._buffer.pop()
    }
}

impl PBFReader<BufReader<File>> {
    /// Creates a PBF Reader from a path.
    pub fn from_filename(filename: impl AsRef<Path>) -> Result<Self> {
        let filename: &Path = filename.as_ref();
        Ok(Self::new(BufReader::new(File::open(filename)?)))
    }
}

impl<R: Read> Iterator for PBFNodePositionReader<R> {
}

impl<R: Read> OSMReader for PBFReader<R> {
    type R = R;
    type Obj = StringOSMObj;

    fn new(reader: R) -> PBFReader<R> {
        PBFReader {
            filereader: FileReader::new(reader),
            _buffer: Vec::new(),
            _sorted_assumption: false,
        }
    }

    fn set_sorted_assumption(&mut self, sorted_assumption: bool) {
        self._sorted_assumption = sorted_assumption;
    }
    fn get_sorted_assumption(&mut self) -> bool {
        self._sorted_assumption
    }

    fn inner(&self) -> &R {
        self.filereader.inner()
    }

    fn into_inner(self) -> R {
        self.filereader.into_inner()
    }

    fn next(&mut self) -> Option<StringOSMObj> {
        while self._buffer.is_empty() {
            // get the next file block and fill up our buffer
            // FIXME make this parallel

            // get the next block
            let mut blob = self.filereader.next()?;

            let blob_data = blob_raw_data(&mut blob).unwrap();
            let block: osmformat::PrimitiveBlock = protobuf::parse_from_bytes(&blob_data).unwrap();

            // Turn a block into OSM objects
            let mut objs = decode_block_to_objs(block);

            // we reverse the Vec so that we can .pop from the buffer, rather than .remove(0)
            // IME pop'ing is faster, since it means less memory moving
            objs.reverse();

            self._buffer = objs;
        }

        self._buffer.pop()
    }
}
