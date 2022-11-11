use serde::Serialize;

use util_auth::Claims;

#[derive(Serialize, Debug)]
pub struct DashboardView {
    pub(crate) claims: Claims,
}
