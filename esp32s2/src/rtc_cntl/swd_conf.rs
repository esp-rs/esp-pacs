#[doc = "Register `SWD_CONF` reader"]
pub type R = crate::R<SWD_CONF_SPEC>;
#[doc = "Register `SWD_CONF` writer"]
pub type W = crate::W<SWD_CONF_SPEC>;
#[doc = "Field `SWD_RESET_FLAG` reader - Indicates the super watchdog reset flag."]
pub type SWD_RESET_FLAG_R = crate::BitReader;
#[doc = "Field `SWD_FEED_INT` reader - Receiving this interrupt leads to feeding the super watchdog via SW."]
pub type SWD_FEED_INT_R = crate::BitReader;
#[doc = "Field `SWD_SIGNAL_WIDTH` reader - Adjusts the signal width sent to the super watchdog."]
pub type SWD_SIGNAL_WIDTH_R = crate::FieldReader<u16>;
#[doc = "Field `SWD_SIGNAL_WIDTH` writer - Adjusts the signal width sent to the super watchdog."]
pub type SWD_SIGNAL_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SWD_RST_FLAG_CLR` writer - Set to reset the super watchdog reset flag."]
pub type SWD_RST_FLAG_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWD_FEED` writer - Set to feed the super watchdog via SW."]
pub type SWD_FEED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWD_DISABLE` reader - Set this bit to disable super watchdog."]
pub type SWD_DISABLE_R = crate::BitReader;
#[doc = "Field `SWD_DISABLE` writer - Set this bit to disable super watchdog."]
pub type SWD_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWD_AUTO_FEED_EN` reader - Set this bit to enable automatic watchdog feeding upon interrupts."]
pub type SWD_AUTO_FEED_EN_R = crate::BitReader;
#[doc = "Field `SWD_AUTO_FEED_EN` writer - Set this bit to enable automatic watchdog feeding upon interrupts."]
pub type SWD_AUTO_FEED_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Indicates the super watchdog reset flag."]
    #[inline(always)]
    pub fn swd_reset_flag(&self) -> SWD_RESET_FLAG_R {
        SWD_RESET_FLAG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receiving this interrupt leads to feeding the super watchdog via SW."]
    #[inline(always)]
    pub fn swd_feed_int(&self) -> SWD_FEED_INT_R {
        SWD_FEED_INT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 18:27 - Adjusts the signal width sent to the super watchdog."]
    #[inline(always)]
    pub fn swd_signal_width(&self) -> SWD_SIGNAL_WIDTH_R {
        SWD_SIGNAL_WIDTH_R::new(((self.bits >> 18) & 0x03ff) as u16)
    }
    #[doc = "Bit 30 - Set this bit to disable super watchdog."]
    #[inline(always)]
    pub fn swd_disable(&self) -> SWD_DISABLE_R {
        SWD_DISABLE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Set this bit to enable automatic watchdog feeding upon interrupts."]
    #[inline(always)]
    pub fn swd_auto_feed_en(&self) -> SWD_AUTO_FEED_EN_R {
        SWD_AUTO_FEED_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWD_CONF")
            .field("swd_reset_flag", &self.swd_reset_flag().bit())
            .field("swd_feed_int", &self.swd_feed_int().bit())
            .field("swd_signal_width", &self.swd_signal_width().bits())
            .field("swd_disable", &self.swd_disable().bit())
            .field("swd_auto_feed_en", &self.swd_auto_feed_en().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SWD_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 18:27 - Adjusts the signal width sent to the super watchdog."]
    #[inline(always)]
    #[must_use]
    pub fn swd_signal_width(&mut self) -> SWD_SIGNAL_WIDTH_W<SWD_CONF_SPEC> {
        SWD_SIGNAL_WIDTH_W::new(self, 18)
    }
    #[doc = "Bit 28 - Set to reset the super watchdog reset flag."]
    #[inline(always)]
    #[must_use]
    pub fn swd_rst_flag_clr(&mut self) -> SWD_RST_FLAG_CLR_W<SWD_CONF_SPEC> {
        SWD_RST_FLAG_CLR_W::new(self, 28)
    }
    #[doc = "Bit 29 - Set to feed the super watchdog via SW."]
    #[inline(always)]
    #[must_use]
    pub fn swd_feed(&mut self) -> SWD_FEED_W<SWD_CONF_SPEC> {
        SWD_FEED_W::new(self, 29)
    }
    #[doc = "Bit 30 - Set this bit to disable super watchdog."]
    #[inline(always)]
    #[must_use]
    pub fn swd_disable(&mut self) -> SWD_DISABLE_W<SWD_CONF_SPEC> {
        SWD_DISABLE_W::new(self, 30)
    }
    #[doc = "Bit 31 - Set this bit to enable automatic watchdog feeding upon interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn swd_auto_feed_en(&mut self) -> SWD_AUTO_FEED_EN_W<SWD_CONF_SPEC> {
        SWD_AUTO_FEED_EN_W::new(self, 31)
    }
}
#[doc = "Super watchdog configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swd_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swd_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWD_CONF_SPEC;
impl crate::RegisterSpec for SWD_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swd_conf::R`](R) reader structure"]
impl crate::Readable for SWD_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`swd_conf::W`](W) writer structure"]
impl crate::Writable for SWD_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWD_CONF to value 0x04b0_0000"]
impl crate::Resettable for SWD_CONF_SPEC {
    const RESET_VALUE: u32 = 0x04b0_0000;
}
