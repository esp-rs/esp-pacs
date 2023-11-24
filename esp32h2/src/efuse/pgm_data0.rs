#[doc = "Register `PGM_DATA0` reader"]
pub type R = crate::R<PGM_DATA0_SPEC>;
#[doc = "Register `PGM_DATA0` writer"]
pub type W = crate::W<PGM_DATA0_SPEC>;
#[doc = "Field `PGM_DATA_0` reader - Configures the 0th 32-bit data to be programmed."]
pub type PGM_DATA_0_R = crate::FieldReader<u32>;
#[doc = "Field `PGM_DATA_0` writer - Configures the 0th 32-bit data to be programmed."]
pub type PGM_DATA_0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configures the 0th 32-bit data to be programmed."]
    #[inline(always)]
    pub fn pgm_data_0(&self) -> PGM_DATA_0_R {
        PGM_DATA_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PGM_DATA0")
            .field("pgm_data_0", &format_args!("{}", self.pgm_data_0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PGM_DATA0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Configures the 0th 32-bit data to be programmed."]
    #[inline(always)]
    #[must_use]
    pub fn pgm_data_0(&mut self) -> PGM_DATA_0_W<PGM_DATA0_SPEC> {
        PGM_DATA_0_W::new(self, 0)
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
#[doc = "Register 0 that stores data to be programmed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pgm_data0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_data0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PGM_DATA0_SPEC;
impl crate::RegisterSpec for PGM_DATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pgm_data0::R`](R) reader structure"]
impl crate::Readable for PGM_DATA0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pgm_data0::W`](W) writer structure"]
impl crate::Writable for PGM_DATA0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PGM_DATA0 to value 0"]
impl crate::Resettable for PGM_DATA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
