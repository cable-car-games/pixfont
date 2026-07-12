# Creating a PixFont Studio release

When it's time for a new release, you will need to create a `release/v0.1`
branch and update the `RELEASE-NOTES.md` file.

```bash
git switch main
git pull --rebase
git switch -c release/v0.1
```

The release notes file will contain the content posted in the GitHub Release,
and will be appended to the release announcement on itch.io.

Each push to the `release/v0.1` branch will create "release candidate" artefacts
to test and validate the release before cutting a tag.

Once the branch is created, the only changes to be made should be bug fixes.

After creating the release branch, switch back to `main` and bump the versions
to the next minor version.

## Publishing the release

A release is published by creating a release tag (`v0.1.0`).

```bash
git switch release/v0.1
git pull --rebase
git tag v0.1.0
git push origin v0.1.0
```

Once pushed, the CI will build, then publish the release to GitHub Releases and
itch.io.

While the CI runs, update the itch.io page and draft the release post.

> [!NOTE]
> Currently, the release is created as a draft while fine-tuning the release
> process.
>
> You will need to go into GitHub Releases and publish it from there.
>
> itch.io releases are manual for now, but should be in place by the time
> v0.2 is ready to be created.

After the release is pushed and all the announcements are published, bump the
versions to the next patch version.
