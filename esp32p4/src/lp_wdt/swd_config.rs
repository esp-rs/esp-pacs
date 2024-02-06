#[doc = "Register `SWD_CONFIG` reader"]
pub type R = crate::R<SWD_CONFIG_SPEC>;
#[doc = "Register `SWD_CONFIG` writer"]
pub type W = crate::W<SWD_CONFIG_SPEC>;
#[doc = "Field `SWD_RESET_FLAG` reader - need_des"]
pub type SWD_RESET_FLAG_R = crate::BitReader;
#[doc = "Field `SWD_AUTO_FEED_EN` reader - need_des"]
pub type SWD_AUTO_FEED_EN_R = crate::BitReader;
#[doc = "Field `SWD_AUTO_FEED_EN` writer - need_des"]
pub type SWD_AUTO_FEED_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWD_RST_FLAG_CLR` writer - need_des"]
pub type SWD_RST_FLAG_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWD_SIGNAL_WIDTH` reader - need_des"]
pub type SWD_SIGNAL_WIDTH_R = crate::FieldReader<u16>;
#[doc = "Field `SWD_SIGNAL_WIDTH` writer - need_des"]
pub type SWD_SIGNAL_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SWD_DISABLE` reader - need_des"]
pub type SWD_DISABLE_R = crate::BitReader;
#[doc = "Field `SWD_DISABLE` writer - need_des"]
pub type SWD_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWD_FEED` writer - need_des"]
pub type SWD_FEED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn swd_reset_flag(&self) -> SWD_RESET_FLAG_R {
        SWD_RESET_FLAG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 18 - need_des"]
    #[inline(always)]
    pub fn swd_auto_feed_en(&self) -> SWD_AUTO_FEED_EN_R {
        SWD_AUTO_FEED_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:29 - need_des"]
    #[inline(always)]
    pub fn swd_signal_width(&self) -> SWD_SIGNAL_WIDTH_R {
        SWD_SIGNAL_WIDTH_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn swd_disable(&self) -> SWD_DISABLE_R {
        SWD_DISABLE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWD_CONFIG")
            .field(
                "swd_reset_flag",
                &format_args!("{}", self.swd_reset_flag().bit()),
            )
            .field(
                "swd_auto_feed_en",
                &format_args!("{}", self.swd_auto_feed_en().bit()),
            )
            .field(
                "swd_signal_width",
                &format_args!("{}", self.swd_signal_width().bits()),
            )
            .field("swd_disable", &format_args!("{}", self.swd_disable().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SWD_CONFIG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 18 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn swd_auto_feed_en(&mut self) -> SWD_AUTO_FEED_EN_W<SWD_CONFIG_SPEC> {
        SWD_AUTO_FEED_EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn swd_rst_flag_clr(&mut self) -> SWD_RST_FLAG_CLR_W<SWD_CONFIG_SPEC> {
        SWD_RST_FLAG_CLR_W::new(self, 19)
    }
    #[doc = "Bits 20:29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn swd_signal_width(&mut self) -> SWD_SIGNAL_WIDTH_W<SWD_CONFIG_SPEC> {
        SWD_SIGNAL_WIDTH_W::new(self, 20)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn swd_disable(&mut self) -> SWD_DISABLE_W<SWD_CONFIG_SPEC> {
        SWD_DISABLE_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn swd_feed(&mut self) -> SWD_FEED_W<SWD_CONFIG_SPEC> {
        SWD_FEED_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swd_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swd_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWD_CONFIG_SPEC;
impl crate::RegisterSpec for SWD_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swd_config::R`](R) reader structure"]
impl crate::Readable for SWD_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`swd_config::W`](W) writer structure"]
impl crate::Writable for SWD_CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWD_CONFIG to value 0x12c0_0000"]
impl crate::Resettable for SWD_CONFIG_SPEC {
    const RESET_VALUE: u32 = 0x12c0_0000;
}
