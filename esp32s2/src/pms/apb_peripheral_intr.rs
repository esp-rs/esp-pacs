#[doc = "Register `APB_PERIPHERAL_INTR` reader"]
pub type R = crate::R<APB_PERIPHERAL_INTR_SPEC>;
#[doc = "Register `APB_PERIPHERAL_INTR` writer"]
pub type W = crate::W<APB_PERIPHERAL_INTR_SPEC>;
#[doc = "Field `APB_PERI_BYTE_ERROR_CLR` reader - The clear signal for APB peripheral interrupt."]
pub type APB_PERI_BYTE_ERROR_CLR_R = crate::BitReader;
#[doc = "Field `APB_PERI_BYTE_ERROR_CLR` writer - The clear signal for APB peripheral interrupt."]
pub type APB_PERI_BYTE_ERROR_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_PERI_BYTE_ERROR_EN` reader - The enable signal for APB peripheral access interrupt."]
pub type APB_PERI_BYTE_ERROR_EN_R = crate::BitReader;
#[doc = "Field `APB_PERI_BYTE_ERROR_EN` writer - The enable signal for APB peripheral access interrupt."]
pub type APB_PERI_BYTE_ERROR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_PERI_BYTE_ERROR_INTR` reader - APB peripheral access interrupt signal."]
pub type APB_PERI_BYTE_ERROR_INTR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The clear signal for APB peripheral interrupt."]
    #[inline(always)]
    pub fn apb_peri_byte_error_clr(&self) -> APB_PERI_BYTE_ERROR_CLR_R {
        APB_PERI_BYTE_ERROR_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The enable signal for APB peripheral access interrupt."]
    #[inline(always)]
    pub fn apb_peri_byte_error_en(&self) -> APB_PERI_BYTE_ERROR_EN_R {
        APB_PERI_BYTE_ERROR_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - APB peripheral access interrupt signal."]
    #[inline(always)]
    pub fn apb_peri_byte_error_intr(&self) -> APB_PERI_BYTE_ERROR_INTR_R {
        APB_PERI_BYTE_ERROR_INTR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_PERIPHERAL_INTR")
            .field(
                "apb_peri_byte_error_clr",
                &format_args!("{}", self.apb_peri_byte_error_clr().bit()),
            )
            .field(
                "apb_peri_byte_error_en",
                &format_args!("{}", self.apb_peri_byte_error_en().bit()),
            )
            .field(
                "apb_peri_byte_error_intr",
                &format_args!("{}", self.apb_peri_byte_error_intr().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APB_PERIPHERAL_INTR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The clear signal for APB peripheral interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn apb_peri_byte_error_clr(
        &mut self,
    ) -> APB_PERI_BYTE_ERROR_CLR_W<APB_PERIPHERAL_INTR_SPEC> {
        APB_PERI_BYTE_ERROR_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - The enable signal for APB peripheral access interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn apb_peri_byte_error_en(&mut self) -> APB_PERI_BYTE_ERROR_EN_W<APB_PERIPHERAL_INTR_SPEC> {
        APB_PERI_BYTE_ERROR_EN_W::new(self, 1)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PeribBus2 permission control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_peripheral_intr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_peripheral_intr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB_PERIPHERAL_INTR_SPEC;
impl crate::RegisterSpec for APB_PERIPHERAL_INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb_peripheral_intr::R`](R) reader structure"]
impl crate::Readable for APB_PERIPHERAL_INTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb_peripheral_intr::W`](W) writer structure"]
impl crate::Writable for APB_PERIPHERAL_INTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB_PERIPHERAL_INTR to value 0"]
impl crate::Resettable for APB_PERIPHERAL_INTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
