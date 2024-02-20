from setuptools import setup, find_packages

setup(
    name="merkle-tree-python-common",
    version="0.1",
    packages=["python_common"],
    package_dir={"python_common": "."},
    description="A short description of your project",
    long_description=open("README.md").read(),
    long_description_content_type="text/markdown",
    author="Your Name",
    author_email="your.email@example.com",
    url="https://github.com/yourusername/your-package-name",
    classifiers=[
        "Development Status :: 3 - Alpha",
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Python :: 3.6",
        "Programming Language :: Python :: 3.7",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
    ],
    install_requires=[
        # List your project dependencies here
    ],
)
