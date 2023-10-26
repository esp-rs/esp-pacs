#[doc = "Register `ERR_WARNING_LIMIT` reader"]
pub type R = crate::R<ERR_WARNING_LIMIT_SPEC>;
#[doc = "Register `ERR_WARNING_LIMIT` writer"]
pub type W = crate::W<ERR_WARNING_LIMIT_SPEC>;
#[doc = "Field `ERR_WARNING_LIMIT` reader - Error warning threshold. In the case when any of a error counter value exceeds the threshold, or all the error counter values are below the threshold, an error warning interrupt will be triggered (given the enable signal is valid)."]
pub type ERR_WARNING_LIMIT_R = crate::FieldReader;
#[doc = "Field `ERR_WARNING_LIMIT` writer - Error warning threshold. In the case when any of a error counter value exceeds the threshold, or all the error counter values are below the threshold, an error warning interrupt will be triggered (given the enable signal is valid)."]
pub type ERR_WARNING_LIMIT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Error warning threshold. In the case when any of a error counter value exceeds the threshold, or all the error counter values are below the threshold, an error warning interrupt will be triggered (given the enable signal is valid)."]
    #[inline(always)]
    pub fn err_warning_limit(&self) -> ERR_WARNING_LIMIT_R {
        ERR_WARNING_LIMIT_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ERR_WARNING_LIMIT")
            .field(
                "err_warning_limit",
                &format_args!("{}", self.err_warning_limit().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ERR_WARNING_LIMIT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Error warning threshold. In the case when any of a error counter value exceeds the threshold, or all the error counter values are below the threshold, an error warning interrupt will be triggered (given the enable signal is valid)."]
    #[inline(always)]
    #[must_use]
    pub fn err_warning_limit(&mut self) -> ERR_WARNING_LIMIT_W<ERR_WARNING_LIMIT_SPEC, 0> {
        ERR_WARNING_LIMIT_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Error Warning Limit Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_warning_limit::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_warning_limit::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ERR_WARNING_LIMIT_SPEC;
impl crate::RegisterSpec for ERR_WARNING_LIMIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_warning_limit::R`](R) reader structure"]
impl crate::Readable for ERR_WARNING_LIMIT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`err_warning_limit::W`](W) writer structure"]
impl crate::Writable for ERR_WARNING_LIMIT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ERR_WARNING_LIMIT to value 0x60"]
impl crate::Resettable for ERR_WARNING_LIMIT_SPEC {
    const RESET_VALUE: Self::Ux = 0x60;
}
