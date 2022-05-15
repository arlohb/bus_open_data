use chrono::prelude::*;
use crate::xml_utils::{get_child, get_child_text};

pub enum DirectionRef {
    Outbound,
    Inbound,
}

pub struct FramedVehicleJourneyRef {
    pub data_frame_ref: String,
    pub dated_vehicle_journey_ref: String,
}

pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
}

pub struct MonitoredVehicleJourney {
    pub line_ref: String,
    pub direction_ref: DirectionRef,
    pub framed_vehicle_journey_ref: FramedVehicleJourneyRef,
    pub published_line_name: String,
    pub operator_ref: String,
    pub origin_ref: String,
    pub origin_name: String,
    pub destination_ref: String,
    pub destination_name: String,
    pub origin_aimed_departure_time: DateTime<Utc>,
    pub vehicle_location: Location,
    pub bearing: Option<f64>,
    pub vehicle_ref: String,
}

pub struct VehicleJourney {
    pub driver_ref: u32,
}

pub struct Extensions {
    pub vehicle_journey: VehicleJourney,
}

pub struct VehicleActivity {
    pub recorded_at_time: DateTime<Utc>,
    pub item_identifier: String,
    pub valid_until_time: DateTime<Utc>,
    pub monitored_vehicle_journey: MonitoredVehicleJourney,
    pub extensions: Extensions,
}

impl VehicleActivity {
    pub fn from_node(node: &roxmltree::Node) -> Option<VehicleActivity> {
        Some(VehicleActivity {
            recorded_at_time: get_child_text(node, "RecordedAtTime")?
                .parse::<DateTime<Utc>>().ok()?,
            item_identifier: get_child_text(node, "ItemIdentifier")?,
            valid_until_time: (get_child_text(node, "ValidUntilTime")?
                .split(".").nth(0)?
                .to_string()
                + "Z")
                .parse::<DateTime<Utc>>().ok()?,
            monitored_vehicle_journey: {
                let node = &get_child(node, "MonitoredVehicleJourney")?;

                MonitoredVehicleJourney {
                    line_ref: get_child_text(node, "LineRef")?
                        .to_string(),
                    direction_ref: match get_child_text(node, "DirectionRef")?.as_str() {
                        "OUTBOUND" => DirectionRef::Outbound,
                        "INBOUND" => DirectionRef::Inbound,
                        _ => return None,
                        },
                    framed_vehicle_journey_ref: {
                        let node = &get_child(node, "FramedVehicleJourneyRef")?;
                        
                        FramedVehicleJourneyRef {
                            data_frame_ref: get_child_text(node, "DataFrameRef")?,
                            dated_vehicle_journey_ref: get_child_text(node, "DatedVehicleJourneyRef")?,
                        }
                    },
                    published_line_name: get_child_text(node, "PublishedLineName")?,
                    operator_ref: get_child_text(node, "OperatorRef")?,
                    origin_ref: get_child_text(node, "OriginRef")?,
                    origin_name: get_child_text(node, "OriginName")?,
                    destination_ref: get_child_text(node, "DestinationRef")?,
                    destination_name: get_child_text(node, "DestinationName")?,
                    origin_aimed_departure_time: get_child_text(node, "OriginAimedDepartureTime")?
                        .parse::<DateTime<Utc>>().ok()?,
                    vehicle_location: {
                        let node = &get_child(node, "VehicleLocation")?;

                        Location {
                            longitude: get_child_text(node, "Longitude")?
                                .parse::<f64>().ok()?,
                            latitude: get_child_text(node, "Latitude")?
                                .parse::<f64>().ok()?,
                        }
                    },
                    bearing: match get_child_text(node, "Bearing") {
                        Some(node) => Some(node
                        .parse::<f64>().ok()?),
                        None => None,
                    },
                    vehicle_ref: get_child_text(node, "VehicleRef")?,
                }
            },
            extensions: {
                let node = &get_child(node, "Extensions")?;
                let node = &get_child(node, "VehicleJourney")?;

                Extensions {
                    vehicle_journey: VehicleJourney {
                        driver_ref: get_child_text(node, "DriverRef")?
                            .parse::<u32>()
                            .or::<Result<u32, std::num::ParseIntError>>(Ok(0))
                            .ok()?,
                    },
                }
            },
        })
    }
}
