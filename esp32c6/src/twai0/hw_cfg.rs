#[doc = "Register `HW_CFG` reader"]
pub struct R(crate::R<HW_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HW_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HW_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HW_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HW_CFG` writer"]
pub struct W(crate::W<HW_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HW_CFG_SPEC>;
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
impl From<crate::W<HW_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HW_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HW_STANDBY_EN` reader - Enable function that hardware control standby pin."]
pub type HW_STANDBY_EN_R = crate::BitReader;
#[doc = "Field `HW_STANDBY_EN` writer - Enable function that hardware control standby pin."]
pub type HW_STANDBY_EN_W<'a, const O: u8> = crate::BitWriter<'a, HW_CFG_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Enable function that hardware control standby pin."]
    #[inline(always)]
    pub fn hw_standby_en(&self) -> HW_STANDBY_EN_R {
        HW_STANDBY_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HW_CFG")
            .field(
                "hw_standby_en",
                &format_args!("{}", self.hw_standby_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HW_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Enable function that hardware control standby pin."]
    #[inline(always)]
    #[must_use]
    pub fn hw_standby_en(&mut self) -> HW_STANDBY_EN_W<0> {
        HW_STANDBY_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hardware configure standby pin.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hw_cfg](index.html) module"]
pub struct HW_CFG_SPEC;
impl crate::RegisterSpec for HW_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hw_cfg::R](R) reader structure"]
impl crate::Readable for HW_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hw_cfg::W](W) writer structure"]
impl crate::Writable for HW_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HW_CFG to value 0"]
impl crate::Resettable for HW_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
