#[doc = "Register `SAR1` reader"]
pub type R = crate::R<SAR1_SPEC>;
#[doc = "Register `SAR1` writer"]
pub type W = crate::W<SAR1_SPEC>;
#[doc = "Field `CH1_SAR1` reader - NA"]
pub type CH1_SAR1_R = crate::FieldReader<u32>;
#[doc = "Field `CH1_SAR1` writer - NA"]
pub type CH1_SAR1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn ch1_sar1(&self) -> CH1_SAR1_R {
        CH1_SAR1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR1")
            .field("ch1_sar1", &self.ch1_sar1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_sar1(&mut self) -> CH1_SAR1_W<SAR1_SPEC> {
        CH1_SAR1_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR1_SPEC;
impl crate::RegisterSpec for SAR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar1::R`](R) reader structure"]
impl crate::Readable for SAR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar1::W`](W) writer structure"]
impl crate::Writable for SAR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR1 to value 0"]
impl crate::Resettable for SAR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
