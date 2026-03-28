use askama::Template;
use askama_web::WebTemplate;
use axum::{Router, routing::get};
use tower_http::services::ServeDir;

struct Education {
    degree: &'static str,
    institution: &'static str,
    detail: &'static str,
}

struct Experience {
    role: &'static str,
    org: &'static str,
    period: &'static str,
}

struct Publication {
    authors: &'static str,
    year: &'static str,
    title: &'static str,
    venue: &'static str,
}

#[derive(Template, WebTemplate)]
#[template(path = "index.html")]
struct IndexTemplate {
    education: Vec<Education>,
    experience: Vec<Experience>,
    publications: Vec<Publication>,
}

async fn index() -> IndexTemplate {
    IndexTemplate {
        education: vec![
            Education {
                degree: "Ph.D., Information Technology",
                institution: "University of Cincinnati",
                detail: "Dissertation: Policing in the AI Era: A Mixed-Methods Analysis of Police Adoption of Information Technology",
            },
            Education {
                degree: "M.S., Criminal Justice",
                institution: "University of Cincinnati",
                detail: "Specialization in Data-Driven Public Safety and Technology Adoption",
            },
            Education {
                degree: "B.S., Criminal Justice",
                institution: "Xavier University",
                detail: "Cum Laude, Minor in Political Science",
            },
        ],
        experience: vec![
            Experience {
                role: "Sr. Manager, Corporate AI Solutions",
                org: "Axon Enterprise, Inc.",
                period: "2024 – Present",
            },
            Experience {
                role: "Adjunct Assistant Professor",
                org: "University of Cincinnati, School of Information Technology",
                period: "2025 – Present",
            },
            Experience {
                role: "Assistant Professor",
                org: "University of Cincinnati, School of Information Technology",
                period: "2024",
            },
            Experience {
                role: "Public Safety Innovation Technology Manager",
                org: "City of Boulder, Department of Innovation and Technology",
                period: "2022 – 2024",
            },
            Experience {
                role: "Adjunct Faculty",
                org: "University of Cincinnati, School of Information Technology",
                period: "2022",
            },
            Experience {
                role: "Crime Analyst and Technical Lead",
                org: "University of Cincinnati, Division of Public Safety",
                period: "2018 – 2022",
            },
            Experience {
                role: "Adjunct Faculty",
                org: "Murray State University",
                period: "2018 – 2019",
            },
            Experience {
                role: "Crime Analyst",
                org: "Paducah Police Department",
                period: "2015 – 2022",
            },
        ],
        publications: vec![
            Publication {
                authors: "Zidar, M.",
                year: "2023",
                title: "Policing in the AI Era: A Mixed-Methods Analysis of Police Adoption of Information Technology",
                venue: "Doctoral dissertation, University of Cincinnati",
            },
            Publication {
                authors: "Feldmeyer, B., Cullen, F. T., Sun, D., Kulig, T. C., Chouhy, C., & Zidar, M.",
                year: "2022",
                title: "The community determinants of death: comparing the macro-level predictors of overdose, homicide, and suicide deaths, 2000 to 2015",
                venue: "Socius, 8",
            },
            Publication {
                authors: "Said, H., Zidar, M., Varlioglu, S., & Itodo, C.",
                year: "2021",
                title: "A Framework for the Discipline of Information Technology",
                venue: "Proceedings of the 22nd Annual Conference on Information Technology Education",
            },
            Publication {
                authors: "Ozer, M., Zidar, M., Deryol, R., Varlioglu, S., Eldivan, I. S., & Akbas, H.",
                year: "2020",
                title: "Creating A Real-Time Geocoding System: Implications of Open Source for Public Safety",
                venue: "2020 International Conference on Computational Science and Computational Intelligence (CSCI)",
            },
            Publication {
                authors: "Zidar, M. S., Herold, T. D., & Eck, J. E.",
                year: "2019",
                title: "Superusers of Small-Town Police Resources",
                venue: "The Police Chief, 86(10)",
            },
            Publication {
                authors: "White, D. R., Hepworth, D. P., & Zidar, M. S.",
                year: "2018",
                title: "Texting and Driving: Is It Just Moral Panic?",
                venue: "Deviant Behavior, 39(11)",
            },
            Publication {
                authors: "Zidar, M. S., Shafer, J. G., & Eck, J. E.",
                year: "2017",
                title: "Reframing an Obvious Police Problem: Discovery, Analysis, and Response to a Manufactured Problem in a Small City",
                venue: "Policing: A Journal of Policy and Practice",
            },
            Publication {
                authors: "White, D. R, Kyle, M. J., Galli, P., & Zidar, M. S.",
                year: "2017",
                title: "Youth Attitudes Regarding Police Effectiveness and Trust in One Midsize City",
                venue: "The Police Chief, 84(10)",
            },
        ],
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind to port 3000");

    println!("Listening on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .expect("server error");
}
