#[doc = "Register `DATE` reader"]
pub type R = crate::R<DATE_SPEC>;
#[doc = "Register `DATE` writer"]
pub type W = crate::W<DATE_SPEC>;
#[doc = "Field `RMT_DATE` reader - This is the version register."]
pub type RMT_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `RMT_DATE` writer - This is the version register."]
pub type RMT_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - This is the version register."]
    #[inline(always)]
    pub fn rmt_date(&self) -> RMT_DATE_R {
        RMT_DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATE")
            .field("rmt_date", &self.rmt_date())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:27 - This is the version register."]
    #[inline(always)]
    pub fn rmt_date(&mut self) -> RMT_DATE_W<DATE_SPEC> {
        RMT_DATE_W::new(self, 0)
    }
}
#[doc = "RMT version register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATE_SPEC;
impl crate::RegisterSpec for DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`date::R`](R) reader structure"]
impl crate::Readable for DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`date::W`](W) writer structure"]
impl crate::Writable for DATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATE to value 0x0210_8213"]
impl crate::Resettable for DATE_SPEC {
    const RESET_VALUE: u32 = 0x0210_8213;
}
