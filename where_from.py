import biplist
import xattr
import click


def get_file_source_urls(filename):
    attrs = xattr.xattr(filename)
    return biplist.readPlistFromString(attrs.get('com.apple.metadata:kMDItemWhereFroms'))

@click.command()
@click.argument('filenames', type=click.Path(exists=True), nargs=-1)
def main(filenames):
    for filename in filenames:
        urls = get_file_source_urls(filename)
        if len(urls) > 0:
            print(filename)
        for url in urls:
            print(f"  {url}")
        print()
            

if __name__ == '__main__':
    main()
