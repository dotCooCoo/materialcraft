use egui::vec2;
use egui_kittest::{Harness, kittest::Queryable};
use materialcraft_ui_egui::{MaterialCraftApp, UiState};
use serde_json::json;
use std::time::Duration;

fn app() -> MaterialCraftApp {
    let mut app = MaterialCraftApp::headless(materialcraft_engine::new_session(), UiState::default());
    let node = app.run("graph.node.add", json!({"def": "uniform", "params": {"color": [0.5, 0.75, 1.0, 1.0]}})).unwrap()["id"].as_u64().unwrap();
    app.run("graph.node.add", json!({"def": "output", "connectFrom": {"node": node}, "params": {"usage": 1}})).unwrap();
    app
}

fn settle(harness: &mut Harness<'static, MaterialCraftApp>, after: u64) {
    const RENDER_ATTEMPTS: usize = 300;
    const POLL_INTERVAL: Duration = Duration::from_millis(10);
    for _ in 0..RENDER_ATTEMPTS {
        harness.step();
        let app = harness.state();
        if app.view3d.renders > after && !app.session.dirty && !app.cooker.busy() && !app.view3d.busy() {
            return;
        }

        std::thread::sleep(POLL_INTERVAL);
    }
    panic!("the normal-map preview did not finish rendering");
}

fn toggle(harness: &mut Harness<'static, MaterialCraftApp>) {
    harness.get_by_label("Display").click();
    harness.run_steps(4);
    harness.get_by_label("Flip normal map Y (DirectX)").click();
    harness.run_steps(4);
}

#[test]
#[ignore = "requires an offscreen GPU; run explicitly with --ignored"]
fn normal_convention_snapshot() {
    let mut harness = Harness::builder().with_size(vec2(1400.0, 860.0)).wgpu().build_ui_state(
        |ui, app: &mut MaterialCraftApp| {
            app.step(&ui.ctx().clone());
            app.draw(ui);
        },
        app(),
    );
    settle(&mut harness, 0);
    let rendered = harness.state().view3d.renders;
    toggle(&mut harness);
    settle(&mut harness, rendered);
    harness.get_by_label("Display").click();
    harness.run_steps(4);

    assert!(!harness.state().session.view3d.normal_y_flip);

    let snapshot_dir = std::path::PathBuf::from(std::env::var_os("MC_SNAPSHOT_DIR").expect("MC_SNAPSHOT_DIR"));
    std::fs::create_dir_all(&snapshot_dir).unwrap();
    harness.render().unwrap().save(snapshot_dir.join("normal-convention.png")).unwrap();
}
