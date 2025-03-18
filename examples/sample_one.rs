use rudderanalytics::client::RudderAnalytics;
use rudderanalytics::message::{
    Alias, Batch, BatchMessage, Group, Identify, Message, Page, Screen, Track,
};
use serde_json::json;

#[tokio::main]
async fn main() {
    let rudder_analytics = RudderAnalytics::load(
        "YOUR_WRITE_KEY".to_string(),
        "YOUR_DATA_PLANE_URL".to_string(),
    );
    let identify_msg = Message::Identify(Identify {
        user_id: Some("sample_user_id".to_string()),
        traits: Some(json!({
            "name": "Test User",
            "email": "test@user.com",
        })),
        ..Default::default()
    });
    let track_msg = Message::Track(Track {
        user_id: Some("sample_user_id".to_string()),
        event: "Track Event".to_owned(),
        properties: Some(json!({
            "some property": "some value",
            "some other property": "some other value",
        })),
        ..Default::default()
    });
    let page_msg = Message::Page(Page {
        user_id: Some("sample_user_id".to_string()),
        name: "Cart".to_owned(),
        properties: Some(json!({
            "some property": "some value",
            "some other property": "some other value",
        })),
        ..Default::default()
    });
    let group_msg = Message::Group(Group {
        user_id: Some("sample_user_id".to_string()),
        group_id: "sample_group_id".to_owned(),
        traits: Some(json!({
            "some property": "some value",
            "some other property": "some other value",
        })),
        ..Default::default()
    });
    let screen_msg = Message::Screen(Screen {
        user_id: Some("sample_user_id".to_string()),
        name: "sample screen".to_owned(),
        properties: Some(json!({
            "some property": "some value",
            "some other property": "some other value",
        })),
        ..Default::default()
    });
    let alias_msg = Message::Alias(Alias {
        user_id: "sample_user_id".to_owned(),
        previous_id: "sample_previous_user_id".to_owned(),
        traits: Some(json!({
            "some property": "some value",
            "some other property": "some other value",
        })),
        ..Default::default()
    });
    let batch_msg = Message::Batch(Batch {
        batch: vec![
            BatchMessage::Track(Track {
                user_id: Some("foo".to_string()),
                event: "Foo".to_owned(),
                properties: Some(json!({})),
                ..Default::default()
            }),
            BatchMessage::Track(Track {
                user_id: Some("bar".to_string()),
                event: "Bar".to_owned(),
                properties: Some(json!({})),
                ..Default::default()
            }),
            BatchMessage::Track(Track {
                user_id: Some("baz".to_string()),
                event: "Baz".to_owned(),
                properties: Some(json!({})),
                ..Default::default()
            }),
        ],
        context: Some(json!({
            "foo": "bar",
        })),
        ..Default::default()
    });

    rudder_analytics
        .send(&identify_msg)
        .await
        .expect("Identify call failed to send data to Rudderstack");

    rudder_analytics
        .send(&track_msg)
        .await
        .expect("Track call failed to send data to Rudderstack");

    rudder_analytics
        .send(&page_msg)
        .await
        .expect("Page call failed to send data to Rudderstack");

    rudder_analytics
        .send(&group_msg)
        .await
        .expect("Group call failed to send data to Rudderstack");

    rudder_analytics
        .send(&screen_msg)
        .await
        .expect("Screen call failed to send data to Rudderstack");

    rudder_analytics
        .send(&alias_msg)
        .await
        .expect("Alias call failed to send data to Rudderstack");

    rudder_analytics
        .send(&batch_msg)
        .await
        .expect("Batch call failed to send data to Rudderstack");
}
