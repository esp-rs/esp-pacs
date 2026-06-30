#[doc = "Register `GINTMSK2` reader"]
pub type R = crate::R<GINTMSK2_SPEC>;
#[doc = "Register `GINTMSK2` writer"]
pub type W = crate::W<GINTMSK2_SPEC>;
#[doc = "Field `GINTMSK2` reader - Resvered"]
pub type GINTMSK2_R = crate::FieldReader<u32>;
#[doc = "Field `GINTMSK2` writer - Resvered"]
pub type GINTMSK2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Resvered"]
    #[inline(always)]
    pub fn gintmsk2(&self) -> GINTMSK2_R {
        GINTMSK2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GINTMSK2")
            .field("gintmsk2", &self.gintmsk2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Resvered"]
    #[inline(always)]
    pub fn gintmsk2(&mut self) -> GINTMSK2_W<'_, GINTMSK2_SPEC> {
        GINTMSK2_W::new(self, 0)
    }
}
#[doc = "This register works with the Interrupt Register (GINTSTS2) to interrupt the application. When an interrupt bit is masked, the interrupt associated with that bit is not generated. However, the GINTSTS2 register bit corresponding to that interrupt is still set. Note: The fields of this register change depending on host or device mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`gintmsk2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintmsk2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GINTMSK2_SPEC;
impl crate::RegisterSpec for GINTMSK2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gintmsk2::R`](R) reader structure"]
impl crate::Readable for GINTMSK2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gintmsk2::W`](W) writer structure"]
impl crate::Writable for GINTMSK2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GINTMSK2 to value 0"]
impl crate::Resettable for GINTMSK2_SPEC {}
