use egui::{
    Button, Checkbox, Color32, FontFamily, Frame, Label, RichText, ScrollArea, Slider, TextEdit,
    Widget,
};
use egui_flex::{FlexAlign, FlexDirection, FlexItem, FlexJustify};
use hello_egui::flex::{item, Flex};
use hello_egui::infinite_scroll::InfiniteScroll;
use hello_egui_utils::run;
use std::num::NonZeroUsize;

fn main() {
    let mut infinite = InfiniteScroll::new().end_loader(|cursor, callback| {
        callback(Ok((vec!["hi"], Some(1))));
    });

    run!(move |ui| {
        ui.ctx().options_mut(|opts| {
            opts.max_passes = NonZeroUsize::new(1).unwrap();
        });
        // std::thread::sleep(std::time::Duration::from_millis(100));
        ScrollArea::vertical().show(ui, |ui| {
            let available_rect = ui.max_rect();
            infinite.ui(ui, 10, |ui, idx, text| {
                // The scope and the set max height is *essential* to get egui_flex work well
                // in a scroll area. Since egui_flex checks the available space and limits each items
                // size to that, we need to give it some space to work with.
                ui.scope(|ui| {
                    ui.set_max_height(available_rect.height());
                    
                    ui.label(format!("Item {:?}", ui.id()));

                    // Flex::vertical().w_full().show(ui, |flex| {
                    //     flex.add(item(), Label::new(*text));
                    // });

                    // let frame = egui::Frame::group(ui.style());
                    //
                    // Flex::new()
                    //     .justify(FlexJustify::Center)
                    //     .direction(FlexDirection::Vertical)
                    //     .align_items(FlexAlign::Center)
                    //     .w_full()
                    //     .show(ui, |outer_flex| {
                    //         outer_flex.add(
                    //             item().frame(frame),
                    //             Label::new(
                    //                 RichText::new("🏠Home")
                    //                     .size(48.0)
                    //                     .family(FontFamily::Proportional),
                    //             ),
                    //         );
                    //
                    //         outer_flex.add(item(), Checkbox::new(&mut false, "Some text"));
                    //     });

                    ui.ctx().debug_painter().debug_rect(
                        ui.available_rect_before_wrap(),
                        Color32::RED,
                        "",
                    );

                    ui.ctx().options_mut(|opts| {
                        opts.max_passes = NonZeroUsize::new(1).unwrap();
                    });

                    ui.horizontal_top(|ui| {
                        let items = vec![
                            "I",
                            "can have veeeeeeeeeeeeery long",
                            "and",
                            "very",
                            "short",
                            "and",
                            "multi\nline",
                            "or\neven\nmore\nlines\n\n\nhi",
                            "and",
                            "even",
                            "some middle length",
                            "items",
                        ];

                        Flex::new()
                            .w_full()
                            .align_items(egui_flex::FlexAlign::Stretch)
                            .align_items_content(egui::Align2::CENTER_CENTER)
                            .wrap(true)
                            .show(ui, |flex| {
                                flex.add_ui(
                                    FlexItem::default()
                                        .grow(1.0)
                                        .frame(Frame::group(flex.ui().style())),
                                    |ui| {
                                        ui.label("Hello");
                                    },
                                );

                                for item in items {
                                    flex.add_ui(
                                        FlexItem::default()
                                            .grow(1.0)
                                            .frame(Frame::group(flex.ui().style())),
                                        |ui| {
                                            Label::new(item).wrap().ui(ui);
                                        },
                                    );
                                }

                                flex.add_ui(
                                    FlexItem::default()
                                        .grow(1.0)
                                        .basis(200.0)
                                        .frame(Frame::group(flex.ui().style())),
                                    |ui| {
                                        TextEdit::singleline(&mut String::new())
                                            .desired_width(ui.available_width())
                                            .ui(ui);
                                    },
                                );

                                flex.add_ui(
                                    FlexItem::default()
                                        .grow(1.0)
                                        .basis(80.0)
                                        .frame(Frame::group(flex.ui().style())),
                                    |ui| {
                                        ui.add(Label::new("I have flex basis 80").wrap());
                                    },
                                );

                                for align in &[
                                    FlexAlign::Start,
                                    FlexAlign::End,
                                    FlexAlign::Center,
                                    FlexAlign::Stretch,
                                ] {
                                    flex.add_ui(
                                        FlexItem::default()
                                            .grow(1.0)
                                            .align_self(*align)
                                            .frame(Frame::group(flex.ui().style())),
                                        |ui| {
                                            ui.add(
                                                Label::new(format!("I have align-self: {align:?}"))
                                                    .wrap(),
                                            );
                                        },
                                    );
                                }

                                flex.add_ui(FlexItem::new().grow(1.0).basis(150.0), |ui| {
                                    ui.style_mut().spacing.slider_width =
                                        ui.available_width() - 50.0;
                                    Slider::new(&mut 0.0, 0.0..=1000.0).ui(ui);
                                });

                                flex.add_flex(
                                    FlexItem::default()
                                        .grow(1.0)
                                        .frame(egui::Frame::group(flex.ui().style())),
                                    Flex::vertical()
                                        .align_content(egui_flex::FlexAlignContent::Stretch)
                                        .grow_items(1.0),
                                    |flex| {
                                        flex.add(FlexItem::default().grow(1.0), Button::new("btn"));
                                        flex.add(
                                            FlexItem::default(),
                                            Button::new("Very long button"),
                                        );
                                        flex.add_flex(
                                            FlexItem::default().grow(1.0),
                                            Flex::horizontal()
                                                .align_content(egui_flex::FlexAlignContent::Stretch)
                                                .grow_items(1.0),
                                            |flex| {
                                                flex.add(
                                                    FlexItem::default().grow(1.0),
                                                    Button::new("btn"),
                                                );
                                                flex.add(
                                                    FlexItem::default(),
                                                    Button::new("Very long button"),
                                                );
                                            },
                                        );
                                    },
                                );

                                flex.add(
                                    FlexItem::new().grow(1.0),
                                    Button::new("Very long button"),
                                );

                                flex.add(FlexItem::new().grow(1.0), Button::new("Button"));
                                flex.add(
                                    FlexItem::new().grow(1.0),
                                    Button::new("Button wefoijfgiweopjg"),
                                );
                                flex.add(FlexItem::new().grow(1.0), Button::new("Button"));
                                flex.add(FlexItem::new(), Button::new("Simple Button"));

                                flex.add(FlexItem::new(), Checkbox::new(&mut false, "Checkbox"));

                                // flex.add_container(
                                //     FlexItem::default().grow(1.0).basis(100.0),
                                //     |ui, content| {
                                //         ui.group(|ui| {
                                //             content.content(ui, |ui| {
                                //                 ui.vertical(|ui| {
                                //                     Flex::new().show(ui, |flex| {
                                //                         flex.add(
                                //                             FlexItem::new(),
                                //                             Button::new("Button"),
                                //                         );
                                //
                                //                         flex.add(
                                //                             FlexItem::new(),
                                //                             Button::new("Longer Button"),
                                //                         );
                                //
                                //                         flex.add(
                                //                             FlexItem::new(),
                                //                             Button::new(
                                //                                 "Button\nwith\nmultiple\nlines",
                                //                             ),
                                //                         );
                                //                     });
                                //                 });
                                //             })
                                //         })
                                //         .inner
                                //     },
                                // );
                            });
                    });
                });
            });
        });
    });
}
