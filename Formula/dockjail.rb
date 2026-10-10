# Formula template. scripts/render-formula.sh fills in the placeholders; the release workflow writes
# the result to Formula/dockjail.rb on main, so this repository is its own Homebrew tap.
class Dockjail < Formula
  desc "Keeps the macOS Dock locked to one display"
  homepage "https://github.com/mehmetmuejde/dockjail"
  url "https://github.com/mehmetmuejde/dockjail/releases/download/v0.1.2/dockjail-0.1.2-universal-apple-darwin.tar.gz"
  sha256 "8aecae8f93b975df656f9c1aae0fc2301ed37eb6f2605f63ebf24c442247f690"
  license "MIT"

  depends_on :macos

  def install
    bin.install "dockjail"
  end

  def caveats
    <<~EOS
      Lock the Dock to the display it is on:  dockjail on
      List displays and choose one:           dockjail displays / dockjail on INDEX
      Unlock:                                 dockjail off

      dockjail needs the Accessibility permission. Add #{opt_bin}/dockjail
      in System Settings → Privacy & Security → Accessibility.
      After an upgrade macOS asks for it again.
    EOS
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/dockjail --version")
  end
end
