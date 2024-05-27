///Register `LC_REG_DATE` reader
pub type R = crate::R<LC_REG_DATE_SPEC>;
///Register `LC_REG_DATE` writer
pub type W = crate::W<LC_REG_DATE_SPEC>;
///Field `LC_DATE` reader - LCD_CAM version control register
pub type LC_DATE_R = crate::FieldReader<u32>;
///Field `LC_DATE` writer - LCD_CAM version control register
pub type LC_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    ///Bits 0:27 - LCD_CAM version control register
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
    ///Bits 0:27 - LCD_CAM version control register
    #[inline(always)]
    #[must_use]
    pub fn lc_date(&mut self) -> LC_DATE_W<LC_REG_DATE_SPEC> {
        LC_DATE_W::new(self, 0)
    }
}
/**Version register

You can [`read`](crate::generic::Reg::read) this register and get [`lc_reg_date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lc_reg_date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LC_REG_DATE_SPEC;
impl crate::RegisterSpec for LC_REG_DATE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lc_reg_date::R`](R) reader structure
impl crate::Readable for LC_REG_DATE_SPEC {}
///`write(|w| ..)` method takes [`lc_reg_date::W`](W) writer structure
impl crate::Writable for LC_REG_DATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LC_REG_DATE to value 0x0230_3090
impl crate::Resettable for LC_REG_DATE_SPEC {
    const RESET_VALUE: u32 = 0x0230_3090;
}
