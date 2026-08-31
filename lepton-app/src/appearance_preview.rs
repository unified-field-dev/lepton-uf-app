//! Live theme preview strip for the Appearance settings page.

use leptos::prelude::*;
use uf_product::components::{
    Avatar, AvatarConfig, Badge, BadgeAppearance, BadgeColor, BadgeSize, Persona, PersonaConfig,
    PersonaSize,
};
use uf_product::primitives::*;

#[component]
pub fn AppearancePreviewGallery() -> impl IntoView {
    let notifications = RwSignal::new(true);
    let plan = RwSignal::new(Some("standard".to_string()));
    let terms = RwSignal::new(true);
    let input_value = RwSignal::new(String::new());
    let tab = RwSignal::new("overview".to_string());

    view! {
        <Material variant=MaterialVariant::Solid elevation=MaterialElevation::Resting>
            <Flex
                vertical=true
                gap=FlexGap::Medium
                full_width=true
                padding=SpacingInset::all_l()
            >
                <Flex wrap=FlexWrap::Wrap gap=FlexGap::Medium align=FlexAlign::Center>
                    <Button appearance=ButtonAppearance::Primary>"Primary"</Button>
                    <Button appearance=ButtonAppearance::Secondary>"Secondary"</Button>
                    <Button appearance=ButtonAppearance::Subtle>"Subtle"</Button>
                    <Button appearance=ButtonAppearance::Transparent>"Transparent"</Button>
                </Flex>
                <Flex wrap=FlexWrap::Wrap gap=FlexGap::Medium align=FlexAlign::Center>
                    <Avatar config=AvatarConfig::name("Alex Rivera") />
                    <Persona config=PersonaConfig {
                        name: Some("Jordan Lee".into()),
                        size: PersonaSize::Medium,
                        ..Default::default()
                    } />
                    <Badge appearance=BadgeAppearance::Filled color=BadgeColor::Brand size=BadgeSize::Medium>
                        "Brand"
                    </Badge>
                    <Badge appearance=BadgeAppearance::Tint color=BadgeColor::Success size=BadgeSize::Medium>
                        "Success"
                    </Badge>
                </Flex>
                <Switch bind=SwitchBind::from(notifications) label="Email notifications" />
                <Checkbox checked=terms label="Accept terms".to_string() size=Signal::from(CheckboxSize::Medium) />
                <RadioGroup bind=RadioGroupBind::from(plan)>
                    <Radio value="standard".to_string() label="Standard".to_string() />
                    <Radio value="premium".to_string() label="Premium".to_string() />
                </RadioGroup>
                <TabList selected_value=tab>
                    <Tab value="overview".to_string()>"Overview"</Tab>
                    <Tab value="settings".to_string()>"Settings"</Tab>
                </TabList>
                <Link href="#appearance-preview">"Learn more about theming"</Link>
                <Input
                    bind=input_value
                    appearance=InputAppearance::with_placeholder("Sample input")
                />
            </Flex>
        </Material>
    }
}
