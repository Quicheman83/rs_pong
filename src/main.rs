use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "PONG".to_owned(),
        window_width: 800,
        window_height: 600,
        window_resizable: false, // Locks the window size
        ..Default::default()
    }
}

struct bouton {
	x: f32,
	y: f32,
	s_x: f32,
	s_y: f32,
	text: String,
	s_txt: f32,
	btn_status: i32,
	/*
	btn status :
	0: normal
	1: cursor on the button
	2: cliked
	*/
}

impl bouton {
	fn new(x: f32, y: f32, s_x: f32, s_y: f32, text: String, s_txt: f32) -> Self {
		Self {
			x,
			y,
			s_x,
			s_y,
			text,
			s_txt,
			btn_status: 0,
		}
	}
	
	fn update(&mut self) {
		let (mut mouse_x, mut mouse_y) = mouse_position();
		if mouse_x > self.x && mouse_x < self.x + self.s_x {
			if mouse_y > self.y && mouse_y < self.y + self.s_y {
				if is_mouse_button_down(MouseButton::Left) {
					self.btn_status = 2;
					//println!("BUTTON CLICKED");
				} else {
					self.btn_status = 1;
					//println!("BUTTON SELECT");
				}
			} else {
				self.btn_status = 0;
			}
		} else {
			self.btn_status = 0;
		}
	}
	
	fn draw(&self) {
		if self.btn_status == 0 {
			draw_rectangle(self.x, self.y, self.s_x, self.s_y, GRAY);
			draw_text(self.text.clone(), self.x, self.y + self.s_y - 8.0, self.s_txt, BLACK);
		}
		if self.btn_status == 1 {
			draw_rectangle(self.x, self.y, self.s_x, self.s_y, YELLOW);
			draw_text(self.text.clone(), self.x, self.y + self.s_y - 8.0, self.s_txt, BLACK);
		}
		if self.btn_status == 2 {
			draw_rectangle(self.x, self.y, self.s_x, self.s_y, RED);
			draw_text(self.text.clone(), self.x, self.y + self.s_y - 8.0, self.s_txt, YELLOW);
		}
	}
	
	fn isClicked(&self) -> bool {
		if self.btn_status == 2 {
			return true;
		} else {
			return false;
		}
	}
}



struct raquette {
	x: f32,
	y: f32,
	pl_ctrl: bool,
	/*
	pl_ctrl si activé capture la position de la sourie pour l'axe Y de la raquette
	*/
}

impl raquette {
	fn new(x: f32) -> Self {
		Self {
			x,
			y: 300.0,
			pl_ctrl: false,
		}
	}
	
	fn draw(&self) {
		draw_rectangle(self.x, self.y , 11.0, 45.0, WHITE);
	}
	
	fn enable_pl_ctrl(&mut self) {
		self.pl_ctrl = true;
	}
	
	fn update(&mut self) {
		let (mut mouse_x, mut mouse_y) = mouse_position();
		if self.pl_ctrl {
			self.y = mouse_y;
		}
	}
	
	fn set_y(&mut self, dat: f32) {
		self.y = dat;
	}
}

struct ball {
	x: f32,
	y: f32,
}

impl ball {
	fn new(x: f32, y: f32) -> Self {
		Self {
			x,
			y,
		}
	}
	
	fn draw(&self) {
		draw_circle(self.x, self.y, 15.0, YELLOW);
	}
}





#[macroquad::main(window_conf)]
async fn main() {
	//draw_line(140.0, 40.0, 400.0, 200.0, 15.0, BLUE);
	
	//vars
	//let mut scene: i32 = 2;
	let mut scene: i32 = 2;
	let (mut mouse_x, mut mouse_y) = mouse_position();
	
	//scene 2 moving txt data
	let m_txt_1_speed: f32 = 0.8;
	
	let mut m_txt_1_x: f32 = 200.0;
	let mut m_txt_1_y: f32 = 400.0;
	let mut m_txt_1_x_vel: f32 = m_txt_1_speed;
	let mut m_txt_1_y_vel: f32 = m_txt_1_speed;
	
	//scene 0 (error code)
	let mut error_msg = String::from("Aucune erreur");
	let mut btn_error = bouton::new(20.0, 540.0, 80.0, 40.0, String::from("MENU"), 40.0);
	
	
	//scene 2 (main menu)
	let mut btn_1 = bouton::new(0.0, 240.0, 100.0, 40.0, String::from("JOUER"), 40.0);
	let mut btn_2 = bouton::new(0.0, 280.0, 175.0, 40.0, String::from("parametres"), 40.0);
	
	//scene 3 data (jeux)
	let mut r1 = raquette::new(30.0);
	r1.enable_pl_ctrl();
	
	let mut r2 = raquette::new(759.0);
	
	let p1_score: i32 = 0;
	let p2_score: i32 = 0;
	
	let mut ball1 = ball::new(400.0, 300.0);
	
	
    loop {
		//code commun
		(mouse_x, mouse_y) = mouse_position();
		
		//scenes
		if scene == 2 {
			clear_background(BLACK);
			
			//btn update
			btn_1.update();
			btn_2.update();
			
			//draw
			draw_text(String::from("PONG 0.1"),m_txt_1_x, m_txt_1_y, 40.0, BLUE);
			
			btn_1.draw();
			btn_2.draw();
			
			//btn handle
			if btn_1.isClicked() {
				error_msg = String::from("non disponible (WIP)");
				scene = 0;
				//scene = 3;
			}
			
			/*
			if btn_2.isClicked() {
				error_msg = String::from("Parametres pas encore implémenté");
				scene = 0;
			}
			*/
			
			//m_txt_1 calculs
			if m_txt_1_y <= 17.0 {
				m_txt_1_y_vel = m_txt_1_y_vel * -1.0;
			}
			if m_txt_1_y >= 600.0 {
				m_txt_1_y_vel = m_txt_1_y_vel * -1.0;
			}
			
			if m_txt_1_x <= 0.0 {
				m_txt_1_x_vel = m_txt_1_x_vel * -1.0;
			}
			if m_txt_1_x >= 670.0 {
				m_txt_1_x_vel = m_txt_1_x_vel * -1.0;
			}
			
			m_txt_1_x = m_txt_1_x + m_txt_1_x_vel;
			m_txt_1_y = m_txt_1_y + m_txt_1_y_vel;
			
			
		}
		
		if scene == 3 {
			//scene du jeux principal (WIP)
			clear_background(BLACK);
			
			//terrain
			draw_rectangle(0.0, 0.0, 800.0, 30.0, WHITE);
			draw_rectangle(0.0, 570.0, 800.0, 30.0, WHITE);
			
			//raquette 1 
			r1.draw();
			r1.update();
			
			//raquette 2
			r2.draw();
			
			//ball
			ball1.draw();
			
			
		}
		
		
		
		
		if scene == 0 {
			clear_background(BLUE);
			draw_text("ERROR :", 20.0, 40.0, 30.0, WHITE);
			draw_text(error_msg.clone(), 20.0, 70.0, 30.0, WHITE);
			
			btn_error.update();
			btn_error.draw();
			
			if btn_error.isClicked() {
				scene = 2;
			}
			
		}
		
        next_frame().await
    }
}