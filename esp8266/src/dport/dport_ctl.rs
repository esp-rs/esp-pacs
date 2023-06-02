#[doc = "Register `DPORT_CTL` reader"]
pub struct R(crate::R<DPORT_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPORT_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPORT_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPORT_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPORT_CTL` writer"]
pub struct W(crate::W<DPORT_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPORT_CTL_SPEC>;
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
impl From<crate::W<DPORT_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPORT_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DPORT_CTL_DOUBLE_CLK` reader - "]
pub type DPORT_CTL_DOUBLE_CLK_R = crate::BitReader;
#[doc = "Field `DPORT_CTL_DOUBLE_CLK` writer - "]
pub type DPORT_CTL_DOUBLE_CLK_W<'a, const O: u8> = crate::BitWriter<'a, DPORT_CTL_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dport_ctl_double_clk(&self) -> DPORT_CTL_DOUBLE_CLK_R {
        DPORT_CTL_DOUBLE_CLK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPORT_CTL")
            .field(
                "dport_ctl_double_clk",
                &format_args!("{}", self.dport_ctl_double_clk().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DPORT_CTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dport_ctl_double_clk(&mut self) -> DPORT_CTL_DOUBLE_CLK_W<0> {
        DPORT_CTL_DOUBLE_CLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DPORT_CTL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dport_ctl](index.html) module"]
pub struct DPORT_CTL_SPEC;
impl crate::RegisterSpec for DPORT_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dport_ctl::R](R) reader structure"]
impl crate::Readable for DPORT_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dport_ctl::W](W) writer structure"]
impl crate::Writable for DPORT_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPORT_CTL to value 0"]
impl crate::Resettable for DPORT_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
