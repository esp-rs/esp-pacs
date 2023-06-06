#[doc = "Register `POWER_CK_WAIT_CNTL` reader"]
pub struct R(crate::R<POWER_CK_WAIT_CNTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POWER_CK_WAIT_CNTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POWER_CK_WAIT_CNTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POWER_CK_WAIT_CNTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POWER_CK_WAIT_CNTL` writer"]
pub struct W(crate::W<POWER_CK_WAIT_CNTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POWER_CK_WAIT_CNTL_SPEC>;
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
impl From<crate::W<POWER_CK_WAIT_CNTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POWER_CK_WAIT_CNTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAIT_XTL_STABLE` reader - need_des"]
pub type WAIT_XTL_STABLE_R = crate::FieldReader<u16>;
#[doc = "Field `WAIT_XTL_STABLE` writer - need_des"]
pub type WAIT_XTL_STABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, POWER_CK_WAIT_CNTL_SPEC, 16, O, u16>;
#[doc = "Field `WAIT_PLL_STABLE` reader - need_des"]
pub type WAIT_PLL_STABLE_R = crate::FieldReader<u16>;
#[doc = "Field `WAIT_PLL_STABLE` writer - need_des"]
pub type WAIT_PLL_STABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, POWER_CK_WAIT_CNTL_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn wait_xtl_stable(&self) -> WAIT_XTL_STABLE_R {
        WAIT_XTL_STABLE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - need_des"]
    #[inline(always)]
    pub fn wait_pll_stable(&self) -> WAIT_PLL_STABLE_R {
        WAIT_PLL_STABLE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWER_CK_WAIT_CNTL")
            .field(
                "wait_xtl_stable",
                &format_args!("{}", self.wait_xtl_stable().bits()),
            )
            .field(
                "wait_pll_stable",
                &format_args!("{}", self.wait_pll_stable().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<POWER_CK_WAIT_CNTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn wait_xtl_stable(&mut self) -> WAIT_XTL_STABLE_W<0> {
        WAIT_XTL_STABLE_W::new(self)
    }
    #[doc = "Bits 16:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn wait_pll_stable(&mut self) -> WAIT_PLL_STABLE_W<16> {
        WAIT_PLL_STABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power_ck_wait_cntl](index.html) module"]
pub struct POWER_CK_WAIT_CNTL_SPEC;
impl crate::RegisterSpec for POWER_CK_WAIT_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [power_ck_wait_cntl::R](R) reader structure"]
impl crate::Readable for POWER_CK_WAIT_CNTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [power_ck_wait_cntl::W](W) writer structure"]
impl crate::Writable for POWER_CK_WAIT_CNTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POWER_CK_WAIT_CNTL to value 0x0100_0100"]
impl crate::Resettable for POWER_CK_WAIT_CNTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0100;
}
