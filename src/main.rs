use concoct::composable::{container, state, stream, text};
use concoct::compose;
use concoct::dimension::Padding;
use concoct::dimension::{IntoPixels, Size};
use concoct::{run, Modifier};
use nostr::{RelayMessage, SubscriptionFilter};
use nostr_sdk::prelude::{FromBech32, Keys, SecretKey};
use nostr_sdk::{Client, RelayPoolNotification};
use skia_safe::RGB;
use taffy::style::{AlignItems, Dimension, FlexDirection, JustifyContent};
use tokio_stream::wrappers::BroadcastStream;

const BECH32_SK: &str = "nsec1ufnus6pju578ste3v90xd5m2decpuzpql2295m3sknqcjzyys9ls0qlc85";

async fn get_notifications() -> BroadcastStream<RelayPoolNotification> {
    let secret_key = SecretKey::from_bech32(BECH32_SK).unwrap();
    let my_keys = Keys::new(secret_key);

    let client = Client::new(&my_keys);
    client
        .add_relay("wss://relay.damus.io", None)
        .await
        .unwrap();

    client.connect().await;
    dbg!("connect!");

    let subscription = SubscriptionFilter::new().limit(5);
    client.subscribe(vec![subscription]).await.unwrap();

    BroadcastStream::new(client.notifications())
}

#[compose]
fn app() {
    let notifications = state(|| Vec::new());
    state(|| {
        stream(get_notifications, move |res| {
            dbg!("notification!");
            notifications.get().as_mut().push(res.unwrap());
        });
    });

    container(
        Modifier::default()
            .flex_direction(FlexDirection::Column)
            .flex_shrink(0.)
            .gap(Size::from(Dimension::Points(20.dp())))
            .size(Size::default().width(Dimension::Points(600.dp()))),
        move || {
            for notification in notifications.get().as_ref().iter().rev() {
                if let RelayPoolNotification::Message(_url, msg) = notification {
                    if let RelayMessage::Event {
                        subscription_id: _,
                        event,
                    } = msg
                    {
                        let content = event.content.clone();
                        container(
                            Modifier::default()
                                .size(Size::default().width(Dimension::Percent(1.)))
                                .padding(Padding::default().veritcal(20.dp()))
                                .background_color(RGB::from((225, 225, 235))),
                            move || text(Modifier::default().font_size(14.dp()), content.clone()),
                        )
                    }
                }
            }
        },
    )
}

#[tokio::main]
async fn main() {
    run(app);
}
