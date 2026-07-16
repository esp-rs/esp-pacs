#[doc = "Register `FLASH_SUS_CTRL` reader"]
pub type R = crate::R<FLASH_SUS_CTRL_SPEC>;
#[doc = "Register `FLASH_SUS_CTRL` writer"]
pub type W = crate::W<FLASH_SUS_CTRL_SPEC>;
#[doc = "Field `FLASH_PER` reader - "]
pub type FLASH_PER_R = crate::BitReader;
#[doc = "Field `FLASH_PER` writer - "]
pub type FLASH_PER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_PES` reader - "]
pub type FLASH_PES_R = crate::BitReader;
#[doc = "Field `FLASH_PES` writer - "]
pub type FLASH_PES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_PER_WAIT_EN` reader - "]
pub type FLASH_PER_WAIT_EN_R = crate::BitReader;
#[doc = "Field `FLASH_PER_WAIT_EN` writer - "]
pub type FLASH_PER_WAIT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_PES_WAIT_EN` reader - "]
pub type FLASH_PES_WAIT_EN_R = crate::BitReader;
#[doc = "Field `FLASH_PES_WAIT_EN` writer - "]
pub type FLASH_PES_WAIT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PES_PER_EN` reader - "]
pub type PES_PER_EN_R = crate::BitReader;
#[doc = "Field `PES_PER_EN` writer - "]
pub type PES_PER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_PES_EN` reader - "]
pub type FLASH_PES_EN_R = crate::BitReader;
#[doc = "Field `FLASH_PES_EN` writer - "]
pub type FLASH_PES_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PESR_END_MSK` reader - "]
pub type PESR_END_MSK_R = crate::FieldReader<u16>;
#[doc = "Field `PESR_END_MSK` writer - "]
pub type PESR_END_MSK_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `FMEM_RD_SUS_2B` reader - "]
pub type FMEM_RD_SUS_2B_R = crate::BitReader;
#[doc = "Field `FMEM_RD_SUS_2B` writer - "]
pub type FMEM_RD_SUS_2B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PER_END_EN` reader - "]
pub type PER_END_EN_R = crate::BitReader;
#[doc = "Field `PER_END_EN` writer - "]
pub type PER_END_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PES_END_EN` reader - "]
pub type PES_END_EN_R = crate::BitReader;
#[doc = "Field `PES_END_EN` writer - "]
pub type PES_END_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUS_TIMEOUT_CNT` reader - "]
pub type SUS_TIMEOUT_CNT_R = crate::FieldReader;
#[doc = "Field `SUS_TIMEOUT_CNT` writer - "]
pub type SUS_TIMEOUT_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn flash_per(&self) -> FLASH_PER_R {
        FLASH_PER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn flash_pes(&self) -> FLASH_PES_R {
        FLASH_PES_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn flash_per_wait_en(&self) -> FLASH_PER_WAIT_EN_R {
        FLASH_PER_WAIT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn flash_pes_wait_en(&self) -> FLASH_PES_WAIT_EN_R {
        FLASH_PES_WAIT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pes_per_en(&self) -> PES_PER_EN_R {
        PES_PER_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn flash_pes_en(&self) -> FLASH_PES_EN_R {
        FLASH_PES_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:21"]
    #[inline(always)]
    pub fn pesr_end_msk(&self) -> PESR_END_MSK_R {
        PESR_END_MSK_R::new(((self.bits >> 6) & 0xffff) as u16)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn fmem_rd_sus_2b(&self) -> FMEM_RD_SUS_2B_R {
        FMEM_RD_SUS_2B_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn per_end_en(&self) -> PER_END_EN_R {
        PER_END_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pes_end_en(&self) -> PES_END_EN_R {
        PES_END_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn sus_timeout_cnt(&self) -> SUS_TIMEOUT_CNT_R {
        SUS_TIMEOUT_CNT_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_SUS_CTRL")
            .field("flash_per", &self.flash_per())
            .field("flash_pes", &self.flash_pes())
            .field("flash_per_wait_en", &self.flash_per_wait_en())
            .field("flash_pes_wait_en", &self.flash_pes_wait_en())
            .field("pes_per_en", &self.pes_per_en())
            .field("flash_pes_en", &self.flash_pes_en())
            .field("pesr_end_msk", &self.pesr_end_msk())
            .field("fmem_rd_sus_2b", &self.fmem_rd_sus_2b())
            .field("per_end_en", &self.per_end_en())
            .field("pes_end_en", &self.pes_end_en())
            .field("sus_timeout_cnt", &self.sus_timeout_cnt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn flash_per(&mut self) -> FLASH_PER_W<'_, FLASH_SUS_CTRL_SPEC> {
        FLASH_PER_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn flash_pes(&mut self) -> FLASH_PES_W<'_, FLASH_SUS_CTRL_SPEC> {
        FLASH_PES_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn flash_per_wait_en(&mut self) -> FLASH_PER_WAIT_EN_W<'_, FLASH_SUS_CTRL_SPEC> {
        FLASH_PER_WAIT_EN_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn flash_pes_wait_en(&mut self) -> FLASH_PES_WAIT_EN_W<'_, FLASH_SUS_CTRL_SPEC> {
        FLASH_PES_WAIT_EN_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pes_per_en(&mut self) -> PES_PER_EN_W<'_, FLASH_SUS_CTRL_SPEC> {
        PES_PER_EN_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn flash_pes_en(&mut self) -> FLASH_PES_EN_W<'_, FLASH_SUS_CTRL_SPEC> {
        FLASH_PES_EN_W::new(self, 5)
    }
    #[doc = "Bits 6:21"]
    #[inline(always)]
    pub fn pesr_end_msk(&mut self) -> PESR_END_MSK_W<'_, FLASH_SUS_CTRL_SPEC> {
        PESR_END_MSK_W::new(self, 6)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn fmem_rd_sus_2b(&mut self) -> FMEM_RD_SUS_2B_W<'_, FLASH_SUS_CTRL_SPEC> {
        FMEM_RD_SUS_2B_W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn per_end_en(&mut self) -> PER_END_EN_W<'_, FLASH_SUS_CTRL_SPEC> {
        PER_END_EN_W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pes_end_en(&mut self) -> PES_END_EN_W<'_, FLASH_SUS_CTRL_SPEC> {
        PES_END_EN_W::new(self, 24)
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn sus_timeout_cnt(&mut self) -> SUS_TIMEOUT_CNT_W<'_, FLASH_SUS_CTRL_SPEC> {
        SUS_TIMEOUT_CNT_W::new(self, 25)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_sus_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_sus_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_SUS_CTRL_SPEC;
impl crate::RegisterSpec for FLASH_SUS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_sus_ctrl::R`](R) reader structure"]
impl crate::Readable for FLASH_SUS_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flash_sus_ctrl::W`](W) writer structure"]
impl crate::Writable for FLASH_SUS_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASH_SUS_CTRL to value 0"]
impl crate::Resettable for FLASH_SUS_CTRL_SPEC {}
