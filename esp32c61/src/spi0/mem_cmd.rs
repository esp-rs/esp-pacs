#[doc = "Register `MEM_CMD` reader"]
pub type R = crate::R<MEM_CMD_SPEC>;
#[doc = "Field `MEM_MST_ST` reader - The current status of SPI0 master FSM: spi0_mst_st. 0: idle state, 1:SPI0_GRANT , 2: program/erase suspend state, 3: SPI0 read data state, 4: wait cache/EDMA sent data is stored in SPI0 TX FIFO, 5: SPI0 write data state."]
pub type MEM_MST_ST_R = crate::FieldReader;
#[doc = "Field `MEM_SLV_ST` reader - The current status of SPI0 slave FSM: mspi_st. 0: idle state, 1: preparation state, 2: send command state, 3: send address state, 4: wait state, 5: read data state, 6:write data state, 7: done state, 8: read data end state."]
pub type MEM_SLV_ST_R = crate::FieldReader;
#[doc = "Field `MEM_USR` reader - SPI0 USR_CMD start bit, only used when SPI_MEM_AXI_REQ_EN is cleared. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type MEM_USR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - The current status of SPI0 master FSM: spi0_mst_st. 0: idle state, 1:SPI0_GRANT , 2: program/erase suspend state, 3: SPI0 read data state, 4: wait cache/EDMA sent data is stored in SPI0 TX FIFO, 5: SPI0 write data state."]
    #[inline(always)]
    pub fn mem_mst_st(&self) -> MEM_MST_ST_R {
        MEM_MST_ST_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - The current status of SPI0 slave FSM: mspi_st. 0: idle state, 1: preparation state, 2: send command state, 3: send address state, 4: wait state, 5: read data state, 6:write data state, 7: done state, 8: read data end state."]
    #[inline(always)]
    pub fn mem_slv_st(&self) -> MEM_SLV_ST_R {
        MEM_SLV_ST_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - SPI0 USR_CMD start bit, only used when SPI_MEM_AXI_REQ_EN is cleared. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn mem_usr(&self) -> MEM_USR_R {
        MEM_USR_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_CMD")
            .field("mem_mst_st", &self.mem_mst_st())
            .field("mem_slv_st", &self.mem_slv_st())
            .field("mem_usr", &self.mem_usr())
            .finish()
    }
}
#[doc = "SPI0 FSM status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_cmd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_CMD_SPEC;
impl crate::RegisterSpec for MEM_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_cmd::R`](R) reader structure"]
impl crate::Readable for MEM_CMD_SPEC {}
#[doc = "`reset()` method sets MEM_CMD to value 0"]
impl crate::Resettable for MEM_CMD_SPEC {}
