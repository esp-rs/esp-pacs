#[doc = "Register `PGM_CHECK_VALUE1` reader"]
pub type R = crate::R<PGM_CHECK_VALUE1_SPEC>;
#[doc = "Register `PGM_CHECK_VALUE1` writer"]
pub type W = crate::W<PGM_CHECK_VALUE1_SPEC>;
#[doc = "Field `PGM_RS_DATA_1` reader - The content of the 1st 32-bit RS code to be programmed."]
pub type PGM_RS_DATA_1_R = crate::FieldReader<u32>;
#[doc = "Field `PGM_RS_DATA_1` writer - The content of the 1st 32-bit RS code to be programmed."]
pub type PGM_RS_DATA_1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The content of the 1st 32-bit RS code to be programmed."]
    #[inline(always)]
    pub fn pgm_rs_data_1(&self) -> PGM_RS_DATA_1_R {
        PGM_RS_DATA_1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PGM_CHECK_VALUE1")
            .field(
                "pgm_rs_data_1",
                &format_args!("{}", self.pgm_rs_data_1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PGM_CHECK_VALUE1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - The content of the 1st 32-bit RS code to be programmed."]
    #[inline(always)]
    #[must_use]
    pub fn pgm_rs_data_1(&mut self) -> PGM_RS_DATA_1_W<PGM_CHECK_VALUE1_SPEC> {
        PGM_RS_DATA_1_W::new(self, 0)
    }
}
#[doc = "Register 1 that stores the RS code to be programmed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pgm_check_value1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_check_value1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PGM_CHECK_VALUE1_SPEC;
impl crate::RegisterSpec for PGM_CHECK_VALUE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pgm_check_value1::R`](R) reader structure"]
impl crate::Readable for PGM_CHECK_VALUE1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pgm_check_value1::W`](W) writer structure"]
impl crate::Writable for PGM_CHECK_VALUE1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PGM_CHECK_VALUE1 to value 0"]
impl crate::Resettable for PGM_CHECK_VALUE1_SPEC {
    const RESET_VALUE: u32 = 0;
}
