#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `INT` reader - RTC GPIO 0 ~ 21 interrupt status"]
pub type INT_R = crate::FieldReader<u32>;
#[doc = "Field `INT` writer - RTC GPIO 0 ~ 21 interrupt status"]
pub type INT_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 10:31 - RTC GPIO 0 ~ 21 interrupt status"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS").field("int", &self.int()).finish()
    }
}
impl W {
    #[doc = "Bits 10:31 - RTC GPIO 0 ~ 21 interrupt status"]
    #[inline(always)]
    pub fn int(&mut self) -> INT_W<'_, STATUS_SPEC> {
        INT_W::new(self, 10)
    }
}
#[doc = "RTC GPIO 0 ~ 21 interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {}
