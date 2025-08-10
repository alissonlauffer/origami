use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::Cell;

use gtk::graphene;

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties)]
    #[properties(wrapper_type = super::LoadingIndicator)]
    pub struct LoadingIndicator {
        pub(super) start_time: Cell<i64>,

        #[property(get, set, minimum = 0.0, maximum = 1.0)]
        pub(super) progress: Cell<f64>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for LoadingIndicator {
        const NAME: &'static str = "OriLoadingIndicator";
        type Type = super::LoadingIndicator;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.set_css_name("loadingindicator");
        }
    }

    impl ObjectImpl for LoadingIndicator {
        fn constructed(&self) {
            self.obj().connect_visible_notify(|widget| {
                if widget.is_visible() {
                    widget.imp().start_time.set(widget.time());
                    widget.add_tick_callback(|widget, _clock| {
                        widget.queue_draw();
                        glib::ControlFlow::from(widget.is_visible())
                    });
                }
            });
        }

        fn properties() -> &'static [glib::ParamSpec] {
            Self::derived_properties()
        }

        fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            Self::derived_set_property(self, id, value, pspec)
        }

        fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            Self::derived_property(self, id, pspec)
        }
    }

    impl WidgetImpl for LoadingIndicator {
        fn realize(&self) {
            self.parent_realize();
            self.obj().notify("visible");
        }

        fn measure(&self, _orientation: gtk::Orientation, for_size: i32) -> (i32, i32, i32, i32) {
            if for_size < 0 {
                (0, 32, -1, -1)
            } else {
                (0, for_size, -1, -1)
            }
        }

        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            let widget = self.obj();
            let size = widget.width() as f32;
            let bounds = graphene::Rect::new(0.0, 0.0, size, size);

            let context = snapshot.append_cairo(&bounds);
            let color = widget.color();
            context.set_source_rgba(
                color.red() as _,
                color.green() as _,
                color.blue() as _,
                color.alpha() as _,
            );
            let half_size = size as f64 / 2.0;

            let pi = std::f64::consts::PI;

            context.set_line_width(2.0);

            let time = widget.time() - self.start_time.get();
            let shift = (time as f64 / 300000.0) % (2.0 * pi);

            let start = shift - 0.5 * pi;
            let diff = self.progress.get().max(0.04) * 2.0 * pi;

            context.arc(half_size, half_size, half_size - 2.0, start, start + diff);
            context.stroke().unwrap();
        }
    }
}

glib::wrapper! {
    #[doc(alias = "OriLoadingIndicator")]
    /// Circular loading indicator
    ///
    /// # Properties
    /// * progress: [f64] between 0 and 1
    pub struct LoadingIndicator(ObjectSubclass<imp::LoadingIndicator>)
        @extends gtk::Widget, gtk::ConstraintTarget, gtk::Buildable, gtk::Accessible;
}

impl LoadingIndicator {
    fn time(&self) -> i64 {
        self.frame_clock()
            .and_then(|clk| clk.current_timings())
            .map(|t| t.frame_time())
            .unwrap_or_default()
    }
}
