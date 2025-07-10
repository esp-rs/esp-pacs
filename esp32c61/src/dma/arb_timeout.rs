#[doc = "Register `ARB_TIMEOUT` reader"]
pub type R = crate::R<ARB_TIMEOUT_SPEC>;
#[doc = "Register `ARB_TIMEOUT` writer"]
pub type W = crate::W<ARB_TIMEOUT_SPEC>;
#[doc = "Field `NUM` reader - Configures the time slot. Measurement unit: AHB bus clock cycle."]
pub type NUM_R = crate::FieldReader<u16>;
#[doc = "Field `NUM` writer - Configures the time slot. Measurement unit: AHB bus clock cycle."]
pub type NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Configures the time slot. Measurement unit: AHB bus clock cycle."]
    #[inline(always)]
    pub fn num(&self) -> NUM_R {
        NUM_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ARB_TIMEOUT")
            .field("num", &self.num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Configures the time slot. Measurement unit: AHB bus clock cycle."]
    #[inline(always)]
    pub fn num(&mut self) -> NUM_W<ARB_TIMEOUT_SPEC> {
        NUM_W::new(self, 0)
    }
}
#[doc = "TX arbitration timeout configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_timeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_timeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARB_TIMEOUT_SPEC;
impl crate::RegisterSpec for ARB_TIMEOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arb_timeout::R`](R) reader structure"]
impl crate::Readable for ARB_TIMEOUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`arb_timeout::W`](W) writer structure"]
impl crate::Writable for ARB_TIMEOUT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ARB_TIMEOUT to value 0"]
impl crate::Resettable for ARB_TIMEOUT_SPEC {}
