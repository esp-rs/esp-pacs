#[doc = "Register `BUFFER_STATUS_LEVEL` reader"]
pub type R = crate::R<BUFFER_STATUS_LEVEL_SPEC>;
#[doc = "Field `CMD_BUF_EMPTY_CNT` reader - Command Buffer Empty Locations contains the number of empty locations in the command buffer."]
pub type CMD_BUF_EMPTY_CNT_R = crate::FieldReader;
#[doc = "Field `RESP_BUF_CNT` reader - Response Buffer Level Value contains the number of valid data entries in the response buffer."]
pub type RESP_BUF_CNT_R = crate::FieldReader;
#[doc = "Field `IBI_DATA_BUF_CNT` reader - IBI Buffer Level Value contains the number of valid entries in the IBI Buffer. This is field is used in master mode."]
pub type IBI_DATA_BUF_CNT_R = crate::FieldReader;
#[doc = "Field `IBI_STATUS_BUF_CNT` reader - IBI Buffer Status Count contains the number of IBI status entries in the IBI Buffer. This field is used in master mode."]
pub type IBI_STATUS_BUF_CNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - Command Buffer Empty Locations contains the number of empty locations in the command buffer."]
    #[inline(always)]
    pub fn cmd_buf_empty_cnt(&self) -> CMD_BUF_EMPTY_CNT_R {
        CMD_BUF_EMPTY_CNT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Response Buffer Level Value contains the number of valid data entries in the response buffer."]
    #[inline(always)]
    pub fn resp_buf_cnt(&self) -> RESP_BUF_CNT_R {
        RESP_BUF_CNT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - IBI Buffer Level Value contains the number of valid entries in the IBI Buffer. This is field is used in master mode."]
    #[inline(always)]
    pub fn ibi_data_buf_cnt(&self) -> IBI_DATA_BUF_CNT_R {
        IBI_DATA_BUF_CNT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - IBI Buffer Status Count contains the number of IBI status entries in the IBI Buffer. This field is used in master mode."]
    #[inline(always)]
    pub fn ibi_status_buf_cnt(&self) -> IBI_STATUS_BUF_CNT_R {
        IBI_STATUS_BUF_CNT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUFFER_STATUS_LEVEL")
            .field(
                "cmd_buf_empty_cnt",
                &format_args!("{}", self.cmd_buf_empty_cnt().bits()),
            )
            .field(
                "resp_buf_cnt",
                &format_args!("{}", self.resp_buf_cnt().bits()),
            )
            .field(
                "ibi_data_buf_cnt",
                &format_args!("{}", self.ibi_data_buf_cnt().bits()),
            )
            .field(
                "ibi_status_buf_cnt",
                &format_args!("{}", self.ibi_status_buf_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BUFFER_STATUS_LEVEL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "BUFFER_STATUS_LEVEL reflects the status level of Buffers in the controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buffer_status_level::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUFFER_STATUS_LEVEL_SPEC;
impl crate::RegisterSpec for BUFFER_STATUS_LEVEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buffer_status_level::R`](R) reader structure"]
impl crate::Readable for BUFFER_STATUS_LEVEL_SPEC {}
#[doc = "`reset()` method sets BUFFER_STATUS_LEVEL to value 0x10"]
impl crate::Resettable for BUFFER_STATUS_LEVEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
