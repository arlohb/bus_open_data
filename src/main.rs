mod data;
mod xml_utils;

#[derive(Debug)]
pub enum Err {
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

  let elements = doc.root()
    .first_child().ok_or(Err::Xml)?      // Siri
    .first_child().ok_or(Err::Xml)?      // ServiceDelivery
    .children().nth(2).ok_or(Err::Xml)?  // VehicleMonitoringDelivery
    .children().filter(|node|
      node.tag_name().name() == "VehicleActivity"
    )                                    // VehicleActivity
    .collect::<Vec<_>>();

  let mut activities = elements
    .iter()
    .map(|node| data::VehicleActivity::from_node(&node))
    .filter(|option| option.is_some())
    .map(|option| option.unwrap())
    .collect::<Vec<_>>();
  
  println!("xml: {}, parsed: {}", elements.len(), activities.len());

  activities.sort_by(|a, b|
    a.monitored_vehicle_journey.line_ref.cmp(&b.monitored_vehicle_journey.line_ref)
  );
  
  activities
    .iter()
    .for_each(|activity| println!("{:4} {:<10} {:<10}",
      activity.monitored_vehicle_journey.line_ref.as_str(),
      activity.monitored_vehicle_journey.vehicle_location.latitude,
      activity.monitored_vehicle_journey.vehicle_location.longitude,
    ));

  Ok(())
}
