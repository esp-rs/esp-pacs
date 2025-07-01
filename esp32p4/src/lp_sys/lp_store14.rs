#[doc = "Register `LP_STORE14` reader"]
pub type R = crate::R<LP_STORE14_SPEC>;
#[doc = "Register `LP_STORE14` writer"]
pub type W = crate::W<LP_STORE14_SPEC>;
#[doc = "Field `LP_SCRATCH14` reader - need_des"]
pub type LP_SCRATCH14_R = crate::FieldReader<u32>;
#[doc = "Field `LP_SCRATCH14` writer - need_des"]
pub type LP_SCRATCH14_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_scratch14(&self) -> LP_SCRATCH14_R {
        LP_SCRATCH14_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_STORE14")
            .field("lp_scratch14", &self.lp_scratch14())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_scratch14(&mut self) -> LP_SCRATCH14_W<LP_STORE14_SPEC> {
        LP_SCRATCH14_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_store14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_store14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_STORE14_SPEC;
impl crate::RegisterSpec for LP_STORE14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_store14::R`](R) reader structure"]
impl crate::Readable for LP_STORE14_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_store14::W`](W) writer structure"]
impl crate::Writable for LP_STORE14_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_STORE14 to value 0"]
impl crate::Resettable for LP_STORE14_SPEC {}
