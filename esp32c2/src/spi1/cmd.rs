#[doc = "Register `CMD` reader"]
pub struct R(crate::R<CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI1_MST_ST` reader - The current status of SPI1 master FSM."]
pub type SPI1_MST_ST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MSPI_ST` reader - The current status of SPI1 slave FSM: mspi_st. 0: idle state, 1: preparation state, 2: send command state, 3: send address state, 4: wait state, 5: read data state, 6:write data state, 7: done state, 8: read data end state."]
pub type MSPI_ST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLASH_PE` reader - In user mode, it is set to indicate that program/erase operation will be triggered. The bit is combined with spi_mem_usr bit. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_PE_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_PE` writer - In user mode, it is set to indicate that program/erase operation will be triggered. The bit is combined with spi_mem_usr bit. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `USR` reader - User define command enable. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type USR_R = crate::BitReader<bool>;
#[doc = "Field `USR` writer - User define command enable. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type USR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `FLASH_HPM` reader - Drive Flash into high performance mode. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_HPM_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_HPM` writer - Drive Flash into high performance mode. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_HPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `FLASH_RES` reader - This bit combined with reg_resandres bit releases Flash from the power-down state or high performance mode and obtains the devices ID. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_RES_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_RES` writer - This bit combined with reg_resandres bit releases Flash from the power-down state or high performance mode and obtains the devices ID. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_RES_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `FLASH_DP` reader - Drive Flash into power down. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_DP_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_DP` writer - Drive Flash into power down. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_DP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `FLASH_CE` reader - Chip erase enable. Chip erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_CE_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_CE` writer - Chip erase enable. Chip erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_CE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `FLASH_BE` reader - Block erase enable(32KB) . Block erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_BE_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_BE` writer - Block erase enable(32KB) . Block erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_BE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `FLASH_SE` reader - Sector erase enable(4KB). Sector erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_SE_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_SE` writer - Sector erase enable(4KB). Sector erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_SE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `FLASH_PP` reader - Page program enable(1 byte ~256 bytes data to be programmed). Page program operation will be triggered when the bit is set. The bit will be cleared once the operation done .1: enable 0: disable."]
pub type FLASH_PP_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_PP` writer - Page program enable(1 byte ~256 bytes data to be programmed). Page program operation will be triggered when the bit is set. The bit will be cleared once the operation done .1: enable 0: disable."]
pub type FLASH_PP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `FLASH_WRSR` reader - Write status register enable. Write status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_WRSR_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_WRSR` writer - Write status register enable. Write status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_WRSR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `FLASH_RDSR` reader - Read status register-1. Read status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_RDSR_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_RDSR` writer - Read status register-1. Read status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_RDSR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `FLASH_RDID` reader - Read JEDEC ID . Read ID command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
pub type FLASH_RDID_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_RDID` writer - Read JEDEC ID . Read ID command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
pub type FLASH_RDID_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `FLASH_WRDI` reader - Write flash disable. Write disable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
pub type FLASH_WRDI_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_WRDI` writer - Write flash disable. Write disable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
pub type FLASH_WRDI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `FLASH_WREN` reader - Write flash enable. Write enable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
pub type FLASH_WREN_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_WREN` writer - Write flash enable. Write enable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
pub type FLASH_WREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `FLASH_READ` reader - Read flash enable. Read flash operation will be triggered when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
pub type FLASH_READ_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_READ` writer - Read flash enable. Read flash operation will be triggered when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
pub type FLASH_READ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - The current status of SPI1 master FSM."]
    #[inline(always)]
    pub fn spi1_mst_st(&self) -> SPI1_MST_ST_R {
        SPI1_MST_ST_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - The current status of SPI1 slave FSM: mspi_st. 0: idle state, 1: preparation state, 2: send command state, 3: send address state, 4: wait state, 5: read data state, 6:write data state, 7: done state, 8: read data end state."]
    #[inline(always)]
    pub fn mspi_st(&self) -> MSPI_ST_R {
        MSPI_ST_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 17 - In user mode, it is set to indicate that program/erase operation will be triggered. The bit is combined with spi_mem_usr bit. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_pe(&self) -> FLASH_PE_R {
        FLASH_PE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - User define command enable. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn usr(&self) -> USR_R {
        USR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Drive Flash into high performance mode. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_hpm(&self) -> FLASH_HPM_R {
        FLASH_HPM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - This bit combined with reg_resandres bit releases Flash from the power-down state or high performance mode and obtains the devices ID. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_res(&self) -> FLASH_RES_R {
        FLASH_RES_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Drive Flash into power down. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_dp(&self) -> FLASH_DP_R {
        FLASH_DP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Chip erase enable. Chip erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_ce(&self) -> FLASH_CE_R {
        FLASH_CE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Block erase enable(32KB) . Block erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_be(&self) -> FLASH_BE_R {
        FLASH_BE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Sector erase enable(4KB). Sector erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_se(&self) -> FLASH_SE_R {
        FLASH_SE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Page program enable(1 byte ~256 bytes data to be programmed). Page program operation will be triggered when the bit is set. The bit will be cleared once the operation done .1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_pp(&self) -> FLASH_PP_R {
        FLASH_PP_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Write status register enable. Write status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_wrsr(&self) -> FLASH_WRSR_R {
        FLASH_WRSR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Read status register-1. Read status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_rdsr(&self) -> FLASH_RDSR_R {
        FLASH_RDSR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Read JEDEC ID . Read ID command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_rdid(&self) -> FLASH_RDID_R {
        FLASH_RDID_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Write flash disable. Write disable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_wrdi(&self) -> FLASH_WRDI_R {
        FLASH_WRDI_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Write flash enable. Write enable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_wren(&self) -> FLASH_WREN_R {
        FLASH_WREN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Read flash enable. Read flash operation will be triggered when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_read(&self) -> FLASH_READ_R {
        FLASH_READ_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - In user mode, it is set to indicate that program/erase operation will be triggered. The bit is combined with spi_mem_usr bit. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_pe(&mut self) -> FLASH_PE_W<17> {
        FLASH_PE_W::new(self)
    }
    #[doc = "Bit 18 - User define command enable. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn usr(&mut self) -> USR_W<18> {
        USR_W::new(self)
    }
    #[doc = "Bit 19 - Drive Flash into high performance mode. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_hpm(&mut self) -> FLASH_HPM_W<19> {
        FLASH_HPM_W::new(self)
    }
    #[doc = "Bit 20 - This bit combined with reg_resandres bit releases Flash from the power-down state or high performance mode and obtains the devices ID. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_res(&mut self) -> FLASH_RES_W<20> {
        FLASH_RES_W::new(self)
    }
    #[doc = "Bit 21 - Drive Flash into power down. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_dp(&mut self) -> FLASH_DP_W<21> {
        FLASH_DP_W::new(self)
    }
    #[doc = "Bit 22 - Chip erase enable. Chip erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_ce(&mut self) -> FLASH_CE_W<22> {
        FLASH_CE_W::new(self)
    }
    #[doc = "Bit 23 - Block erase enable(32KB) . Block erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_be(&mut self) -> FLASH_BE_W<23> {
        FLASH_BE_W::new(self)
    }
    #[doc = "Bit 24 - Sector erase enable(4KB). Sector erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_se(&mut self) -> FLASH_SE_W<24> {
        FLASH_SE_W::new(self)
    }
    #[doc = "Bit 25 - Page program enable(1 byte ~256 bytes data to be programmed). Page program operation will be triggered when the bit is set. The bit will be cleared once the operation done .1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_pp(&mut self) -> FLASH_PP_W<25> {
        FLASH_PP_W::new(self)
    }
    #[doc = "Bit 26 - Write status register enable. Write status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_wrsr(&mut self) -> FLASH_WRSR_W<26> {
        FLASH_WRSR_W::new(self)
    }
    #[doc = "Bit 27 - Read status register-1. Read status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_rdsr(&mut self) -> FLASH_RDSR_W<27> {
        FLASH_RDSR_W::new(self)
    }
    #[doc = "Bit 28 - Read JEDEC ID . Read ID command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_rdid(&mut self) -> FLASH_RDID_W<28> {
        FLASH_RDID_W::new(self)
    }
    #[doc = "Bit 29 - Write flash disable. Write disable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_wrdi(&mut self) -> FLASH_WRDI_W<29> {
        FLASH_WRDI_W::new(self)
    }
    #[doc = "Bit 30 - Write flash enable. Write enable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_wren(&mut self) -> FLASH_WREN_W<30> {
        FLASH_WREN_W::new(self)
    }
    #[doc = "Bit 31 - Read flash enable. Read flash operation will be triggered when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_read(&mut self) -> FLASH_READ_W<31> {
        FLASH_READ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 memory command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd::R](R) reader structure"]
impl crate::Readable for CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
