#[doc = "Register `SW_STANDBY_CFG` reader"]
pub struct R(crate::R<SW_STANDBY_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SW_STANDBY_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SW_STANDBY_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SW_STANDBY_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SW_STANDBY_CFG` writer"]
pub struct W(crate::W<SW_STANDBY_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SW_STANDBY_CFG_SPEC>;
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
impl From<crate::W<SW_STANDBY_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SW_STANDBY_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_STANDBY_EN` reader - Enable standby pin."]
pub type SW_STANDBY_EN_R = crate::BitReader;
#[doc = "Field `SW_STANDBY_EN` writer - Enable standby pin."]
pub type SW_STANDBY_EN_W<'a, const O: u8> = crate::BitWriter<'a, SW_STANDBY_CFG_SPEC, O>;
#[doc = "Field `SW_STANDBY_CLR` reader - Clear standby pin."]
pub type SW_STANDBY_CLR_R = crate::BitReader;
#[doc = "Field `SW_STANDBY_CLR` writer - Clear standby pin."]
pub type SW_STANDBY_CLR_W<'a, const O: u8> = crate::BitWriter<'a, SW_STANDBY_CFG_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Enable standby pin."]
    #[inline(always)]
    pub fn sw_standby_en(&self) -> SW_STANDBY_EN_R {
        SW_STANDBY_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear standby pin."]
    #[inline(always)]
    pub fn sw_standby_clr(&self) -> SW_STANDBY_CLR_R {
        SW_STANDBY_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SW_STANDBY_CFG")
            .field(
                "sw_standby_en",
                &format_args!("{}", self.sw_standby_en().bit()),
            )
            .field(
                "sw_standby_clr",
                &format_args!("{}", self.sw_standby_clr().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SW_STANDBY_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Enable standby pin."]
    #[inline(always)]
    #[must_use]
    pub fn sw_standby_en(&mut self) -> SW_STANDBY_EN_W<0> {
        SW_STANDBY_EN_W::new(self)
    }
    #[doc = "Bit 1 - Clear standby pin."]
    #[inline(always)]
    #[must_use]
    pub fn sw_standby_clr(&mut self) -> SW_STANDBY_CLR_W<1> {
        SW_STANDBY_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software configure standby pin directly.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_standby_cfg](index.html) module"]
pub struct SW_STANDBY_CFG_SPEC;
impl crate::RegisterSpec for SW_STANDBY_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sw_standby_cfg::R](R) reader structure"]
impl crate::Readable for SW_STANDBY_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sw_standby_cfg::W](W) writer structure"]
impl crate::Writable for SW_STANDBY_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SW_STANDBY_CFG to value 0x02"]
impl crate::Resettable for SW_STANDBY_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
