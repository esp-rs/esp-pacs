#[doc = "Register `LP_STORE%s` reader"]
pub type R = crate::R<LP_STORE_SPEC>;
#[doc = "Register `LP_STORE%s` writer"]
pub type W = crate::W<LP_STORE_SPEC>;
#[doc = "Field `LP_SCRATCH0` reader - "]
pub type LP_SCRATCH0_R = crate::FieldReader<u32>;
#[doc = "Field `LP_SCRATCH0` writer - "]
pub type LP_SCRATCH0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn lp_scratch0(&self) -> LP_SCRATCH0_R {
        LP_SCRATCH0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_STORE")
            .field("lp_scratch0", &self.lp_scratch0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn lp_scratch0(&mut self) -> LP_SCRATCH0_W<'_, LP_STORE_SPEC> {
        LP_SCRATCH0_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_store::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_store::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_STORE_SPEC;
impl crate::RegisterSpec for LP_STORE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_store::R`](R) reader structure"]
impl crate::Readable for LP_STORE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_store::W`](W) writer structure"]
impl crate::Writable for LP_STORE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_STORE%s to value 0"]
impl crate::Resettable for LP_STORE_SPEC {}
