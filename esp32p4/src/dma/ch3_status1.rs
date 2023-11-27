#[doc = "Register `CH3_STATUS1` reader"]
pub type R = crate::R<CH3_STATUS1_SPEC>;
#[doc = "Field `CH3_DATA_LEFT_IN_FIFO` reader - NA"]
pub type CH3_DATA_LEFT_IN_FIFO_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:14 - NA"]
    #[inline(always)]
    pub fn ch3_data_left_in_fifo(&self) -> CH3_DATA_LEFT_IN_FIFO_R {
        CH3_DATA_LEFT_IN_FIFO_R::new((self.bits & 0x7fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH3_STATUS1")
            .field(
                "ch3_data_left_in_fifo",
                &format_args!("{}", self.ch3_data_left_in_fifo().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH3_STATUS1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_status1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH3_STATUS1_SPEC;
impl crate::RegisterSpec for CH3_STATUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3_status1::R`](R) reader structure"]
impl crate::Readable for CH3_STATUS1_SPEC {}
#[doc = "`reset()` method sets CH3_STATUS1 to value 0"]
impl crate::Resettable for CH3_STATUS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
