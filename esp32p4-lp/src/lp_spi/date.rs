#[doc = "Register `DATE` reader"]
pub type R = crate::R<DATE_SPEC>;
#[doc = "Register `DATE` writer"]
pub type W = crate::W<DATE_SPEC>;
#[doc = "Field `LP_REG_DATE` reader - SPI register version."]
pub type LP_REG_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `LP_REG_DATE` writer - SPI register version."]
pub type LP_REG_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - SPI register version."]
    #[inline(always)]
    pub fn lp_reg_date(&self) -> LP_REG_DATE_R {
        LP_REG_DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATE")
            .field("lp_reg_date", &self.lp_reg_date())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:27 - SPI register version."]
    #[inline(always)]
    pub fn lp_reg_date(&mut self) -> LP_REG_DATE_W<'_, DATE_SPEC> {
        LP_REG_DATE_W::new(self, 0)
    }
}
#[doc = "Version control\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATE_SPEC;
impl crate::RegisterSpec for DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`date::R`](R) reader structure"]
impl crate::Readable for DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`date::W`](W) writer structure"]
impl crate::Writable for DATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATE to value 0"]
impl crate::Resettable for DATE_SPEC {}
