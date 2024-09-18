class Ezcfg < Formula
    version "0.0.3"

    desc "Use soft link to centrally manage all your personal profiles"
    homepage "https://github.com/tim101010101/ezcfg"
    
    url "https://github.com/tim101010101/ezcfg/releases/download/#{version}/ezcfg_#{version}.tar.gz"
    sha256 "590eeaf93dd4958c62d4a1c07efeb0a0285b1b529cfed6190a9b960dd7d5873a"
  
    def install
      bin.install "ezcfg"
    end
  end