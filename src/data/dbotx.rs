use crate::{config::Config, model::Kline};

/// Fetch latest klines from DBotX service.
///
/// The URL template is expected to be provided via `DBOTX_KLINE_URL` env and
/// should include `{chain}`, `{pair}`, `{interval}` and `{end}` placeholders.
pub async fn fetch_klines(cfg: &Config, end: i64) -> Result<Vec<Kline>, reqwest::Error> {
    let url = cfg
        .dbotx_kline_url
        .replace("{chain}", &cfg.dbotx_chain)
        .replace("{pair}", &cfg.dbotx_pair)
        .replace("{interval}", "1m")
        .replace("{end}", &end.to_string());

    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header("DBOTX-API-KEY", &cfg.dbotx_api_key)
        .send()
        .await?
        .json::<Vec<Kline>>()
        .await?;
    Ok(res)
}
