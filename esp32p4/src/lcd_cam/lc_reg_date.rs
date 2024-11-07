#[doc = "Register `LC_REG_DATE` reader"]
pub type R = crate::R<LC_REG_DATE_SPEC>;
#[doc = "Register `LC_REG_DATE` writer"]
pub type W = crate::W<LC_REG_DATE_SPEC>;
#[doc = "Field `LC_DATE` reader - LCD_CAM version control register"]
pub type LC_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `LC_DATE` writer - LCD_CAM version control register"]
pub type LC_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - LCD_CAM version control register"]
    #[inline(always)]
    pub fn lc_date(&self) -> LC_DATE_R {
        LC_DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LC_REG_DATE")
            .field("lc_date", &self.lc_date())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:27 - LCD_CAM version control register"]
    #[inline(always)]
    pub fn lc_date(&mut self) -> LC_DATE_W<LC_REG_DATE_SPEC> {
        LC_DATE_W::new(self, 0)
    }
}
#[doc = "Version register\n\nYou can [`read`](crate::Reg::read) this register and get [`lc_reg_date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lc_reg_date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LC_REG_DATE_SPEC;
impl crate::RegisterSpec for LC_REG_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lc_reg_date::R`](R) reader structure"]
impl crate::Readable for LC_REG_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lc_reg_date::W`](W) writer structure"]
impl crate::Writable for LC_REG_DATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LC_REG_DATE to value 0x0230_3090"]
impl crate::Resettable for LC_REG_DATE_SPEC {
    const RESET_VALUE: u32 = 0x0230_3090;
}
