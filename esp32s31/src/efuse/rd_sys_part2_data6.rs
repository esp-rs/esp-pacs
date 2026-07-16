#[doc = "Register `RD_SYS_PART2_DATA6` reader"]
pub type R = crate::R<RD_SYS_PART2_DATA6_SPEC>;
#[doc = "Register `RD_SYS_PART2_DATA6` writer"]
pub type W = crate::W<RD_SYS_PART2_DATA6_SPEC>;
#[doc = "Field `USB_DEVICE_EXCHG_PINS` reader - Represents whether enable usb device exchange pins of D+ and D- or not. \\\\ 1: enabled\\\\ 0: disabled\\\\"]
pub type USB_DEVICE_EXCHG_PINS_R = crate::BitReader;
#[doc = "Field `USB_DEVICE_DREFH` reader - Represents the single-end input threhold vrefh, 1.76 V to 2 V with step of 80 mV."]
pub type USB_DEVICE_DREFH_R = crate::FieldReader;
#[doc = "Field `USB_DEVICE_DREFL` reader - Represents the usb device single-end input low threhold, 0.8 V to 1.04 V with step of 80 mV."]
pub type USB_DEVICE_DREFL_R = crate::FieldReader;
#[doc = "Field `RD_RESERVE_9_197` reader - "]
pub type RD_RESERVE_9_197_R = crate::FieldReader<u16>;
#[doc = "Field `RD_RESERVE_9_197` writer - "]
pub type RD_RESERVE_9_197_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PVT_0_CELL_SELECT` reader - Power glitch monitor PVT cell select."]
pub type PVT_0_CELL_SELECT_R = crate::FieldReader;
#[doc = "Field `PVT_1_CELL_SELECT` reader - Power glitch monitor PVT cell select."]
pub type PVT_1_CELL_SELECT_R = crate::FieldReader;
#[doc = "Field `RD_RESERVE_9_223` reader - "]
pub type RD_RESERVE_9_223_R = crate::BitReader;
#[doc = "Field `RD_RESERVE_9_223` writer - "]
pub type RD_RESERVE_9_223_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bits 5:16"]
    #[inline(always)]
    pub fn rd_reserve_9_197(&self) -> RD_RESERVE_9_197_R {
        RD_RESERVE_9_197_R::new(((self.bits >> 5) & 0x0fff) as u16)
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
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rd_reserve_9_223(&self) -> RD_RESERVE_9_223_R {
        RD_RESERVE_9_223_R::new(((self.bits >> 31) & 1) != 0)
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
            .field("rd_reserve_9_197", &self.rd_reserve_9_197())
            .field("rd_reserve_9_223", &self.rd_reserve_9_223())
            .finish()
    }
}
impl W {
    #[doc = "Bits 5:16"]
    #[inline(always)]
    pub fn rd_reserve_9_197(&mut self) -> RD_RESERVE_9_197_W<'_, RD_SYS_PART2_DATA6_SPEC> {
        RD_RESERVE_9_197_W::new(self, 5)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rd_reserve_9_223(&mut self) -> RD_RESERVE_9_223_W<'_, RD_SYS_PART2_DATA6_SPEC> {
        RD_RESERVE_9_223_W::new(self, 31)
    }
}
#[doc = "Represents rd_sys_part2_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_sys_part2_data6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_sys_part2_data6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_SYS_PART2_DATA6_SPEC;
impl crate::RegisterSpec for RD_SYS_PART2_DATA6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_sys_part2_data6::R`](R) reader structure"]
impl crate::Readable for RD_SYS_PART2_DATA6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rd_sys_part2_data6::W`](W) writer structure"]
impl crate::Writable for RD_SYS_PART2_DATA6_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RD_SYS_PART2_DATA6 to value 0"]
impl crate::Resettable for RD_SYS_PART2_DATA6_SPEC {}
