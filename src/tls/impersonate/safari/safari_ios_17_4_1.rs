use super::CIPHER_LIST;
use crate::tls::builder::{SafariTlsBuilder, TlsBuilder};
use crate::tls::{Http2FrameSettings, TlsSettings};
use crate::tls::{ImpersonateSettings, TlsResult};
use http::{
    header::{ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, USER_AGENT},
    HeaderMap, HeaderValue,
};

pub(crate) fn get_settings(
    settings: ImpersonateSettings,
) -> TlsResult<(TlsSettings, impl FnOnce(&mut HeaderMap))> {
    Ok((
        TlsSettings {
            builder: SafariTlsBuilder::new(&CIPHER_LIST)?,
            extension: settings.extension,
            http2: Http2FrameSettings {
                initial_stream_window_size: Some(2097152),
                initial_connection_window_size: Some(10551295),
                max_concurrent_streams: Some(100),
                max_header_list_size: None,
                header_table_size: None,
                enable_push: Some(false),
                headers_priority: settings.headers_priority,
                headers_pseudo_order: settings.headers_pseudo_order,
                settings_order: settings.settings_order,
            },
        },
        header_initializer,
    ))
}

fn header_initializer(headers: &mut HeaderMap) {
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"),
    );
    headers.insert("sec-fetch-site", HeaderValue::from_static("none"));
    headers.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, deflate, br"),
    );
    headers.insert("sec-fetch-mode", HeaderValue::from_static("navigate"));
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (iPad; CPU OS 17_4_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.4.1 Mobile/15E148 Safari/604.1"));
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
    headers.insert("sec-fetch-dest", HeaderValue::from_static("document"));
}
