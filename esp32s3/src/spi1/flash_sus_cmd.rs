#[doc = "Register `FLASH_SUS_CMD` reader"]
pub type R = crate::R<FLASH_SUS_CMD_SPEC>;
#[doc = "Register `FLASH_SUS_CMD` writer"]
pub type W = crate::W<FLASH_SUS_CMD_SPEC>;
#[doc = "Field `FLASH_PER` reader - program erase resume bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_PER_R = crate::BitReader;
#[doc = "Field `FLASH_PER` writer - program erase resume bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_PER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_PES` reader - program erase suspend bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_PES_R = crate::BitReader;
#[doc = "Field `FLASH_PES` writer - program erase suspend bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FLASH_PES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_PER_WAIT_EN` reader - Set this bit to add delay time after program erase resume(PER) is sent."]
pub type FLASH_PER_WAIT_EN_R = crate::BitReader;
#[doc = "Field `FLASH_PER_WAIT_EN` writer - Set this bit to add delay time after program erase resume(PER) is sent."]
pub type FLASH_PER_WAIT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_PES_WAIT_EN` reader - Set this bit to add delay time after program erase suspend(PES) command is sent."]
pub type FLASH_PES_WAIT_EN_R = crate::BitReader;
#[doc = "Field `FLASH_PES_WAIT_EN` writer - Set this bit to add delay time after program erase suspend(PES) command is sent."]
pub type FLASH_PES_WAIT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PES_PER_EN` reader - Set this bit to enable PES transfer trigger PES transfer option."]
pub type PES_PER_EN_R = crate::BitReader;
#[doc = "Field `PES_PER_EN` writer - Set this bit to enable PES transfer trigger PES transfer option."]
pub type PES_PER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PESR_IDLE_EN` reader - 1: Separate PER flash wait idle and PES flash wait idle. 0: Not separate."]
pub type PESR_IDLE_EN_R = crate::BitReader;
#[doc = "Field `PESR_IDLE_EN` writer - 1: Separate PER flash wait idle and PES flash wait idle. 0: Not separate."]
pub type PESR_IDLE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - program erase resume bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_per(&self) -> FLASH_PER_R {
        FLASH_PER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - program erase suspend bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_pes(&self) -> FLASH_PES_R {
        FLASH_PES_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to add delay time after program erase resume(PER) is sent."]
    #[inline(always)]
    pub fn flash_per_wait_en(&self) -> FLASH_PER_WAIT_EN_R {
        FLASH_PER_WAIT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to add delay time after program erase suspend(PES) command is sent."]
    #[inline(always)]
    pub fn flash_pes_wait_en(&self) -> FLASH_PES_WAIT_EN_R {
        FLASH_PES_WAIT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to enable PES transfer trigger PES transfer option."]
    #[inline(always)]
    pub fn pes_per_en(&self) -> PES_PER_EN_R {
        PES_PER_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: Separate PER flash wait idle and PES flash wait idle. 0: Not separate."]
    #[inline(always)]
    pub fn pesr_idle_en(&self) -> PESR_IDLE_EN_R {
        PESR_IDLE_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_SUS_CMD")
            .field("flash_per", &self.flash_per())
            .field("flash_pes", &self.flash_pes())
            .field("flash_per_wait_en", &self.flash_per_wait_en())
            .field("flash_pes_wait_en", &self.flash_pes_wait_en())
            .field("pes_per_en", &self.pes_per_en())
            .field("pesr_idle_en", &self.pesr_idle_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - program erase resume bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn flash_per(&mut self) -> FLASH_PER_W<FLASH_SUS_CMD_SPEC> {
        FLASH_PER_W::new(self, 0)
    }
    #[doc = "Bit 1 - program erase suspend bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn flash_pes(&mut self) -> FLASH_PES_W<FLASH_SUS_CMD_SPEC> {
        FLASH_PES_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to add delay time after program erase resume(PER) is sent."]
    #[inline(always)]
    #[must_use]
    pub fn flash_per_wait_en(&mut self) -> FLASH_PER_WAIT_EN_W<FLASH_SUS_CMD_SPEC> {
        FLASH_PER_WAIT_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to add delay time after program erase suspend(PES) command is sent."]
    #[inline(always)]
    #[must_use]
    pub fn flash_pes_wait_en(&mut self) -> FLASH_PES_WAIT_EN_W<FLASH_SUS_CMD_SPEC> {
        FLASH_PES_WAIT_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to enable PES transfer trigger PES transfer option."]
    #[inline(always)]
    #[must_use]
    pub fn pes_per_en(&mut self) -> PES_PER_EN_W<FLASH_SUS_CMD_SPEC> {
        PES_PER_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - 1: Separate PER flash wait idle and PES flash wait idle. 0: Not separate."]
    #[inline(always)]
    #[must_use]
    pub fn pesr_idle_en(&mut self) -> PESR_IDLE_EN_W<FLASH_SUS_CMD_SPEC> {
        PESR_IDLE_EN_W::new(self, 5)
    }
}
#[doc = "SPI1 flash suspend control register\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_sus_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_sus_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_SUS_CMD_SPEC;
impl crate::RegisterSpec for FLASH_SUS_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_sus_cmd::R`](R) reader structure"]
impl crate::Readable for FLASH_SUS_CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flash_sus_cmd::W`](W) writer structure"]
impl crate::Writable for FLASH_SUS_CMD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_SUS_CMD to value 0"]
impl crate::Resettable for FLASH_SUS_CMD_SPEC {
    const RESET_VALUE: u32 = 0;
}
