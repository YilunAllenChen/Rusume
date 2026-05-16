use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use web_sys::{window, UrlSearchParams};

use super::basic::Basic;
use super::education::Education;
use super::experiences::Experience;
use super::experiences::RolePeriod;
use super::open_source::OpenSource;
use super::projects::Project;
use super::skills::SkillCategory;

pub const STATE_VERSION: u8 = 1;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppState {
    #[serde(default = "default_state_version")]
    pub v: u8,
    #[serde(default)]
    pub basic: Basic,
    #[serde(default)]
    pub educations: Vec<Education>,
    #[serde(default)]
    pub skills: Vec<SkillCategory>,
    #[serde(default)]
    pub experiences: Vec<Experience>,
    #[serde(default)]
    pub projects: Vec<Project>,
    #[serde(default)]
    pub open_sources: Vec<OpenSource>,
}

fn default_state_version() -> u8 {
    STATE_VERSION
}

pub fn default_seed_state() -> AppState {
    AppState {
        v: STATE_VERSION,
        basic: Basic {
            name: "Yilun \"Allen\" Chen".to_string(),
            email: "allenchenyilun1999@gmail.com".to_string(),
            phone: "404-409-9683".to_string(),
            linkedin_url: Some("https://www.linkedin.com/in/yilun-allen-chen-572b71141/".to_string()),
            github_url: Some("https://github.com/YilunAllenChen".to_string()),
        },
        educations: vec![
            Education {
                school: "Georgia Tech".to_string(),
                degree: "M.S.".to_string(),
                major: "Computer Engineering".to_string(),
                start_date: "2021/08".to_string(),
                end_date: "2022/05".to_string(),
                gpa: "3.87 / 4.0".to_string(),
            },
            Education {
                school: "Georgia Tech".to_string(),
                degree: "B.S.".to_string(),
                major: "Computer Engineering".to_string(),
                start_date: "2017/08".to_string(),
                end_date: "2021/05".to_string(),
                gpa: "3.86 / 4.0".to_string(),
            },
        ],
        skills: vec![],
        experiences: vec![
            Experience {
                employer: "DRW".to_string(),
                team: Some("Cumberland / FICC Options".to_string()),
                title: "Head of Tools Engineering".to_string(),
                dates: "2025.01 - Present".to_string(),
                roles: vec![RolePeriod {
                    title: "Head of Tools Engineering".to_string(),
                    dates: "2025.01 - Present".to_string(),
                    location: "Chicago, IL".to_string(),
                }],
                location: "Chicago, IL".to_string(),
                description: [
                    "Lead, manage, and grow a **global team from 4 to 10 quantitative developers**, servicing 10 market making and 8 prop desks.",
                    "Standardize, document and optimize team operations, boosting engineer productivity by lowering team support burden by **80%**.",
                    "**Allocate team resources** between strategic initiatives and tech debt elimination, deprecated 95% legacy pipelines.",
                    "Own an option & portfolio analytics platform, cutting down trader idea-to-prototype turnaround time by 10x.",
                    "Migrated group toolchain to uv/ruff/ty, cutting CI/lint time by 90% and improving iteration speed for 40 engineers across 3 teams",
                    "**Stack**: Python, Java, C++, Kafka, DeltaLake, FastAPI, Plotly Dash, Kubernetes",
                ]
                .join("\n"),
            },
            Experience {
                employer: "DRW".to_string(),
                team: Some("FICC Options".to_string()),
                title: "Software Engineer, Senior Software Engineer".to_string(),
                dates: "2022.07 - 2025.01".to_string(),
                roles: vec![
                    RolePeriod {
                        title: "Senior Software Engineer (promoted from SDE, Jul 2024)".to_string(),
                        dates: "2022.07 - 2025.01".to_string(),
                        location: "Chicago, IL".to_string(),
                    },
                ],
                location: "Chicago, IL".to_string(),
                description: [
                    "Own, develop and manage the **streaming data platform** for both historical and live use cases, used by >200 systems worldwide.",
                    "Maintain a generic **stream-processing** system to ingest, clean, transform and aggregate data following the Medallion architecture.",
                    "Pioneered the implementation of a **bitemporal** binary streaming protocol that focuses on efficiency and the ability to time travel.",
                    "Spearhead the design and development of core option pricing datasets & processes with **>$10M/yr** estimated materiality.",
                    "Rearchitected a volatility dynamics computation & visualization pipeline, making it **8x** faster and **10x** more resource efficient.",
                    "Coordinated collaboration across 3 teams and 10+ engineers to integrate exotic options pricing & risks into existing trading systems.",
                    "**Stack**: Python, Rust, Java, Kafka, DeltaLake, gRPC/Protobuf, Arrow, DuckDB, PostgreSQL", ]
                .join("\n"),
            },
            Experience {
                employer: "DRW".to_string(),
                team: Some("FICC Options".to_string()),
                title: "Software Developer Intern".to_string(),
                dates: "2021.06 - 2021.08".to_string(),
                roles: vec![
                    RolePeriod {
                        title: "Software Developer Intern".to_string(),
                        dates: "2021.06 - 2021.08".to_string(),
                        location: "Chicago, IL".to_string(),
                    },
                ],
                location: "Chicago, IL".to_string(),
                description: [
                    "Built a streaming data system allowing access to live and historical data with the same API and onboarded 4 production datasets.",
                    "Designed and built a dashboard to monitor and manage it, speeding up onboarding by 5x",
                    "**Stack**: Python, Kafka, Presto/Trino",
                ]
                .join("\n"),
            },
            Experience {
                employer: "Uber Advanced Technology Group".to_string(),
                team: Some("Platform Validation".to_string()),
                title: "Software Engineering Intern".to_string(),
                dates: "2020.05 - 2020.07".to_string(),
                roles: vec![RolePeriod {
                    title: "Software Engineering Intern".to_string(),
                    dates: "2020.05 - 2020.07".to_string(),
                    location: "Atlanta, GA".to_string(),
                }],
                location: "Atlanta, GA".to_string(),
                description: [
                    "Rearchitected a fleet orchestration system for autonomous vehicles for field tests, improving availability by 7x",
                    "Built slack integration with access gating for the system for non-technical staff",
                    "**Stack**: Python (asyncio), PostgreSQL",
                ]
                .join("\n"),
            },
            Experience {
                employer: "Georgia Institute of Technology".to_string(),
                team: Some("GRITS Lab".to_string()),
                title: "Robotics Research Assistant".to_string(),
                dates: "2019.05 - 2022.05".to_string(),
                roles: vec![RolePeriod {
                    title: "Robotics Research Assistant".to_string(),
                    dates: "2019.05 - 2022.05".to_string(),
                    location: "Atlanta, GA".to_string(),
                }],
                location: "Atlanta, GA".to_string(),
                description: [
                    "Designed and built hardware, firmware and software for various robotics systems. Examples: [Slothbot](https://atlantabg.org/conservation-research/look-up-its-slothbot/), [Brushbot v2](https://github.com/YilunAllenChen/BrushBot)",
                    "Revamped and optimized a giga-scale urban traffic simulation testbed that backs [the GreenEVT paper](https://arxiv.org/pdf/2305.12722)",
                    "**Stack**: Python, ROS, Gazebo, Embedded C++, Autodesk Eagle",
                ].join("\n"),
            }
        ],
        projects: vec![
            Project {
                name: "Incrementars".to_string(),
                description: "Adapton-like incremental / self-adapting computing framework for Rust.".to_string(),
                technologies: "Rust".to_string(),
                url: Some("https://github.com/YilunAllenChen/incrementars".to_string()),
            },
            Project {
                name: "Rusume".to_string(),
                description: "WASM resume builder with live preview".to_string(),
                technologies: "Rust".to_string(),
                url: Some("https://yilunallenchen.github.io/Rusume/#/".to_string()),
            },
            Project {
                name: "Exchange Simulator".to_string(),
                description: "FIFO matching engine game with bots, news feeds and leaderboard.".to_string(),
                technologies: "Python, Rust, TypeScript".to_string(),
                url: Some("https://tradingsim.allenchen.dev/".to_string()),
            },
            Project {
                name: "Iterr".to_string(),
                description:
                    "Rust-style iterator pattern in python: lazy, minimal, and type safe all the way.".to_string(),
                technologies: "Python".to_string(),
                url: Some("https://github.com/YilunAllenChen/iterr".to_string()),
            },
            Project {
                name: "DaVinci Ergo Lab".to_string(),
                description: "Affordable ergonomic split mechanical keyboards (30 customers)".to_string(),
                technologies: "Python, C++".to_string(),
                url: Some("https://davinci-ergo-lab.com/".to_string()),
            },
        ],
        open_sources: vec![
            OpenSource {
                name: "kafka-rust #222: seek".to_string(),
                url: "https://github.com/kafka-rust/kafka-rust/pull/222".to_string(),
            },
            OpenSource {
                name: "kafka-rust #223: list offsets".to_string(),
                url: "https://github.com/kafka-rust/kafka-rust/pull/223".to_string(),
            },
            OpenSource {
                name: "gleam-stdlib #769: list.max".to_string(),
                url: "https://github.com/gleam-lang/stdlib/pull/769".to_string(),
            },
            OpenSource {
                name: "ruff #23643: completion ranking".to_string(),
                url: "https://github.com/astral-sh/ruff/pull/23643".to_string(),
            },
        ],
    }
}

pub fn encode_state(state: &AppState) -> Result<String, String> {
    let json = serde_json::to_vec(state).map_err(|e| e.to_string())?;
    Ok(URL_SAFE_NO_PAD.encode(json))
}

pub fn decode_state(encoded: &str) -> Result<AppState, String> {
    let bytes = URL_SAFE_NO_PAD.decode(encoded).map_err(|e| e.to_string())?;
    let state: AppState = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    if state.v != STATE_VERSION {
        return Err(format!(
            "unsupported state version: {}, expected {}",
            state.v, STATE_VERSION
        ));
    }
    Ok(state)
}

pub fn load_state_from_url() -> AppState {
    let Some(window) = window() else {
        return default_seed_state();
    };

    let Ok(search) = window.location().search() else {
        return default_seed_state();
    };

    let Ok(params) = UrlSearchParams::new_with_str(&search) else {
        return default_seed_state();
    };

    let Some(raw) = params.get("state") else {
        return default_seed_state();
    };

    decode_state(&raw).unwrap_or_else(|_| default_seed_state())
}

pub fn write_state_to_url(state: &AppState) {
    let Some(window) = window() else {
        return;
    };

    let Ok(encoded_state) = encode_state(state) else {
        return;
    };

    let Ok(history) = window.history() else {
        return;
    };

    let location = window.location();
    let pathname = location.pathname().unwrap_or_else(|_| "/".to_string());
    let hash = location.hash().unwrap_or_default();
    let new_url = format!("{}?state={}{}", pathname, encoded_state, hash);
    let _ = history.replace_state_with_url(&JsValue::NULL, "", Some(&new_url));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_roundtrip_encode_decode() {
        let state = default_seed_state();
        let encoded = encode_state(&state).expect("state should encode");
        let decoded = decode_state(&encoded).expect("state should decode");
        assert_eq!(state, decoded);
    }

    #[test]
    fn decode_fails_for_malformed_state() {
        let result = decode_state("not-valid-base64");
        assert!(result.is_err());
    }

    #[test]
    fn decode_fails_for_wrong_version() {
        let mut state = default_seed_state();
        state.v = 99;
        let encoded = encode_state(&state).expect("state should encode");
        let result = decode_state(&encoded);
        assert!(result.is_err());
    }
}
