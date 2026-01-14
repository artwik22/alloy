use gtk4::prelude::*;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

const MAX_POINTS: usize = 120; // 60 seconds at 2 updates per second

pub struct Graph {
    data: VecDeque<f64>,
    color: (f64, f64, f64),
    label: String,
}

impl Graph {
    pub fn new(color: (f64, f64, f64), label: String) -> Self {
        Self {
            data: VecDeque::with_capacity(MAX_POINTS),
            color,
            label,
        }
    }
    
    pub fn add_point(&mut self, value: f64) {
        if self.data.len() >= MAX_POINTS {
            self.data.pop_front();
        }
        self.data.push_back(value);
    }
    
    pub fn draw(&self, context: &cairo::Context, width: f64, height: f64) {
        if self.data.is_empty() {
            return;
        }
        
        // Enable antialiasing
        context.set_antialias(cairo::Antialias::Best);
        
        // Draw background
        context.set_source_rgba(0.1, 0.1, 0.1, 1.0);
        context.rectangle(0.0, 0.0, width, height);
        context.fill().unwrap();
        
        // Draw grid lines
        context.set_source_rgba(0.3, 0.3, 0.3, 0.5);
        context.set_line_width(1.0);
        
        for i in 0..5 {
            let y = (height / 5.0) * i as f64;
            context.move_to(0.0, y);
            context.line_to(width, y);
            context.stroke().unwrap();
        }
        
        // Draw graph line
        if self.data.len() < 2 {
            return;
        }
        
        let step_x = width / (self.data.len() - 1) as f64;
        let max_value = 100.0; // Percentage scale
        
        context.set_source_rgb(self.color.0, self.color.1, self.color.2);
        context.set_line_width(2.0);
        context.set_line_cap(cairo::LineCap::Round);
        context.set_line_join(cairo::LineJoin::Round);
        
        let mut first = true;
        for (i, &value) in self.data.iter().enumerate() {
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
        
        // Draw filled area under curve
        if let Some(&_last_value) = self.data.back() {
            context.line_to(width, height);
            context.line_to(0.0, height);
            context.close_path();
            
            let gradient = cairo::LinearGradient::new(0.0, 0.0, 0.0, height);
            gradient.add_color_stop_rgba(0.0, self.color.0, self.color.1, self.color.2, 0.3);
            gradient.add_color_stop_rgba(1.0, self.color.0, self.color.1, self.color.2, 0.0);
            
            context.set_source(&gradient);
            context.fill().unwrap();
        }
    }
}

pub struct GraphWidget {
    widget: gtk4::DrawingArea,
    graph: Arc<Mutex<Graph>>,
}

impl GraphWidget {
    pub fn new(color: (f64, f64, f64), label: String) -> Self {
        let widget = gtk4::DrawingArea::new();
        widget.set_hexpand(true);
        widget.set_vexpand(true);
        widget.set_content_width(400);
        widget.set_content_height(150);
        
        let graph = Arc::new(Mutex::new(Graph::new(color, label)));
        let graph_clone = Arc::clone(&graph);
        
        widget.set_draw_func(move |_widget: &gtk4::DrawingArea, context: &cairo::Context, width: i32, height: i32| {
            let graph = graph_clone.lock().unwrap();
            graph.draw(context, width as f64, height as f64);
        });
        
        Self { widget, graph }
    }
    
    pub fn widget(&self) -> &gtk4::DrawingArea {
        &self.widget
    }
    
    pub fn add_point(&self, value: f64) {
        let mut graph = self.graph.lock().unwrap();
        graph.add_point(value);
        self.widget.queue_draw();
    }
}
