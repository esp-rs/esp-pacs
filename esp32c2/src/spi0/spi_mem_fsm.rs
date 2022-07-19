#[doc = "Register `SPI_MEM_FSM` reader"]
pub struct R(crate::R<SPI_MEM_FSM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_FSM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_FSM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_FSM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_FSM` writer"]
pub struct W(crate::W<SPI_MEM_FSM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_FSM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SPI_MEM_FSM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_FSM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_MEM_CSPI_ST` reader - The current status of SPI0 slave FSM: spi0_slv_st. 0: idle state, 1: preparation state, 2: send command state, 3: send address state, 4: wait state, 5: read data state, 6:write data state, 7: done state, 8: read data end state."]
pub type SPI_MEM_CSPI_ST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_MEM_EM_ST` reader - The current status of SPI0 master FSM: spi0_mst_st. 0: idle state, 1:EM_CACHE_GRANT , 2: program/erase suspend state, 3: SPI0 read data state, 4: wait cache/EDMA sent data is stored in SPI0 TX FIFO, 5: SPI0 write data state."]
pub type SPI_MEM_EM_ST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_MEM_CSPI_LOCK_DELAY_TIME` reader - The lock delay time of SPI0/1 arbiter by spi0_slv_st, after PER is sent by SPI1."]
pub type SPI_MEM_CSPI_LOCK_DELAY_TIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_MEM_CSPI_LOCK_DELAY_TIME` writer - The lock delay time of SPI0/1 arbiter by spi0_slv_st, after PER is sent by SPI1."]
pub type SPI_MEM_CSPI_LOCK_DELAY_TIME_W<'a> =
    crate::FieldWriter<'a, u32, SPI_MEM_FSM_SPEC, u8, u8, 5, 7>;
impl R {
    #[doc = "Bits 0:3 - The current status of SPI0 slave FSM: spi0_slv_st. 0: idle state, 1: preparation state, 2: send command state, 3: send address state, 4: wait state, 5: read data state, 6:write data state, 7: done state, 8: read data end state."]
    #[inline(always)]
    pub fn spi_mem_cspi_st(&self) -> SPI_MEM_CSPI_ST_R {
        SPI_MEM_CSPI_ST_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - The current status of SPI0 master FSM: spi0_mst_st. 0: idle state, 1:EM_CACHE_GRANT , 2: program/erase suspend state, 3: SPI0 read data state, 4: wait cache/EDMA sent data is stored in SPI0 TX FIFO, 5: SPI0 write data state."]
    #[inline(always)]
    pub fn spi_mem_em_st(&self) -> SPI_MEM_EM_ST_R {
        SPI_MEM_EM_ST_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:11 - The lock delay time of SPI0/1 arbiter by spi0_slv_st, after PER is sent by SPI1."]
    #[inline(always)]
    pub fn spi_mem_cspi_lock_delay_time(&self) -> SPI_MEM_CSPI_LOCK_DELAY_TIME_R {
        SPI_MEM_CSPI_LOCK_DELAY_TIME_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 7:11 - The lock delay time of SPI0/1 arbiter by spi0_slv_st, after PER is sent by SPI1."]
    #[inline(always)]
    pub fn spi_mem_cspi_lock_delay_time(&mut self) -> SPI_MEM_CSPI_LOCK_DELAY_TIME_W {
        SPI_MEM_CSPI_LOCK_DELAY_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 FSM status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_fsm](index.html) module"]
pub struct SPI_MEM_FSM_SPEC;
impl crate::RegisterSpec for SPI_MEM_FSM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_fsm::R](R) reader structure"]
impl crate::Readable for SPI_MEM_FSM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_fsm::W](W) writer structure"]
impl crate::Writable for SPI_MEM_FSM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_MEM_FSM to value 0x0200"]
impl crate::Resettable for SPI_MEM_FSM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}
