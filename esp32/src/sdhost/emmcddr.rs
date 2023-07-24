#[doc = "Register `EMMCDDR` reader"]
pub struct R(crate::R<EMMCDDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMMCDDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMMCDDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMMCDDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMMCDDR` writer"]
pub struct W(crate::W<EMMCDDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMMCDDR_SPEC>;
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
impl From<crate::W<EMMCDDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMMCDDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HALFSTARTBIT` reader - Control for start bit detection mechanism duration of start bit.Each bit refers to one slot.Set this bit to 1 for eMMC4.5 and above,set to 0 for SD applications.For eMMC4.5,start bit can be: 1'b0-Full cycle. 1'b1-less than one full cycle."]
pub type HALFSTARTBIT_R = crate::FieldReader;
#[doc = "Field `HALFSTARTBIT` writer - Control for start bit detection mechanism duration of start bit.Each bit refers to one slot.Set this bit to 1 for eMMC4.5 and above,set to 0 for SD applications.For eMMC4.5,start bit can be: 1'b0-Full cycle. 1'b1-less than one full cycle."]
pub type HALFSTARTBIT_W<'a, const O: u8> = crate::FieldWriter<'a, EMMCDDR_SPEC, 2, O>;
#[doc = "Field `HS400_MODE` reader - Set 1 to enable HS400 mode."]
pub type HS400_MODE_R = crate::BitReader;
#[doc = "Field `HS400_MODE` writer - Set 1 to enable HS400 mode."]
pub type HS400_MODE_W<'a, const O: u8> = crate::BitWriter<'a, EMMCDDR_SPEC, O>;
impl R {
    #[doc = "Bits 0:1 - Control for start bit detection mechanism duration of start bit.Each bit refers to one slot.Set this bit to 1 for eMMC4.5 and above,set to 0 for SD applications.For eMMC4.5,start bit can be: 1'b0-Full cycle. 1'b1-less than one full cycle."]
    #[inline(always)]
    pub fn halfstartbit(&self) -> HALFSTARTBIT_R {
        HALFSTARTBIT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 31 - Set 1 to enable HS400 mode."]
    #[inline(always)]
    pub fn hs400_mode(&self) -> HS400_MODE_R {
        HS400_MODE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMMCDDR")
            .field(
                "halfstartbit",
                &format_args!("{}", self.halfstartbit().bits()),
            )
            .field("hs400_mode", &format_args!("{}", self.hs400_mode().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EMMCDDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Control for start bit detection mechanism duration of start bit.Each bit refers to one slot.Set this bit to 1 for eMMC4.5 and above,set to 0 for SD applications.For eMMC4.5,start bit can be: 1'b0-Full cycle. 1'b1-less than one full cycle."]
    #[inline(always)]
    #[must_use]
    pub fn halfstartbit(&mut self) -> HALFSTARTBIT_W<0> {
        HALFSTARTBIT_W::new(self)
    }
    #[doc = "Bit 31 - Set 1 to enable HS400 mode."]
    #[inline(always)]
    #[must_use]
    pub fn hs400_mode(&mut self) -> HS400_MODE_W<31> {
        HS400_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eMMC DDR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emmcddr](index.html) module"]
pub struct EMMCDDR_SPEC;
impl crate::RegisterSpec for EMMCDDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emmcddr::R](R) reader structure"]
impl crate::Readable for EMMCDDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emmcddr::W](W) writer structure"]
impl crate::Writable for EMMCDDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EMMCDDR to value 0"]
impl crate::Resettable for EMMCDDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
