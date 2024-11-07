#[doc = "Register `DAR1` reader"]
pub type R = crate::R<DAR1_SPEC>;
#[doc = "Register `DAR1` writer"]
pub type W = crate::W<DAR1_SPEC>;
#[doc = "Field `CH1_DAR1` reader - NA"]
pub type CH1_DAR1_R = crate::FieldReader<u32>;
#[doc = "Field `CH1_DAR1` writer - NA"]
pub type CH1_DAR1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn ch1_dar1(&self) -> CH1_DAR1_R {
        CH1_DAR1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAR1")
            .field("ch1_dar1", &self.ch1_dar1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn ch1_dar1(&mut self) -> CH1_DAR1_W<DAR1_SPEC> {
        CH1_DAR1_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dar1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAR1_SPEC;
impl crate::RegisterSpec for DAR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dar1::R`](R) reader structure"]
impl crate::Readable for DAR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dar1::W`](W) writer structure"]
impl crate::Writable for DAR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAR1 to value 0"]
impl crate::Resettable for DAR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
