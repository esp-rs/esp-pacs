#[doc = "Register `LP_STORE7` reader"]
pub type R = crate::R<LP_STORE7_SPEC>;
#[doc = "Register `LP_STORE7` writer"]
pub type W = crate::W<LP_STORE7_SPEC>;
#[doc = "Field `LP_SCRATCH7` reader - need_des"]
pub type LP_SCRATCH7_R = crate::FieldReader<u32>;
#[doc = "Field `LP_SCRATCH7` writer - need_des"]
pub type LP_SCRATCH7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_scratch7(&self) -> LP_SCRATCH7_R {
        LP_SCRATCH7_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_STORE7")
            .field("lp_scratch7", &self.lp_scratch7())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_scratch7(&mut self) -> LP_SCRATCH7_W<LP_STORE7_SPEC> {
        LP_SCRATCH7_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_store7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_store7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_STORE7_SPEC;
impl crate::RegisterSpec for LP_STORE7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_store7::R`](R) reader structure"]
impl crate::Readable for LP_STORE7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_store7::W`](W) writer structure"]
impl crate::Writable for LP_STORE7_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_STORE7 to value 0"]
impl crate::Resettable for LP_STORE7_SPEC {
    const RESET_VALUE: u32 = 0;
}
