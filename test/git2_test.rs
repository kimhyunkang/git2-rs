extern mod std;
extern mod git2;

#[test]
fn repo_head() {
    let repo = git2::repository::open("fixture").unwrap();
    match repo.head() {
        Some(head_id) => {
            let oid = head_id.resolve();
            assert_eq!(oid.to_str(), ~"21002f5d3f411fe990e13604273a51cd598a4a51")
        }
        None => fail!(~"failed to resolve head"),
    }
}

#[test]
fn repo_lookup() {
    let repo = git2::repository::open("fixture").unwrap();

    match repo.lookup("refs/heads/master") {
        None => fail!(~"failed to lookup master ref"),
        Some(master_ref) => {
            let master_id = master_ref.resolve();
            assert_eq!(master_id.to_str(), ~"21002f5d3f411fe990e13604273a51cd598a4a51")
        },
    }
}

#[test]
fn repo_empty() {
    let repo = git2::repository::open("fixture").unwrap();
    assert_eq!(repo.is_empty(), false)
}

#[test]
fn repo_bare() {
    let repo = git2::repository::open("fixture").unwrap();
    assert_eq!(repo.is_bare(), false)
}

#[test]
fn repo_oid() {
    let oid_str = ~"93d8ad7e3f5a300d2b4e18e9f31247a95e5cc37f";
    let oid = git2::oid::from_str(oid_str);
    assert_eq!(oid.to_str(), oid_str)
}

#[test]
fn repo_lookup_commit() {
    let repo = git2::repository::open("fixture").unwrap();
    let oid = git2::oid::from_str(&"21002f5d3f411fe990e13604273a51cd598a4a51");
    match repo.lookup_commit(&oid) {
        None => {
            fail!(~"commit does not exist")
        },
        Some(commit) => {
            assert!(commit.parents().is_empty(), ~"the first commit should have no parents");
        },
    }
}

#[test]
fn commit_apis() {
    let repo = git2::repository::open("fixture").unwrap();
    let oid = git2::oid::from_str(&"21002f5d3f411fe990e13604273a51cd598a4a51");
    let time_str = "Tue, 11 Jun 2013 19:14:48";
    let rfc822z = "%a, %d %b %Y %T";
    let tm = std::time::strptime(time_str, rfc822z).unwrap();
    let signature = git2::Signature {
        name: ~"김현강",
        email: ~"kimhyunkang@gmail.com",
        when: git2::Time {
            time: tm.to_timespec().sec,
            offset: 9 * 60,     // original time is +0900
        }
    };

    match repo.lookup_commit(&oid) {
        None => {
            fail!(~"commit does not exist")
        },
        Some(commit) => {
            assert_eq!(commit.message(), ~"Create README.md");
            assert_eq!(commit.id(), &oid);
            assert_eq!(commit.author(), copy signature);
            assert_eq!(commit.committer(), copy signature);
        },
    }
}

#[test]
fn commit() {
    let repo = git2::repository::open("fixture").unwrap();
    let parent_id = git2::oid::from_str(&"21002f5d3f411fe990e13604273a51cd598a4a51");
    let parent = match repo.lookup_commit(&parent_id) {
        None => fail!(~"commit does not exist"),
        Some(c) => c,
    };

    let time_str = "Sat, 15 Jun 2013 03:40:22";
    let rfc822z = "%a, %d %b %Y %T";
    let tm = std::time::strptime(time_str, rfc822z).unwrap();
    let sig = git2::Signature {
        name: ~"김현강",
        email: ~"kimhyunkang@gmail.com",
        when: git2::Time {
            time: tm.to_timespec().sec,
            offset: 9 * 60,     // original time is +0900
        }
    };

    let text = "blob text\n";
    let blob = repo.blob_create_frombuffer(str::as_bytes_slice(text)).unwrap();

    let mut treebuilder = git2::TreeBuilder::from_tree(parent.tree());
    treebuilder.insert(&"test_blob.txt", blob.id(), git2::GIT_FILEMODE_BLOB);
    let tree_id = treebuilder.write(repo);
    let tree = match repo.lookup_tree(&tree_id) {
        None => fail!(~"tree does not exist"),
        Some(t) => t,
    };

    let message = ~"commit test";
    let oid = repo.commit(None, &sig, &sig, None, message, tree, ~[parent]);
    match repo.lookup_commit(&oid) {
        None => fail!(~"failed to create commit"),
        Some(new_commit) => {
            assert_eq!(new_commit.message(), message)
        }
    };
}
