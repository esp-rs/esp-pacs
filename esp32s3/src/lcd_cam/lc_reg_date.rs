#[doc = "Register `LC_REG_DATE` reader"]
pub type R = crate::R<LC_REG_DATE_SPEC>;
#[doc = "Register `LC_REG_DATE` writer"]
pub type W = crate::W<LC_REG_DATE_SPEC>;
#[doc = "Field `LC_DATE` reader - LCD_CAM version control register"]
pub type LC_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `LC_DATE` writer - LCD_CAM version control register"]
pub type LC_DATE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 28, O, u32>;
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
            .field("lc_date", &format_args!("{}", self.lc_date().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LC_REG_DATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:27 - LCD_CAM version control register"]
    #[inline(always)]
    #[must_use]
    pub fn lc_date(&mut self) -> LC_DATE_W<LC_REG_DATE_SPEC, 0> {
        LC_DATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lc_reg_date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lc_reg_date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LC_REG_DATE_SPEC;
impl crate::RegisterSpec for LC_REG_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lc_reg_date::R`](R) reader structure"]
impl crate::Readable for LC_REG_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lc_reg_date::W`](W) writer structure"]
impl crate::Writable for LC_REG_DATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LC_REG_DATE to value 0x0200_3020"]
impl crate::Resettable for LC_REG_DATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_3020;
}
