#[doc = "Register `STATUS_W1TC` reader"]
pub type R = crate::R<STATUS_W1TC_SPEC>;
#[doc = "Register `STATUS_W1TC` writer"]
pub type W = crate::W<STATUS_W1TC_SPEC>;
#[doc = "Field `STATUS_INT_W1TC` reader - GPIO0~31 interrupt status write 1 to clear"]
pub type STATUS_INT_W1TC_R = crate::FieldReader<u32>;
#[doc = "Field `STATUS_INT_W1TC` writer - GPIO0~31 interrupt status write 1 to clear"]
pub type STATUS_INT_W1TC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO0~31 interrupt status write 1 to clear"]
    #[inline(always)]
    pub fn status_int_w1tc(&self) -> STATUS_INT_W1TC_R {
        STATUS_INT_W1TC_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS_W1TC")
            .field(
                "status_int_w1tc",
                &format_args!("{}", self.status_int_w1tc().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATUS_W1TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO0~31 interrupt status write 1 to clear"]
    #[inline(always)]
    #[must_use]
    pub fn status_int_w1tc(&mut self) -> STATUS_INT_W1TC_W<STATUS_W1TC_SPEC> {
        STATUS_INT_W1TC_W::new(self, 0)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status_w1tc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_w1tc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_W1TC_SPEC;
impl crate::RegisterSpec for STATUS_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_w1tc::R`](R) reader structure"]
impl crate::Readable for STATUS_W1TC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status_w1tc::W`](W) writer structure"]
impl crate::Writable for STATUS_W1TC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS_W1TC to value 0"]
impl crate::Resettable for STATUS_W1TC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
