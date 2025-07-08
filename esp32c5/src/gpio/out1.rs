#[doc = "Register `OUT1` reader"]
pub type R = crate::R<OUT1_SPEC>;
#[doc = "Register `OUT1` writer"]
pub type W = crate::W<OUT1_SPEC>;
#[doc = "Field `DATA_ORIG` reader - Configures the output value of GPIO32 ~ 32 output in simple GPIO output mode.\\\\ 0: Low level\\\\ 1: High level\\\\"]
pub type DATA_ORIG_R = crate::BitReader;
#[doc = "Field `DATA_ORIG` writer - Configures the output value of GPIO32 ~ 32 output in simple GPIO output mode.\\\\ 0: Low level\\\\ 1: High level\\\\"]
pub type DATA_ORIG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures the output value of GPIO32 ~ 32 output in simple GPIO output mode.\\\\ 0: Low level\\\\ 1: High level\\\\"]
    #[inline(always)]
    pub fn data_orig(&self) -> DATA_ORIG_R {
        DATA_ORIG_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT1")
            .field("data_orig", &self.data_orig())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures the output value of GPIO32 ~ 32 output in simple GPIO output mode.\\\\ 0: Low level\\\\ 1: High level\\\\"]
    #[inline(always)]
    pub fn data_orig(&mut self) -> DATA_ORIG_W<OUT1_SPEC> {
        DATA_ORIG_W::new(self, 0)
    }
}
#[doc = "GPIO output register for GPIO32-32\n\nYou can [`read`](crate::Reg::read) this register and get [`out1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT1_SPEC;
impl crate::RegisterSpec for OUT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out1::R`](R) reader structure"]
impl crate::Readable for OUT1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out1::W`](W) writer structure"]
impl crate::Writable for OUT1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT1 to value 0"]
impl crate::Resettable for OUT1_SPEC {}
