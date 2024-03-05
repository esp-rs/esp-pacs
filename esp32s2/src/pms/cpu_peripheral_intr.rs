#[doc = "Register `CPU_PERIPHERAL_INTR` reader"]
pub type R = crate::R<CPU_PERIPHERAL_INTR_SPEC>;
#[doc = "Register `CPU_PERIPHERAL_INTR` writer"]
pub type W = crate::W<CPU_PERIPHERAL_INTR_SPEC>;
#[doc = "Field `CPU_PERI_BYTE_ERROR_CLR` reader - The clear signal for CPU peripheral access interrupt."]
pub type CPU_PERI_BYTE_ERROR_CLR_R = crate::BitReader;
#[doc = "Field `CPU_PERI_BYTE_ERROR_CLR` writer - The clear signal for CPU peripheral access interrupt."]
pub type CPU_PERI_BYTE_ERROR_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU_PERI_BYTE_ERROR_EN` reader - The enable signal for CPU peripheral access interrupt."]
pub type CPU_PERI_BYTE_ERROR_EN_R = crate::BitReader;
#[doc = "Field `CPU_PERI_BYTE_ERROR_EN` writer - The enable signal for CPU peripheral access interrupt."]
pub type CPU_PERI_BYTE_ERROR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU_PERI_BYTE_ERROR_INTR` reader - CPU peripheral access interrupt signal."]
pub type CPU_PERI_BYTE_ERROR_INTR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The clear signal for CPU peripheral access interrupt."]
    #[inline(always)]
    pub fn cpu_peri_byte_error_clr(&self) -> CPU_PERI_BYTE_ERROR_CLR_R {
        CPU_PERI_BYTE_ERROR_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The enable signal for CPU peripheral access interrupt."]
    #[inline(always)]
    pub fn cpu_peri_byte_error_en(&self) -> CPU_PERI_BYTE_ERROR_EN_R {
        CPU_PERI_BYTE_ERROR_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU peripheral access interrupt signal."]
    #[inline(always)]
    pub fn cpu_peri_byte_error_intr(&self) -> CPU_PERI_BYTE_ERROR_INTR_R {
        CPU_PERI_BYTE_ERROR_INTR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_PERIPHERAL_INTR")
            .field(
                "cpu_peri_byte_error_clr",
                &format_args!("{}", self.cpu_peri_byte_error_clr().bit()),
            )
            .field(
                "cpu_peri_byte_error_en",
                &format_args!("{}", self.cpu_peri_byte_error_en().bit()),
            )
            .field(
                "cpu_peri_byte_error_intr",
                &format_args!("{}", self.cpu_peri_byte_error_intr().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CPU_PERIPHERAL_INTR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The clear signal for CPU peripheral access interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_peri_byte_error_clr(
        &mut self,
    ) -> CPU_PERI_BYTE_ERROR_CLR_W<CPU_PERIPHERAL_INTR_SPEC> {
        CPU_PERI_BYTE_ERROR_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - The enable signal for CPU peripheral access interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_peri_byte_error_en(&mut self) -> CPU_PERI_BYTE_ERROR_EN_W<CPU_PERIPHERAL_INTR_SPEC> {
        CPU_PERI_BYTE_ERROR_EN_W::new(self, 1)
    }
}
#[doc = "PeribBus1 permission control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_peripheral_intr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_peripheral_intr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_PERIPHERAL_INTR_SPEC;
impl crate::RegisterSpec for CPU_PERIPHERAL_INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_peripheral_intr::R`](R) reader structure"]
impl crate::Readable for CPU_PERIPHERAL_INTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpu_peripheral_intr::W`](W) writer structure"]
impl crate::Writable for CPU_PERIPHERAL_INTR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPU_PERIPHERAL_INTR to value 0"]
impl crate::Resettable for CPU_PERIPHERAL_INTR_SPEC {
    const RESET_VALUE: u32 = 0;
}
