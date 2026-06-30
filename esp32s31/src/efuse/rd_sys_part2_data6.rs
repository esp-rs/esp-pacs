#[doc = "Register `RD_SYS_PART2_DATA6` reader"]
pub type R = crate::R<RD_SYS_PART2_DATA6_SPEC>;
#[doc = "Field `USB_DEVICE_EXCHG_PINS` reader - Represents whether enable usb device exchange pins of D+ and D- or not. \\\\ 1: enabled\\\\ 0: disabled\\\\"]
pub type USB_DEVICE_EXCHG_PINS_R = crate::BitReader;
#[doc = "Field `USB_DEVICE_DREFH` reader - Represents the single-end input threhold vrefh, 1.76 V to 2 V with step of 80 mV."]
pub type USB_DEVICE_DREFH_R = crate::FieldReader;
#[doc = "Field `USB_DEVICE_DREFL` reader - Represents the usb device single-end input low threhold, 0.8 V to 1.04 V with step of 80 mV."]
pub type USB_DEVICE_DREFL_R = crate::FieldReader;
#[doc = "Field `PVT_0_CELL_SELECT` reader - Power glitch monitor PVT cell select."]
pub type PVT_0_CELL_SELECT_R = crate::FieldReader;
#[doc = "Field `PVT_1_CELL_SELECT` reader - Power glitch monitor PVT cell select."]
pub type PVT_1_CELL_SELECT_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Represents whether enable usb device exchange pins of D+ and D- or not. \\\\ 1: enabled\\\\ 0: disabled\\\\"]
    #[inline(always)]
    pub fn usb_device_exchg_pins(&self) -> USB_DEVICE_EXCHG_PINS_R {
        USB_DEVICE_EXCHG_PINS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Represents the single-end input threhold vrefh, 1.76 V to 2 V with step of 80 mV."]
    #[inline(always)]
    pub fn usb_device_drefh(&self) -> USB_DEVICE_DREFH_R {
        USB_DEVICE_DREFH_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Represents the usb device single-end input low threhold, 0.8 V to 1.04 V with step of 80 mV."]
    #[inline(always)]
    pub fn usb_device_drefl(&self) -> USB_DEVICE_DREFL_R {
        USB_DEVICE_DREFL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 17:23 - Power glitch monitor PVT cell select."]
    #[inline(always)]
    pub fn pvt_0_cell_select(&self) -> PVT_0_CELL_SELECT_R {
        PVT_0_CELL_SELECT_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Power glitch monitor PVT cell select."]
    #[inline(always)]
    pub fn pvt_1_cell_select(&self) -> PVT_1_CELL_SELECT_R {
        PVT_1_CELL_SELECT_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_SYS_PART2_DATA6")
            .field("usb_device_exchg_pins", &self.usb_device_exchg_pins())
            .field("usb_device_drefh", &self.usb_device_drefh())
            .field("usb_device_drefl", &self.usb_device_drefl())
            .field("pvt_0_cell_select", &self.pvt_0_cell_select())
            .field("pvt_1_cell_select", &self.pvt_1_cell_select())
            .finish()
    }
}
#[doc = "Represents rd_sys_part2_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_sys_part2_data6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_SYS_PART2_DATA6_SPEC;
impl crate::RegisterSpec for RD_SYS_PART2_DATA6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_sys_part2_data6::R`](R) reader structure"]
impl crate::Readable for RD_SYS_PART2_DATA6_SPEC {}
#[doc = "`reset()` method sets RD_SYS_PART2_DATA6 to value 0"]
impl crate::Resettable for RD_SYS_PART2_DATA6_SPEC {}
