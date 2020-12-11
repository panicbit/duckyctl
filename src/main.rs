use anyhow::*;
use hidapi::{HidApi, HidDevice};

const VID_DUCKY: u16 = 0x04d9;
const PID_DUCKY_ONE_2_TKL: u16 = 0x0356;

fn main() -> Result<()> {
    let hid = HidApi::new()
        .context("Failed to initialize hidapi")?;

    let device = hid.device_list()
        .filter(|dev| dev.vendor_id() == VID_DUCKY)
        .filter(|dev| dev.product_id() == PID_DUCKY_ONE_2_TKL)
        .filter(|dev| dev.interface_number() == 1)
        .next()
        .context("Could not find Duck One 2 TKL")?;

    println!("{:#?}", device.path());

    let device = device.open_device(&hid)
        .context("Failed to open device")?;
    
    // programming()
    device.write(&[0x41, 0x01])?;

    let mut colors = vec![0; 3 * 6 * 22];

    for i in 0..6 * 22 {
        if let Some(color) = colors.chunks_mut(3).nth(i) {
            color[0] = 0x00;
            color[1] = 0xFF;
            color[2] = 0x00;
        }

        set_custom_colors(&device, &colors)?;

        std::thread::sleep_ms(50);
    }

    std::thread::sleep_ms(2000);
    device.write(&[0x41, 0x00])?;

    Ok(())
}

fn set_custom_colors(device: &HidDevice, colors: &[u8]) -> Result<()>{
    device.write(&[
        0x56, 0x81, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x00, 0x00, // packet count?
        0xaa, 0xaa, 0xaa, 0xaa
    ])?;

    let prefix = &[
        0x01, 0x00, 0x00, 0x00,
        0x80, 0x01, 0x00, 0xc1,
        0x00, 0x00, 0x00, 0x00,
        0xff, 0xff, 0xff, 0xff,
        0x00, 0x00, 0x00, 0x00,
    ];

    let packet_count = 8;
    let payload_len = 60;
    let mut data = prefix.to_vec();
    data.extend(colors);

    for packet_index in 0..packet_count {
        let packet_index = packet_index as u8;
        let mut message = vec![0x56, 0x83, packet_index, 0x00];
        let packet = data
            .chunks(payload_len)
            .nth(packet_index as usize)
            .unwrap_or_default();
        message.extend_from_slice(packet);
        
        device.write(&message)
            .context("Failed to write packet")?;
    }
    device.write(&[0x51, 0x28, 0x00, 0x00, 0xff])?;

    // println!("{:#x?}", result);
    Ok(())
}
