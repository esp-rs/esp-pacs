#[doc = "Register `CH%s_GAMMA_RD_ADDR` reader"]
pub struct R(crate::R<CH_GAMMA_RD_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_GAMMA_RD_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_GAMMA_RD_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_GAMMA_RD_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH%s_GAMMA_RD_ADDR` writer"]
pub struct W(crate::W<CH_GAMMA_RD_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH_GAMMA_RD_ADDR_SPEC>;
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
impl From<crate::W<CH_GAMMA_RD_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH_GAMMA_RD_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH_GAMMA_RD_ADDR` reader - Ledc ch%s gamma ram read address."]
pub type CH_GAMMA_RD_ADDR_R = crate::FieldReader;
#[doc = "Field `CH_GAMMA_RD_ADDR` writer - Ledc ch%s gamma ram read address."]
pub type CH_GAMMA_RD_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, CH_GAMMA_RD_ADDR_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Ledc ch%s gamma ram read address."]
    #[inline(always)]
    pub fn ch_gamma_rd_addr(&self) -> CH_GAMMA_RD_ADDR_R {
        CH_GAMMA_RD_ADDR_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_GAMMA_RD_ADDR")
            .field(
                "ch_gamma_rd_addr",
                &format_args!("{}", self.ch_gamma_rd_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_GAMMA_RD_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Ledc ch%s gamma ram read address."]
    #[inline(always)]
    #[must_use]
    pub fn ch_gamma_rd_addr(&mut self) -> CH_GAMMA_RD_ADDR_W<0> {
        CH_GAMMA_RD_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ledc ch%s gamma ram read address register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_gamma_rd_addr](index.html) module"]
pub struct CH_GAMMA_RD_ADDR_SPEC;
impl crate::RegisterSpec for CH_GAMMA_RD_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_gamma_rd_addr::R](R) reader structure"]
impl crate::Readable for CH_GAMMA_RD_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch_gamma_rd_addr::W](W) writer structure"]
impl crate::Writable for CH_GAMMA_RD_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH%s_GAMMA_RD_ADDR to value 0"]
impl crate::Resettable for CH_GAMMA_RD_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
