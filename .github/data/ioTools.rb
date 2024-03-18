# Documentation: https://docs.brew.sh/Formula-Cookbook
#                https://www.rubydoc.info/github/Homebrew/brew/master/Formula
# PLEASE REMOVE ALL GENERATED COMMENTS BEFORE SUBMITTING YOUR PULL REQUEST!
class Aklog < Formula
  desc "ioTools using rust build."
  homepage "https://github.com/wswenyue/ioTools"
  url "#_url_#"
  sha256 "#_sha256_#"
  version '#_version_#'

  def install
    bin.install "ioTools"
  end

  test do
    system bin/"ioTools", "--version"
  end

end