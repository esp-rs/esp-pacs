#[doc = "Register `DAR0` reader"]
pub type R = crate::R<DAR0_SPEC>;
#[doc = "Register `DAR0` writer"]
pub type W = crate::W<DAR0_SPEC>;
#[doc = "Field `CH1_DAR0` reader - NA"]
pub type CH1_DAR0_R = crate::FieldReader<u32>;
#[doc = "Field `CH1_DAR0` writer - NA"]
pub type CH1_DAR0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn ch1_dar0(&self) -> CH1_DAR0_R {
        CH1_DAR0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAR0")
            .field("ch1_dar0", &self.ch1_dar0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_dar0(&mut self) -> CH1_DAR0_W<DAR0_SPEC> {
        CH1_DAR0_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dar0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAR0_SPEC;
impl crate::RegisterSpec for DAR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dar0::R`](R) reader structure"]
impl crate::Readable for DAR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dar0::W`](W) writer structure"]
impl crate::Writable for DAR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAR0 to value 0"]
impl crate::Resettable for DAR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
