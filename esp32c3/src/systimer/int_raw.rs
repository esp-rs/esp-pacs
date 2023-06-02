#[doc = "Register `INT_RAW` reader"]
pub struct R(crate::R<INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_RAW` writer"]
pub struct W(crate::W<INT_RAW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_RAW_SPEC>;
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
impl From<crate::W<INT_RAW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_RAW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TARGET0_INT_RAW` reader - interupt0 raw"]
pub type TARGET0_INT_RAW_R = crate::BitReader;
#[doc = "Field `TARGET0_INT_RAW` writer - interupt0 raw"]
pub type TARGET0_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `TARGET1_INT_RAW` reader - interupt1 raw"]
pub type TARGET1_INT_RAW_R = crate::BitReader;
#[doc = "Field `TARGET1_INT_RAW` writer - interupt1 raw"]
pub type TARGET1_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `TARGET2_INT_RAW` reader - interupt2 raw"]
pub type TARGET2_INT_RAW_R = crate::BitReader;
#[doc = "Field `TARGET2_INT_RAW` writer - interupt2 raw"]
pub type TARGET2_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
impl R {
    #[doc = "Bit 0 - interupt0 raw"]
    #[inline(always)]
    pub fn target0_int_raw(&self) -> TARGET0_INT_RAW_R {
        TARGET0_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - interupt1 raw"]
    #[inline(always)]
    pub fn target1_int_raw(&self) -> TARGET1_INT_RAW_R {
        TARGET1_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - interupt2 raw"]
    #[inline(always)]
    pub fn target2_int_raw(&self) -> TARGET2_INT_RAW_R {
        TARGET2_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field(
                "target0_int_raw",
                &format_args!("{}", self.target0_int_raw().bit()),
            )
            .field(
                "target1_int_raw",
                &format_args!("{}", self.target1_int_raw().bit()),
            )
            .field(
                "target2_int_raw",
                &format_args!("{}", self.target2_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - interupt0 raw"]
    #[inline(always)]
    #[must_use]
    pub fn target0_int_raw(&mut self) -> TARGET0_INT_RAW_W<0> {
        TARGET0_INT_RAW_W::new(self)
    }
    #[doc = "Bit 1 - interupt1 raw"]
    #[inline(always)]
    #[must_use]
    pub fn target1_int_raw(&mut self) -> TARGET1_INT_RAW_W<1> {
        TARGET1_INT_RAW_W::new(self)
    }
    #[doc = "Bit 2 - interupt2 raw"]
    #[inline(always)]
    #[must_use]
    pub fn target2_int_raw(&mut self) -> TARGET2_INT_RAW_W<2> {
        TARGET2_INT_RAW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSTIMER_INT_RAW.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_raw::W](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
