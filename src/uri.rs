//! HTTP RequestUris
use url;
use url::Url;

/// The Request-URI of a Request's StartLine.
///
/// From Section 5.3, Request Target:
/// > Once an inbound connection is obtained, the client sends an HTTP
/// > request message (Section 3) with a request-target derived from the
/// > target URI.  There are four distinct formats for the request-target,
/// > depending on both the method being requested and whether the request
/// > is to a proxy.
/// >
/// > ```notrust
/// > request-target = origin-form
/// >                / absolute-form
/// >                / authority-form
/// >                / asterisk-form
/// > ```
#[derive(Debug, PartialEq, Clone)]
pub enum RequestUri {
    /// The most common request target, an absolute path and optional query.
    ///
    /// For example, the line `GET /where?q=now HTTP/1.1` would parse the URI
    /// component `/where?q=now` as the AbsolutePath
    AbsolutePath(PathQueryFragment),

    /// An absolute URI. Used in conjunction with proxies.
    ///
    /// > When making a request to a proxy, other than a CONNECT or server-wide
    /// > OPTIONS request (as detailed below), a client MUST send the target
    /// > URI in absolute-form as the request-target.
    ///
    /// An example StartLine with an `AbsoluteUri` would be
    /// `GET http://www.example.org/pub/WWW/TheProject.html HTTP/1.1`.
    AbsoluteUri(Url),

    /// The authority form is only for use with `CONNECT` requests.
    ///
    /// An example StartLine: `CONNECT www.example.com:80 HTTP/1.1`.
    Authority(String),

    /// The star is used to target the entire server, instead of a specific resource.
    ///
    /// This is only used for a server-wide `OPTIONS` request.
    Star,
}

#[derive(Debug, PartialEq, Clone)]
/// An absolute URL path as seen by a server, such as /where?q=now
pub struct PathQueryFragment {
    /// The path component, as a String
    pub path: Vec<String>,
    /// The query component, optional, as a String.  Use query_pairs() method if it is
    /// application/x-www-form-urlencoded to break it down into a vector of (key,value)
    /// pairs.
    pub query: Option<String>,
    /// The HTTP RFC does not identify a fragment here, but it is generally parsed
    // in practice and discarded in case it exists
    pub fragment: Option<String>
}

impl PathQueryFragment {
    /// Parse the query string, if any, as `application/x-www-form-urlencoded`
    /// and return a vector of (key, value) pairs.
    #[inline]
    pub fn query_pairs(&self) -> Option<Vec<(String, String)>> {
        self.query.as_ref().map(|query| url::form_urlencoded::parse(query.as_bytes()))
    }
}
