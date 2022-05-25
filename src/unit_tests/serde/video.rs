use crate::types::resource::Stream;
use crate::types::resource::{
    MetaItem, MetaItemPreview, SeriesInfo, StreamBehaviorHints, StreamSource, Video,
};
use crate::unit_tests::serde::default_tokens_ext::{DefaultFlattenTokens, DefaultTokens};
use chrono::prelude::TimeZone;
use chrono::Utc;
use serde_test::{assert_de_tokens, assert_tokens, Token};

#[test]
fn video() {
    assert_tokens(
        &vec![
            Video {
                id: "id".to_owned(),
                title: "title".to_owned(),
                released: Some(Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 0, 0)),
                overview: Some("overview".to_owned()),
                thumbnail: Some("thumbnail".to_owned()),
                streams: vec![],
                series_info: Some(SeriesInfo::default()),
                trailer_streams: vec![],
            },
            Video {
                id: "id".to_owned(),
                title: "title".to_owned(),
                released: None,
                overview: None,
                thumbnail: None,
                streams: vec![],
                series_info: None,
                trailer_streams: vec![],
            },
        ],
        &[
            vec![
                Token::Seq { len: Some(2) },
                Token::Map { len: None },
                Token::Str("id"),
                Token::Str("id"),
                Token::Str("title"),
                Token::Str("title"),
                Token::Str("released"),
                Token::Some,
                Token::Str("2020-01-01T00:00:00Z"),
                Token::Str("overview"),
                Token::Some,
                Token::Str("overview"),
                Token::Str("thumbnail"),
                Token::Some,
                Token::Str("thumbnail"),
                Token::Str("streams"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
            ],
            SeriesInfo::default_flatten_tokens(),
            vec![
                Token::Str("trailerStreams"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::MapEnd,
                Token::Map { len: None },
                Token::Str("id"),
                Token::Str("id"),
                Token::Str("title"),
                Token::Str("title"),
                Token::Str("released"),
                Token::None,
                Token::Str("overview"),
                Token::None,
                Token::Str("thumbnail"),
                Token::None,
                Token::Str("streams"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("trailerStreams"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::MapEnd,
                Token::SeqEnd,
            ],
        ]
        .concat(),
    );
    assert_de_tokens(
        &vec![
            Video {
                id: "id".to_owned(),
                title: "".to_owned(),
                released: None,
                overview: None,
                thumbnail: None,
                streams: vec![],
                series_info: None,
                trailer_streams: vec![],
            },
            Video {
                id: "id".to_owned(),
                title: "title".to_owned(),
                released: None,
                overview: None,
                thumbnail: None,
                streams: vec![Stream {
                    source: StreamSource::default(),
                    name: None,
                    description: None,
                    thumbnail: None,
                    subtitles: vec![],
                    behavior_hints: StreamBehaviorHints::default(),
                }],
                series_info: None,
                trailer_streams: vec![],
            },
            Video {
                id: "id".to_owned(),
                title: "title".to_owned(),
                released: None,
                overview: None,
                thumbnail: None,
                streams: vec![Stream {
                    source: StreamSource::default(),
                    name: None,
                    description: None,
                    thumbnail: None,
                    subtitles: vec![],
                    behavior_hints: StreamBehaviorHints::default(),
                }],
                series_info: None,
                trailer_streams: vec![],
            },
        ],
        &[
            vec![
                Token::Seq { len: Some(3) },
                Token::Map { len: None },
                Token::Str("id"),
                Token::Str("id"),
                Token::MapEnd,
                Token::Map { len: None },
                Token::Str("id"),
                Token::Str("id"),
                Token::Str("title"),
                Token::Str("title"),
                Token::Str("released"),
                Token::None,
                Token::Str("overview"),
                Token::None,
                Token::Str("thumbnail"),
                Token::None,
                Token::Str("stream"),
                Token::Map { len: None },
            ],
            StreamSource::default_flatten_tokens(),
            vec![
                Token::Str("name"),
                Token::None,
                Token::Str("description"),
                Token::None,
                Token::Str("thumbnail"),
                Token::None,
                Token::Str("subtitles"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("behaviorHints"),
            ],
            StreamBehaviorHints::default_tokens(),
            vec![
                Token::MapEnd,
                Token::Str("trailerStreams"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::MapEnd,
                Token::Map { len: None },
                Token::Str("id"),
                Token::Str("id"),
                Token::Str("title"),
                Token::Str("title"),
                Token::Str("released"),
                Token::None,
                Token::Str("overview"),
                Token::None,
                Token::Str("thumbnail"),
                Token::None,
                Token::Str("streams"),
                Token::Seq { len: Some(1) },
                Token::Map { len: None },
            ],
            StreamSource::default_flatten_tokens(),
            vec![
                Token::Str("name"),
                Token::None,
                Token::Str("description"),
                Token::None,
                Token::Str("thumbnail"),
                Token::None,
                Token::Str("subtitles"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("behaviorHints"),
            ],
            StreamBehaviorHints::default_tokens(),
            vec![
                Token::MapEnd,
                Token::SeqEnd,
                Token::Str("trailerStreams"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::MapEnd,
                Token::SeqEnd,
            ],
        ]
        .concat(),
    );
}

#[test]
fn videos_minimal() {
    assert_de_tokens(
        &MetaItem {
            preview: MetaItemPreview {
                id: "id".to_owned(),
                r#type: "type".to_owned(),
                name: "".to_owned(),
                ..Default::default()
            },
            // Nothing to sort against. The ordering is from the addon
            videos: vec![
                Video {
                    id: "2".to_owned(),
                    title: "".to_owned(),
                    released: None,
                    overview: None,
                    thumbnail: None,
                    streams: vec![],
                    series_info: None,
                    trailer_streams: vec![],
                },
                Video {
                    id: "1".to_owned(),
                    title: "".to_owned(),
                    released: None,
                    overview: None,
                    thumbnail: None,
                    streams: vec![],
                    series_info: None,
                    trailer_streams: vec![],
                },
                Video {
                    id: "3".to_owned(),
                    title: "".to_owned(),
                    released: None,
                    overview: None,
                    thumbnail: None,
                    streams: vec![],
                    series_info: None,
                    trailer_streams: vec![],
                },
            ],
            ..Default::default()
        },
        &[
            Token::Struct {
                name: "MetaItem",
                len: 2,
            },
            Token::Str("id"),
            Token::Str("id"),
            Token::Str("type"),
            Token::Str("type"),
            Token::Str("videos"),
            Token::Seq { len: None },
            Token::Map { len: None },
            Token::Str("id"),
            Token::Str("2"),
            Token::MapEnd,
            Token::Map { len: None },
            Token::Str("id"),
            Token::Str("1"),
            Token::MapEnd,
            Token::Map { len: None },
            Token::Str("id"),
            Token::Str("3"),
            Token::MapEnd,
            Token::SeqEnd,
            Token::StructEnd,
        ],
    );
}

#[test]
fn videos_released_equal() {
    assert_de_tokens(
        &MetaItem {
            preview: MetaItemPreview {
                id: "id".to_owned(),
                r#type: "type".to_owned(),
                name: "".to_owned(),
                ..Default::default()
            },
            // All have same date. The ordering is from the addon
            videos: vec![
                Video {
                    id: "2".to_owned(),
                    title: "".to_owned(),
                    released: Some(Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 0, 0)),
                    overview: None,
                    thumbnail: None,
                    streams: vec![],
                    series_info: None,
                    trailer_streams: vec![],
                },
                Video {
                    id: "1".to_owned(),
                    title: "".to_owned(),
                    released: Some(Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 0, 0)),
                    overview: None,
                    thumbnail: None,
                    streams: vec![],
                    series_info: None,
                    trailer_streams: vec![],
                },
                Video {
                    id: "3".to_owned(),
                    title: "".to_owned(),
                    released: Some(Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 0, 0)),
                    overview: None,
                    thumbnail: None,
                    streams: vec![],
                    series_info: None,
                    trailer_streams: vec![],
                },
            ],
            ..Default::default()
        },
        &[
            Token::Struct {
                name: "MetaItem",
                len: 2,
            },
            Token::Str("id"),
            Token::Str("id"),
            Token::Str("type"),
            Token::Str("type"),
            Token::Str("videos"),
            Token::Seq { len: None },
            Token::Map { len: None },
            Token::Str("id"),
            Token::Str("2"),
            Token::Str("released"),
            Token::Some,
            Token::Str("2020-01-01T00:00:00Z"),
            Token::MapEnd,
            Token::Map { len: None },
            Token::Str("id"),
            Token::Str("1"),
            Token::Str("released"),
            Token::Some,
            Token::Str("2020-01-01T00:00:00Z"),
            Token::MapEnd,
            Token::Map { len: None },
            Token::Str("id"),
            Token::Str("3"),
            Token::Str("released"),
            Token::Some,
            Token::Str("2020-01-01T00:00:00Z"),
            Token::MapEnd,
            Token::SeqEnd,
            Token::StructEnd,
        ],
    );
}

#[test]
fn videos_released_sequal() {
    assert_de_tokens(
        &MetaItem {
            preview: MetaItemPreview {
                id: "id".to_owned(),
                r#type: "type".to_owned(),
                name: "".to_owned(),
                ..Default::default()
            },
            // There is no series_info. Order by date descending.
            // If no date - at the end and the order is defined by addon
            videos: vec![
                Video {
                    id: "3".to_owned(),
                    title: "".to_owned(),
                    released: Some(Utc.ymd(2020, 3, 1).and_hms_milli(0, 0, 0, 0)),
                    overview: None,
                    thumbnail: None,
                    streams: vec![],
                    series_info: None,
                    trailer_streams: vec![],
                },
                Video {
                    id: "2".to_owned(),
                    title: "".to_owned(),
                    released: Some(Utc.ymd(2020, 2, 1).and_hms_milli(0, 0, 0, 0)),
                    overview: None,
                    thumbnail: None,
                    streams: vec![],
                    series_info: None,
                    trailer_streams: vec![],
                },
                Video {
                    id: "1".to_owned(),
                    title: "".to_owned(),
                    released: Some(Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 0, 0)),
                    overview: None,
                    thumbnail: None,
                    streams: vec![],
                    series_info: None,
                    trailer_streams: vec![],
                },
                Video {
                    id: "nd1".to_owned(),
                    title: "".to_owned(),
                    released: None,
                    overview: None,
                    thumbnail: None,
                    streams: vec![],
                    series_info: None,
                    trailer_streams: vec![],
                },
                Video {
                    id: "nd2".to_owned(),
                    title: "".to_owned(),
                    released: None,
                    overview: None,
                    thumbnail: None,
                    streams: vec![],
                    series_info: None,
                    trailer_streams: vec![],
                },
            ],
            ..Default::default()
        },
        &[
            Token::Struct {
                name: "MetaItem",
                len: 2,
            },
            Token::Str("id"),
            Token::Str("id"),
            Token::Str("type"),
            Token::Str("type"),
            Token::Str("videos"),
            Token::Seq { len: None },
            Token::Map { len: None },
            Token::Str("id"),
            Token::Str("nd1"),
            Token::MapEnd,
            Token::Map { len: None },
            Token::Str("id"),
            Token::Str("nd2"),
            Token::MapEnd,
            Token::Map { len: None },
            Token::Str("id"),
            Token::Str("2"),
            Token::Str("released"),
            Token::Some,
            Token::Str("2020-02-01T00:00:00Z"),
            Token::MapEnd,
            Token::Map { len: None },
            Token::Str("id"),
            Token::Str("1"),
            Token::Str("released"),
            Token::Some,
            Token::Str("2020-01-01T00:00:00Z"),
            Token::MapEnd,
            Token::Map { len: None },
            Token::Str("id"),
            Token::Str("3"),
            Token::Str("released"),
            Token::Some,
            Token::Str("2020-03-01T00:00:00Z"),
            Token::MapEnd,
            Token::SeqEnd,
            Token::StructEnd,
        ],
    );
}

#[test]
fn various_videos_deserialization() {
    assert_de_tokens(
        &MetaItem {
            preview: MetaItemPreview {
                id: "id".to_owned(),
                r#type: "type".to_owned(),
                name: "".to_owned(),
                ..Default::default()
            },
            // Sort by season, then episode. Special at the end.
            // If no series_info sort by date ascending
            // If no date - sort to the end. Preserve order from addon
            videos: vec![
                Video {
                    id: "S01E01".to_owned(),
                    title: "".to_owned(),
                    released: Some(Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 0, 0)),
                    overview: None,
                    thumbnail: None,
                    streams: vec![],
                    series_info: Some(SeriesInfo {
                        season: 1,
                        episode: 1,
                    }),
                    trailer_streams: vec![],
                },
                Video {
                    id: "S01E02".to_owned(),
                    title: "".to_owned(),
                    released: Some(Utc.ymd(2020, 2, 1).and_hms_milli(0, 0, 0, 0)),
                    overview: None,
                    thumbnail: None,
                    streams: vec![],
                    series_info: Some(SeriesInfo {
                        season: 1,
                        episode: 2,
                    }),
                    trailer_streams: vec![],
                },
                Video {
                    id: "S02E01".to_owned(),
                    title: "".to_owned(),
                    released: Some(Utc.ymd(2020, 3, 1).and_hms_milli(0, 0, 0, 0)),
                    overview: None,
                    thumbnail: None,
                    streams: vec![],
                    series_info: Some(SeriesInfo {
                        season: 2,
                        episode: 1,
                    }),
                    trailer_streams: vec![],
                },
                Video {
                    id: "special1".to_owned(),
                    title: "".to_owned(),
                    released: Some(Utc.ymd(2020, 5, 1).and_hms_milli(0, 0, 0, 0)),
                    overview: None,
                    thumbnail: None,
                    streams: vec![],
                    series_info: Some(SeriesInfo {
                        season: 0,
                        episode: 1,
                    }),
                    trailer_streams: vec![],
                },
                Video {
                    id: "special2".to_owned(),
                    title: "".to_owned(),
                    released: Some(Utc.ymd(2020, 5, 1).and_hms_milli(0, 0, 0, 0)),
                    overview: None,
                    thumbnail: None,
                    streams: vec![],
                    series_info: Some(SeriesInfo {
                        season: 0,
                        episode: 2,
                    }),
                    trailer_streams: vec![],
                },
                Video {
                    id: "M1".to_owned(),
                    title: "".to_owned(),
                    released: Some(Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 0, 0)),
                    overview: None,
                    thumbnail: None,
                    streams: vec![],
                    series_info: None,
                    trailer_streams: vec![],
                },
                Video {
                    id: "M2".to_owned(),
                    title: "".to_owned(),
                    released: Some(Utc.ymd(2020, 2, 1).and_hms_milli(0, 0, 0, 0)),
                    overview: None,
                    thumbnail: None,
                    streams: vec![],
                    series_info: None,
                    trailer_streams: vec![],
                },
                Video {
                    id: "nd1".to_owned(),
                    title: "".to_owned(),
                    released: None,
                    overview: None,
                    thumbnail: None,
                    streams: vec![],
                    series_info: None,
                    trailer_streams: vec![],
                },
                Video {
                    id: "nd2".to_owned(),
                    title: "".to_owned(),
                    released: None,
                    overview: None,
                    thumbnail: None,
                    streams: vec![],
                    series_info: None,
                    trailer_streams: vec![],
                },
            ],
            ..Default::default()
        },
        &[
            Token::Struct {
                name: "MetaItem",
                len: 2,
            },
            Token::Str("id"),
            Token::Str("id"),
            Token::Str("type"),
            Token::Str("type"),
            Token::Str("videos"),
            Token::Seq { len: None },
            Token::Map { len: None },
            Token::Str("id"),
            Token::Str("special2"),
            Token::Str("released"),
            Token::Some,
            Token::Str("2020-05-01T00:00:00Z"),
            Token::Str("season"),
            Token::I32(0),
            Token::Str("episode"),
            Token::I32(2),
            Token::MapEnd,
            Token::Map { len: None },
            Token::Str("id"),
            Token::Str("S01E02"),
            Token::Str("released"),
            Token::Some,
            Token::Str("2020-02-01T00:00:00Z"),
            Token::Str("season"),
            Token::I32(1),
            Token::Str("episode"),
            Token::I32(2),
            Token::MapEnd,
            Token::Map { len: None },
            Token::Str("id"),
            Token::Str("special1"),
            Token::Str("released"),
            Token::Some,
            Token::Str("2020-05-01T00:00:00Z"),
            Token::Str("season"),
            Token::I32(0),
            Token::Str("episode"),
            Token::I32(1),
            Token::MapEnd,
            Token::Map { len: None },
            Token::Str("id"),
            Token::Str("S01E01"),
            Token::Str("released"),
            Token::Some,
            Token::Str("2020-01-01T00:00:00Z"),
            Token::Str("season"),
            Token::I32(1),
            Token::Str("episode"),
            Token::I32(1),
            Token::MapEnd,
            Token::Map { len: None },
            Token::Str("id"),
            Token::Str("M2"),
            Token::Str("released"),
            Token::Some,
            Token::Str("2020-02-01T00:00:00Z"),
            Token::MapEnd,
            Token::Map { len: None },
            Token::Str("id"),
            Token::Str("M1"),
            Token::Str("released"),
            Token::Some,
            Token::Str("2020-01-01T00:00:00Z"),
            Token::MapEnd,
            Token::Map { len: None },
            Token::Str("id"),
            Token::Str("nd1"),
            Token::MapEnd,
            Token::Map { len: None },
            Token::Str("id"),
            Token::Str("nd2"),
            Token::MapEnd,
            Token::Map { len: None },
            Token::Str("id"),
            Token::Str("S02E01"),
            Token::Str("released"),
            Token::Some,
            Token::Str("2020-03-01T00:00:00Z"),
            Token::Str("season"),
            Token::I32(2),
            Token::Str("episode"),
            Token::I32(1),
            Token::MapEnd,
            Token::SeqEnd,
            Token::StructEnd,
        ],
    );
}
