# CI TEMPLATE — not for direct use.
# The release-tarballs workflow rewrites Formula/image-view.rb in the tap repo
# (nikolareljin/homebrew-tap) with real per-platform URLs and SHA256 hashes.
# This file is a checked-in reference only; url/sha256 below are placeholders.

class ImageView < Formula
  desc "Render images directly in your terminal"
  homepage "https://github.com/nikolareljin/image-view"
  url "https://github.com/nikolareljin/image-view/archive/refs/tags/0.6.2.tar.gz"
  sha256 "PLACEHOLDER_SHA256_UPDATED_BY_CI"
  version "0.6.2"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
  end

  test do
    assert_match "image-view", shell_output("#{bin}/image-view --help")
  end
end
