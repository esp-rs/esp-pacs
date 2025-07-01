#[doc = "Register `DSTATAR1` reader"]
pub type R = crate::R<DSTATAR1_SPEC>;
#[doc = "Register `DSTATAR1` writer"]
pub type W = crate::W<DSTATAR1_SPEC>;
#[doc = "Field `CH1_DSTATAR1` reader - NA"]
pub type CH1_DSTATAR1_R = crate::FieldReader<u32>;
#[doc = "Field `CH1_DSTATAR1` writer - NA"]
pub type CH1_DSTATAR1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn ch1_dstatar1(&self) -> CH1_DSTATAR1_R {
        CH1_DSTATAR1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSTATAR1")
            .field("ch1_dstatar1", &self.ch1_dstatar1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn ch1_dstatar1(&mut self) -> CH1_DSTATAR1_W<DSTATAR1_SPEC> {
        CH1_DSTATAR1_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dstatar1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dstatar1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSTATAR1_SPEC;
impl crate::RegisterSpec for DSTATAR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dstatar1::R`](R) reader structure"]
impl crate::Readable for DSTATAR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dstatar1::W`](W) writer structure"]
impl crate::Writable for DSTATAR1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSTATAR1 to value 0"]
impl crate::Resettable for DSTATAR1_SPEC {}
