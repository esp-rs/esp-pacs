#[doc = "Register `IBUS0_ABANDON_CNT` reader"]
pub type R = crate::R<IBUS0_ABANDON_CNT_SPEC>;
#[doc = "Field `IBUS0_ABANDON_CNT` reader - The bits are used to count the number of the abandoned ibus0 access."]
pub type IBUS0_ABANDON_CNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - The bits are used to count the number of the abandoned ibus0 access."]
    #[inline(always)]
    pub fn ibus0_abandon_cnt(&self) -> IBUS0_ABANDON_CNT_R {
        IBUS0_ABANDON_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IBUS0_ABANDON_CNT")
            .field("ibus0_abandon_cnt", &self.ibus0_abandon_cnt())
            .finish()
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus0_abandon_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IBUS0_ABANDON_CNT_SPEC;
impl crate::RegisterSpec for IBUS0_ABANDON_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ibus0_abandon_cnt::R`](R) reader structure"]
impl crate::Readable for IBUS0_ABANDON_CNT_SPEC {}
#[doc = "`reset()` method sets IBUS0_ABANDON_CNT to value 0"]
impl crate::Resettable for IBUS0_ABANDON_CNT_SPEC {}
