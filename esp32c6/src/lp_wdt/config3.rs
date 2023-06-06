#[doc = "Register `CONFIG3` reader"]
pub struct R(crate::R<CONFIG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG3` writer"]
pub struct W(crate::W<CONFIG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG3_SPEC>;
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
impl From<crate::W<CONFIG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_STG2_HOLD` reader - need_des"]
pub type WDT_STG2_HOLD_R = crate::FieldReader<u32>;
#[doc = "Field `WDT_STG2_HOLD` writer - need_des"]
pub type WDT_STG2_HOLD_W<'a, const O: u8> = crate::FieldWriter<'a, CONFIG3_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn wdt_stg2_hold(&self) -> WDT_STG2_HOLD_R {
        WDT_STG2_HOLD_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG3")
            .field(
                "wdt_stg2_hold",
                &format_args!("{}", self.wdt_stg2_hold().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONFIG3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg2_hold(&mut self) -> WDT_STG2_HOLD_W<0> {
        WDT_STG2_HOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config3](index.html) module"]
pub struct CONFIG3_SPEC;
impl crate::RegisterSpec for CONFIG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config3::R](R) reader structure"]
impl crate::Readable for CONFIG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config3::W](W) writer structure"]
impl crate::Writable for CONFIG3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFIG3 to value 0x0fff"]
impl crate::Resettable for CONFIG3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff;
}
