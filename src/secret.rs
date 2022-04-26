use ansi_term::Colour::{Blue, Purple};
use anyhow::{anyhow, Result};
use k8s_openapi::api::core::v1::Secret;
use k8s_openapi::{http, Metadata};
use krew_wasm_plugin_sdk::kube_config::ConnectionConfig;
use krew_wasm_plugin_sdk::wasi_outbound_http_helper_k8s::make_request;
use term_table::row::Row;
use term_table::table_cell::TableCell;
use x509_parser::certificate::X509Certificate;
use x509_parser::traits::FromDer;

use crate::print_cert;

pub(crate) fn decode(name: &str, namespace: &str, disable_cert_decoding: bool) -> Result<()> {
    let connection_config = ConnectionConfig::from_kube_config().map_err(|e| anyhow!("{:?}", e))?;
    let req_cfg_id = connection_config
        .register()
        .map_err(|e| anyhow!("{:?}", e))?;

    let secret =
        get_secret(&connection_config, &req_cfg_id, name, namespace)?.ok_or_else(|| {
            anyhow!(
                "Cannot find secret '{}' inside of namespace '{}'",
                name,
                namespace
            )
        })?;

    let mut table = term_table::Table::new();
    table.style = term_table::TableStyle::thin();

    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("Name:", 1, term_table::table_cell::Alignment::Left),
        TableCell::new_with_alignment(name, 1, term_table::table_cell::Alignment::Left),
    ]));
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("Namespace:", 1, term_table::table_cell::Alignment::Left),
        TableCell::new_with_alignment(namespace, 1, term_table::table_cell::Alignment::Left),
    ]));

    let labels: Vec<String> = match secret.metadata().labels.as_ref() {
        Some(labels) => labels
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect(),
        None => vec!["<none>".to_string()],
    };
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("Labels:", 1, term_table::table_cell::Alignment::Left),
        TableCell::new_with_alignment(
            labels.join("\n"),
            1,
            term_table::table_cell::Alignment::Left,
        ),
    ]));

    let annotations: Vec<String> = match secret.metadata().annotations.as_ref() {
        Some(annotations) => annotations
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect(),
        None => vec!["<none>".to_string()],
    };
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("Annotations:", 1, term_table::table_cell::Alignment::Left),
        TableCell::new_with_alignment(
            annotations.join("\n"),
            1,
            term_table::table_cell::Alignment::Left,
        ),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("Type:", 1, term_table::table_cell::Alignment::Left),
        TableCell::new_with_alignment(
            secret.type_.unwrap_or_else(|| "Unknown".to_string()),
            1,
            term_table::table_cell::Alignment::Left,
        ),
    ]));

    println!("{}", table.render());

    println!("{}", Purple.bold().paint("Data"));

    for (key, value) in secret.data.unwrap_or_default() {
        println!("\n{}:", Blue.bold().paint(key));

        let decoded_value_str = String::from_utf8(value.0.clone())
            .unwrap_or_else(|_| "The base64 decoded value of key '{}' is not UTF-8".to_string());

        if disable_cert_decoding {
            println!("{}", decoded_value_str);
        } else {
            let pem_data = match pem::parse(value.0) {
                Ok(data) => data,
                Err(_) => {
                    println!("{}", decoded_value_str);
                    continue;
                }
            };
            let cert_data: Option<(&[u8], X509Certificate)> = if pem_data.tag == "CERTIFICATE" {
                X509Certificate::from_der(pem_data.contents.as_slice())
                    .map_or_else(|_| None, |(d, c)| Some((d, c)))
            } else {
                None
            };
            match cert_data {
                None => println!("{}", decoded_value_str),
                Some((_, cert)) => print_cert::print_x509_info(&cert)?,
            }
        }
    }

    Ok(())
}

fn get_secret(
    connection_config: &ConnectionConfig,
    req_cfg_id: &str,
    name: &str,
    namespace: &str,
) -> Result<Option<Secret>> {
    let (k8s_req, response_body) =
        Secret::read_namespaced_secret(name, namespace, Default::default())?;

    let response =
        make_request(k8s_req, connection_config, req_cfg_id).map_err(|e| anyhow!("{:?}", e))?;

    // Got a status code from executing the request.
    let status_code = http::StatusCode::from_u16(response.status)?;
    if status_code == http::StatusCode::NOT_FOUND {
        return Ok(None);
    }

    // Construct the `ResponseBody<ListResponse<Event>>` using the
    // constructor returned by the API function.
    let mut response_body = response_body(status_code);

    response_body.append_slice(
        response
            .body
            .ok_or_else(|| anyhow!("no response body"))?
            .as_slice(),
    );
    let response = response_body.parse();

    let secret = match response {
        // Successful response (HTTP 200 and parsed successfully)
        Ok(k8s_openapi::api::core::v1::ReadNamespacedSecretResponse::Ok(secret)) => Some(secret),

        // Some unexpected response
        // (not HTTP 200, but still parsed successfully)
        Ok(other) => return Err(anyhow!("expected Ok but got {} {:?}", status_code, other)),

        // Some other error, like the response body being
        // malformed JSON or invalid UTF-8.
        Err(err) => return Err(anyhow!("error: {} {:?}", status_code, err)),
    };

    Ok(secret)
}
