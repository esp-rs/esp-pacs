#[doc = "Register `RD_SYS_PART2_DATA7` reader"]
pub type R = crate::R<RD_SYS_PART2_DATA7_SPEC>;
#[doc = "Field `PUMP_DRV` reader - Use to configure charge pump voltage gain"]
pub type PUMP_DRV_R = crate::FieldReader;
#[doc = "Field `USB_DEVICE_EXCHG_PINS` reader - Enable usb device exchange pins of D+ and D-."]
pub type USB_DEVICE_EXCHG_PINS_R = crate::BitReader;
#[doc = "Field `USB_OTG11_EXCHG_PINS` reader - Enable usb otg11 exchange pins of D+ and D-."]
pub type USB_OTG11_EXCHG_PINS_R = crate::BitReader;
#[doc = "Field `USB_DEVICE_DREFH` reader - usb intphy of usb device single-end input high threshold, 1.76V to 2V. step by 80mV."]
pub type USB_DEVICE_DREFH_R = crate::FieldReader;
#[doc = "Field `USB_OTG11_DREFH` reader - usb intphy of usb otg11 single-end input high threshold, 1.76V to 2V. step by 80mV."]
pub type USB_OTG11_DREFH_R = crate::FieldReader;
#[doc = "Field `USB_DEVICE_DREFL` reader - usb intphy of usb device single-end input low threshold, 0.8V to 1.04V. step by 80mV."]
pub type USB_DEVICE_DREFL_R = crate::FieldReader;
#[doc = "Field `USB_OTG11_DREFL` reader - usb intphy of usb otg11 single-end input low threshold, 0.8V to 1.04V. step by 80mV."]
pub type USB_OTG11_DREFL_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Use to configure charge pump voltage gain"]
    #[inline(always)]
    pub fn pump_drv(&self) -> PUMP_DRV_R {
        PUMP_DRV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Enable usb device exchange pins of D+ and D-."]
    #[inline(always)]
    pub fn usb_device_exchg_pins(&self) -> USB_DEVICE_EXCHG_PINS_R {
        USB_DEVICE_EXCHG_PINS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable usb otg11 exchange pins of D+ and D-."]
    #[inline(always)]
    pub fn usb_otg11_exchg_pins(&self) -> USB_OTG11_EXCHG_PINS_R {
        USB_OTG11_EXCHG_PINS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - usb intphy of usb device single-end input high threshold, 1.76V to 2V. step by 80mV."]
    #[inline(always)]
    pub fn usb_device_drefh(&self) -> USB_DEVICE_DREFH_R {
        USB_DEVICE_DREFH_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - usb intphy of usb otg11 single-end input high threshold, 1.76V to 2V. step by 80mV."]
    #[inline(always)]
    pub fn usb_otg11_drefh(&self) -> USB_OTG11_DREFH_R {
        USB_OTG11_DREFH_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - usb intphy of usb device single-end input low threshold, 0.8V to 1.04V. step by 80mV."]
    #[inline(always)]
    pub fn usb_device_drefl(&self) -> USB_DEVICE_DREFL_R {
        USB_DEVICE_DREFL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - usb intphy of usb otg11 single-end input low threshold, 0.8V to 1.04V. step by 80mV."]
    #[inline(always)]
    pub fn usb_otg11_drefl(&self) -> USB_OTG11_DREFL_R {
        USB_OTG11_DREFL_R::new(((self.bits >> 12) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_SYS_PART2_DATA7")
            .field("pump_drv", &self.pump_drv())
            .field("usb_device_exchg_pins", &self.usb_device_exchg_pins())
            .field("usb_otg11_exchg_pins", &self.usb_otg11_exchg_pins())
            .field("usb_device_drefh", &self.usb_device_drefh())
            .field("usb_otg11_drefh", &self.usb_otg11_drefh())
            .field("usb_device_drefl", &self.usb_device_drefl())
            .field("usb_otg11_drefl", &self.usb_otg11_drefl())
            .finish()
    }
}
#[doc = "Represents rd_sys_part2_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_sys_part2_data7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_SYS_PART2_DATA7_SPEC;
impl crate::RegisterSpec for RD_SYS_PART2_DATA7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_sys_part2_data7::R`](R) reader structure"]
impl crate::Readable for RD_SYS_PART2_DATA7_SPEC {}
#[doc = "`reset()` method sets RD_SYS_PART2_DATA7 to value 0"]
impl crate::Resettable for RD_SYS_PART2_DATA7_SPEC {}
