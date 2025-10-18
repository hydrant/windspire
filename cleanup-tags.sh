#!/bin/bash

# Script to delete all Git tags (local and remote)
# Use with caution - this cannot be easily undone!

set -e

echo "🔍 Checking for tags..."

# Get all local tags
LOCAL_TAGS=$(git tag -l)

if [ -z "$LOCAL_TAGS" ]; then
    echo "✅ No local tags found."
else
    echo "📋 Found the following local tags:"
    echo "$LOCAL_TAGS"
    echo ""
    
    read -p "⚠️  Delete all local tags? (y/N): " confirm_local
    if [ "$confirm_local" = "y" ] || [ "$confirm_local" = "Y" ]; then
        echo "🗑️  Deleting local tags..."
        git tag -l | xargs git tag -d
        echo "✅ Local tags deleted."
    else
        echo "⏭️  Skipping local tag deletion."
    fi
fi

echo ""
echo "🔍 Checking remote tags..."

# Get all remote tags
REMOTE_TAGS=$(git ls-remote --tags origin | awk '{print $2}' | sed 's|refs/tags/||' | grep -v '\^{}')

if [ -z "$REMOTE_TAGS" ]; then
    echo "✅ No remote tags found."
else
    echo "📋 Found the following remote tags:"
    echo "$REMOTE_TAGS"
    echo ""
    
    read -p "⚠️  Delete all remote tags? This affects everyone! (y/N): " confirm_remote
    if [ "$confirm_remote" = "y" ] || [ "$confirm_remote" = "Y" ]; then
        echo "🗑️  Deleting remote tags..."
        git ls-remote --tags origin | awk '{print $2}' | sed 's|refs/tags/||' | grep -v '\^{}' | xargs -I {} git push origin :refs/tags/{}
        echo "✅ Remote tags deleted."
    else
        echo "⏭️  Skipping remote tag deletion."
    fi
fi

echo ""
echo "✨ Cleanup complete!"
