# boats's personal barricade

This is a tool to automatically sign git commits, replacing gpg for that
purpose. It is very opinionated, and only useful if you use gpg the same way I
do.

# How to install

```
cargo install bpb
```

# How to set up

Once you've installed this program, you should run the `bpb init` subcommand.
This command expects you to pass a userid argument. For example, this is how I
would init it:

```
bpb init "withoutboats <boats@mozilla.com>"
```

You can pass any string you want as your userid, but `"$NAME <$EMAIL>"` is the
conventional standard for OpenPGP userids.

This will create a file at ~/.bpb_keys.toml. This file contains your bpb public
and private keys.

It also prints your public key in OpenPGP format, so that you can upload it
again. You can print your public key more times with:

```
bpb print
```

If you want to use it to sign git commits, you also need to inform git to call
it instead of gpg. You can do this with this command:

```
git config --global gpg.program bpb
```

You should also provide the public key to people who want to verify your
commits. Personally, I just upload the public key to GitHub; you may have other
requirements.

# How it replaces gpg

If this program receives a `-s` argument, it reads from stdin and then writes a
signature to stdout. If it receives any arguments it doesn't recognize, it
delegates to the gpg binary in your path.

This means that this program can be used to replace gpg as a signing tool, but
it does not replace any other functionality. For example, if you want to verify
the signatures on other peoples' git commits, it will shell out to gpg.

# Storing your private key

By default, your private key is stored as a hex string in `~/.bpb_keys.toml`.
However, if you are uncomfortable with the possibility of someone reading your
private key from your home directory, you can instead store it somewhere else.
To do this, replace the `key` field with a `program` field, and `bpb` will run
this program, expecting it to print your key to stdout.
