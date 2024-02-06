#[doc = "Register `CH2_STATUS1` reader"]
pub type R = crate::R<CH2_STATUS1_SPEC>;
#[doc = "Field `CH2_DATA_LEFT_IN_FIFO` reader - NA"]
pub type CH2_DATA_LEFT_IN_FIFO_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:14 - NA"]
    #[inline(always)]
    pub fn ch2_data_left_in_fifo(&self) -> CH2_DATA_LEFT_IN_FIFO_R {
        CH2_DATA_LEFT_IN_FIFO_R::new((self.bits & 0x7fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH2_STATUS1")
            .field(
                "ch2_data_left_in_fifo",
                &format_args!("{}", self.ch2_data_left_in_fifo().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH2_STATUS1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_status1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH2_STATUS1_SPEC;
impl crate::RegisterSpec for CH2_STATUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2_status1::R`](R) reader structure"]
impl crate::Readable for CH2_STATUS1_SPEC {}
#[doc = "`reset()` method sets CH2_STATUS1 to value 0"]
impl crate::Resettable for CH2_STATUS1_SPEC {
    const RESET_VALUE: u32 = 0;
}
