class Ezcfg < Formula
    version "0.0.4"

    desc "Use soft link to centrally manage all your personal profiles"
    homepage "https://github.com/tim101010101/ezcfg"
    
    url "https://github.com/tim101010101/ezcfg/releases/download/#{version}/ezcfg_#{version}.tar.gz"
    sha256 "a9fd6fb8be5c9974bc690e369fd64a18c117f1e8e1bd839f78992a2d962d5b7d"
  
    def install
      bin.install "ezcfg"
    end
  end