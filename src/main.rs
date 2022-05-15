#[derive(Debug)]
enum Err {
  Reqwest,
  Xml,
}

#[tokio::main]
async fn main() -> Result<(), Err> {
  let client = reqwest::Client::new();
  let res = client
    .get(format!(
      "https://data.bus-data.dft.gov.uk/api/v1/datafeed/1695/?api_key={}",
      include_str!("../api_key.txt")
    ))
    .send()
    .await
    .unwrap();

  let text = res.text().await.map_err(|_| Err::Reqwest)?;

  let doc = roxmltree::Document::parse(&text).unwrap();

  let activities = doc.root()
    .first_child().ok_or(Err::Xml)?      // Siri
    .first_child().ok_or(Err::Xml)?      // ServiceDelivery
    .children().nth(2).ok_or(Err::Xml)?  // VehicleMonitoringDelivery
    .children().filter(|node|
      node.tag_name().name() == "VehicleActivity"
    )
    .collect::<Vec<_>>();                // VehicleActivity
  
  println!("{}", activities.len());

  Ok(())
}
