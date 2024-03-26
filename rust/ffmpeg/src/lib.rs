/**
 * see https://github.com/zmwangx/rust-ffmpeg/tree/master/examples
 **/
#[cfg(test)]
mod test_ffmpeg{
    extern crate ffmpeg_next as ffmpeg;

    use std::path::PathBuf;
    use ffmpeg_next::{codec, encoder, format, log, media, Rational};

    fn get_test_video_path() -> PathBuf {
        return PathBuf::from(home::home_dir().unwrap().join("Desktop/fcml.mp4"));
    }
    fn get_test_video_out_path() -> PathBuf {
        return PathBuf::from(home::home_dir().unwrap().join("Desktop/out.mp4"));
    }

    #[test]
    fn metadata(){
        ffmpeg::init().unwrap();
        let input: PathBuf = get_test_video_path();
        match ffmpeg::format::input(&input.as_path()) {
            Ok(context) => {
                for (k, v) in context.metadata().iter() {
                    println!("{}: {}", k, v);
                }

                if let Some(stream) = context.streams().best(ffmpeg::media::Type::Video) {
                    println!("Best video stream index: {}", stream.index());
                }

                if let Some(stream) = context.streams().best(ffmpeg::media::Type::Audio) {
                    println!("Best audio stream index: {}", stream.index());
                }

                if let Some(stream) = context.streams().best(ffmpeg::media::Type::Subtitle) {
                    println!("Best subtitle stream index: {}", stream.index());
                }

                println!(
                    "duration (seconds): {:.2}",
                    context.duration() as f64 / f64::from(ffmpeg::ffi::AV_TIME_BASE)
                );

                for stream in context.streams() {
                    println!("stream index {}:", stream.index());
                    println!("\ttime_base: {}", stream.time_base());
                    println!("\tstart_time: {}", stream.start_time());
                    println!("\tduration (stream timebase): {}", stream.duration());
                    println!(
                        "\tduration (seconds): {:.2}",
                        stream.duration() as f64 * f64::from(stream.time_base())
                    );
                    println!("\tframes: {}", stream.frames());
                    println!("\tdisposition: {:?}", stream.disposition());
                    println!("\tdiscard: {:?}", stream.discard());
                    println!("\trate: {}", stream.rate());

                    let codec = ffmpeg::codec::context::Context::from_parameters(stream.parameters()).unwrap();

                    println!("\tmedium: {:?}", codec.medium());
                    println!("\tid: {:?}", codec.id());
                    if codec.medium() == ffmpeg::media::Type::Video {
                        if let Ok(video) = codec.decoder().video() {
                            println!("\tbit_rate: {}", video.bit_rate());
                            println!("\tmax_rate: {}", video.max_bit_rate());
                            println!("\tdelay: {}", video.delay());
                            println!("\tvideo.width: {}", video.width());
                            println!("\tvideo.height: {}", video.height());
                            println!("\tvideo.format: {:?}", video.format());
                            println!("\tvideo.has_b_frames: {}", video.has_b_frames());
                            println!("\tvideo.aspect_ratio: {}", video.aspect_ratio());
                            println!("\tvideo.color_space: {:?}", video.color_space());
                            println!("\tvideo.color_range: {:?}", video.color_range());
                            println!("\tvideo.color_primaries: {:?}", video.color_primaries());
                            println!(
                                "\tvideo.color_transfer_characteristic: {:?}",
                                video.color_transfer_characteristic()
                            );
                            println!("\tvideo.chroma_location: {:?}", video.chroma_location());
                            println!("\tvideo.references: {}", video.references());
                            println!("\tvideo.intra_dc_precision: {}", video.intra_dc_precision());
                        }
                    } else if codec.medium() == ffmpeg::media::Type::Audio {
                        if let Ok(audio) = codec.decoder().audio() {
                            println!("\tbit_rate: {}", audio.bit_rate());
                            println!("\tmax_rate: {}", audio.max_bit_rate());
                            println!("\tdelay: {}", audio.delay());
                            println!("\taudio.rate: {}", audio.rate());
                            println!("\taudio.channels: {}", audio.channels());
                            println!("\taudio.format: {:?}", audio.format());
                            println!("\taudio.frames: {}", audio.frames());
                            println!("\taudio.align: {}", audio.align());
                            println!("\taudio.channel_layout: {:?}", audio.channel_layout());
                        }
                    }



                }
            }

            Err(error) => println!("error: {}", error),
        }
    }

    #[test]
    fn chapters() {
        ffmpeg::init().unwrap();

        match ffmpeg::format::input(&get_test_video_path()) {
            Ok(ictx) => {
                println!("Nb chapters: {}", ictx.nb_chapters());

                for chapter in ictx.chapters() {
                    println!("chapter id {}:", chapter.id());
                    println!("\ttime_base: {}", chapter.time_base());
                    println!("\tstart: {}", chapter.start());
                    println!("\tend: {}", chapter.end());

                    for (k, v) in chapter.metadata().iter() {
                        println!("\t{}: {}", k, v);
                    }
                }

                let mut octx = ffmpeg::format::output(&"test.mkv").expect("Couldn't open test file");

                for chapter in ictx.chapters() {
                    let title = match chapter.metadata().get("title") {
                        Some(title) => String::from(title),
                        None => String::new(),
                    };

                    match octx.add_chapter(
                        chapter.id(),
                        chapter.time_base(),
                        chapter.start(),
                        chapter.end(),
                        &title,
                    ) {
                        Ok(chapter) => println!("Added chapter with id {} to output", chapter.id()),
                        Err(error) => {
                            println!("Error adding chapter with id: {} - {}", chapter.id(), error)
                        }
                    }
                }

                println!("\nOuput: nb chapters: {}", octx.nb_chapters());
                for chapter in octx.chapters() {
                    println!("chapter id {}:", chapter.id());
                    println!("\ttime_base: {}", chapter.time_base());
                    println!("\tstart: {}", chapter.start());
                    println!("\tend: {}", chapter.end());
                    for (k, v) in chapter.metadata().iter() {
                        println!("\t{}: {}", k, v);
                    }
                }
            }

            Err(error) => println!("error: {}", error),
        }
    }


    #[test]
    fn remux(){
        let input_file = get_test_video_path();
        let output_file = get_test_video_out_path();

        ffmpeg::init().unwrap();
        log::set_level(log::Level::Info);

        let mut ictx = format::input(&input_file).unwrap();
        let mut octx = format::output(&output_file).unwrap();

        let mut stream_mapping = vec![0; ictx.nb_streams() as _];
        let mut ist_time_bases = vec![Rational(0, 1); ictx.nb_streams() as _];
        let mut ost_index = 0;
        for (ist_index, ist) in ictx.streams().enumerate() {
            let ist_medium = ist.parameters().medium();
            if ist_medium != media::Type::Audio
                && ist_medium != media::Type::Video
                && ist_medium != media::Type::Subtitle
            {
                stream_mapping[ist_index] = -1;
                continue;
            }
            stream_mapping[ist_index] = ost_index;
            ist_time_bases[ist_index] = ist.time_base();
            ost_index += 1;
            let mut ost = octx.add_stream(encoder::find(codec::Id::None)).unwrap();
            ost.set_parameters(ist.parameters());
            // We need to set codec_tag to 0 lest we run into incompatible codec tag
            // issues when muxing into a different container format. Unfortunately
            // there's no high level API to do this (yet).
            unsafe {
                (*ost.parameters().as_mut_ptr()).codec_tag = 0;
            }
        }

        octx.set_metadata(ictx.metadata().to_owned());
        octx.write_header().unwrap();

        for (stream, mut packet) in ictx.packets() {
            let ist_index = stream.index();
            let ost_index = stream_mapping[ist_index];
            if ost_index < 0 {
                continue;
            }
            let ost = octx.stream(ost_index as _).unwrap();
            packet.rescale_ts(ist_time_bases[ist_index], ost.time_base());
            packet.set_position(-1);
            packet.set_stream(ost_index as _);
            packet.write_interleaved(&mut octx).unwrap();
        }

        octx.write_trailer().unwrap();
    }


}