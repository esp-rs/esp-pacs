#[doc = "Register `PGM_CHECK_VALUE0` reader"]
pub type R = crate::R<PGM_CHECK_VALUE0_SPEC>;
#[doc = "Register `PGM_CHECK_VALUE0` writer"]
pub type W = crate::W<PGM_CHECK_VALUE0_SPEC>;
#[doc = "Field `PGM_RS_DATA_0` reader - The content of the 0th 32-bit RS code to be programmed."]
pub type PGM_RS_DATA_0_R = crate::FieldReader<u32>;
#[doc = "Field `PGM_RS_DATA_0` writer - The content of the 0th 32-bit RS code to be programmed."]
pub type PGM_RS_DATA_0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The content of the 0th 32-bit RS code to be programmed."]
    #[inline(always)]
    pub fn pgm_rs_data_0(&self) -> PGM_RS_DATA_0_R {
        PGM_RS_DATA_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PGM_CHECK_VALUE0")
            .field(
                "pgm_rs_data_0",
                &format_args!("{}", self.pgm_rs_data_0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PGM_CHECK_VALUE0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - The content of the 0th 32-bit RS code to be programmed."]
    #[inline(always)]
    #[must_use]
    pub fn pgm_rs_data_0(&mut self) -> PGM_RS_DATA_0_W<PGM_CHECK_VALUE0_SPEC> {
        PGM_RS_DATA_0_W::new(self, 0)
    }
}
#[doc = "Register 0 that stores the RS code to be programmed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pgm_check_value0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_check_value0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PGM_CHECK_VALUE0_SPEC;
impl crate::RegisterSpec for PGM_CHECK_VALUE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pgm_check_value0::R`](R) reader structure"]
impl crate::Readable for PGM_CHECK_VALUE0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pgm_check_value0::W`](W) writer structure"]
impl crate::Writable for PGM_CHECK_VALUE0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PGM_CHECK_VALUE0 to value 0"]
impl crate::Resettable for PGM_CHECK_VALUE0_SPEC {
    const RESET_VALUE: u32 = 0;
}
