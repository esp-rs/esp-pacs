#[doc = "Register `PG_CTRL` reader"]
pub struct R(crate::R<PG_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PG_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PG_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PG_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PG_CTRL` writer"]
pub struct W(crate::W<PG_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PG_CTRL_SPEC>;
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
impl From<crate::W<PG_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PG_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POWER_GLITCH_DSENSE` reader - power glitch desense"]
pub type POWER_GLITCH_DSENSE_R = crate::FieldReader;
#[doc = "Field `POWER_GLITCH_DSENSE` writer - power glitch desense"]
pub type POWER_GLITCH_DSENSE_W<'a, const O: u8> = crate::FieldWriter<'a, PG_CTRL_SPEC, 2, O>;
#[doc = "Field `POWER_GLITCH_FORCE_PD` reader - force disable power glitch"]
pub type POWER_GLITCH_FORCE_PD_R = crate::BitReader;
#[doc = "Field `POWER_GLITCH_FORCE_PD` writer - force disable power glitch"]
pub type POWER_GLITCH_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, PG_CTRL_SPEC, O>;
#[doc = "Field `POWER_GLITCH_FORCE_PU` reader - force enable power glitch"]
pub type POWER_GLITCH_FORCE_PU_R = crate::BitReader;
#[doc = "Field `POWER_GLITCH_FORCE_PU` writer - force enable power glitch"]
pub type POWER_GLITCH_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, PG_CTRL_SPEC, O>;
#[doc = "Field `POWER_GLITCH_EFUSE_SEL` reader - use efuse value control power glitch enable"]
pub type POWER_GLITCH_EFUSE_SEL_R = crate::BitReader;
#[doc = "Field `POWER_GLITCH_EFUSE_SEL` writer - use efuse value control power glitch enable"]
pub type POWER_GLITCH_EFUSE_SEL_W<'a, const O: u8> = crate::BitWriter<'a, PG_CTRL_SPEC, O>;
#[doc = "Field `POWER_GLITCH_EN` reader - enable power glitch"]
pub type POWER_GLITCH_EN_R = crate::BitReader;
#[doc = "Field `POWER_GLITCH_EN` writer - enable power glitch"]
pub type POWER_GLITCH_EN_W<'a, const O: u8> = crate::BitWriter<'a, PG_CTRL_SPEC, O>;
impl R {
    #[doc = "Bits 26:27 - power glitch desense"]
    #[inline(always)]
    pub fn power_glitch_dsense(&self) -> POWER_GLITCH_DSENSE_R {
        POWER_GLITCH_DSENSE_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - force disable power glitch"]
    #[inline(always)]
    pub fn power_glitch_force_pd(&self) -> POWER_GLITCH_FORCE_PD_R {
        POWER_GLITCH_FORCE_PD_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - force enable power glitch"]
    #[inline(always)]
    pub fn power_glitch_force_pu(&self) -> POWER_GLITCH_FORCE_PU_R {
        POWER_GLITCH_FORCE_PU_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - use efuse value control power glitch enable"]
    #[inline(always)]
    pub fn power_glitch_efuse_sel(&self) -> POWER_GLITCH_EFUSE_SEL_R {
        POWER_GLITCH_EFUSE_SEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - enable power glitch"]
    #[inline(always)]
    pub fn power_glitch_en(&self) -> POWER_GLITCH_EN_R {
        POWER_GLITCH_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PG_CTRL")
            .field(
                "power_glitch_dsense",
                &format_args!("{}", self.power_glitch_dsense().bits()),
            )
            .field(
                "power_glitch_force_pd",
                &format_args!("{}", self.power_glitch_force_pd().bit()),
            )
            .field(
                "power_glitch_force_pu",
                &format_args!("{}", self.power_glitch_force_pu().bit()),
            )
            .field(
                "power_glitch_efuse_sel",
                &format_args!("{}", self.power_glitch_efuse_sel().bit()),
            )
            .field(
                "power_glitch_en",
                &format_args!("{}", self.power_glitch_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PG_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 26:27 - power glitch desense"]
    #[inline(always)]
    #[must_use]
    pub fn power_glitch_dsense(&mut self) -> POWER_GLITCH_DSENSE_W<26> {
        POWER_GLITCH_DSENSE_W::new(self)
    }
    #[doc = "Bit 28 - force disable power glitch"]
    #[inline(always)]
    #[must_use]
    pub fn power_glitch_force_pd(&mut self) -> POWER_GLITCH_FORCE_PD_W<28> {
        POWER_GLITCH_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 29 - force enable power glitch"]
    #[inline(always)]
    #[must_use]
    pub fn power_glitch_force_pu(&mut self) -> POWER_GLITCH_FORCE_PU_W<29> {
        POWER_GLITCH_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 30 - use efuse value control power glitch enable"]
    #[inline(always)]
    #[must_use]
    pub fn power_glitch_efuse_sel(&mut self) -> POWER_GLITCH_EFUSE_SEL_W<30> {
        POWER_GLITCH_EFUSE_SEL_W::new(self)
    }
    #[doc = "Bit 31 - enable power glitch"]
    #[inline(always)]
    #[must_use]
    pub fn power_glitch_en(&mut self) -> POWER_GLITCH_EN_W<31> {
        POWER_GLITCH_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pg_ctrl](index.html) module"]
pub struct PG_CTRL_SPEC;
impl crate::RegisterSpec for PG_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pg_ctrl::R](R) reader structure"]
impl crate::Readable for PG_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pg_ctrl::W](W) writer structure"]
impl crate::Writable for PG_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PG_CTRL to value 0"]
impl crate::Resettable for PG_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
