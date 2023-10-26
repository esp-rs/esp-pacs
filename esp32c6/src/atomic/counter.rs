#[doc = "Register `COUNTER` reader"]
pub type R = crate::R<COUNTER_SPEC>;
#[doc = "Register `COUNTER` writer"]
pub type W = crate::W<COUNTER_SPEC>;
#[doc = "Field `WAIT_COUNTER` reader - delay counter"]
pub type WAIT_COUNTER_R = crate::FieldReader<u16>;
#[doc = "Field `WAIT_COUNTER` writer - delay counter"]
pub type WAIT_COUNTER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - delay counter"]
    #[inline(always)]
    pub fn wait_counter(&self) -> WAIT_COUNTER_R {
        WAIT_COUNTER_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COUNTER")
            .field(
                "wait_counter",
                &format_args!("{}", self.wait_counter().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<COUNTER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - delay counter"]
    #[inline(always)]
    #[must_use]
    pub fn wait_counter(&mut self) -> WAIT_COUNTER_W<COUNTER_SPEC, 0> {
        WAIT_COUNTER_W::new(self)
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
#[doc = "wait counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`counter::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`counter::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COUNTER_SPEC;
impl crate::RegisterSpec for COUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`counter::R`](R) reader structure"]
impl crate::Readable for COUNTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`counter::W`](W) writer structure"]
impl crate::Writable for COUNTER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COUNTER to value 0"]
impl crate::Resettable for COUNTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
