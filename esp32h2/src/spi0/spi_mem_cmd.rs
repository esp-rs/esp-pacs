#[doc = "Register `SPI_MEM_CMD` reader"]
pub struct R(crate::R<SPI_MEM_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SPI_MEM_MST_ST` reader - The current status of SPI0 master FSM: spi0_mst_st. 0: idle state, 1:SPI0_GRANT , 2: program/erase suspend state, 3: SPI0 read data state, 4: wait cache/EDMA sent data is stored in SPI0 TX FIFO, 5: SPI0 write data state."]
pub type SPI_MEM_MST_ST_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_SLV_ST` reader - The current status of SPI0 slave FSM: mspi_st. 0: idle state, 1: preparation state, 2: send command state, 3: send address state, 4: wait state, 5: read data state, 6:write data state, 7: done state, 8: read data end state."]
pub type SPI_MEM_SLV_ST_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_USR` reader - SPI0 USR_CMD start bit, only used when SPI_MEM_AXI_REQ_EN is cleared. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type SPI_MEM_USR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - The current status of SPI0 master FSM: spi0_mst_st. 0: idle state, 1:SPI0_GRANT , 2: program/erase suspend state, 3: SPI0 read data state, 4: wait cache/EDMA sent data is stored in SPI0 TX FIFO, 5: SPI0 write data state."]
    #[inline(always)]
    pub fn spi_mem_mst_st(&self) -> SPI_MEM_MST_ST_R {
        SPI_MEM_MST_ST_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - The current status of SPI0 slave FSM: mspi_st. 0: idle state, 1: preparation state, 2: send command state, 3: send address state, 4: wait state, 5: read data state, 6:write data state, 7: done state, 8: read data end state."]
    #[inline(always)]
    pub fn spi_mem_slv_st(&self) -> SPI_MEM_SLV_ST_R {
        SPI_MEM_SLV_ST_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - SPI0 USR_CMD start bit, only used when SPI_MEM_AXI_REQ_EN is cleared. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_usr(&self) -> SPI_MEM_USR_R {
        SPI_MEM_USR_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_CMD")
            .field(
                "spi_mem_mst_st",
                &format_args!("{}", self.spi_mem_mst_st().bits()),
            )
            .field(
                "spi_mem_slv_st",
                &format_args!("{}", self.spi_mem_slv_st().bits()),
            )
            .field("spi_mem_usr", &format_args!("{}", self.spi_mem_usr().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_CMD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "SPI0 FSM status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_cmd](index.html) module"]
pub struct SPI_MEM_CMD_SPEC;
impl crate::RegisterSpec for SPI_MEM_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_cmd::R](R) reader structure"]
impl crate::Readable for SPI_MEM_CMD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPI_MEM_CMD to value 0"]
impl crate::Resettable for SPI_MEM_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
