#[doc = "Register `CH4_STATUS1` reader"]
pub type R = crate::R<CH4_STATUS1_SPEC>;
#[doc = "Field `CH4_DATA_LEFT_IN_FIFO` reader - NA"]
pub type CH4_DATA_LEFT_IN_FIFO_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:14 - NA"]
    #[inline(always)]
    pub fn ch4_data_left_in_fifo(&self) -> CH4_DATA_LEFT_IN_FIFO_R {
        CH4_DATA_LEFT_IN_FIFO_R::new((self.bits & 0x7fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH4_STATUS1")
            .field(
                "ch4_data_left_in_fifo",
                &format_args!("{}", self.ch4_data_left_in_fifo().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH4_STATUS1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_status1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH4_STATUS1_SPEC;
impl crate::RegisterSpec for CH4_STATUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch4_status1::R`](R) reader structure"]
impl crate::Readable for CH4_STATUS1_SPEC {}
#[doc = "`reset()` method sets CH4_STATUS1 to value 0"]
impl crate::Resettable for CH4_STATUS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
