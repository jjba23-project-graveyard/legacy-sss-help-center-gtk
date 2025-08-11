// sss-help

// Copyright © Josep Bigorra <jjbigorra@gmail.com>

// sss-help is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// sss-help is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with sss-help.  If not, see <https://www.gnu.org/licenses/>.

use serde_json::Value;
use std::fmt;
use std::process::Command;

#[derive(Debug)]
pub enum KeybindingError {
    DconfCommand(std::io::Error),
    DconfFailed(String),
    Base16(base16ct::Error),
    Utf8(std::string::FromUtf8Error),
    Json(serde_json::Error),
}

impl fmt::Display for KeybindingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DconfCommand(e) => write!(f, "Failed to execute dconf command: {}", e),
            Self::DconfFailed(stderr) => write!(f, "dconf command failed: {}", stderr),
            Self::Base16(e) => write!(f, "Failed to decode Base16 string: {}", e),
            Self::Utf8(e) => write!(f, "Failed to parse UTF-8 string: {}", e),
            Self::Json(e) => write!(f, "Failed to parse JSON: {}", e),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SystemKeybindings {
    pub power_user_bindings: Value,
    pub universal_session_bindings: Value,
}

fn parse_bindings(
    command_output: Result<std::process::Output, std::io::Error>,
) -> Result<Value, KeybindingError> {
    let output = command_output.map_err(KeybindingError::DconfCommand)?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(KeybindingError::DconfFailed(stderr));
    }

    let raw_stdout = String::from_utf8(output.stdout).map_err(KeybindingError::Utf8)?;

    // dconf wraps strings in single quotes, remove them
    let base16_encoded = raw_stdout.trim().trim_matches('\'');

    // Decode from Base16, parse to UTF-8, then parse as JSON
    let mut buf = [0u8; 16];
    let decoded_bytes =
        base16ct::lower::decode(base16_encoded, &mut buf).map_err(KeybindingError::Base16)?;
    let json_string = String::from_utf8(decoded_bytes.to_vec()).map_err(KeybindingError::Utf8)?;
    let parsed_json = serde_json::from_str(&json_string).map_err(KeybindingError::Json)?;

    Ok(parsed_json)
}

fn fetch_single_binding(dconf_key: &str) -> Result<Value, KeybindingError> {
    let command_output = Command::new("dconf").arg("read").arg(dconf_key).output();
    parse_bindings(command_output)
}

pub fn fetch_all_binding_data() -> Result<SystemKeybindings, KeybindingError> {
    let power_user_bindings = fetch_single_binding("/sss/power-user-keybindings-docs")?;
    let universal_session_bindings =
        fetch_single_binding("/sss/universal-session-keybindings-docs")?;

    Ok(SystemKeybindings {
        power_user_bindings,
        universal_session_bindings,
    })
}
