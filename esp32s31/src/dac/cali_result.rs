#[doc = "Register `CALI_RESULT` reader"]
pub type R = crate::R<CALI_RESULT_SPEC>;
#[doc = "Register `CALI_RESULT` writer"]
pub type W = crate::W<CALI_RESULT_SPEC>;
#[doc = "Field `PAD_0` reader - cali result register for pad 0 output"]
pub type PAD_0_R = crate::FieldReader<u16>;
#[doc = "Field `PAD_0` writer - cali result register for pad 0 output"]
pub type PAD_0_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PAD_1` reader - cali result register for pad 1 output"]
pub type PAD_1_R = crate::FieldReader<u16>;
#[doc = "Field `PAD_1` writer - cali result register for pad 1 output"]
pub type PAD_1_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - cali result register for pad 0 output"]
    #[inline(always)]
    pub fn pad_0(&self) -> PAD_0_R {
        PAD_0_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - cali result register for pad 1 output"]
    #[inline(always)]
    pub fn pad_1(&self) -> PAD_1_R {
        PAD_1_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CALI_RESULT")
            .field("pad_0", &self.pad_0())
            .field("pad_1", &self.pad_1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - cali result register for pad 0 output"]
    #[inline(always)]
    pub fn pad_0(&mut self) -> PAD_0_W<'_, CALI_RESULT_SPEC> {
        PAD_0_W::new(self, 0)
    }
    #[doc = "Bits 12:23 - cali result register for pad 1 output"]
    #[inline(always)]
    pub fn pad_1(&mut self) -> PAD_1_W<'_, CALI_RESULT_SPEC> {
        PAD_1_W::new(self, 12)
    }
}
#[doc = "DAC CALI result register\n\nYou can [`read`](crate::Reg::read) this register and get [`cali_result::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cali_result::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALI_RESULT_SPEC;
impl crate::RegisterSpec for CALI_RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cali_result::R`](R) reader structure"]
impl crate::Readable for CALI_RESULT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cali_result::W`](W) writer structure"]
impl crate::Writable for CALI_RESULT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CALI_RESULT to value 0"]
impl crate::Resettable for CALI_RESULT_SPEC {}
