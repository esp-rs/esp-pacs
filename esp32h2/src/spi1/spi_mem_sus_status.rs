#[doc = "Register `SPI_MEM_SUS_STATUS` reader"]
pub struct R(crate::R<SPI_MEM_SUS_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_SUS_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_SUS_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_SUS_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_SUS_STATUS` writer"]
pub struct W(crate::W<SPI_MEM_SUS_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_SUS_STATUS_SPEC>;
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
impl From<crate::W<SPI_MEM_SUS_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_SUS_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_MEM_FLASH_SUS` reader - The status of flash suspend, only used in SPI1."]
pub type SPI_MEM_FLASH_SUS_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FLASH_SUS` writer - The status of flash suspend, only used in SPI1."]
pub type SPI_MEM_FLASH_SUS_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_SUS_STATUS_SPEC, O>;
#[doc = "Field `SPI_MEM_WAIT_PESR_CMD_2B` reader - 1: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND\\[15:0\\] to check SUS/SUS1/SUS2 bit. 0: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND\\[7:0\\] to check SUS/SUS1/SUS2 bit."]
pub type SPI_MEM_WAIT_PESR_CMD_2B_R = crate::BitReader;
#[doc = "Field `SPI_MEM_WAIT_PESR_CMD_2B` writer - 1: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND\\[15:0\\] to check SUS/SUS1/SUS2 bit. 0: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND\\[7:0\\] to check SUS/SUS1/SUS2 bit."]
pub type SPI_MEM_WAIT_PESR_CMD_2B_W<'a, const O: u8> =
    crate::BitWriter<'a, SPI_MEM_SUS_STATUS_SPEC, O>;
#[doc = "Field `SPI_MEM_FLASH_HPM_DLY_128` reader - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after HPM command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after HPM command is sent."]
pub type SPI_MEM_FLASH_HPM_DLY_128_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FLASH_HPM_DLY_128` writer - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after HPM command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after HPM command is sent."]
pub type SPI_MEM_FLASH_HPM_DLY_128_W<'a, const O: u8> =
    crate::BitWriter<'a, SPI_MEM_SUS_STATUS_SPEC, O>;
#[doc = "Field `SPI_MEM_FLASH_RES_DLY_128` reader - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after RES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after RES command is sent."]
pub type SPI_MEM_FLASH_RES_DLY_128_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FLASH_RES_DLY_128` writer - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after RES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after RES command is sent."]
pub type SPI_MEM_FLASH_RES_DLY_128_W<'a, const O: u8> =
    crate::BitWriter<'a, SPI_MEM_SUS_STATUS_SPEC, O>;
#[doc = "Field `SPI_MEM_FLASH_DP_DLY_128` reader - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after DP command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after DP command is sent."]
pub type SPI_MEM_FLASH_DP_DLY_128_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FLASH_DP_DLY_128` writer - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after DP command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after DP command is sent."]
pub type SPI_MEM_FLASH_DP_DLY_128_W<'a, const O: u8> =
    crate::BitWriter<'a, SPI_MEM_SUS_STATUS_SPEC, O>;
#[doc = "Field `SPI_MEM_FLASH_PER_DLY_128` reader - Valid when SPI_MEM_FLASH_PER_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after PER command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after PER command is sent."]
pub type SPI_MEM_FLASH_PER_DLY_128_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FLASH_PER_DLY_128` writer - Valid when SPI_MEM_FLASH_PER_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after PER command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after PER command is sent."]
pub type SPI_MEM_FLASH_PER_DLY_128_W<'a, const O: u8> =
    crate::BitWriter<'a, SPI_MEM_SUS_STATUS_SPEC, O>;
#[doc = "Field `SPI_MEM_FLASH_PES_DLY_128` reader - Valid when SPI_MEM_FLASH_PES_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after PES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after PES command is sent."]
pub type SPI_MEM_FLASH_PES_DLY_128_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FLASH_PES_DLY_128` writer - Valid when SPI_MEM_FLASH_PES_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after PES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after PES command is sent."]
pub type SPI_MEM_FLASH_PES_DLY_128_W<'a, const O: u8> =
    crate::BitWriter<'a, SPI_MEM_SUS_STATUS_SPEC, O>;
#[doc = "Field `SPI_MEM_SPI0_LOCK_EN` reader - 1: Enable SPI0 lock SPI0/1 arbiter option. 0: Disable it."]
pub type SPI_MEM_SPI0_LOCK_EN_R = crate::BitReader;
#[doc = "Field `SPI_MEM_SPI0_LOCK_EN` writer - 1: Enable SPI0 lock SPI0/1 arbiter option. 0: Disable it."]
pub type SPI_MEM_SPI0_LOCK_EN_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_SUS_STATUS_SPEC, O>;
#[doc = "Field `SPI_MEM_FLASH_PESR_CMD_2B` reader - 1: The bit length of Program/Erase Suspend/Resume command is 16. 0: The bit length of Program/Erase Suspend/Resume command is 8."]
pub type SPI_MEM_FLASH_PESR_CMD_2B_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FLASH_PESR_CMD_2B` writer - 1: The bit length of Program/Erase Suspend/Resume command is 16. 0: The bit length of Program/Erase Suspend/Resume command is 8."]
pub type SPI_MEM_FLASH_PESR_CMD_2B_W<'a, const O: u8> =
    crate::BitWriter<'a, SPI_MEM_SUS_STATUS_SPEC, O>;
#[doc = "Field `SPI_MEM_FLASH_PER_COMMAND` reader - Program/Erase resume command."]
pub type SPI_MEM_FLASH_PER_COMMAND_R = crate::FieldReader<u16>;
#[doc = "Field `SPI_MEM_FLASH_PER_COMMAND` writer - Program/Erase resume command."]
pub type SPI_MEM_FLASH_PER_COMMAND_W<'a, const O: u8> =
    crate::FieldWriter<'a, SPI_MEM_SUS_STATUS_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bit 0 - The status of flash suspend, only used in SPI1."]
    #[inline(always)]
    pub fn spi_mem_flash_sus(&self) -> SPI_MEM_FLASH_SUS_R {
        SPI_MEM_FLASH_SUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND\\[15:0\\] to check SUS/SUS1/SUS2 bit. 0: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND\\[7:0\\] to check SUS/SUS1/SUS2 bit."]
    #[inline(always)]
    pub fn spi_mem_wait_pesr_cmd_2b(&self) -> SPI_MEM_WAIT_PESR_CMD_2B_R {
        SPI_MEM_WAIT_PESR_CMD_2B_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after HPM command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after HPM command is sent."]
    #[inline(always)]
    pub fn spi_mem_flash_hpm_dly_128(&self) -> SPI_MEM_FLASH_HPM_DLY_128_R {
        SPI_MEM_FLASH_HPM_DLY_128_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after RES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after RES command is sent."]
    #[inline(always)]
    pub fn spi_mem_flash_res_dly_128(&self) -> SPI_MEM_FLASH_RES_DLY_128_R {
        SPI_MEM_FLASH_RES_DLY_128_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after DP command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after DP command is sent."]
    #[inline(always)]
    pub fn spi_mem_flash_dp_dly_128(&self) -> SPI_MEM_FLASH_DP_DLY_128_R {
        SPI_MEM_FLASH_DP_DLY_128_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Valid when SPI_MEM_FLASH_PER_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after PER command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after PER command is sent."]
    #[inline(always)]
    pub fn spi_mem_flash_per_dly_128(&self) -> SPI_MEM_FLASH_PER_DLY_128_R {
        SPI_MEM_FLASH_PER_DLY_128_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Valid when SPI_MEM_FLASH_PES_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after PES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after PES command is sent."]
    #[inline(always)]
    pub fn spi_mem_flash_pes_dly_128(&self) -> SPI_MEM_FLASH_PES_DLY_128_R {
        SPI_MEM_FLASH_PES_DLY_128_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: Enable SPI0 lock SPI0/1 arbiter option. 0: Disable it."]
    #[inline(always)]
    pub fn spi_mem_spi0_lock_en(&self) -> SPI_MEM_SPI0_LOCK_EN_R {
        SPI_MEM_SPI0_LOCK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - 1: The bit length of Program/Erase Suspend/Resume command is 16. 0: The bit length of Program/Erase Suspend/Resume command is 8."]
    #[inline(always)]
    pub fn spi_mem_flash_pesr_cmd_2b(&self) -> SPI_MEM_FLASH_PESR_CMD_2B_R {
        SPI_MEM_FLASH_PESR_CMD_2B_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Program/Erase resume command."]
    #[inline(always)]
    pub fn spi_mem_flash_per_command(&self) -> SPI_MEM_FLASH_PER_COMMAND_R {
        SPI_MEM_FLASH_PER_COMMAND_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_SUS_STATUS")
            .field(
                "spi_mem_flash_sus",
                &format_args!("{}", self.spi_mem_flash_sus().bit()),
            )
            .field(
                "spi_mem_wait_pesr_cmd_2b",
                &format_args!("{}", self.spi_mem_wait_pesr_cmd_2b().bit()),
            )
            .field(
                "spi_mem_flash_hpm_dly_128",
                &format_args!("{}", self.spi_mem_flash_hpm_dly_128().bit()),
            )
            .field(
                "spi_mem_flash_res_dly_128",
                &format_args!("{}", self.spi_mem_flash_res_dly_128().bit()),
            )
            .field(
                "spi_mem_flash_dp_dly_128",
                &format_args!("{}", self.spi_mem_flash_dp_dly_128().bit()),
            )
            .field(
                "spi_mem_flash_per_dly_128",
                &format_args!("{}", self.spi_mem_flash_per_dly_128().bit()),
            )
            .field(
                "spi_mem_flash_pes_dly_128",
                &format_args!("{}", self.spi_mem_flash_pes_dly_128().bit()),
            )
            .field(
                "spi_mem_spi0_lock_en",
                &format_args!("{}", self.spi_mem_spi0_lock_en().bit()),
            )
            .field(
                "spi_mem_flash_pesr_cmd_2b",
                &format_args!("{}", self.spi_mem_flash_pesr_cmd_2b().bit()),
            )
            .field(
                "spi_mem_flash_per_command",
                &format_args!("{}", self.spi_mem_flash_per_command().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_SUS_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The status of flash suspend, only used in SPI1."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_flash_sus(&mut self) -> SPI_MEM_FLASH_SUS_W<0> {
        SPI_MEM_FLASH_SUS_W::new(self)
    }
    #[doc = "Bit 1 - 1: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND\\[15:0\\] to check SUS/SUS1/SUS2 bit. 0: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND\\[7:0\\] to check SUS/SUS1/SUS2 bit."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_wait_pesr_cmd_2b(&mut self) -> SPI_MEM_WAIT_PESR_CMD_2B_W<1> {
        SPI_MEM_WAIT_PESR_CMD_2B_W::new(self)
    }
    #[doc = "Bit 2 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after HPM command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after HPM command is sent."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_flash_hpm_dly_128(&mut self) -> SPI_MEM_FLASH_HPM_DLY_128_W<2> {
        SPI_MEM_FLASH_HPM_DLY_128_W::new(self)
    }
    #[doc = "Bit 3 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after RES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after RES command is sent."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_flash_res_dly_128(&mut self) -> SPI_MEM_FLASH_RES_DLY_128_W<3> {
        SPI_MEM_FLASH_RES_DLY_128_W::new(self)
    }
    #[doc = "Bit 4 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after DP command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after DP command is sent."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_flash_dp_dly_128(&mut self) -> SPI_MEM_FLASH_DP_DLY_128_W<4> {
        SPI_MEM_FLASH_DP_DLY_128_W::new(self)
    }
    #[doc = "Bit 5 - Valid when SPI_MEM_FLASH_PER_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after PER command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after PER command is sent."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_flash_per_dly_128(&mut self) -> SPI_MEM_FLASH_PER_DLY_128_W<5> {
        SPI_MEM_FLASH_PER_DLY_128_W::new(self)
    }
    #[doc = "Bit 6 - Valid when SPI_MEM_FLASH_PES_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after PES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after PES command is sent."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_flash_pes_dly_128(&mut self) -> SPI_MEM_FLASH_PES_DLY_128_W<6> {
        SPI_MEM_FLASH_PES_DLY_128_W::new(self)
    }
    #[doc = "Bit 7 - 1: Enable SPI0 lock SPI0/1 arbiter option. 0: Disable it."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_spi0_lock_en(&mut self) -> SPI_MEM_SPI0_LOCK_EN_W<7> {
        SPI_MEM_SPI0_LOCK_EN_W::new(self)
    }
    #[doc = "Bit 15 - 1: The bit length of Program/Erase Suspend/Resume command is 16. 0: The bit length of Program/Erase Suspend/Resume command is 8."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_flash_pesr_cmd_2b(&mut self) -> SPI_MEM_FLASH_PESR_CMD_2B_W<15> {
        SPI_MEM_FLASH_PESR_CMD_2B_W::new(self)
    }
    #[doc = "Bits 16:31 - Program/Erase resume command."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_flash_per_command(&mut self) -> SPI_MEM_FLASH_PER_COMMAND_W<16> {
        SPI_MEM_FLASH_PER_COMMAND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 flash suspend status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_sus_status](index.html) module"]
pub struct SPI_MEM_SUS_STATUS_SPEC;
impl crate::RegisterSpec for SPI_MEM_SUS_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_sus_status::R](R) reader structure"]
impl crate::Readable for SPI_MEM_SUS_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_sus_status::W](W) writer structure"]
impl crate::Writable for SPI_MEM_SUS_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_SUS_STATUS to value 0x7a7a_0000"]
impl crate::Resettable for SPI_MEM_SUS_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x7a7a_0000;
}
