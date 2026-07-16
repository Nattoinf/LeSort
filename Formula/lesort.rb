class Lesort < Formula
  desc "File organization analyzer and scoring tool"
  homepage "https://github.com/Nattoinf/LeSort"
  url "https://github.com/Nattoinf/LeSort/archive/refs/tags/v0.1.1.tar.gz"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "build", "--release"
    bin.install "target/release/lesort"
  end

  test do
    system "#{bin}/lesort", "--version"
  end
end
