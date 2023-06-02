#[doc = "Register `FUNC2_2` reader"]
pub struct R(crate::R<FUNC2_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FUNC2_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FUNC2_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FUNC2_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FUNC2_2` writer"]
pub struct W(crate::W<FUNC2_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FUNC2_2_SPEC>;
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
impl From<crate::W<FUNC2_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FUNC2_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC_FUNC1_MDSTAT` reader - *******Description***********"]
pub type SLC_FUNC1_MDSTAT_R = crate::BitReader;
#[doc = "Field `SLC_FUNC1_MDSTAT` writer - *******Description***********"]
pub type SLC_FUNC1_MDSTAT_W<'a, const O: u8> = crate::BitWriter<'a, FUNC2_2_SPEC, O>;
impl R {
    #[doc = "Bit 0 - *******Description***********"]
    #[inline(always)]
    pub fn slc_func1_mdstat(&self) -> SLC_FUNC1_MDSTAT_R {
        SLC_FUNC1_MDSTAT_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUNC2_2")
            .field(
                "slc_func1_mdstat",
                &format_args!("{}", self.slc_func1_mdstat().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FUNC2_2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc_func1_mdstat(&mut self) -> SLC_FUNC1_MDSTAT_W<0> {
        SLC_FUNC1_MDSTAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "*******Description***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [func2_2](index.html) module"]
pub struct FUNC2_2_SPEC;
impl crate::RegisterSpec for FUNC2_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [func2_2::R](R) reader structure"]
impl crate::Readable for FUNC2_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [func2_2::W](W) writer structure"]
impl crate::Writable for FUNC2_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FUNC2_2 to value 0x01"]
impl crate::Resettable for FUNC2_2_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
