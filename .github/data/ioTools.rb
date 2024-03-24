# Documentation: https://docs.brew.sh/Formula-Cookbook
#                https://www.rubydoc.info/github/Homebrew/brew/master/Formula
# PLEASE REMOVE ALL GENERATED COMMENTS BEFORE SUBMITTING YOUR PULL REQUEST!
class Iotools < Formula
  desc "ioTools using rust build."
  homepage "https://github.com/wswenyue/ioTools"
  url "#_url_#"
  sha256 "#_sha256_#"
  version '#_version_#'

  def install
    bin.install "io_tools" => "iotools"
    man1.install "io_tools.1" => "iotools.1"
  end

  test do
       system bin/"iotools", "--version"
  end

end