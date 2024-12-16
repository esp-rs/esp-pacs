#[doc = "Register `CMD` reader"]
pub type R = crate::R<CMD_SPEC>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMD_SPEC>;
#[doc = "Field `FLASH_PER` reader - program erase resume bit program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_PER_R = crate::BitReader;
#[doc = "Field `FLASH_PER` writer - program erase resume bit program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_PER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_PES` reader - program erase suspend bit program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_PES_R = crate::BitReader;
#[doc = "Field `FLASH_PES` writer - program erase suspend bit program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_PES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR` reader - User define command enable. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type USR_R = crate::BitReader;
#[doc = "Field `USR` writer - User define command enable. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type USR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_HPM` reader - Drive Flash into high performance mode. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_HPM_R = crate::BitReader;
#[doc = "Field `FLASH_HPM` writer - Drive Flash into high performance mode. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_HPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_RES` reader - This bit combined with reg_resandres bit releases Flash from the power-down state or high performance mode and obtains the devices ID. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_RES_R = crate::BitReader;
#[doc = "Field `FLASH_RES` writer - This bit combined with reg_resandres bit releases Flash from the power-down state or high performance mode and obtains the devices ID. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_RES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_DP` reader - Drive Flash into power down. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_DP_R = crate::BitReader;
#[doc = "Field `FLASH_DP` writer - Drive Flash into power down. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_DP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_CE` reader - Chip erase enable. Chip erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_CE_R = crate::BitReader;
#[doc = "Field `FLASH_CE` writer - Chip erase enable. Chip erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_CE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_BE` reader - Block erase enable. A 64KB block is erased via SPI command D8H. Block erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_BE_R = crate::BitReader;
#[doc = "Field `FLASH_BE` writer - Block erase enable. A 64KB block is erased via SPI command D8H. Block erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_BE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_SE` reader - Sector erase enable. A 4KB sector is erased via SPI command 20H. Sector erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_SE_R = crate::BitReader;
#[doc = "Field `FLASH_SE` writer - Sector erase enable. A 4KB sector is erased via SPI command 20H. Sector erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_SE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_PP` reader - Page program enable(1 byte ~256 bytes data to be programmed). Page program operation will be triggered when the bit is set. The bit will be cleared once the operation done .1: enable 0: disable."]
pub type FLASH_PP_R = crate::BitReader;
#[doc = "Field `FLASH_PP` writer - Page program enable(1 byte ~256 bytes data to be programmed). Page program operation will be triggered when the bit is set. The bit will be cleared once the operation done .1: enable 0: disable."]
pub type FLASH_PP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_WRSR` reader - Write status register enable. Write status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_WRSR_R = crate::BitReader;
#[doc = "Field `FLASH_WRSR` writer - Write status register enable. Write status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_WRSR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_RDSR` reader - Read status register-1. Read status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_RDSR_R = crate::BitReader;
#[doc = "Field `FLASH_RDSR` writer - Read status register-1. Read status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_RDSR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_RDID` reader - Read JEDEC ID . Read ID command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
pub type FLASH_RDID_R = crate::BitReader;
#[doc = "Field `FLASH_RDID` writer - Read JEDEC ID . Read ID command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
pub type FLASH_RDID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_WRDI` reader - Write flash disable. Write disable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
pub type FLASH_WRDI_R = crate::BitReader;
#[doc = "Field `FLASH_WRDI` writer - Write flash disable. Write disable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
pub type FLASH_WRDI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_WREN` reader - Write flash enable. Write enable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
pub type FLASH_WREN_R = crate::BitReader;
#[doc = "Field `FLASH_WREN` writer - Write flash enable. Write enable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
pub type FLASH_WREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_READ` reader - Read flash enable. Read flash operation will be triggered when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
pub type FLASH_READ_R = crate::BitReader;
#[doc = "Field `FLASH_READ` writer - Read flash enable. Read flash operation will be triggered when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
pub type FLASH_READ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - program erase resume bit program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_per(&self) -> FLASH_PER_R {
        FLASH_PER_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - program erase suspend bit program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_pes(&self) -> FLASH_PES_R {
        FLASH_PES_R::new(((self.bits >> 17) & 1) != 0)
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
    #[doc = "Bit 23 - Block erase enable. A 64KB block is erased via SPI command D8H. Block erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_be(&self) -> FLASH_BE_R {
        FLASH_BE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Sector erase enable. A 4KB sector is erased via SPI command 20H. Sector erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
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
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD")
            .field("flash_per", &self.flash_per())
            .field("flash_pes", &self.flash_pes())
            .field("usr", &self.usr())
            .field("flash_hpm", &self.flash_hpm())
            .field("flash_res", &self.flash_res())
            .field("flash_dp", &self.flash_dp())
            .field("flash_ce", &self.flash_ce())
            .field("flash_be", &self.flash_be())
            .field("flash_se", &self.flash_se())
            .field("flash_pp", &self.flash_pp())
            .field("flash_wrsr", &self.flash_wrsr())
            .field("flash_rdsr", &self.flash_rdsr())
            .field("flash_rdid", &self.flash_rdid())
            .field("flash_wrdi", &self.flash_wrdi())
            .field("flash_wren", &self.flash_wren())
            .field("flash_read", &self.flash_read())
            .finish()
    }
}
impl W {
    #[doc = "Bit 16 - program erase resume bit program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_per(&mut self) -> FLASH_PER_W<CMD_SPEC> {
        FLASH_PER_W::new(self, 16)
    }
    #[doc = "Bit 17 - program erase suspend bit program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_pes(&mut self) -> FLASH_PES_W<CMD_SPEC> {
        FLASH_PES_W::new(self, 17)
    }
    #[doc = "Bit 18 - User define command enable. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn usr(&mut self) -> USR_W<CMD_SPEC> {
        USR_W::new(self, 18)
    }
    #[doc = "Bit 19 - Drive Flash into high performance mode. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_hpm(&mut self) -> FLASH_HPM_W<CMD_SPEC> {
        FLASH_HPM_W::new(self, 19)
    }
    #[doc = "Bit 20 - This bit combined with reg_resandres bit releases Flash from the power-down state or high performance mode and obtains the devices ID. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_res(&mut self) -> FLASH_RES_W<CMD_SPEC> {
        FLASH_RES_W::new(self, 20)
    }
    #[doc = "Bit 21 - Drive Flash into power down. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_dp(&mut self) -> FLASH_DP_W<CMD_SPEC> {
        FLASH_DP_W::new(self, 21)
    }
    #[doc = "Bit 22 - Chip erase enable. Chip erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_ce(&mut self) -> FLASH_CE_W<CMD_SPEC> {
        FLASH_CE_W::new(self, 22)
    }
    #[doc = "Bit 23 - Block erase enable. A 64KB block is erased via SPI command D8H. Block erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_be(&mut self) -> FLASH_BE_W<CMD_SPEC> {
        FLASH_BE_W::new(self, 23)
    }
    #[doc = "Bit 24 - Sector erase enable. A 4KB sector is erased via SPI command 20H. Sector erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_se(&mut self) -> FLASH_SE_W<CMD_SPEC> {
        FLASH_SE_W::new(self, 24)
    }
    #[doc = "Bit 25 - Page program enable(1 byte ~256 bytes data to be programmed). Page program operation will be triggered when the bit is set. The bit will be cleared once the operation done .1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_pp(&mut self) -> FLASH_PP_W<CMD_SPEC> {
        FLASH_PP_W::new(self, 25)
    }
    #[doc = "Bit 26 - Write status register enable. Write status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_wrsr(&mut self) -> FLASH_WRSR_W<CMD_SPEC> {
        FLASH_WRSR_W::new(self, 26)
    }
    #[doc = "Bit 27 - Read status register-1. Read status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_rdsr(&mut self) -> FLASH_RDSR_W<CMD_SPEC> {
        FLASH_RDSR_W::new(self, 27)
    }
    #[doc = "Bit 28 - Read JEDEC ID . Read ID command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_rdid(&mut self) -> FLASH_RDID_W<CMD_SPEC> {
        FLASH_RDID_W::new(self, 28)
    }
    #[doc = "Bit 29 - Write flash disable. Write disable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_wrdi(&mut self) -> FLASH_WRDI_W<CMD_SPEC> {
        FLASH_WRDI_W::new(self, 29)
    }
    #[doc = "Bit 30 - Write flash enable. Write enable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_wren(&mut self) -> FLASH_WREN_W<CMD_SPEC> {
        FLASH_WREN_W::new(self, 30)
    }
    #[doc = "Bit 31 - Read flash enable. Read flash operation will be triggered when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_read(&mut self) -> FLASH_READ_W<CMD_SPEC> {
        FLASH_READ_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: u32 = 0;
}
