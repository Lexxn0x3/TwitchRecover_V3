# 📺 Twitch Recover v3 Continued

**Twitch Recover v3 Continued** is a powerful and versatile tool designed to help you recover, download, and process Twitch VODs with ease. Whether you're looking to retrieve a VOD from a Twitchtracker URL, manually recover a VOD, or concatenate and remux existing `.ts` files, this tool has got you covered!

## 🚀 Features

- **Recover VODs from Twitchtracker URLs**: Provide a Twitchtracker URL to automatically download and process the VOD.
- **Manual VOD Recovery**: Manually specify streamer name, VOD ID, and date to recover a VOD.
- **Concatenate and Remux Existing `.ts` Files**: Use existing `.ts` files from a directory to create a final video file.
- **Interactive Cleanup**: Optionally delete temporary `.ts` files after processing.

## 📋 How to Use

### Download and Install

1. **Download the latest release:**
    - Go to the [Releases](https://github.com/Lexxn0x3/TwitchRecover_V3/releases) page.
    - Download the appropriate version for your operating system.

2. **Extract the downloaded archive.**

3. **Install `ffmpeg`:**

    - **Windows:**
        1. Install Chocolatey if you haven't already:
            ```sh
            Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
            ```
        2. Install `ffmpeg` using Chocolatey:
            ```sh
            choco install ffmpeg
            ```

    - **macOS:**
        1. Install Homebrew if you haven't already: 
            ```sh
            /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
            ```
        2. Install `ffmpeg` using Homebrew:
            ```sh
            brew install ffmpeg
            ```

    - **Linux:**
        1. Install `ffmpeg` using your package manager (e.g., for Debian/Ubuntu):
            ```sh
            sudo apt update
            sudo apt install ffmpeg
            ```

### Usage

#### Recover VOD from Twitchtracker URL

1. **Go to [Twitchtracker](https://twitchtracker.com/)**:
    - Search for the channel.
    - Go to the "Streams" section.
    - Click on the stream you want to recover.
    - Copy the link of the stream.

2. **Run the tool with the copied link:**

    ```sh
    ./twitch_recover_v3_continued twitchtracker "https://twitchtracker.com/streamer_id/streams/twitch_tracker_vod_id"
    ```

#### Manually Recover VOD

```sh
./twitch_recover_v3_continued manual "streamer_name" "vod_id" "2022-10-29 13:06"
```
#### Concatenate and Remux Existing .ts Files
```sh
./twitch_recover_v3_continued concat "path/to/ts/files"
```
#### Example
1. Recover VOD from Twitchtracker URL:
```sh
./twitch_recover_v3_continued twitchtracker "https://twitchtracker.com/bayernfursofficial/streams/42467228808"
```
3. Manually Recover VOD:
```sh
./twitch_recover_v3_continued manual "bayernfursofficial" "42467228808" "2022-10-29 13:06"
```
4. Concatenate and Remux Existing .ts Files:
```sh
./twitch_recover_v3_continued concat "./temp"
```

## Cleanup
After processing, the program will ask if you want to delete the temporary .ts files. Respond with yes or no as needed.
📄 License
This project is licensed under the MIT License.
