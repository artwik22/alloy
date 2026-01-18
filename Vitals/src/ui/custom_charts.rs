use gtk4::prelude::*;
use gtk4::DrawingArea;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

const SPARKLINE_MAX_POINTS: usize = 60;
const DETAILED_MAX_POINTS: usize = 240;

// Sparkline Widget for Dashboard Cards
pub struct SparklineWidget {
    widget: DrawingArea,
    data: Arc<Mutex<VecDeque<f64>>>,
    color: (f64, f64, f64),
}

impl SparklineWidget {
    pub fn new(color: (f64, f64, f64)) -> Self {
        let widget = DrawingArea::new();
        // Use minimum sizes instead of fixed sizes for responsiveness
        widget.set_content_height(50);  // Minimum height
        widget.set_content_width(150);  // Minimum width
        widget.set_hexpand(true);
        widget.set_vexpand(false);
        
        let data = Arc::new(Mutex::new(VecDeque::with_capacity(SPARKLINE_MAX_POINTS)));
        let color_arc = Arc::new(Mutex::new(color));
        let data_clone = Arc::clone(&data);
        let color_clone = Arc::clone(&color_arc);
        
        widget.set_draw_func(move |_widget, context, width, height| {
            let data = data_clone.lock().unwrap();
            let color = *color_clone.lock().unwrap();
            Self::draw_sparkline(context, &data, width as f64, height as f64, color);
        });
        
        Self { widget, data, color }
    }
    
    pub fn set_color(&mut self, color: (f64, f64, f64)) {
        self.color = color;
        self.widget.queue_draw();
    }
    
    fn draw_sparkline(
        context: &cairo::Context,
        data: &VecDeque<f64>,
        width: f64,
        height: f64,
        color: (f64, f64, f64),
    ) {
        context.set_antialias(cairo::Antialias::Best);
        
        // Background is transparent, let parent handle it
        
        if data.len() < 2 {
            return;
        }
        
        let step_x = width / (data.len() - 1) as f64;
        let max_value = 100.0;
        
        // Draw filled area first
        context.set_source_rgba(color.0, color.1, color.2, 0.2);
        context.move_to(0.0, height);
        
        for (i, &value) in data.iter().enumerate() {
            let x = i as f64 * step_x;
            let y = height - (value / max_value) * height;
            context.line_to(x, y);
        }
        
        context.line_to(width, height);
        context.close_path();
        context.fill().unwrap();
        
        // Draw line
        context.set_source_rgb(color.0, color.1, color.2);
        context.set_line_width(2.0);
        context.set_line_cap(cairo::LineCap::Round);
        context.set_line_join(cairo::LineJoin::Round);
        
        let mut first = true;
        for (i, &value) in data.iter().enumerate() {
            let x = i as f64 * step_x;
            let y = height - (value / max_value) * height;
            
            if first {
                context.move_to(x, y);
                first = false;
            } else {
                context.line_to(x, y);
            }
        }
        
        context.stroke().unwrap();
    }
    
    pub fn widget(&self) -> &DrawingArea {
        &self.widget
    }
    
    pub fn add_point(&self, value: f64) {
        let mut data = self.data.lock().unwrap();
        if data.len() >= SPARKLINE_MAX_POINTS {
            data.pop_front();
        }
        data.push_back(value);
        self.widget.queue_draw();
    }
}

// Detailed Chart Widget for full views
pub struct DetailedChartWidget {
    widget: DrawingArea,
    data: Arc<Mutex<VecDeque<f64>>>,
    color: (f64, f64, f64),
    label: String,
    show_grid: bool,
}

impl DetailedChartWidget {
    pub fn new(color: (f64, f64, f64), label: String) -> Self {
        let widget = DrawingArea::new();
        // Use minimum sizes for responsiveness
        widget.set_content_height(300);  // Minimum height
        widget.set_content_width(400);   // Minimum width
        widget.set_hexpand(true);
        widget.set_vexpand(true);
        
        let data = Arc::new(Mutex::new(VecDeque::with_capacity(DETAILED_MAX_POINTS)));
        let data_clone = Arc::clone(&data);
        let label_clone = label.clone();
        
        widget.set_draw_func(move |_widget, context, width, height| {
            let data = data_clone.lock().unwrap();
            Self::draw_detailed(
                context,
                &data,
                width as f64,
                height as f64,
                color,
                &label_clone,
            );
        });
        
        Self {
            widget,
            data,
            color,
            label,
            show_grid: true,
        }
    }
    
    fn draw_detailed(
        context: &cairo::Context,
        data: &VecDeque<f64>,
        width: f64,
        height: f64,
        color: (f64, f64, f64),
        _label: &str,
    ) {
        context.set_antialias(cairo::Antialias::Best);
        
        // Draw semi-transparent background
        context.set_source_rgba(0.0, 0.0, 0.0, 0.1);
        context.rectangle(0.0, 0.0, width, height);
        context.fill().unwrap();
        
        // Draw grid lines
        context.set_source_rgba(0.5, 0.5, 0.5, 0.2);
        context.set_line_width(1.0);
        
        for i in 0..5 {
            let y = (height / 5.0) * i as f64;
            context.move_to(0.0, y);
            context.line_to(width, y);
        }
        context.stroke().unwrap();
        
        if data.len() < 2 {
            return;
        }
        
        let step_x = width / (data.len() - 1) as f64;
        let max_value = 100.0;
        
        // Draw filled gradient area
        let gradient = cairo::LinearGradient::new(0.0, 0.0, 0.0, height);
        gradient.add_color_stop_rgba(0.0, color.0, color.1, color.2, 0.4);
        gradient.add_color_stop_rgba(1.0, color.0, color.1, color.2, 0.05);
        
        context.set_source(&gradient);
        context.move_to(0.0, height);
        
        for (i, &value) in data.iter().enumerate() {
            let x = i as f64 * step_x;
            let y = height - (value / max_value) * height;
            context.line_to(x, y);
        }
        
        context.line_to(width, height);
        context.close_path();
        context.fill().unwrap();
        
        // Draw main line
        context.set_source_rgb(color.0, color.1, color.2);
        context.set_line_width(2.5);
        context.set_line_cap(cairo::LineCap::Round);
        context.set_line_join(cairo::LineJoin::Round);
        
        let mut first = true;
        for (i, &value) in data.iter().enumerate() {
            let x = i as f64 * step_x;
            let y = height - (value / max_value) * height;
            
            if first {
                context.move_to(x, y);
                first = false;
            } else {
                context.line_to(x, y);
            }
        }
        
        context.stroke().unwrap();
    }
    
    pub fn widget(&self) -> &DrawingArea {
        &self.widget
    }
    
    pub fn add_point(&self, value: f64) {
        let mut data = self.data.lock().unwrap();
        if data.len() >= DETAILED_MAX_POINTS {
            data.pop_front();
        }
        data.push_back(value);
        self.widget.queue_draw();
    }
    
    pub fn clear(&self) {
        let mut data = self.data.lock().unwrap();
        data.clear();
        self.widget.queue_draw();
    }
}

// Multi-line chart for network view (upload + download)
pub struct DualLineChartWidget {
    widget: DrawingArea,
    data1: Arc<Mutex<VecDeque<f64>>>,
    data2: Arc<Mutex<VecDeque<f64>>>,
    color1: (f64, f64, f64),
    color2: (f64, f64, f64),
    label1: String,
    label2: String,
}

impl DualLineChartWidget {
    pub fn new(
        color1: (f64, f64, f64),
        color2: (f64, f64, f64),
        label1: String,
        label2: String,
    ) -> Self {
        let widget = DrawingArea::new();
        // Use minimum sizes for responsiveness
        widget.set_content_height(300);  // Minimum height
        widget.set_content_width(400);   // Minimum width
        widget.set_hexpand(true);
        widget.set_vexpand(true);
        
        let data1 = Arc::new(Mutex::new(VecDeque::with_capacity(DETAILED_MAX_POINTS)));
        let data2 = Arc::new(Mutex::new(VecDeque::with_capacity(DETAILED_MAX_POINTS)));
        let data1_clone = Arc::clone(&data1);
        let data2_clone = Arc::clone(&data2);
        
        widget.set_draw_func(move |_widget, context, width, height| {
            let data1 = data1_clone.lock().unwrap();
            let data2 = data2_clone.lock().unwrap();
            Self::draw_dual(
                context,
                &data1,
                &data2,
                width as f64,
                height as f64,
                color1,
                color2,
            );
        });
        
        Self {
            widget,
            data1,
            data2,
            color1,
            color2,
            label1,
            label2,
        }
    }
    
    fn draw_dual(
        context: &cairo::Context,
        data1: &VecDeque<f64>,
        data2: &VecDeque<f64>,
        width: f64,
        height: f64,
        color1: (f64, f64, f64),
        color2: (f64, f64, f64),
    ) {
        context.set_antialias(cairo::Antialias::Best);
        
        // Background
        context.set_source_rgba(0.0, 0.0, 0.0, 0.1);
        context.rectangle(0.0, 0.0, width, height);
        context.fill().unwrap();
        
        // Grid
        context.set_source_rgba(0.5, 0.5, 0.5, 0.2);
        context.set_line_width(1.0);
        for i in 0..5 {
            let y = (height / 5.0) * i as f64;
            context.move_to(0.0, y);
            context.line_to(width, y);
        }
        context.stroke().unwrap();
        
        // Find max value for scaling
        let max_value = 100.0;
        
        // Draw first line (e.g., download)
        if data1.len() >= 2 {
            let step_x = width / (data1.len() - 1) as f64;
            
            context.set_source_rgba(color1.0, color1.1, color1.2, 0.3);
            context.move_to(0.0, height);
            for (i, &value) in data1.iter().enumerate() {
                let x = i as f64 * step_x;
                let y = height - (value / max_value) * height;
                context.line_to(x, y);
            }
            context.line_to(width, height);
            context.close_path();
            context.fill().unwrap();
            
            context.set_source_rgb(color1.0, color1.1, color1.2);
            context.set_line_width(2.5);
            let mut first = true;
            for (i, &value) in data1.iter().enumerate() {
                let x = i as f64 * step_x;
                let y = height - (value / max_value) * height;
                if first {
                    context.move_to(x, y);
                    first = false;
                } else {
                    context.line_to(x, y);
                }
            }
            context.stroke().unwrap();
        }
        
        // Draw second line (e.g., upload)
        if data2.len() >= 2 {
            let step_x = width / (data2.len() - 1) as f64;
            
            context.set_source_rgba(color2.0, color2.1, color2.2, 0.3);
            context.move_to(0.0, height);
            for (i, &value) in data2.iter().enumerate() {
                let x = i as f64 * step_x;
                let y = height - (value / max_value) * height;
                context.line_to(x, y);
            }
            context.line_to(width, height);
            context.close_path();
            context.fill().unwrap();
            
            context.set_source_rgb(color2.0, color2.1, color2.2);
            context.set_line_width(2.5);
            let mut first = true;
            for (i, &value) in data2.iter().enumerate() {
                let x = i as f64 * step_x;
                let y = height - (value / max_value) * height;
                if first {
                    context.move_to(x, y);
                    first = false;
                } else {
                    context.line_to(x, y);
                }
            }
            context.stroke().unwrap();
        }
    }
    
    pub fn widget(&self) -> &DrawingArea {
        &self.widget
    }
    
    pub fn add_points(&self, value1: f64, value2: f64) {
        let mut data1 = self.data1.lock().unwrap();
        let mut data2 = self.data2.lock().unwrap();
        
        if data1.len() >= DETAILED_MAX_POINTS {
            data1.pop_front();
        }
        if data2.len() >= DETAILED_MAX_POINTS {
            data2.pop_front();
        }
        
        data1.push_back(value1);
        data2.push_back(value2);
        
        self.widget.queue_draw();
    }
}
