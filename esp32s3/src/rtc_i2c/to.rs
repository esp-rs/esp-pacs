#[doc = "Register `TO` reader"]
pub type R = crate::R<TO_SPEC>;
#[doc = "Register `TO` writer"]
pub type W = crate::W<TO_SPEC>;
#[doc = "Field `TIME_OUT` reader - time out threshold"]
pub type TIME_OUT_R = crate::FieldReader<u32>;
#[doc = "Field `TIME_OUT` writer - time out threshold"]
pub type TIME_OUT_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - time out threshold"]
    #[inline(always)]
    pub fn time_out(&self) -> TIME_OUT_R {
        TIME_OUT_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TO")
            .field("time_out", &self.time_out())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - time out threshold"]
    #[inline(always)]
    pub fn time_out(&mut self) -> TIME_OUT_W<TO_SPEC> {
        TIME_OUT_W::new(self, 0)
    }
}
#[doc = "configure time out\n\nYou can [`read`](crate::Reg::read) this register and get [`to::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`to::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TO_SPEC;
impl crate::RegisterSpec for TO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`to::R`](R) reader structure"]
impl crate::Readable for TO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`to::W`](W) writer structure"]
impl crate::Writable for TO_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TO to value 0x0001_0000"]
impl crate::Resettable for TO_SPEC {
    const RESET_VALUE: u32 = 0x0001_0000;
}
