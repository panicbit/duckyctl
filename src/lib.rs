use anyhow::*;
use hidapi::{HidApi, HidDevice};

const VID_DUCKY: u16 = 0x04d9;
const PID_DUCKY_ONE_2_RGB_TKL: u16 = 0x0356;

pub fn hid() -> hidapi::HidResult<HidApi> {
    HidApi::new()
}

pub struct Keyboard {
    device: HidDevice,
    colors: Vec<u8>,
}

impl Keyboard {
    const NUM_KEYS: usize = 22 * 6;

    pub fn open(hid_api: &HidApi) -> Result<Self> {
        let device = hid_api.device_list()
            .filter(|dev| dev.vendor_id() == VID_DUCKY)
            .filter(|dev| dev.product_id() == PID_DUCKY_ONE_2_RGB_TKL)
            .filter(|dev| dev.interface_number() == 1)
            .next()
            .context("Could not find Duck One 2 TKL")?;

        println!("{:#?}", device.path());

        let device = device.open_device(&hid_api)
            .context("Failed to open device. Are access permissions sufficient?")?;

        Ok(Self {
            device,
            colors: vec![0; 3 * Self::NUM_KEYS],
        })
    }

    pub fn clear_colors(&mut self) {
        for value in &mut self.colors {
            *value = 0;
        }
    }

    pub fn colors_mut(&mut self) -> impl Iterator<Item = &mut [u8]> {
        self.colors
            .chunks_mut(3)
    }

    pub fn set_color(&mut self, index: usize, new_color: (u8, u8, u8)) {
        let color = self.colors_mut().nth(index);

        if let Some(color) = color {
            color[0] = new_color.0;
            color[1] = new_color.1;
            color[2] = new_color.2;
        }
    }

    pub fn set_all_colors(&mut self, new_color: (u8, u8, u8)) {
        for color in self.colors_mut() {
            color[0] = new_color.0;
            color[1] = new_color.1;
            color[2] = new_color.2;
        }
    }

    pub fn set_static_colors(&self) -> Result<()> {
        self.enter_programming_mode()?;

        self.device.write(&[
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
        data.extend(self.colors.iter());

        for packet_index in 0..packet_count {
            let packet_index = packet_index as u8;
            let mut message = vec![0x56, 0x83, packet_index, 0x00];
            let packet = data
                .chunks(payload_len)
                .nth(packet_index as usize)
                .unwrap_or_default();
            message.extend_from_slice(packet);

            self.device.write(&message)
                .context("Failed to write packet")?;
        }

        self.device.write(&[0x51, 0x28, 0x00, 0x00, 0xff])?;

        Ok(())
    }

    pub fn enter_autonomous_mode(&self) -> Result<()> {
        self.device.write(&[0x41, 0x00])?;
        Ok(())
    }

    fn enter_programming_mode(&self) -> Result<()> {
        self.device.write(&[0x41, 0x01])?;
        Ok(())
    }
}
