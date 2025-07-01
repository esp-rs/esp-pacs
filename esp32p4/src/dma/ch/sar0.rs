#[doc = "Register `SAR0` reader"]
pub type R = crate::R<SAR0_SPEC>;
#[doc = "Register `SAR0` writer"]
pub type W = crate::W<SAR0_SPEC>;
#[doc = "Field `CH1_SAR0` reader - NA"]
pub type CH1_SAR0_R = crate::FieldReader<u32>;
#[doc = "Field `CH1_SAR0` writer - NA"]
pub type CH1_SAR0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn ch1_sar0(&self) -> CH1_SAR0_R {
        CH1_SAR0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR0")
            .field("ch1_sar0", &self.ch1_sar0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn ch1_sar0(&mut self) -> CH1_SAR0_W<SAR0_SPEC> {
        CH1_SAR0_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`sar0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR0_SPEC;
impl crate::RegisterSpec for SAR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar0::R`](R) reader structure"]
impl crate::Readable for SAR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar0::W`](W) writer structure"]
impl crate::Writable for SAR0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR0 to value 0"]
impl crate::Resettable for SAR0_SPEC {}
