#[doc = "Register `IBI_DATA_BUF` reader"]
pub type R = crate::R<IBI_DATA_BUF_SPEC>;
#[doc = "Field `IBI_DATA` reader - NA"]
pub type IBI_DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn ibi_data(&self) -> IBI_DATA_R {
        IBI_DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IBI_DATA_BUF")
            .field("ibi_data", &self.ibi_data().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IBI_DATA_BUF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ibi_data_buf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IBI_DATA_BUF_SPEC;
impl crate::RegisterSpec for IBI_DATA_BUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ibi_data_buf::R`](R) reader structure"]
impl crate::Readable for IBI_DATA_BUF_SPEC {}
#[doc = "`reset()` method sets IBI_DATA_BUF to value 0"]
impl crate::Resettable for IBI_DATA_BUF_SPEC {
    const RESET_VALUE: u32 = 0;
}
