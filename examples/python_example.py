import twikit

def main():
    b = twikit.Bookmark()
    b.add("https://example.com")
    print("Bookmarks:", b.list())

    c = twikit.Community()
    c.create("My Community")
    print("Communities:", c.list())

    g = twikit.Geo()
    d = g.distance((40.7128, -74.0060), (34.0522, -118.2437))
    print("Distance (km):", d)

    u = twikit.User()
    u.create("alice")
    print("User alice:", u.get("alice"))

if __name__ == '__main__':
    main()
