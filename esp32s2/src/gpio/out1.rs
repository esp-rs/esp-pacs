#[doc = "Register `OUT1` reader"]
pub type R = crate::R<OUT1_SPEC>;
#[doc = "Register `OUT1` writer"]
pub type W = crate::W<OUT1_SPEC>;
#[doc = "Field `DATA_ORIG` reader - GPIO32 ~ 53 output value in simple GPIO output mode. The values of bit0 ~ bit13 correspond to GPIO32 ~ GPIO45. Bit14 ~ bit21 are invalid."]
pub type DATA_ORIG_R = crate::FieldReader<u32>;
#[doc = "Field `DATA_ORIG` writer - GPIO32 ~ 53 output value in simple GPIO output mode. The values of bit0 ~ bit13 correspond to GPIO32 ~ GPIO45. Bit14 ~ bit21 are invalid."]
pub type DATA_ORIG_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - GPIO32 ~ 53 output value in simple GPIO output mode. The values of bit0 ~ bit13 correspond to GPIO32 ~ GPIO45. Bit14 ~ bit21 are invalid."]
    #[inline(always)]
    pub fn data_orig(&self) -> DATA_ORIG_R {
        DATA_ORIG_R::new(self.bits & 0x003f_ffff)
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
    #[doc = "Bits 0:21 - GPIO32 ~ 53 output value in simple GPIO output mode. The values of bit0 ~ bit13 correspond to GPIO32 ~ GPIO45. Bit14 ~ bit21 are invalid."]
    #[inline(always)]
    #[must_use]
    pub fn data_orig(&mut self) -> DATA_ORIG_W<OUT1_SPEC> {
        DATA_ORIG_W::new(self, 0)
    }
}
#[doc = "GPIO32 ~ 53 output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT1_SPEC;
impl crate::RegisterSpec for OUT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out1::R`](R) reader structure"]
impl crate::Readable for OUT1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out1::W`](W) writer structure"]
impl crate::Writable for OUT1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT1 to value 0"]
impl crate::Resettable for OUT1_SPEC {
    const RESET_VALUE: u32 = 0;
}
