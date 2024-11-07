#[doc = "Register `SSTATAR0` reader"]
pub type R = crate::R<SSTATAR0_SPEC>;
#[doc = "Register `SSTATAR0` writer"]
pub type W = crate::W<SSTATAR0_SPEC>;
#[doc = "Field `CH1_SSTATAR0` reader - NA"]
pub type CH1_SSTATAR0_R = crate::FieldReader<u32>;
#[doc = "Field `CH1_SSTATAR0` writer - NA"]
pub type CH1_SSTATAR0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn ch1_sstatar0(&self) -> CH1_SSTATAR0_R {
        CH1_SSTATAR0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSTATAR0")
            .field("ch1_sstatar0", &self.ch1_sstatar0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn ch1_sstatar0(&mut self) -> CH1_SSTATAR0_W<SSTATAR0_SPEC> {
        CH1_SSTATAR0_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`sstatar0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstatar0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSTATAR0_SPEC;
impl crate::RegisterSpec for SSTATAR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sstatar0::R`](R) reader structure"]
impl crate::Readable for SSTATAR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sstatar0::W`](W) writer structure"]
impl crate::Writable for SSTATAR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSTATAR0 to value 0"]
impl crate::Resettable for SSTATAR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
