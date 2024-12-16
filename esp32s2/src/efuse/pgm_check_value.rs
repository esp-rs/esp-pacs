#[doc = "Register `PGM_CHECK_VALUE%s` reader"]
pub type R = crate::R<PGM_CHECK_VALUE_SPEC>;
#[doc = "Register `PGM_CHECK_VALUE%s` writer"]
pub type W = crate::W<PGM_CHECK_VALUE_SPEC>;
#[doc = "Field `PGM_RS_DATA` reader - The content of the %sth 32-bit RS code to be programmed."]
pub type PGM_RS_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `PGM_RS_DATA` writer - The content of the %sth 32-bit RS code to be programmed."]
pub type PGM_RS_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The content of the %sth 32-bit RS code to be programmed."]
    #[inline(always)]
    pub fn pgm_rs_data(&self) -> PGM_RS_DATA_R {
        PGM_RS_DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PGM_CHECK_VALUE")
            .field("pgm_rs_data", &self.pgm_rs_data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - The content of the %sth 32-bit RS code to be programmed."]
    #[inline(always)]
    pub fn pgm_rs_data(&mut self) -> PGM_RS_DATA_W<PGM_CHECK_VALUE_SPEC> {
        PGM_RS_DATA_W::new(self, 0)
    }
}
#[doc = "Register %s that stores the RS code to be programmed.\n\nYou can [`read`](crate::Reg::read) this register and get [`pgm_check_value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgm_check_value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PGM_CHECK_VALUE_SPEC;
impl crate::RegisterSpec for PGM_CHECK_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pgm_check_value::R`](R) reader structure"]
impl crate::Readable for PGM_CHECK_VALUE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pgm_check_value::W`](W) writer structure"]
impl crate::Writable for PGM_CHECK_VALUE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PGM_CHECK_VALUE%s to value 0"]
impl crate::Resettable for PGM_CHECK_VALUE_SPEC {
    const RESET_VALUE: u32 = 0;
}
