#[doc = "Register `SSTATAR1` reader"]
pub type R = crate::R<SSTATAR1_SPEC>;
#[doc = "Register `SSTATAR1` writer"]
pub type W = crate::W<SSTATAR1_SPEC>;
#[doc = "Field `CH1_SSTATAR1` reader - NA"]
pub type CH1_SSTATAR1_R = crate::FieldReader<u32>;
#[doc = "Field `CH1_SSTATAR1` writer - NA"]
pub type CH1_SSTATAR1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn ch1_sstatar1(&self) -> CH1_SSTATAR1_R {
        CH1_SSTATAR1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSTATAR1")
            .field("ch1_sstatar1", &self.ch1_sstatar1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn ch1_sstatar1(&mut self) -> CH1_SSTATAR1_W<SSTATAR1_SPEC> {
        CH1_SSTATAR1_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`sstatar1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstatar1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSTATAR1_SPEC;
impl crate::RegisterSpec for SSTATAR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sstatar1::R`](R) reader structure"]
impl crate::Readable for SSTATAR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sstatar1::W`](W) writer structure"]
impl crate::Writable for SSTATAR1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSTATAR1 to value 0"]
impl crate::Resettable for SSTATAR1_SPEC {}
