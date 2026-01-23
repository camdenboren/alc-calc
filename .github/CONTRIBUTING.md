<!-- omit in toc -->

# Contributing to alc-calc

First off, thanks for taking the time to contribute!

All types of contributions are encouraged and valued. See the [Table of Contents](#table-of-contents) for different ways to help and details about how this project handles them. Please make sure to read the relevant section before making your contribution. It will make it a lot easier for us maintainers and smooth out the experience for all involved. The community looks forward to your contributions.

> And if you like the project, but just don't have time to contribute, that's fine. There are other easy ways to support the project and show your appreciation, which we would also be very happy about:
>
> - Star the project
> - Tweet about it
> - Refer this project in your project's readme
> - Mention the project at local meetups and tell your friends/colleagues

<!-- omit in toc -->

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [I Have a Question](#i-have-a-question)
- [I Want To Contribute](#i-want-to-contribute)
  - [Reporting Bugs](#reporting-bugs)
  - [Suggesting Enhancements](#suggesting-enhancements)
  - [Your First Code Contribution](#your-first-code-contribution)
  - [Build](#Build)
  - [Structure](#Structure)
- [Join The Project Team](#join-the-project-team)

## Code of Conduct

This project and everyone participating in it is governed by the
[alc-calc Code of Conduct](https://github.com/camdenboren/alc-calc/blob/main/.github/CODE_OF_CONDUCT.md).
By participating, you are expected to uphold this code. Please report unacceptable behavior
to [camdenboren](https://github.com/camdenboren/).

## I Have a Question

Before you ask a question, it is best to search for existing [Issues](https://github.com/camdenboren/alc-calc/issues) that might help you. In case you have found a suitable issue and still need clarification, you can write your question in this issue. It is also advisable to search the internet for answers first.

If you then still feel the need to ask a question and need clarification, we recommend the following:

- Open an [Issue](https://github.com/camdenboren/alc-calc/issues/new).
- Provide as much context as you can about what you're running into.
- Provide project and platform versions (Node.js, npm, etc.), depending on what seems relevant.

We will then take care of the issue as soon as possible.

<!--
You might want to create a separate issue tag for questions and include it in this description. People should then tag their issues accordingly.

Depending on how large the project is, you may want to outsource the questioning, e.g. to Stack Overflow or Gitter. You may add additional contact and information possibilities:
- IRC
- Slack
- Gitter
- Stack Overflow tag
- Blog
- FAQ
- Roadmap
- E-Mail List
- Forum
-->

## I Want to Contribute

> ### Legal Notice <!-- omit in toc -->
>
> When contributing to this project, you must agree that you have authored 100% of the content, that you have the necessary rights to the content and that the content you contribute may be provided under the project license.

### Reporting Bugs

<!-- omit in toc -->

#### Before Submitting a Bug Report

A good bug report shouldn't leave others needing to chase you up for more information. Therefore, we ask you to investigate carefully, collect information and describe the issue in detail in your report. Please complete the following steps in advance to help us fix any potential bug as fast as possible.

- Make sure that you are using the latest version.
- Determine if your bug is really a bug and not an error on your side e.g. using incompatible environment components/versions. If you are looking for support, you might want to check [this section](#i-have-a-question)).
- To see if other users have experienced (and potentially already solved) the same issue you are having, check if there is not already a bug report existing for your bug or error in the [bug tracker](https://github.com/camdenboren/alc-calc/issues?q=label%3Abug).
- Also make sure to search the internet (including Stack Overflow) to see if users outside of the GitHub community have discussed the issue.
- Collect information about the bug:
  - Stack trace (Traceback)
  - OS, Platform, and Version (Windows, Linux, macOS, x86, ARM)
  - Version of the interpreter, compiler, SDK, runtime environment, package manager, depending on what seems relevant.
  - Possibly your input and the output
  - Can you reliably reproduce the issue? And can you also reproduce it with older versions?

<!-- omit in toc -->

#### How Do I Submit a Good Bug Report?

> You must never report security related issues, vulnerabilities, or bugs including sensitive information to the issue tracker, or elsewhere in public. Instead sensitive bugs must be sent to [camdenboren](https://github.com/camdenboren/).

<!-- You may add a PGP key to allow the messages to be sent encrypted as well. -->

We use GitHub issues to track bugs and errors. If you run into an issue with the project:

- Open an [Issue](https://github.com/camdenboren/alc-calc/issues/new). (Since we can't be sure at this point whether it is a bug or not, we ask you not to talk about a bug yet and not to label the issue.)
- Explain the behavior you would expect and the actual behavior.
- Please provide as much context as possible and describe the _reproduction steps_ that someone else can follow to recreate the issue on their own. This usually includes your code. For good bug reports you should isolate the problem and create a reduced test case.
- Provide the information you collected in the previous section.

Once it's filed:

- The project team will label the issue accordingly.
- A team member will try to reproduce the issue with your provided steps. If there are no reproduction steps or no obvious way to reproduce the issue, the team will ask you for those steps and mark the issue as `needs-repro`. Bugs with the `needs-repro` tag will not be addressed until they are reproduced.
- If the team is able to reproduce the issue, it will be marked `needs-fix`, as well as possibly other tags (such as `critical`), and the issue will be left to be [implemented by someone](#your-first-code-contribution).

<!-- You might want to create an issue template for bugs and errors that can be used as a guide and that defines the structure of the information to be included. If you do so, reference it here in the description. -->

### Suggesting Enhancements

This section guides you through submitting an enhancement suggestion for alc-calc, **including completely new features and minor improvements to existing functionality**. Following these guidelines will help maintainers and the community to understand your suggestion and find related suggestions.

<!-- omit in toc -->

#### Before Submitting an Enhancement

- Make sure that you are using the latest version.
- Perform a [search](https://github.com/camdenboren/alc-calc/issues) to see if the enhancement has already been suggested. If it has, add a comment to the existing issue instead of opening a new one.
- Find out whether your idea fits with the scope and aims of the project. It's up to you to make a strong case to convince the project's developers of the merits of this feature. Keep in mind that we want features that will be useful to the majority of our users and not just a small subset. If you're just targeting a minority of users, consider writing an add-on or plugin library.

<!-- omit in toc -->

#### How Do I Submit a Good Enhancement Suggestion?

Enhancement suggestions are tracked as [GitHub issues](https://github.com/camdenboren/alc-calc/issues).

- Use a **clear and descriptive title** for the issue to identify the suggestion.
- Provide a **step-by-step description of the suggested enhancement** in as many details as possible.
- **Describe the current behavior** and **explain which behavior you expected to see instead** and why. At this point you can also tell which alternatives do not work for you.
- You may want to **include screenshots or screen recordings** which help you demonstrate the steps or point out the part which the suggestion is related to. You can use [LICEcap](https://www.cockos.com/licecap/) to record GIFs on macOS and Windows, and the built-in [screen recorder in GNOME](https://help.gnome.org/users/gnome-help/stable/screen-shot-record.html.en) or [SimpleScreenRecorder](https://github.com/MaartenBaert/ssr) on Linux. <!-- this should only be included if the project has a GUI -->
- **Explain why this enhancement would be useful** to most alc-calc users. You may also want to point out the other projects that solved it better and which could serve as inspiration.

<!-- You might want to create an issue template for enhancement suggestions that can be used as a guide and that defines the structure of the information to be included. If you do so, reference it here in the description. -->

### Your First Code Contribution

Before submitting a PR, ensure it's addressed by a [GitHub issue](https://github.com/camdenboren/alc-calc/issues). Once you're sure the item is addressed, follow these steps:

1.  [Fork this repository](https://github.com/camdenboren/alc-calc/fork)
2.  Check out the source code with:

    ```shell
    git clone https://github.com/camdenboren/alc-calc.git
    ```

3.  Start a new git branch with

    ```shell
    git checkout -b feature/your-feature
    ```

4.  Make desired changes
5.  Add relevant tests
6.  Make sure that your code is properly formatted `format`, that your branch builds `build`
7.  Finally, [create a pull request](https://help.github.com/articles/creating-a-pull-request). We'll then review and merge it

### Build

#### Nix

Cargo is the underlying build system, _but_ Nix is the 'meta' build system

For the uninitiated, Nix reproducibly supplies all project dependencies (including rust, Darwin SDKs, custom scripts, etc.) without the need for containers. From a practical standpoint, this means any Linux or macOS user can _reliably_ run this project with a single command

```shell
nix run github:camdenboren/alc-calc
```

The `build` and `format` scripts are very useful for any contributors (these aren't directly used in CI, but they smooth out the PR process)

After cloning, you can access the development environment (including these scripts) with

```shell
nix develop
```

#### App Bundles: Non-Windows

App bundles for Linux and macOS users **not** using Nix will also be provided on each release

You can generate these bundles manually by cloning and

> On Linux: adding system dependencies (tested on Ubuntu 24.04–don't use NixOS since it breaks the bundle for non-Nix users)

```shell
{
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
. "$HOME/.cargo/env"
sudo apt update
sudo apt install -y pkg-config libx11-dev libx11-xcb-dev libxkbcommon-x11-dev
cargo install cargo-bundle
}
```

> On macOS: preparing the signing certificate (tested on Sequoia v15.5)

- Download intermediate certificates: WWDR G2-G6 from [Certificate Authority]
- Create a dev certificate in Xcode
  - `XCode -> Settings -> Accounts -> Manage Certificates -> + -> Apple Development`
  - Right-Click Certificate -> `Export Certificate`
- Import the dev certificate in Keychain Access
  - `File -> Import Items...`
  - The 'Name' field of this cert will be the `$CERT_IDENTITY` in the next step (it can also be added to a `.env` file, which is automatically loaded via the bundle devShell)

Accessing the bundle devShell

```shell
nix develop .#bundle
```

> [!NOTE]
> The devShell is **not** required to build alc-calc, but is convenient if you're used to _the nix way_. On Linux, you'll just need to install boxes (v2.3.1) and set `export CUR_OS="linux"` to execute the following commands and the associated script. On macOS, you'll also need to install rustc + cargo (v1.91.1) and create-dmg (v1.2.2), follow the [macOS steps in the GPUI README], and set both `export CUR_OS="mac"` and `export CERT_IDENTITY="Apple Development: email (ID)"`

Then executing the script for your current OS

> For Nix

```shell
{
if [ $CUR_OS = "mac" ]; then
  cargo install cargo-bundle
  echo ""
fi
bundle-$CUR_OS
}
```

> For Non-Nix

```shell
{
if [ $CUR_OS = "mac" ]; then
  cargo install cargo-bundle
  echo ""
fi
chmod +x ./script/bundle-$CUR_OS
./script/bundle-$CUR_OS
}
```

<i>The bundle scripts are implemented sans-Nix since bundles created w/ cargo-bundle from nixpkgs link to dynamic libraries in `/nix/store/*`, breaking the bundle for non-Nix users</i>

#### App Bundles: Windows

Though cargo-bundle's Windows support is experimental (and broken for me), App bundles for Windows users will also be provided on each release via cargo-packager

You can generate these bundles manually by cloning, installing rustc + cargo (v1.91.1 msvc w/ W11 SDK) and boxes (v2.3.1), then installing cargo-packager and executing the Windows script

```powershell
cargo install cargo-packager; .\script\bundle-windows.ps1
```

#### Binary Cache

You can leverage the binary cache by adding [Garnix] to your nix-config

```nix
nix.settings.substituters = [ "https://cache.garnix.io" ];
nix.settings.trusted-public-keys = [ "cache.garnix.io:CTFPyKSLcx5RMJKfLo5EEPUObbA78b0YQ2DTCJXqr9g=" ];
```

#### Updating Dependencies

Update nixpkgs

```shell
nix flake update
```

Update each cargo dep to the most recent on [crates.io] via `cargo-edit`

> [!NOTE]
> You'll need to install `cargo-edit` (v0.13.8) first if you're not using Nix

```shell
cargo upgrade
```

Manually update `cargoHash` in [package.nix]

Then run the `build` and `format` scripts after fixing any breakage

```shell
{
build
format
}
```

### Structure

`src` contains two crates

- The binary crate (`main`) serves only as an entry point into the library crate, which contains the UI and calculation logic

- `ui` contains the main window, with individual views, components, and utilities delegated to other modules like `ui::comp::table`

## Join the Project Team

Requests to join the project team may be submitted to the responsible community leaders at [camdenboren](https://github.com/camdenboren/).

<!-- omit in toc -->

## Attribution

This guide is based on the **contributing-gen**. [Make your own](https://github.com/bttger/contributing-gen)!

[Certificate Authority]: https://www.apple.com/certificateauthority/
[macOS steps in the GPUI README]: https://github.com/zed-industries/zed/blob/main/crates/gpui/README.md#macos
[Garnix]: https://garnix.io/
[crates.io]: https://crates.io
[package.nix]: ../nix/package.nix
