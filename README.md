# Setup
The setup can be broken down into a few steps:
1. Creating a Monzo OAuth application
2. Running the application
3. Authorizing your Monzo account

## Creating a Monzo OAuth application
Head over the the [Monzo developer portal](https://developers.monzo.com) and sign in.
You'll receive an email from Monzo with a link to click to authenticate you.

Press clients in the navbar. You should be taken to a page like this:

![Clients page](/docs/images/clients_page.png?raw=true)

Press the `New OAuth Client` button and fill out the form, ensuring:
1. Confidentiality is set to `Confidential`: this means that we can refresh our access without requiring user action
2. The redirect URL is set to the URL the exporter will be running at + `/callback`,
for example `http://127.0.0.1:8080/callback`

![Clients creation](/docs/images/client_creation.png?raw=true)

## Running the application
### Building
Simply run `cargo build --release`, or download a pre-build binary from the [releases page](https://github.com/RyanW02/monzo-exporter/releases)

If you do not have cargo installed, you can follow the [book](https://doc.rust-lang.org/cargo/getting-started/installation.html) on how to do so.

### Configuration
Create a `config.toml` file in the working directory the application will be run from (this is usually the same as the directory the binary is located in), completing it as follows:

```toml
account_id = "acc_xxxxxxxxxxxxxxxxxxxxxx"
client_id = "oauth2client_xxxxxxxxxxxxxxxxxxxxxx"
client_secret = "mnzconf.xxxxxx/xxxxxx/xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
server_addr = "0.0.0.0:8080"
redirect_uri = "http://127.0.0.1:8080"
auth_key = "$Kq1wJ@HF18yN^4I"
```

**account_id**: Retrieve from the [Monzo Developer playground](https://developers.monzo.com/api/playground):
Ensure you've approved the login request in your Monzo app, or the account ID will not show up.

**client_id** & **client_secret**: Retrieve from your [OAuth client management page](https://developers.monzo.com/apps/home):

![Clients keys](/docs/images/client_keys.png?raw=true)

**server_addr**: This will be the address that the web server binds to to server the statistics to Prometheus,
and also to redirect you to the login page.

**redirect_uri**: The URL that you will be redirected back to after authorizing, excluding `/callback`.
Cannot be `0.0.0.0` since your browser must navigate to it.

**auth_key**: Key required to authorize your Monzo account to prevent other users from logging in to theirs on your exporter.
Change from the default key provided.

### Running & Authorizing Your Monzo Account
Simply execute your binary:

```bash
$ ./monzo-exporter
Error reading tokens from disk (IOError(Os { code: 2, kind: NotFound, message: "No such file or directory" })).
This just means you haven't logged in yet: Visit http://127.0.0.1:8080/authorize?key=$Kq1wJ@HF18yN^4I to do so
```

You'll be warned that the exporter was unable to read the `tokens.json` file on the first run:
This is normal and just means you haven't authenticated yet. Visit the link displayed to you to do so.

You'll be redirected to Monzo's website. Enter your email address in the form shown:

![Login form](/docs/images/login_form.png?raw=true)

Monzo will then email you a link to click to authorize your exporter.

![Login email](/docs/images/login_email.png?raw=true)

Upon clicking the link, you'll see `Received new tokens` logged to your console.

You'll also see``Error retrieving balance: HTTPError(reqwest::Error { kind: Decode, source: Error("missing field `balance`", line: 1, column: 207) })``
printed every 15 seconds until you authorize the application in your Monzo app.

Upon the Monzo app on your smartphone and you should see an alert saying "Allow access to your data".
Click this, and click approve in the next dialog. You'll be asked to enter your PIN code.

![Mobile approve](/docs/images/login_email.png?raw=true)

You'll need to re-complete the authorization process every 90 days, as Monzo doesn't let us keep refreshing access tokens
for longer than that.

The error should stop being printed, and your the exporter will start serving Prometheus compatible statistics at the
`/stats` endpoint:

![Stats](/docs/images/stats.png?raw=true)

(Actual balance censored for obvious reasons)

You can now configure Prometheus to scrape from the exporter in `/etc/prometheus/prometheus.yml` like so:

```yaml
scrape_configs:
  - job_name: 'monzo'
    scheme: 'http'
    metrics_path: '/monzo'
    static_configs:
    - targets:
      - 'xx.xx.xx.xx'
```

And we're done!