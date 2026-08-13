#[doc = "Register `RX_MEM_INFO` reader"]
pub type R = crate::R<RX_MEM_INFO_SPEC>;
#[doc = "Field `RX_BUFF_SIZE` reader - Size of RX buffer in 32-bit words."]
pub type RX_BUFF_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `RX_FREE` reader - Number of free 32 bit words in RX buffer."]
pub type RX_FREE_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:12 - Size of RX buffer in 32-bit words."]
    #[inline(always)]
    pub fn rx_buff_size(&self) -> RX_BUFF_SIZE_R {
        RX_BUFF_SIZE_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Number of free 32 bit words in RX buffer."]
    #[inline(always)]
    pub fn rx_free(&self) -> RX_FREE_R {
        RX_FREE_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_MEM_INFO")
            .field("rx_buff_size", &self.rx_buff_size())
            .field("rx_free", &self.rx_free())
            .finish()
    }
}
#[doc = "TWAI FD rx memory information register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_mem_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_MEM_INFO_SPEC;
impl crate::RegisterSpec for RX_MEM_INFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_mem_info::R`](R) reader structure"]
impl crate::Readable for RX_MEM_INFO_SPEC {}
#[doc = "`reset()` method sets RX_MEM_INFO to value 0x0080_0080"]
impl crate::Resettable for RX_MEM_INFO_SPEC {
    const RESET_VALUE: u32 = 0x0080_0080;
}
