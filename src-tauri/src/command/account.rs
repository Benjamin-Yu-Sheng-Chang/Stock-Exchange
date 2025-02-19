use crate::objects::account::Account;
use crate::objects::response::AccountResponse;
#[tauri::command]
pub fn create_account<'a>(name: String, balance: u64) -> Result<AccountResponse, String> {
    let account = Account::new(name, balance);
    // TODO: save account to database
    Ok(AccountResponse {
        id: account.id.to_string(),
        name: account.name.to_string(),
        balance: account.balance,
        occupied_balance: account.occupied_balance,
        created_at: account.created_at.to_rfc3339(),
    })
}

pub mod frontend {
    #[tauri::command]
    pub async fn create_account_window(app: tauri::AppHandle, account_id: String) {
        // TODO: check if account exists

        crate::command::utils::create_window(
            app,
            format!("account-{}", account_id),
            format!("/account/{}", account_id),
        )
        .await;
    }
}
