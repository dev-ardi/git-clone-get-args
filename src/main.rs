use std::process::Command;

/*
gc() {
    if rg -q "://" <<< "$1"; then
        url="$1"
    else
        url="https://github.com/$1"
    fi

    if [ "$1" = "-h" ] || [ "$1" = "--help" ] || [ "$1" = "help" ]; then
        printf """\
#git clone on rails
  for more info, cat ~/s/help_scripts/git.sh

  ex: gc neovim/neovim
"""
        return 0
    elif [ "$#" = 1 ]; then
        split_array=$(echo "$1" | tr "/" "\n")
        filename=$(echo "$split_array" | tail -n 1)
        cd /tmp && rm -rf ./${filename} && git clone --depth=1 "$url" 1>&2
        cd "$filename"
        echo $(pwd)
    elif [ "$#" = 2 ]; then
        git clone --depth=1 "$url" $2
    fi
}

* */
fn main() {
    let input = std::env::args().nth(1).expect("expected 1 arg");
    "https://github.com/Valera6/dots/blob/master/home/v/s/help_scripts/git.sh";
    let gh_prefix = "https://github.com/";
    let repo;
    let out: String = if input.starts_with(gh_prefix) {
        let output = input.strip_prefix(gh_prefix).unwrap();
        //strip path
        let mut path = output.split('/');
        let username = path.next().unwrap();
        repo = path.next().unwrap().to_owned();
        // Maybe some logic
        format!("git@github.com:{username}/{repo}.git")
    } else if input.ends_with(".git") {
        repo = input
            .strip_suffix(".git")
            .unwrap()
            .split('/')
            .rev()
            .next()
            .unwrap()
            .to_owned();
        input
    } else if input.starts_with("https:") {
        repo = input
            .strip_suffix('/')
            .unwrap()
            .split('/')
            .rev()
            .next()
            .unwrap()
            .to_owned();
        input
    } else {
        panic!("unhandled git url")
    };
    let output = Command::new("git").args(["clone", &out]).output().unwrap();
    eprintln!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    print!("{repo}")
}
