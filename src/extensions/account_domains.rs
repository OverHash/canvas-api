use serde::Deserialize;

use crate::CanvasClient;

#[derive(Deserialize, Debug)]
pub struct AccountDomain {
    /// The name of the domain.
    name: String,
    /// The domain.
    domain: String,
    /// Which authentication_provider param to pass to the oauth flow.
    authentication_provider: Option<String>,
}

impl AccountDomain {
    /// The name of the domain.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The domain.
    pub fn domain(&self) -> &str {
        &self.domain
    }

    /// Which authentication_provider param to pass to the oauth flow.
    pub fn authentication_provider(&self) -> Option<&String> {
        self.authentication_provider.as_ref()
    }
}

pub struct AccountDomainSearch<'a> {
    client: &'a CanvasClient,

    name: Option<String>,
    domain: Option<String>,
}

impl<'a> AccountDomainSearch<'a> {
    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name;
    }

    pub fn set_domain(&mut self, domain: Option<String>) {
        self.domain = domain;
    }

    pub async fn search(self) -> Result<Vec<AccountDomain>, crate::Error> {
        let res: Vec<AccountDomain> = self
            .client
            .make_query("v1/accounts/search")
            .query(&[("domain", self.domain)])
            .query(&[("name", self.name)])
            .send()
            .await?
            .json()
            .await?;

        Ok(res)
    }
}

pub trait AccountDomainsExt {
    /// Returns a list of up to 5 matching account domains.
    ///
    /// The returned result can be further refined by using the
    /// [`AccountDomainSearch::set_name`] and
    /// [`AccountDomainSearch::set_domain`] options.
    ///
    /// # Example
    /// ```rs
    /// let account_domains = canvas_client
    ///     .search_account_domains()
    ///     .set_name(Some(String::from("utah")))
    ///     .search()
    ///     .await?;
    /// ```
    fn search_account_domains(&self) -> AccountDomainSearch;
}

impl AccountDomainsExt for CanvasClient {
    fn search_account_domains(&self) -> AccountDomainSearch {
        AccountDomainSearch {
            client: self,
            domain: None,
            name: None,
        }
    }
}
