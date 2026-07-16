class Lesort < Formula
  desc "File organization analyzer and scoring tool"
  homepage "https://github.com/Nattoinf/LeSort"
  url "https://github.com/Nattoinf/LeSort/archive/refs/tags/v0.1.1.tar.gz"
  sha256 "012556d2dd75db314c234c48b7438ea9dec1b73f7a1521303326b8af7a636272"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "build", "--release"
    bin.install "target/release/lesort"
  end

  test do
    system bin/"lesort", "--version"
  end
end
