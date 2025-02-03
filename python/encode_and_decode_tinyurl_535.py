class Codec:
    # gonna do this stupid style, just a counter
    urls = {}
    hashes = {}
    chars = string.ascii_letters + string.digits


    def tostr(self) ->str:
        code = ''.join(random.choice(self.chars) for i in range(6))
        return "https://tinyurl.com/" + code

    def encode(self, longUrl: str) -> str:
        """Encodes a URL to a shortened URL.
        """
        if longUrl in self.urls: return self.urls[longUrl]
        code = self.tostr()
        while code in self.hashes: code = self.tostr()
        self.hashes[code] = longUrl
        self.urls[longUrl] = code
        return code
        

    def decode(self, shortUrl: str) -> str:
        """Decodes a shortened URL to its original URL.
        """
        return self.hashes[shortUrl]
        

# Your Codec object will be instantiated and called as such:
# codec = Codec()
# codec.decode(codec.encode(url))

