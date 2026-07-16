#[doc = "Register `NAND_FLASH_EN` reader"]
pub type R = crate::R<NAND_FLASH_EN_SPEC>;
#[doc = "Register `NAND_FLASH_EN` writer"]
pub type W = crate::W<NAND_FLASH_EN_SPEC>;
#[doc = "Field `NAND_FLASH_EN` reader - "]
pub type NAND_FLASH_EN_R = crate::BitReader;
#[doc = "Field `NAND_FLASH_EN` writer - "]
pub type NAND_FLASH_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAND_FLASH_SEQ_HD_INDEX` reader - "]
pub type NAND_FLASH_SEQ_HD_INDEX_R = crate::FieldReader<u16>;
#[doc = "Field `NAND_FLASH_SEQ_HD_INDEX` writer - "]
pub type NAND_FLASH_SEQ_HD_INDEX_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `NAND_FLASH_SEQ_USR_TRIG` reader - "]
pub type NAND_FLASH_SEQ_USR_TRIG_R = crate::BitReader;
#[doc = "Field `NAND_FLASH_SEQ_USR_TRIG` writer - "]
pub type NAND_FLASH_SEQ_USR_TRIG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAND_FLASH_LUT_EN` reader - "]
pub type NAND_FLASH_LUT_EN_R = crate::BitReader;
#[doc = "Field `NAND_FLASH_LUT_EN` writer - "]
pub type NAND_FLASH_LUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAND_FLASH_SEQ_USR_WEND` reader - "]
pub type NAND_FLASH_SEQ_USR_WEND_R = crate::BitReader;
#[doc = "Field `NAND_FLASH_SEQ_USR_WEND` writer - "]
pub type NAND_FLASH_SEQ_USR_WEND_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn nand_flash_en(&self) -> NAND_FLASH_EN_R {
        NAND_FLASH_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:15"]
    #[inline(always)]
    pub fn nand_flash_seq_hd_index(&self) -> NAND_FLASH_SEQ_HD_INDEX_R {
        NAND_FLASH_SEQ_HD_INDEX_R::new(((self.bits >> 1) & 0x7fff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn nand_flash_seq_usr_trig(&self) -> NAND_FLASH_SEQ_USR_TRIG_R {
        NAND_FLASH_SEQ_USR_TRIG_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn nand_flash_lut_en(&self) -> NAND_FLASH_LUT_EN_R {
        NAND_FLASH_LUT_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn nand_flash_seq_usr_wend(&self) -> NAND_FLASH_SEQ_USR_WEND_R {
        NAND_FLASH_SEQ_USR_WEND_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NAND_FLASH_EN")
            .field("nand_flash_en", &self.nand_flash_en())
            .field("nand_flash_seq_hd_index", &self.nand_flash_seq_hd_index())
            .field("nand_flash_seq_usr_trig", &self.nand_flash_seq_usr_trig())
            .field("nand_flash_lut_en", &self.nand_flash_lut_en())
            .field("nand_flash_seq_usr_wend", &self.nand_flash_seq_usr_wend())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn nand_flash_en(&mut self) -> NAND_FLASH_EN_W<'_, NAND_FLASH_EN_SPEC> {
        NAND_FLASH_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:15"]
    #[inline(always)]
    pub fn nand_flash_seq_hd_index(&mut self) -> NAND_FLASH_SEQ_HD_INDEX_W<'_, NAND_FLASH_EN_SPEC> {
        NAND_FLASH_SEQ_HD_INDEX_W::new(self, 1)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn nand_flash_seq_usr_trig(&mut self) -> NAND_FLASH_SEQ_USR_TRIG_W<'_, NAND_FLASH_EN_SPEC> {
        NAND_FLASH_SEQ_USR_TRIG_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn nand_flash_lut_en(&mut self) -> NAND_FLASH_LUT_EN_W<'_, NAND_FLASH_EN_SPEC> {
        NAND_FLASH_LUT_EN_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn nand_flash_seq_usr_wend(&mut self) -> NAND_FLASH_SEQ_USR_WEND_W<'_, NAND_FLASH_EN_SPEC> {
        NAND_FLASH_SEQ_USR_WEND_W::new(self, 18)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nand_flash_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NAND_FLASH_EN_SPEC;
impl crate::RegisterSpec for NAND_FLASH_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nand_flash_en::R`](R) reader structure"]
impl crate::Readable for NAND_FLASH_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nand_flash_en::W`](W) writer structure"]
impl crate::Writable for NAND_FLASH_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NAND_FLASH_EN to value 0xfffe"]
impl crate::Resettable for NAND_FLASH_EN_SPEC {
    const RESET_VALUE: u32 = 0xfffe;
}
