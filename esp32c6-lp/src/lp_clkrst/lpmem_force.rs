#[doc = "Register `LPMEM_FORCE` reader"]
pub struct R(crate::R<LPMEM_FORCE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPMEM_FORCE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPMEM_FORCE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPMEM_FORCE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPMEM_FORCE` writer"]
pub struct W(crate::W<LPMEM_FORCE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPMEM_FORCE_SPEC>;
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
impl From<crate::W<LPMEM_FORCE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPMEM_FORCE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPMEM_CLK_FORCE_ON` reader - need_des"]
pub type LPMEM_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `LPMEM_CLK_FORCE_ON` writer - need_des"]
pub type LPMEM_CLK_FORCE_ON_W<'a, const O: u8> = crate::BitWriter<'a, LPMEM_FORCE_SPEC, O>;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lpmem_clk_force_on(&self) -> LPMEM_CLK_FORCE_ON_R {
        LPMEM_CLK_FORCE_ON_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPMEM_FORCE")
            .field(
                "lpmem_clk_force_on",
                &format_args!("{}", self.lpmem_clk_force_on().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LPMEM_FORCE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lpmem_clk_force_on(&mut self) -> LPMEM_CLK_FORCE_ON_W<31> {
        LPMEM_CLK_FORCE_ON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmem_force](index.html) module"]
pub struct LPMEM_FORCE_SPEC;
impl crate::RegisterSpec for LPMEM_FORCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpmem_force::R](R) reader structure"]
impl crate::Readable for LPMEM_FORCE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpmem_force::W](W) writer structure"]
impl crate::Writable for LPMEM_FORCE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPMEM_FORCE to value 0"]
impl crate::Resettable for LPMEM_FORCE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
