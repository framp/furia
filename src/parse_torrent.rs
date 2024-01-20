use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

#[derive(Debug, Deserialize, Serialize)]
struct Node(String, i64);

#[derive(Debug, Deserialize, Serialize)]
struct File {
    path: Vec<String>,
    length: i64,
    #[serde(default)]
    md5sum: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub name: String,
    pub pieces: ByteBuf,
    #[serde(rename = "piece length")]
    pub piece_length: i64,
    #[serde(default)]
    pub md5sum: Option<String>,
    #[serde(default)]
    pub length: Option<i64>,
    #[serde(default)]
    pub files: Option<Vec<File>>,
    #[serde(default)]
    pub private: Option<u8>,
    #[serde(default)]
    pub path: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "root hash")]
    pub root_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TorrentFile {
    pub info: Info,
    #[serde(default)]
    pub announce: String,
    #[serde(default)]
    nodes: Option<Vec<Node>>,
    #[serde(default)]
    encoding: Option<String>,
    #[serde(default)]
    httpseeds: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "announce-list")]
    announce_list: Option<Vec<Vec<String>>>,
    #[serde(default)]
    #[serde(rename = "creation date")]
    creation_date: Option<i64>,
    #[serde(rename = "comment")]
    comment: Option<String>,
    #[serde(default)]
    #[serde(rename = "created by")]
    created_by: Option<String>,
}

pub fn parse_torrent(file_path: &str) -> TorrentFile {
    let torrent_file = std::fs::read(file_path).expect("Unable to read file");
    serde_bencode::from_bytes(&torrent_file).expect("Unable to parse torrent file")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_parses_a_torrent_file() {
        let torrent = parse_torrent("./data/centos-6.5.torrent");
        assert_eq!(
            "http://linuxtracker.org:2710/00000000000000000000000000000000/announce",
            torrent.announce
        );
        assert_eq!(Some(1385853586), torrent.creation_date);
        assert_eq!("CentOS-6.5-x86_64-minimal", torrent.info.name);
        assert_eq!(524288, torrent.info.piece_length);
    }
}
