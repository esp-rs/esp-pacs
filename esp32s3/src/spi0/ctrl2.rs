#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<CTRL2_SPEC>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<CTRL2_SPEC>;
#[doc = "Field `CS_SETUP_TIME` reader - (cycles-1) of PREP phase by SPI_CLK, which is the SPI_CS setup time. These bits are combined with SPI_MEM_CS_SETUP bit."]
pub type CS_SETUP_TIME_R = crate::FieldReader;
#[doc = "Field `CS_SETUP_TIME` writer - (cycles-1) of PREP phase by SPI_CLK, which is the SPI_CS setup time. These bits are combined with SPI_MEM_CS_SETUP bit."]
pub type CS_SETUP_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CS_HOLD_TIME` reader - SPI Bus CS (SPI_CS) signal is delayed to inactive by SPI Bus clock (SPI_CLK), which is the SPI_CS hold time in non-ECC mode. These bits are combined with SPI_MEM_CS_HOLD bit."]
pub type CS_HOLD_TIME_R = crate::FieldReader;
#[doc = "Field `CS_HOLD_TIME` writer - SPI Bus CS (SPI_CS) signal is delayed to inactive by SPI Bus clock (SPI_CLK), which is the SPI_CS hold time in non-ECC mode. These bits are combined with SPI_MEM_CS_HOLD bit."]
pub type CS_HOLD_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ECC_CS_HOLD_TIME` reader - SPI_MEM_CS_HOLD_TIME + SPI_MEM_ECC_CS_HOLD_TIME is the SPI_CS hold cycle in ECC mode when accessed flash."]
pub type ECC_CS_HOLD_TIME_R = crate::FieldReader;
#[doc = "Field `ECC_CS_HOLD_TIME` writer - SPI_MEM_CS_HOLD_TIME + SPI_MEM_ECC_CS_HOLD_TIME is the SPI_CS hold cycle in ECC mode when accessed flash."]
pub type ECC_CS_HOLD_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ECC_SKIP_PAGE_CORNER` reader - 1: MSPI skips page corner when accesses flash. 0: Not skip page corner when accesses flash."]
pub type ECC_SKIP_PAGE_CORNER_R = crate::BitReader;
#[doc = "Field `ECC_SKIP_PAGE_CORNER` writer - 1: MSPI skips page corner when accesses flash. 0: Not skip page corner when accesses flash."]
pub type ECC_SKIP_PAGE_CORNER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_16TO18_BYTE_EN` reader - Set this bit to enable MSPI ECC 16 bytes data with 2 ECC bytes mode when accesses flash."]
pub type ECC_16TO18_BYTE_EN_R = crate::BitReader;
#[doc = "Field `ECC_16TO18_BYTE_EN` writer - Set this bit to enable MSPI ECC 16 bytes data with 2 ECC bytes mode when accesses flash."]
pub type ECC_16TO18_BYTE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_HOLD_DELAY` reader - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to flash. tSHSL is (SPI_MEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
pub type CS_HOLD_DELAY_R = crate::FieldReader;
#[doc = "Field `CS_HOLD_DELAY` writer - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to flash. tSHSL is (SPI_MEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
pub type CS_HOLD_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SYNC_RESET` reader - The FSM will be reset."]
pub type SYNC_RESET_R = crate::BitReader;
#[doc = "Field `SYNC_RESET` writer - The FSM will be reset."]
pub type SYNC_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - (cycles-1) of PREP phase by SPI_CLK, which is the SPI_CS setup time. These bits are combined with SPI_MEM_CS_SETUP bit."]
    #[inline(always)]
    pub fn cs_setup_time(&self) -> CS_SETUP_TIME_R {
        CS_SETUP_TIME_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - SPI Bus CS (SPI_CS) signal is delayed to inactive by SPI Bus clock (SPI_CLK), which is the SPI_CS hold time in non-ECC mode. These bits are combined with SPI_MEM_CS_HOLD bit."]
    #[inline(always)]
    pub fn cs_hold_time(&self) -> CS_HOLD_TIME_R {
        CS_HOLD_TIME_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:12 - SPI_MEM_CS_HOLD_TIME + SPI_MEM_ECC_CS_HOLD_TIME is the SPI_CS hold cycle in ECC mode when accessed flash."]
    #[inline(always)]
    pub fn ecc_cs_hold_time(&self) -> ECC_CS_HOLD_TIME_R {
        ECC_CS_HOLD_TIME_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 13 - 1: MSPI skips page corner when accesses flash. 0: Not skip page corner when accesses flash."]
    #[inline(always)]
    pub fn ecc_skip_page_corner(&self) -> ECC_SKIP_PAGE_CORNER_R {
        ECC_SKIP_PAGE_CORNER_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set this bit to enable MSPI ECC 16 bytes data with 2 ECC bytes mode when accesses flash."]
    #[inline(always)]
    pub fn ecc_16to18_byte_en(&self) -> ECC_16TO18_BYTE_EN_R {
        ECC_16TO18_BYTE_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 25:30 - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to flash. tSHSL is (SPI_MEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
    #[inline(always)]
    pub fn cs_hold_delay(&self) -> CS_HOLD_DELAY_R {
        CS_HOLD_DELAY_R::new(((self.bits >> 25) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - The FSM will be reset."]
    #[inline(always)]
    pub fn sync_reset(&self) -> SYNC_RESET_R {
        SYNC_RESET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("cs_setup_time", &self.cs_setup_time())
            .field("cs_hold_time", &self.cs_hold_time())
            .field("ecc_cs_hold_time", &self.ecc_cs_hold_time())
            .field("ecc_skip_page_corner", &self.ecc_skip_page_corner())
            .field("ecc_16to18_byte_en", &self.ecc_16to18_byte_en())
            .field("cs_hold_delay", &self.cs_hold_delay())
            .field("sync_reset", &self.sync_reset())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - (cycles-1) of PREP phase by SPI_CLK, which is the SPI_CS setup time. These bits are combined with SPI_MEM_CS_SETUP bit."]
    #[inline(always)]
    pub fn cs_setup_time(&mut self) -> CS_SETUP_TIME_W<CTRL2_SPEC> {
        CS_SETUP_TIME_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - SPI Bus CS (SPI_CS) signal is delayed to inactive by SPI Bus clock (SPI_CLK), which is the SPI_CS hold time in non-ECC mode. These bits are combined with SPI_MEM_CS_HOLD bit."]
    #[inline(always)]
    pub fn cs_hold_time(&mut self) -> CS_HOLD_TIME_W<CTRL2_SPEC> {
        CS_HOLD_TIME_W::new(self, 5)
    }
    #[doc = "Bits 10:12 - SPI_MEM_CS_HOLD_TIME + SPI_MEM_ECC_CS_HOLD_TIME is the SPI_CS hold cycle in ECC mode when accessed flash."]
    #[inline(always)]
    pub fn ecc_cs_hold_time(&mut self) -> ECC_CS_HOLD_TIME_W<CTRL2_SPEC> {
        ECC_CS_HOLD_TIME_W::new(self, 10)
    }
    #[doc = "Bit 13 - 1: MSPI skips page corner when accesses flash. 0: Not skip page corner when accesses flash."]
    #[inline(always)]
    pub fn ecc_skip_page_corner(&mut self) -> ECC_SKIP_PAGE_CORNER_W<CTRL2_SPEC> {
        ECC_SKIP_PAGE_CORNER_W::new(self, 13)
    }
    #[doc = "Bit 14 - Set this bit to enable MSPI ECC 16 bytes data with 2 ECC bytes mode when accesses flash."]
    #[inline(always)]
    pub fn ecc_16to18_byte_en(&mut self) -> ECC_16TO18_BYTE_EN_W<CTRL2_SPEC> {
        ECC_16TO18_BYTE_EN_W::new(self, 14)
    }
    #[doc = "Bits 25:30 - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to flash. tSHSL is (SPI_MEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
    #[inline(always)]
    pub fn cs_hold_delay(&mut self) -> CS_HOLD_DELAY_W<CTRL2_SPEC> {
        CS_HOLD_DELAY_W::new(self, 25)
    }
    #[doc = "Bit 31 - The FSM will be reset."]
    #[inline(always)]
    pub fn sync_reset(&mut self) -> SYNC_RESET_W<CTRL2_SPEC> {
        SYNC_RESET_W::new(self, 31)
    }
}
#[doc = "SPI0 control 2 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0x2c21"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: u32 = 0x2c21;
}
