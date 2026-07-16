#[doc = "Register `RD_REPEAT_DATA0` reader"]
pub type R = crate::R<RD_REPEAT_DATA0_SPEC>;
#[doc = "Register `RD_REPEAT_DATA0` writer"]
pub type W = crate::W<RD_REPEAT_DATA0_SPEC>;
#[doc = "Field `RD_DIS` reader - Represents whether reading of individual eFuse block(block4~block9) is disabled or enabled.\\\\ 1: disabled\\\\ 0: enabled\\\\"]
pub type RD_DIS_R = crate::FieldReader;
#[doc = "Field `RD_RESERVE_0_39` reader - "]
pub type RD_RESERVE_0_39_R = crate::FieldReader;
#[doc = "Field `RD_RESERVE_0_39` writer - "]
pub type RD_RESERVE_0_39_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DIS_USB_JTAG` reader - Represents whether the function of usb switch to jtag is disabled or enabled.\\\\ 1: disabled\\\\ 0: enabled\\\\"]
pub type DIS_USB_JTAG_R = crate::BitReader;
#[doc = "Field `DIS_USB_SERIAL_JTAG` reader - Represents whether USB-Serial-JTAG is disabled or enabled.\\\\ 1: disabled\\\\ 0: enabled\\\\"]
pub type DIS_USB_SERIAL_JTAG_R = crate::BitReader;
#[doc = "Field `DIS_FORCE_DOWNLOAD` reader - Represents whether the function that forces chip into download mode is disabled or enabled.\\\\ 1: disabled\\\\ 0: enabled\\\\"]
pub type DIS_FORCE_DOWNLOAD_R = crate::BitReader;
#[doc = "Field `SPI_DOWNLOAD_MSPI_DIS` reader - Represents whether SPI0 controller during boot_mode_download is disabled or enabled.\\\\ 1: disabled\\\\ 0: enabled\\\\"]
pub type SPI_DOWNLOAD_MSPI_DIS_R = crate::BitReader;
#[doc = "Field `DIS_TWAI` reader - Represents whether TWAI function is disabled or enabled.\\\\ 1: disabled\\\\ 0: enabled\\\\"]
pub type DIS_TWAI_R = crate::BitReader;
#[doc = "Field `JTAG_SEL_ENABLE` reader - Represents whether the selection between usb_to_jtag and pad_to_jtag through strapping gpio15 when both EFUSE_DIS_PAD_JTAG and EFUSE_DIS_USB_JTAG are equal to 0 is enabled or disabled.\\\\ 1: enabled\\\\ 0: disabled\\\\"]
pub type JTAG_SEL_ENABLE_R = crate::BitReader;
#[doc = "Field `SOFT_DIS_JTAG` reader - Represents whether JTAG is disabled in soft way.\\\\ Odd number: disabled\\\\ Even number: enabled\\\\"]
pub type SOFT_DIS_JTAG_R = crate::FieldReader;
#[doc = "Field `DIS_PAD_JTAG` reader - Represents whether JTAG is disabled in the hard way(permanently).\\\\ 1: disabled\\\\ 0: enabled\\\\"]
pub type DIS_PAD_JTAG_R = crate::BitReader;
#[doc = "Field `DIS_DOWNLOAD_MANUAL_ENCRYPT` reader - Represents whether flash encrypt function is disabled or enabled(except in SPI boot mode).\\\\ 1: disabled\\\\ 0: enabled\\\\"]
pub type DIS_DOWNLOAD_MANUAL_ENCRYPT_R = crate::BitReader;
#[doc = "Field `RD_RESERVE_0_53` reader - "]
pub type RD_RESERVE_0_53_R = crate::BitReader;
#[doc = "Field `RD_RESERVE_0_53` writer - "]
pub type RD_RESERVE_0_53_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS_WIFI6` reader - Represents thether the WIFI6 feature is enable or disabled. 1: WIFI6 is disable, 0: WIFI6 is enabled."]
pub type DIS_WIFI6_R = crate::BitReader;
#[doc = "Field `HUK_GEN_STATE` reader - Represents the control of validation of HUK generate mode. Odd of 1 is invalid, even of 1 is valid."]
pub type HUK_GEN_STATE_R = crate::FieldReader;
#[doc = "Field `RD_RESERVE_0_60` reader - "]
pub type RD_RESERVE_0_60_R = crate::FieldReader;
#[doc = "Field `RD_RESERVE_0_60` writer - "]
pub type RD_RESERVE_0_60_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:6 - Represents whether reading of individual eFuse block(block4~block9) is disabled or enabled.\\\\ 1: disabled\\\\ 0: enabled\\\\"]
    #[inline(always)]
    pub fn rd_dis(&self) -> RD_DIS_R {
        RD_DIS_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn rd_reserve_0_39(&self) -> RD_RESERVE_0_39_R {
        RD_RESERVE_0_39_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10 - Represents whether the function of usb switch to jtag is disabled or enabled.\\\\ 1: disabled\\\\ 0: enabled\\\\"]
    #[inline(always)]
    pub fn dis_usb_jtag(&self) -> DIS_USB_JTAG_R {
        DIS_USB_JTAG_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Represents whether USB-Serial-JTAG is disabled or enabled.\\\\ 1: disabled\\\\ 0: enabled\\\\"]
    #[inline(always)]
    pub fn dis_usb_serial_jtag(&self) -> DIS_USB_SERIAL_JTAG_R {
        DIS_USB_SERIAL_JTAG_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Represents whether the function that forces chip into download mode is disabled or enabled.\\\\ 1: disabled\\\\ 0: enabled\\\\"]
    #[inline(always)]
    pub fn dis_force_download(&self) -> DIS_FORCE_DOWNLOAD_R {
        DIS_FORCE_DOWNLOAD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Represents whether SPI0 controller during boot_mode_download is disabled or enabled.\\\\ 1: disabled\\\\ 0: enabled\\\\"]
    #[inline(always)]
    pub fn spi_download_mspi_dis(&self) -> SPI_DOWNLOAD_MSPI_DIS_R {
        SPI_DOWNLOAD_MSPI_DIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Represents whether TWAI function is disabled or enabled.\\\\ 1: disabled\\\\ 0: enabled\\\\"]
    #[inline(always)]
    pub fn dis_twai(&self) -> DIS_TWAI_R {
        DIS_TWAI_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Represents whether the selection between usb_to_jtag and pad_to_jtag through strapping gpio15 when both EFUSE_DIS_PAD_JTAG and EFUSE_DIS_USB_JTAG are equal to 0 is enabled or disabled.\\\\ 1: enabled\\\\ 0: disabled\\\\"]
    #[inline(always)]
    pub fn jtag_sel_enable(&self) -> JTAG_SEL_ENABLE_R {
        JTAG_SEL_ENABLE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Represents whether JTAG is disabled in soft way.\\\\ Odd number: disabled\\\\ Even number: enabled\\\\"]
    #[inline(always)]
    pub fn soft_dis_jtag(&self) -> SOFT_DIS_JTAG_R {
        SOFT_DIS_JTAG_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Represents whether JTAG is disabled in the hard way(permanently).\\\\ 1: disabled\\\\ 0: enabled\\\\"]
    #[inline(always)]
    pub fn dis_pad_jtag(&self) -> DIS_PAD_JTAG_R {
        DIS_PAD_JTAG_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Represents whether flash encrypt function is disabled or enabled(except in SPI boot mode).\\\\ 1: disabled\\\\ 0: enabled\\\\"]
    #[inline(always)]
    pub fn dis_download_manual_encrypt(&self) -> DIS_DOWNLOAD_MANUAL_ENCRYPT_R {
        DIS_DOWNLOAD_MANUAL_ENCRYPT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rd_reserve_0_53(&self) -> RD_RESERVE_0_53_R {
        RD_RESERVE_0_53_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Represents thether the WIFI6 feature is enable or disabled. 1: WIFI6 is disable, 0: WIFI6 is enabled."]
    #[inline(always)]
    pub fn dis_wifi6(&self) -> DIS_WIFI6_R {
        DIS_WIFI6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:27 - Represents the control of validation of HUK generate mode. Odd of 1 is invalid, even of 1 is valid."]
    #[inline(always)]
    pub fn huk_gen_state(&self) -> HUK_GEN_STATE_R {
        HUK_GEN_STATE_R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn rd_reserve_0_60(&self) -> RD_RESERVE_0_60_R {
        RD_RESERVE_0_60_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA0")
            .field("rd_dis", &self.rd_dis())
            .field("dis_usb_jtag", &self.dis_usb_jtag())
            .field("dis_usb_serial_jtag", &self.dis_usb_serial_jtag())
            .field("dis_force_download", &self.dis_force_download())
            .field("spi_download_mspi_dis", &self.spi_download_mspi_dis())
            .field("dis_twai", &self.dis_twai())
            .field("jtag_sel_enable", &self.jtag_sel_enable())
            .field("soft_dis_jtag", &self.soft_dis_jtag())
            .field("dis_pad_jtag", &self.dis_pad_jtag())
            .field(
                "dis_download_manual_encrypt",
                &self.dis_download_manual_encrypt(),
            )
            .field("dis_wifi6", &self.dis_wifi6())
            .field("huk_gen_state", &self.huk_gen_state())
            .field("rd_reserve_0_39", &self.rd_reserve_0_39())
            .field("rd_reserve_0_53", &self.rd_reserve_0_53())
            .field("rd_reserve_0_60", &self.rd_reserve_0_60())
            .finish()
    }
}
impl W {
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn rd_reserve_0_39(&mut self) -> RD_RESERVE_0_39_W<'_, RD_REPEAT_DATA0_SPEC> {
        RD_RESERVE_0_39_W::new(self, 7)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rd_reserve_0_53(&mut self) -> RD_RESERVE_0_53_W<'_, RD_REPEAT_DATA0_SPEC> {
        RD_RESERVE_0_53_W::new(self, 21)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn rd_reserve_0_60(&mut self) -> RD_RESERVE_0_60_W<'_, RD_REPEAT_DATA0_SPEC> {
        RD_RESERVE_0_60_W::new(self, 28)
    }
}
#[doc = "Represents rd_repeat_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_repeat_data0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_DATA0_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data0::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rd_repeat_data0::W`](W) writer structure"]
impl crate::Writable for RD_REPEAT_DATA0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RD_REPEAT_DATA0 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA0_SPEC {}
