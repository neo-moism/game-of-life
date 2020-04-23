extern crate serde;
use actix_web::{web, App, HttpServer, Responder};
use std::sync::Mutex;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    let addr = args.get(1).expect("addr missing");
    let app_state = web::Data::new(CurrentState {
        states: Mutex::new(vec![]),
    });
    let _ = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/step_cells", web::post().to(refresh_cells))
            .route("/step_digit_cells", web::post().to(refresh_digit_cells))
            .route("/next", web::get().to(next_state))
            .route("/current_state", web::get().to(current_state))
    })
    .bind(addr)?
    .run()
    .await;
    Ok(())
}

struct CurrentState {
    states: Mutex<Vec<Vec<bool>>>,
}

impl ToString for CurrentState {
    fn to_string(&self) -> String {
        self.states
            .lock()
            .unwrap()
            .iter()
            .map(|line| {
                line.iter()
                    .map(|alive| if *alive { 'o' } else { ' ' })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

async fn next_state(data: web::Data<CurrentState>) -> impl Responder {
    let cp = data.states.lock().unwrap().clone();
    refresh_cells(web::Json(cp), data).await
}

async fn current_state(data: web::Data<CurrentState>) -> impl Responder {
    data.get_ref().to_string()
}

async fn refresh_digit_cells(
    req: web::Json<Vec<Vec<i32>>>,
    data: web::Data<CurrentState>,
) -> impl Responder {
    let xx = req
        .clone()
        .into_iter()
        .map(|line| line.into_iter().map(|v| v == 1).collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();
    refresh_cells(web::Json(xx), data).await
}

async fn refresh_cells(
    req: web::Json<Vec<Vec<bool>>>,
    data: web::Data<CurrentState>,
) -> impl Responder {
    let mut cells = req.clone();
    for (i, line) in cells.iter_mut().enumerate() {
        let l = next_state_of_line(&req.0, i);
        *line = l;
    }
    {
        let mut s = data.states.lock().unwrap();
        *s = cells;
    }
    data.to_string()
}

// 每个细胞有两种状态 - 存活或死亡，
// 每个细胞与以自身为中心的周围八格细胞产生互动
// 当前细胞为存活状态时，当周围的存活细胞低于2个时（不包含2个），该细胞变成死亡状态。（模拟生命数量稀少）
// 当前细胞为存活状态时，当周围有2个或3个存活细胞时，该细胞保持原样。
// 当前细胞为存活状态时，当周围有超过3个存活细胞时，该细胞变成死亡状态。（模拟生命数量过多）
// 当前细胞为死亡状态时，当周围有3个存活细胞时，该细胞变成存活状态。（模拟繁殖）
fn next_state_of_line(data: &[Vec<bool>], row: usize) -> Vec<bool> {
    let mut line_info = Vec::with_capacity(data[row].len());
    for i in 0..data[row].len() {
        let mut alive_count = 0;
        if row > 0 {
            let prev_line = data.get(row - 1).unwrap();
            if i > 0 && prev_line[i - 1] {
                alive_count += 1;
            }
            if prev_line[i] {
                alive_count += 1;
            }
            if *prev_line.get(i + 1).unwrap_or(&false) {
                alive_count += 1;
            }
        }
        if i > 0 && data[row][i - 1] {
            alive_count += 1;
        }
        if *data[row].get(i + 1).unwrap_or(&false) {
            alive_count += 1;
        }
        if let Some(next_line) = data.get(row + 1) {
            if i > 0 && next_line[i - 1] {
                alive_count += 1;
            }
            if next_line[i] {
                alive_count += 1;
            }
            if *next_line.get(i + 1).unwrap_or(&false) {
                alive_count += 1;
            }
        }
        line_info.push((data[row][i], alive_count));
    }
    line_info
        .into_iter()
        .map(|(alive, cnt)| {
            if alive {
                match cnt {
                    0 | 1 => false,
                    2 | 3 => alive,
                    _ => false,
                }
            } else {
                cnt == 3
            }
        })
        .collect()
}
