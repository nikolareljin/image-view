# CI TEMPLATE — not for direct use.
# The release-tarballs workflow rewrites Formula/image-view.rb in the tap repo
# (nikolareljin/homebrew-tap) with real per-platform URLs and SHA256 hashes.
# This file is a checked-in reference only; url/sha256 below are placeholders.

class ImageView < Formula
  desc "Render images directly in your terminal"
  homepage "https://github.com/nikolareljin/image-view"
  url "PLACEHOLDER_URL_UPDATED_BY_CI"
  sha256 "PLACEHOLDER_SHA256_UPDATED_BY_CI"
  version "PLACEHOLDER_VERSION_UPDATED_BY_CI"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
  end

  test do
    assert_match "image-view", shell_output("#{bin}/image-view --help")
  end
end
