
/// URL that probably used in the program
pub(crate) mod url {
    use const_format::concatcp;
    /// Server address for 正方教务系统
    pub(crate) const HOME: &str = "http://jwxt.sit.edu.cn";

    /* Login related */

    pub(crate) const LOGIN: &str = concatcp!(HOME, "/jwglxt/xtgl/login_slogin.html");
    pub(crate) const RSA_PUBLIC_KEY: &str = concatcp!(HOME, "/jwglxt/xtgl/login_getPublicKey.html");

    /* function related */

    /// Score list page
    pub(crate) const SCORE_LIST: &str = concatcp!(
        HOME,
        "/jwglxt/cjcx/cjcx_cxDgXscj.html?doType=query&gnmkdm=N305005"
    );
    /// Time tanle page
    pub(crate) const TIME_TABLE: &str =
        concatcp!(HOME, "/jwglxt/kbcx/xskbcx_cxXsKb.html?gnmkdm=N253508");
    /// Personal profile page
    pub(crate) const PROFILE: &str = concatcp!(
        HOME,
        "/jwglxt/xsxxxggl/xsgrxxwh_cxXsgrxx.html?gnmkdm=N100801&layout=default"
    );
    /// Major list page
    pub(crate) const MAJOR_LIST: &str =
        concatcp!(HOME, "/jwglxt/xtgl/comm_cxZyfxList.html?gnmkdm=N214505");
    /// Class list page
    pub(crate) const CLASS_LIST: &str =
        concatcp!(HOME, "/jwglxt/xtgl/comm_cxBjdmList.html?gnmkdm=N214505");
    /// Suggested course and time table
    pub(crate) const SUGGESTED_COURSE: &str =
        concatcp!(HOME, "/jwglxt/kbdy/bjkbdy_cxBjKb.html?gnmkdm=N214505");
    /// Course selection page for available courses
    pub(crate) const AVAIL_COURSE_LIST: &str = concatcp!(
        HOME,
        "/xsxk/zzxkyzb_cxZzxkYzbPartDisplay.html?gnmkdm=N253512"
    );
}

pub(crate) const USERAGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.88 ' \
'Safari/537.36 Edg/87.0.664.66";
