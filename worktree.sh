#!/bin/sh
case $1 in
    init)
        git checkout main
        for b in `cat branch-list.txt`
        do
            git checkout $b
        done
        git checkout main
        ;;
    build)
        git checkout main
        mkdir -p branches
        for b in `cat branch-list.txt`
        do
            git worktree add branches/$b $b
        done
        ;;
    clean)
        git checkout main
        for b in `cat branch-list.txt`
        do
            git worktree remove branches/$b
        done
        rmdir branches
        ;;
esac
