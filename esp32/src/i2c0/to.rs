#[doc = "Register `TO` reader"]
pub type R = crate::R<TO_SPEC>;
#[doc = "Register `TO` writer"]
pub type W = crate::W<TO_SPEC>;
#[doc = "Field `TIME_OUT` reader - This register is used to configure the max clock number of receiving a data."]
pub type TIME_OUT_R = crate::FieldReader<u32>;
#[doc = "Field `TIME_OUT` writer - This register is used to configure the max clock number of receiving a data."]
pub type TIME_OUT_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - This register is used to configure the max clock number of receiving a data."]
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
    #[doc = "Bits 0:19 - This register is used to configure the max clock number of receiving a data."]
    #[inline(always)]
    #[must_use]
    pub fn time_out(&mut self) -> TIME_OUT_W<TO_SPEC> {
        TIME_OUT_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`to::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`to::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TO_SPEC;
impl crate::RegisterSpec for TO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`to::R`](R) reader structure"]
impl crate::Readable for TO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`to::W`](W) writer structure"]
impl crate::Writable for TO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TO to value 0"]
impl crate::Resettable for TO_SPEC {
    const RESET_VALUE: u32 = 0;
}
