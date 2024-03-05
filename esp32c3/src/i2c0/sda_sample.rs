#[doc = "Register `SDA_SAMPLE` reader"]
pub type R = crate::R<SDA_SAMPLE_SPEC>;
#[doc = "Register `SDA_SAMPLE` writer"]
pub type W = crate::W<SDA_SAMPLE_SPEC>;
#[doc = "Field `TIME` reader - reg_sda_sample_time"]
pub type TIME_R = crate::FieldReader<u16>;
#[doc = "Field `TIME` writer - reg_sda_sample_time"]
pub type TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - reg_sda_sample_time"]
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new((self.bits & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDA_SAMPLE")
            .field("time", &format_args!("{}", self.time().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SDA_SAMPLE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:8 - reg_sda_sample_time"]
    #[inline(always)]
    #[must_use]
    pub fn time(&mut self) -> TIME_W<SDA_SAMPLE_SPEC> {
        TIME_W::new(self, 0)
    }
}
#[doc = "I2C_SDA_SAMPLE_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sda_sample::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sda_sample::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDA_SAMPLE_SPEC;
impl crate::RegisterSpec for SDA_SAMPLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sda_sample::R`](R) reader structure"]
impl crate::Readable for SDA_SAMPLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sda_sample::W`](W) writer structure"]
impl crate::Writable for SDA_SAMPLE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDA_SAMPLE to value 0"]
impl crate::Resettable for SDA_SAMPLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
