mod picojs;

use picojs::*;
use image::{ImageBuffer,GrayImage,ConvertBuffer, Rgba};
use std::time::Instant;

fn main(){
    decode(b"fatal error LNK1181: \xce\xde\xb7\xa8\xb4\xf2\xbf\xaa\xca\xe4\xc8\xeb\xce\xc4\xbc\xfe\xa1\xb0libzbar64-0.lib\xa1\xb1\r\n");

    let buf = include_bytes!("../facefinder");

    let mut pico = Pico::new();
    pico.unpack_cascade(buf.to_vec());

    /*
    params = {
        "shiftfactor": 0.1, //将检测窗口移动其大小的10％
        "minsize": 20,      //脸的最小尺寸（不适合实时检测，在这种情况下设置为100）
        "maxsize": 1000,    //脸的最大尺寸
        "scalefactor": 1.1  //用于多尺度处理：当移动到更高比例时，将检测窗口的大小调整10％
    }
     */
    //https://github.com/nenadmarkus/pico
    let params = RunParams::new(1000.0, 20.0, 1.1, 0.1);
    let image = image::open("img1.jpg").unwrap().to_rgba();
    let (width, height) = (image.width(), image.height());
    // let gray = rgba_to_grayscale(image);
    let gray: GrayImage = image.convert();
    let image = Image::new(width as i32, width as usize, height as usize, gray.into_raw());
    let t = Instant::now();

    let mut dets = vec![];
    for _ in 0..10{
        let _ret = pico.run_cascade(&image, &params);

        dets = pico.cluster_detections(0.2);
    }

    println!("耗时{}ms {:?}", t.elapsed().as_millis(), dets);


    let qthresh = 5.0;

    let mut image = image::open("img1.jpg").unwrap();
    for one in dets.chunks(4){
        let r = one[0];//y
        let c = one[1];//x
        let scale = one[2];//直径
        let q = one[3];//得分
        if q > qthresh {
            imageproc::drawing::draw_hollow_circle_mut(&mut image, (c as i32, r as i32), scale as i32/2, Rgba([255, 0, 0, 255]));
        }
    }
    image.save("result.jpg").unwrap();
}

fn decode(bytes:&[u8]){
    let decoded = encoding_rs::Encoding::decode(encoding_rs::GBK, bytes);
    println!("decodeGBK:{:?}", decoded);
}