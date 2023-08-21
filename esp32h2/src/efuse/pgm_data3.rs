#[doc = "Register `PGM_DATA3` reader"]
pub type R = crate::R<PGM_DATA3_SPEC>;
#[doc = "Register `PGM_DATA3` writer"]
pub type W = crate::W<PGM_DATA3_SPEC>;
#[doc = "Field `PGM_DATA_3` reader - Configures the 3rd 32-bit data to be programmed."]
pub type PGM_DATA_3_R = crate::FieldReader<u32>;
#[doc = "Field `PGM_DATA_3` writer - Configures the 3rd 32-bit data to be programmed."]
pub type PGM_DATA_3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Configures the 3rd 32-bit data to be programmed."]
    #[inline(always)]
    pub fn pgm_data_3(&self) -> PGM_DATA_3_R {
        PGM_DATA_3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PGM_DATA3")
            .field("pgm_data_3", &format_args!("{}", self.pgm_data_3().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PGM_DATA3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Configures the 3rd 32-bit data to be programmed."]
    #[inline(always)]
    #[must_use]
    pub fn pgm_data_3(&mut self) -> PGM_DATA_3_W<PGM_DATA3_SPEC, 0> {
        PGM_DATA_3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Register 3 that stores data to be programmed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pgm_data3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_data3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PGM_DATA3_SPEC;
impl crate::RegisterSpec for PGM_DATA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pgm_data3::R`](R) reader structure"]
impl crate::Readable for PGM_DATA3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pgm_data3::W`](W) writer structure"]
impl crate::Writable for PGM_DATA3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PGM_DATA3 to value 0"]
impl crate::Resettable for PGM_DATA3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
