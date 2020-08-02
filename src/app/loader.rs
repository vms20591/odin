use std::collections::HashMap;
use std::error::Error;
use std::fs;
use scraper::{Html, Selector};
use reqwest::{self, blocking, Url};
use shellexpand;

use super::{OPENWRT_ROOT_PAGE, ALL_DEVICES_PAGE, ODIN_DEVICE_PAGE, Model, Manufacturer, Version};
use super::Log;

/// Loads html content from OpenWrt's `supported devices` page
fn load_html_content_from_http() -> Result<Option<String>, Box<dyn Error>> {
    let client = blocking::Client::new();
    let response = client.get(Url::parse(ALL_DEVICES_PAGE)?)
        .send()?;
    let status_code = response.status();
    let response_content = response.text()?;

    if status_code != 200 {
        Log::print_error(format!("Error: {}", response_content));
    
        return Ok(None);
    }

    Ok(Some(response_content))
}

fn load_html_content_from_file(file: &str) -> Option<String> {
    // we don't care about file errors
    match fs::read_to_string(shellexpand::tilde(file).trim()) {
        Ok(html_content) => Some(html_content),
        Err(_) => None
    }
}

/// Loads all brand details by fetching html content from OpenWrt's `supported` page
/// This can be used when network connection is fast & there aren't any network issues
pub fn load_manufacturers(file: Option<&str>) -> Result<Option<Vec<Manufacturer>>, Box<dyn Error>> {
    let file = file.unwrap_or(ODIN_DEVICE_PAGE);

    // try loading content from user given file or one from odin home (if available)
    let html_content = if let Some(html_content) = load_html_content_from_file(file) {
        println!("Loaded content from file: {0}", file);
        
        html_content
    }
    // do it the hard way, http!
    else {
        println!("Loading content from web");

        match load_html_content_from_http()? {
            Some(html_content) => html_content,
            None => {
                return Ok(None);
            }
        }
    };

    load_manufacturers_from(&html_content)
}

/// Loads all brand details from given `html_content`
/// This would be ideal in case where you already have device details stored as html, and
/// it could just be loaded from file & passed
/// Doesn't suffer from any network issues
fn load_manufacturers_from(html_content: &str) -> Result<Option<Vec<Manufacturer>>, Box<dyn Error>> {
    let mut manufacturers: Vec<Manufacturer> = Vec::new();
    let mut manufacturer_models_map: HashMap<String, Vec<Model>> = HashMap::new();
    let document = Html::parse_document(&html_content);
    let tr_selector = Selector::parse(".table.dataaggregation tr")
        .unwrap();
    let td_selector = Selector::parse("td")
        .unwrap();
    let a_selector = Selector::parse("a")
        .unwrap();
    let mut tr_nodes = document.select(&tr_selector)
        .skip(2); // 1st two rows are header & filters
    
    while let Some(node) = tr_nodes.next() {
        let mut td_nodes = node.select(&td_selector) // ideally we get 7 cells
            .skip(1) // skip row number cell
            .take(5); // skips last cell, which is view/edit link
    
        // manufacturer cell
        if let Some(node) = td_nodes.next() {
            let manufacturer = node.text()
                .next()
                .unwrap();
            let models = manufacturer_models_map.entry(manufacturer.to_string())
                .or_insert(Vec::new());
            let mut model_name: String = "".to_string();
            let mut versions: Vec<String> = Vec::new();
            let mut openwrt_version: Version = Version::new("".to_string(), "".to_string());
            let mut device_page: String = "".to_string();
                
            // model name cell
            if let Some(node) = td_nodes.next() {
                if let Some(val) = node.text().next() {
                    model_name = val.to_string();
                }
            }
    
            // model versions cell
            if let Some(node) = td_nodes.next() {
                if let Some(val) = node.text().next() {
                    versions = val.to_string()
                        .split(",")
                        .map(|version| version.trim().to_string())
                        .collect::<Vec<String>>();
                };
            }
    
            // openwrt version cell
            if let Some(node) = td_nodes.next() {
                let mut a_nodes = node.select(&a_selector);
    
                if let Some(node) = a_nodes.next() {
                    let mut version = "".to_string();
    
                    if let Some(val) = node.text().next() {
                        version = val.to_string();
                    };
    
                    let mut link = node.value()
                        .attr("href")
                        .unwrap()
                        .to_string();
                    link = format!("{}{}", OPENWRT_ROOT_PAGE, link);
    
                    openwrt_version = Version::new(version, link);
                }
            }
            
            // release page cell
            if let Some(node) = td_nodes.next() {
                let mut a_nodes = node.select(&a_selector);
    
                if let Some(node) = a_nodes.next() {
                    device_page = node.value()
                        .attr("href")
                        .unwrap()
                        .to_string();
                    device_page = format!("{}{}", OPENWRT_ROOT_PAGE, device_page);
                }
            }
    
            let model = Model::new(model_name, versions, openwrt_version, device_page);
    
            models.push(model);
        }
    }
    
    for (manufacturer, models) in manufacturer_models_map {
        manufacturers.push(Manufacturer::new(manufacturer, models));
    }
    
    if manufacturers.len() > 0 {
        Ok(Some(manufacturers))
    }
    else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_manufacturers_with_all_data_present() {
        let html_content = r"
            <div class='table dataaggregation'>
                <table>
                    <tbody>
                        <tr />
                        <tr />
                        <tr>
                            <td />
                            <td>Abicom International</td>
                            <td>Freedom CPE</td>
                            <td>Rev 05</td>
                            <td>
                                <a href='/releases/10.03'>10.03</a>
                            </td>
                            <td>
                                <a href='/toh/abicom/freedom_cpe'>freedom_cpe</a>
                            </td>
                            <td />
                        </tr>
                    </tbody>
                </table>
            </div>
        ";

        let manufacturers = load_manufacturers_from(html_content);
        assert!(manufacturers.is_ok(), "Error loading html");

        let manufacturers = manufacturers.unwrap();
        assert!(manufacturers.is_some(), "Expected manufacturers, but found none");

        let manufacturers = manufacturers.unwrap();
        assert_eq!(1, manufacturers.len(), "Expected manufacturers, but found none");

        let manufacturer = manufacturers.get(0)
            .unwrap();
        assert_eq!("Abicom International", manufacturer.name());

        let model = manufacturer.models()
            .get(0);
        assert!(model.is_some(), "Expected models, but found none");

        let model = model.unwrap();
        assert_eq!("Freedom CPE", model.name());

        let version = model.versions()
            .get(0);
        assert!(version.is_some(), "Expected versions, but found none");

        let version = version.unwrap();
        assert_eq!("Rev 05", version);

        let openwrt_version = model.openwrt_version();
        assert_eq!("10.03", openwrt_version.version());
        assert_eq!(&format!("{}{}", OPENWRT_ROOT_PAGE, "/releases/10.03"), openwrt_version.link());

        let device_page = model.device_page();
        assert_eq!(&format!("{}{}", OPENWRT_ROOT_PAGE, "/toh/abicom/freedom_cpe"), device_page);
    }

    #[test]
    fn loads_multiple_manufacturers() {
        let html_content = r"
            <div class='table dataaggregation'>
                <table>
                    <tbody>
                        <tr />
                        <tr />
                        <tr>
                            <td />
                            <td>Abicom International</td>
                            <td>Freedom CPE</td>
                            <td>Rev 05</td>
                            <td>
                                <a href='/releases/10.03'>10.03</a>
                            </td>
                            <td>
                                <a href='/toh/abicom/freedom_cpe'>freedom_cpe</a>
                            </td>
                            <td />
                        </tr>
                        <tr>
                            <td />
                            <td>Actiontec</td>
                            <td>GT701</td>
                            <td>C, D</td>
                            <td>
                                <a href='/releases/10.03.1'>10.03.1</a>
                            </td>
                            <td>
                                <a href='/toh/actiontec/gt701d'>gt701d</a>
                            </td>
                            <td />
                        </tr>
                    </tbody>
                </table>
            </div>
        ";

        let manufacturers = load_manufacturers_from(html_content);
        assert!(manufacturers.is_ok(), "Error loading html");

        let manufacturers = manufacturers.unwrap();
        assert!(manufacturers.is_some(), "Expected multiple manufacturers, but found none");

        let manufacturers = manufacturers.unwrap();
        assert_eq!(2, manufacturers.len(), "Expected multiple manufacturers, but found {}", manufacturers.len());
    }

    #[test]
    fn loads_multiple_models() {
        let html_content = r"
            <div class='table dataaggregation'>
                <table>
                    <tbody>
                        <tr />
                        <tr />
                        <tr>
                            <td />
                            <td>Abicom International</td>
                            <td>Freedom CPE</td>
                            <td>Rev 05</td>
                            <td>
                                <a href='/releases/10.03'>10.03</a>
                            </td>
                            <td>
                                <a href='/toh/abicom/freedom_cpe'>freedom_cpe</a>
                            </td>
                            <td />
                        </tr>
                        <tr>
                            <td />
                            <td>Abicom International</td>
                            <td>Scorpion SC450</td>
                            <td>Rev 02</td>
                            <td>
                                <a href='/releases/19.07.3'>19.07.3</a>
                            </td>
                            <td>
                                <a href='/toh/abicom/scorpion450'>scorpion450</a>
                            </td>
                            <td />
                        </tr>
                    </tbody>
                </table>
            </div>
        ";

        let manufacturers = load_manufacturers_from(html_content);
        assert!(manufacturers.is_ok(), "Error loading html");

        let manufacturers = manufacturers.unwrap();
        assert!(manufacturers.is_some(), "Expected manufacturers, but found none");

        let manufacturers = manufacturers
            .unwrap();
        assert_eq!(1, manufacturers.len(), "Expected one manufacturer, but found {}", manufacturers.len());

        let manufacturer = manufacturers
            .get(0)
            .unwrap();
        let models = manufacturer.models();
        assert_eq!(2, models.len(), "Expected multiple models, but found {}", models.len());
    }

    #[test]
    fn loads_manufacturers_with_model_version_missing() {
        let html_content = r"
            <div class='table dataaggregation'>
                <table>
                    <tbody>
                        <tr />
                        <tr />
                        <tr>
                            <td />
                            <td>Abicom International</td>
                            <td>Freedom CPE</td>
                            <td />
                            <td>
                                <a href='/releases/10.03'>10.03</a>
                            </td>
                            <td>
                                <a href='/toh/abicom/freedom_cpe'>freedom_cpe</a>
                            </td>
                            <td />
                        </tr>
                    </tbody>
                </table>
            </div>
        ";

        let manufacturers = load_manufacturers_from(html_content);
        assert!(manufacturers.is_ok(), "Error loading html");

        let manufacturers = manufacturers.unwrap();
        assert!(manufacturers.is_some(), "Expected manufacturers, but found none");

        let manufacturers = manufacturers.unwrap();
        assert_eq!(1, manufacturers.len(), "Expected manufacturers, but found none");
    }

    #[test]
    fn loads_manufacturers_with_openwrt_version_missing() {
        let html_content = r"
            <div class='table dataaggregation'>
                <table>
                    <tbody>
                        <tr />
                        <tr />
                        <tr>
                            <td />
                            <td>Abicom International</td>
                            <td>Freedom CPE</td>
                            <td>Rev 05</td>
                            <td />
                            <td>
                                <a href='/toh/abicom/freedom_cpe'>freedom_cpe</a>
                            </td>
                            <td />
                        </tr>
                    </tbody>
                </table>
            </div>
        ";

        let manufacturers = load_manufacturers_from(html_content);
        assert!(manufacturers.is_ok(), "Error loading html");

        let manufacturers = manufacturers.unwrap();
        assert!(manufacturers.is_some(), "Expected manufacturers, but found none");

        let manufacturers = manufacturers.unwrap();
        assert_eq!(1, manufacturers.len(), "Expected manufacturers, but found none");
    }

    #[test]
    fn loads_manufacturers_with_device_page_missing() {
        let html_content = r"
            <div class='table dataaggregation'>
                <table>
                    <tbody>
                        <tr />
                        <tr />
                        <tr>
                            <td />
                            <td>Abicom International</td>
                            <td>Freedom CPE</td>
                            <td>Rev 05</td>
                            <td>
                                <a href='/releases/10.03'>10.03</a>
                            </td>
                            <td />
                            <td />
                        </tr>
                    </tbody>
                </table>
            </div>
        ";

        let manufacturers = load_manufacturers_from(html_content);
        assert!(manufacturers.is_ok(), "Error loading html");

        let manufacturers = manufacturers.unwrap();
        assert!(manufacturers.is_some(), "Expected manufacturers, but found none");

        let manufacturers = manufacturers.unwrap();
        assert_eq!(1, manufacturers.len(), "Expected manufacturers, but found none");
    }

    #[test]
    fn loads_manufacturers_with_all_data_missing() {
        let html_content = r"
            <div class='table dataaggregation'>
                <table>
                    <tbody>
                        <tr />
                        <tr />
                        <tr>
                            <td />
                            <td />
                            <td />
                            <td />
                            <td />
                            <td />
                            <td />
                        </tr>
                    </tbody>
                </table>
            </div>
        ";

        let manufacturers = load_manufacturers_from(html_content);
        assert!(manufacturers.is_ok(), "Error loading html");

        let manufacturers = manufacturers.unwrap();
        assert!(manufacturers.is_some(), "Expected manufacturers, but found none");

        let manufacturers = manufacturers.unwrap();
        assert_eq!(1, manufacturers.len(), "Expected manufacturers, but found none");
    }

    #[test]
    fn doesnt_load_manufacturers_with_malformed_html() {
        let html_content = r"
            <div class='table dataaggregation
                <table>
                    <tbody>
                        <tr />
                        <tr />
                        <tr>
                            <td />
                            <td>Abicom International</td>
                            <td>Freedom CPE</td>
                            <td>Rev 05</td>
                            <td>
                                <a href='/releases/10.03'>10.03</a>
                            </td>
                            <td>
                                <a href='/toh/abicom/freedom_cpe'>freedom_cpe</a>
                            </td>
                            <td />
                        </tr>
                    </tbody>
                </table>
            </div>
        ";

        let manufacturers = load_manufacturers_from(html_content);
        assert!(manufacturers.is_ok(), "Error loading html");

        let manufacturers = manufacturers.unwrap();
        assert!(manufacturers.is_none(), "Expected no manufacturers, but found one");
    }
}