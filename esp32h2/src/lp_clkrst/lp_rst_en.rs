#[doc = "Register `LP_RST_EN` reader"]
pub struct R(crate::R<LP_RST_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LP_RST_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LP_RST_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LP_RST_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LP_RST_EN` writer"]
pub struct W(crate::W<LP_RST_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LP_RST_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<LP_RST_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LP_RST_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AON_EFUSE_CORE_RESET_EN` reader - need_des"]
pub type AON_EFUSE_CORE_RESET_EN_R = crate::BitReader;
#[doc = "Field `AON_EFUSE_CORE_RESET_EN` writer - need_des"]
pub type AON_EFUSE_CORE_RESET_EN_W<'a, const O: u8> = crate::BitWriter<'a, LP_RST_EN_SPEC, O>;
#[doc = "Field `LP_TIMER_RESET_EN` reader - need_des"]
pub type LP_TIMER_RESET_EN_R = crate::BitReader;
#[doc = "Field `LP_TIMER_RESET_EN` writer - need_des"]
pub type LP_TIMER_RESET_EN_W<'a, const O: u8> = crate::BitWriter<'a, LP_RST_EN_SPEC, O>;
#[doc = "Field `WDT_RESET_EN` reader - need_des"]
pub type WDT_RESET_EN_R = crate::BitReader;
#[doc = "Field `WDT_RESET_EN` writer - need_des"]
pub type WDT_RESET_EN_W<'a, const O: u8> = crate::BitWriter<'a, LP_RST_EN_SPEC, O>;
#[doc = "Field `ANA_PERI_RESET_EN` reader - need_des"]
pub type ANA_PERI_RESET_EN_R = crate::BitReader;
#[doc = "Field `ANA_PERI_RESET_EN` writer - need_des"]
pub type ANA_PERI_RESET_EN_W<'a, const O: u8> = crate::BitWriter<'a, LP_RST_EN_SPEC, O>;
impl R {
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn aon_efuse_core_reset_en(&self) -> AON_EFUSE_CORE_RESET_EN_R {
        AON_EFUSE_CORE_RESET_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn lp_timer_reset_en(&self) -> LP_TIMER_RESET_EN_R {
        LP_TIMER_RESET_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn wdt_reset_en(&self) -> WDT_RESET_EN_R {
        WDT_RESET_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn ana_peri_reset_en(&self) -> ANA_PERI_RESET_EN_R {
        ANA_PERI_RESET_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_RST_EN")
            .field(
                "aon_efuse_core_reset_en",
                &format_args!("{}", self.aon_efuse_core_reset_en().bit()),
            )
            .field(
                "lp_timer_reset_en",
                &format_args!("{}", self.lp_timer_reset_en().bit()),
            )
            .field(
                "wdt_reset_en",
                &format_args!("{}", self.wdt_reset_en().bit()),
            )
            .field(
                "ana_peri_reset_en",
                &format_args!("{}", self.ana_peri_reset_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_RST_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn aon_efuse_core_reset_en(&mut self) -> AON_EFUSE_CORE_RESET_EN_W<28> {
        AON_EFUSE_CORE_RESET_EN_W::new(self)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_timer_reset_en(&mut self) -> LP_TIMER_RESET_EN_W<29> {
        LP_TIMER_RESET_EN_W::new(self)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_reset_en(&mut self) -> WDT_RESET_EN_W<30> {
        WDT_RESET_EN_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn ana_peri_reset_en(&mut self) -> ANA_PERI_RESET_EN_W<31> {
        ANA_PERI_RESET_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lp_rst_en](index.html) module"]
pub struct LP_RST_EN_SPEC;
impl crate::RegisterSpec for LP_RST_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lp_rst_en::R](R) reader structure"]
impl crate::Readable for LP_RST_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lp_rst_en::W](W) writer structure"]
impl crate::Writable for LP_RST_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LP_RST_EN to value 0"]
impl crate::Resettable for LP_RST_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
