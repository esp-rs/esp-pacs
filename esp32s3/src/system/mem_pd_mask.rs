#[doc = "Register `MEM_PD_MASK` reader"]
pub struct R(crate::R<MEM_PD_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_PD_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_PD_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_PD_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEM_PD_MASK` writer"]
pub struct W(crate::W<MEM_PD_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEM_PD_MASK_SPEC>;
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
impl From<crate::W<MEM_PD_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEM_PD_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSLP_MEM_PD_MASK` reader - Set 1 to mask memory power down."]
pub type LSLP_MEM_PD_MASK_R = crate::BitReader;
#[doc = "Field `LSLP_MEM_PD_MASK` writer - Set 1 to mask memory power down."]
pub type LSLP_MEM_PD_MASK_W<'a, const O: u8> = crate::BitWriter<'a, MEM_PD_MASK_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Set 1 to mask memory power down."]
    #[inline(always)]
    pub fn lslp_mem_pd_mask(&self) -> LSLP_MEM_PD_MASK_R {
        LSLP_MEM_PD_MASK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_PD_MASK")
            .field(
                "lslp_mem_pd_mask",
                &format_args!("{}", self.lslp_mem_pd_mask().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MEM_PD_MASK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to mask memory power down."]
    #[inline(always)]
    #[must_use]
    pub fn lslp_mem_pd_mask(&mut self) -> LSLP_MEM_PD_MASK_W<0> {
        LSLP_MEM_PD_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "memory power down mask configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_pd_mask](index.html) module"]
pub struct MEM_PD_MASK_SPEC;
impl crate::RegisterSpec for MEM_PD_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_pd_mask::R](R) reader structure"]
impl crate::Readable for MEM_PD_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mem_pd_mask::W](W) writer structure"]
impl crate::Writable for MEM_PD_MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MEM_PD_MASK to value 0x01"]
impl crate::Resettable for MEM_PD_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
