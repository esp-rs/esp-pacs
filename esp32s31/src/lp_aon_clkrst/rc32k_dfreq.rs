#[doc = "Register `RC32K_DFREQ` reader"]
pub type R = crate::R<RC32K_DFREQ_SPEC>;
#[doc = "Register `RC32K_DFREQ` writer"]
pub type W = crate::W<RC32K_DFREQ_SPEC>;
#[doc = "Field `RC32K_DFREQ` reader - need_des"]
pub type RC32K_DFREQ_R = crate::FieldReader<u32>;
#[doc = "Field `RC32K_DFREQ` writer - need_des"]
pub type RC32K_DFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn rc32k_dfreq(&self) -> RC32K_DFREQ_R {
        RC32K_DFREQ_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RC32K_DFREQ")
            .field("rc32k_dfreq", &self.rc32k_dfreq())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn rc32k_dfreq(&mut self) -> RC32K_DFREQ_W<'_, RC32K_DFREQ_SPEC> {
        RC32K_DFREQ_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`rc32k_dfreq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rc32k_dfreq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RC32K_DFREQ_SPEC;
impl crate::RegisterSpec for RC32K_DFREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rc32k_dfreq::R`](R) reader structure"]
impl crate::Readable for RC32K_DFREQ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rc32k_dfreq::W`](W) writer structure"]
impl crate::Writable for RC32K_DFREQ_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RC32K_DFREQ to value 0x028a"]
impl crate::Resettable for RC32K_DFREQ_SPEC {
    const RESET_VALUE: u32 = 0x028a;
}
