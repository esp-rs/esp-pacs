#[doc = "Register `SWD_CONF` reader"]
pub type R = crate::R<SWD_CONF_SPEC>;
#[doc = "Register `SWD_CONF` writer"]
pub type W = crate::W<SWD_CONF_SPEC>;
#[doc = "Field `SWD_RESET_FLAG` reader - Represents the SWD whether has generated the reset signal.\\\\0 :No \\\\1: Yes"]
pub type SWD_RESET_FLAG_R = crate::BitReader;
#[doc = "Field `SWD_AUTO_FEED_EN` reader - Configure this bit to enable to feed SWD automatically by hardware. \\\\0: Disable \\\\1: Enable"]
pub type SWD_AUTO_FEED_EN_R = crate::BitReader;
#[doc = "Field `SWD_AUTO_FEED_EN` writer - Configure this bit to enable to feed SWD automatically by hardware. \\\\0: Disable \\\\1: Enable"]
pub type SWD_AUTO_FEED_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWD_RST_FLAG_CLR` writer - Configure this bit to clear SWD reset flag.\\\\ 0:Invalid \\\\ 1: Clear the reset flag"]
pub type SWD_RST_FLAG_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWD_SIGNAL_WIDTH` reader - Configure the SWD signal length that output to analog circuit. \\\\ Measurement unit: LP\\_DYN\\_FAST\\_CLK"]
pub type SWD_SIGNAL_WIDTH_R = crate::FieldReader<u16>;
#[doc = "Field `SWD_SIGNAL_WIDTH` writer - Configure the SWD signal length that output to analog circuit. \\\\ Measurement unit: LP\\_DYN\\_FAST\\_CLK"]
pub type SWD_SIGNAL_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SWD_DISABLE` reader - Configure this bit to disable the SWD.\\\\ 0: Enable the SWD\\\\ 1: Disable the SWD"]
pub type SWD_DISABLE_R = crate::BitReader;
#[doc = "Field `SWD_DISABLE` writer - Configure this bit to disable the SWD.\\\\ 0: Enable the SWD\\\\ 1: Disable the SWD"]
pub type SWD_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWD_FEED` writer - Configure this bit to feed the SWD.\\\\ 0: Invalid\\\\ 1: Feed SWD"]
pub type SWD_FEED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Represents the SWD whether has generated the reset signal.\\\\0 :No \\\\1: Yes"]
    #[inline(always)]
    pub fn swd_reset_flag(&self) -> SWD_RESET_FLAG_R {
        SWD_RESET_FLAG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 18 - Configure this bit to enable to feed SWD automatically by hardware. \\\\0: Disable \\\\1: Enable"]
    #[inline(always)]
    pub fn swd_auto_feed_en(&self) -> SWD_AUTO_FEED_EN_R {
        SWD_AUTO_FEED_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:29 - Configure the SWD signal length that output to analog circuit. \\\\ Measurement unit: LP\\_DYN\\_FAST\\_CLK"]
    #[inline(always)]
    pub fn swd_signal_width(&self) -> SWD_SIGNAL_WIDTH_R {
        SWD_SIGNAL_WIDTH_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bit 30 - Configure this bit to disable the SWD.\\\\ 0: Enable the SWD\\\\ 1: Disable the SWD"]
    #[inline(always)]
    pub fn swd_disable(&self) -> SWD_DISABLE_R {
        SWD_DISABLE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWD_CONF")
            .field("swd_reset_flag", &self.swd_reset_flag())
            .field("swd_auto_feed_en", &self.swd_auto_feed_en())
            .field("swd_signal_width", &self.swd_signal_width())
            .field("swd_disable", &self.swd_disable())
            .finish()
    }
}
impl W {
    #[doc = "Bit 18 - Configure this bit to enable to feed SWD automatically by hardware. \\\\0: Disable \\\\1: Enable"]
    #[inline(always)]
    pub fn swd_auto_feed_en(&mut self) -> SWD_AUTO_FEED_EN_W<SWD_CONF_SPEC> {
        SWD_AUTO_FEED_EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - Configure this bit to clear SWD reset flag.\\\\ 0:Invalid \\\\ 1: Clear the reset flag"]
    #[inline(always)]
    pub fn swd_rst_flag_clr(&mut self) -> SWD_RST_FLAG_CLR_W<SWD_CONF_SPEC> {
        SWD_RST_FLAG_CLR_W::new(self, 19)
    }
    #[doc = "Bits 20:29 - Configure the SWD signal length that output to analog circuit. \\\\ Measurement unit: LP\\_DYN\\_FAST\\_CLK"]
    #[inline(always)]
    pub fn swd_signal_width(&mut self) -> SWD_SIGNAL_WIDTH_W<SWD_CONF_SPEC> {
        SWD_SIGNAL_WIDTH_W::new(self, 20)
    }
    #[doc = "Bit 30 - Configure this bit to disable the SWD.\\\\ 0: Enable the SWD\\\\ 1: Disable the SWD"]
    #[inline(always)]
    pub fn swd_disable(&mut self) -> SWD_DISABLE_W<SWD_CONF_SPEC> {
        SWD_DISABLE_W::new(self, 30)
    }
    #[doc = "Bit 31 - Configure this bit to feed the SWD.\\\\ 0: Invalid\\\\ 1: Feed SWD"]
    #[inline(always)]
    pub fn swd_feed(&mut self) -> SWD_FEED_W<SWD_CONF_SPEC> {
        SWD_FEED_W::new(self, 31)
    }
}
#[doc = "Configure the SWD operation\n\nYou can [`read`](crate::Reg::read) this register and get [`swd_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swd_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWD_CONF_SPEC;
impl crate::RegisterSpec for SWD_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swd_conf::R`](R) reader structure"]
impl crate::Readable for SWD_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`swd_conf::W`](W) writer structure"]
impl crate::Writable for SWD_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWD_CONF to value 0x12c0_0000"]
impl crate::Resettable for SWD_CONF_SPEC {
    const RESET_VALUE: u32 = 0x12c0_0000;
}
