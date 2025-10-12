#!/bin/bash

# Standing in the root of a submodule; finding the hooks directory for the submodule in the parent repository.
repo_hooks_path=$(git rev-parse --git-path hooks)
repo_modules_path=$(git rev-parse --git-path modules)

# Copy commit-msg hook to the parent repository hooks directory.
# and make them executable.
echo "Copying GIT hooks to: $repo_hooks_path"
cp ./githooks/commit-msg $repo_hooks_path
chmod +x $repo_hooks_path/*
# Copy pre-defined hooks to the api submodule hooks directory.
# and make them executable.
echo "Copying GIT hooks to: $repo_modules_path/api/hooks/"
cp ./githooks/* $repo_modules_path/api/hooks/
chmod +x $repo_modules_path/api/hooks/*

