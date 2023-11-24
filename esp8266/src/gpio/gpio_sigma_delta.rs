#[doc = "Register `GPIO_SIGMA_DELTA` reader"]
pub type R = crate::R<GPIO_SIGMA_DELTA_SPEC>;
#[doc = "Register `GPIO_SIGMA_DELTA` writer"]
pub type W = crate::W<GPIO_SIGMA_DELTA_SPEC>;
#[doc = "Field `SIGMA_DELTA_TARGET` reader - target level of the sigma-delta. It is a signed byte."]
pub type SIGMA_DELTA_TARGET_R = crate::FieldReader;
#[doc = "Field `SIGMA_DELTA_TARGET` writer - target level of the sigma-delta. It is a signed byte."]
pub type SIGMA_DELTA_TARGET_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SIGMA_DELTA_PRESCALAR` reader - Clock pre-divider for sigma-delta."]
pub type SIGMA_DELTA_PRESCALAR_R = crate::FieldReader;
#[doc = "Field `SIGMA_DELTA_PRESCALAR` writer - Clock pre-divider for sigma-delta."]
pub type SIGMA_DELTA_PRESCALAR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SIGMA_DELTA_ENABLE` reader - 1: enable sigma-delta; 0: disable"]
pub type SIGMA_DELTA_ENABLE_R = crate::BitReader;
#[doc = "Field `SIGMA_DELTA_ENABLE` writer - 1: enable sigma-delta; 0: disable"]
pub type SIGMA_DELTA_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - target level of the sigma-delta. It is a signed byte."]
    #[inline(always)]
    pub fn sigma_delta_target(&self) -> SIGMA_DELTA_TARGET_R {
        SIGMA_DELTA_TARGET_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Clock pre-divider for sigma-delta."]
    #[inline(always)]
    pub fn sigma_delta_prescalar(&self) -> SIGMA_DELTA_PRESCALAR_R {
        SIGMA_DELTA_PRESCALAR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - 1: enable sigma-delta; 0: disable"]
    #[inline(always)]
    pub fn sigma_delta_enable(&self) -> SIGMA_DELTA_ENABLE_R {
        SIGMA_DELTA_ENABLE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_SIGMA_DELTA")
            .field(
                "sigma_delta_enable",
                &format_args!("{}", self.sigma_delta_enable().bit()),
            )
            .field(
                "sigma_delta_prescalar",
                &format_args!("{}", self.sigma_delta_prescalar().bits()),
            )
            .field(
                "sigma_delta_target",
                &format_args!("{}", self.sigma_delta_target().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GPIO_SIGMA_DELTA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - target level of the sigma-delta. It is a signed byte."]
    #[inline(always)]
    #[must_use]
    pub fn sigma_delta_target(&mut self) -> SIGMA_DELTA_TARGET_W<GPIO_SIGMA_DELTA_SPEC> {
        SIGMA_DELTA_TARGET_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Clock pre-divider for sigma-delta."]
    #[inline(always)]
    #[must_use]
    pub fn sigma_delta_prescalar(&mut self) -> SIGMA_DELTA_PRESCALAR_W<GPIO_SIGMA_DELTA_SPEC> {
        SIGMA_DELTA_PRESCALAR_W::new(self, 8)
    }
    #[doc = "Bit 16 - 1: enable sigma-delta; 0: disable"]
    #[inline(always)]
    #[must_use]
    pub fn sigma_delta_enable(&mut self) -> SIGMA_DELTA_ENABLE_W<GPIO_SIGMA_DELTA_SPEC> {
        SIGMA_DELTA_ENABLE_W::new(self, 16)
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
#[doc = "GPIO_SIGMA_DELTA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_sigma_delta::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_sigma_delta::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_SIGMA_DELTA_SPEC;
impl crate::RegisterSpec for GPIO_SIGMA_DELTA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_sigma_delta::R`](R) reader structure"]
impl crate::Readable for GPIO_SIGMA_DELTA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_sigma_delta::W`](W) writer structure"]
impl crate::Writable for GPIO_SIGMA_DELTA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_SIGMA_DELTA to value 0"]
impl crate::Resettable for GPIO_SIGMA_DELTA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
