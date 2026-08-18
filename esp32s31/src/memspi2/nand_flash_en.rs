#[doc = "Register `NAND_FLASH_EN` reader"]
pub type R = crate::R<NAND_FLASH_EN_SPEC>;
#[doc = "Field `EN` reader - "]
pub type EN_R = crate::BitReader;
#[doc = "Field `SEQ_HD_INDEX` reader - "]
pub type SEQ_HD_INDEX_R = crate::FieldReader<u16>;
#[doc = "Field `SEQ_USR_TRIG` reader - "]
pub type SEQ_USR_TRIG_R = crate::BitReader;
#[doc = "Field `LUT_EN` reader - "]
pub type LUT_EN_R = crate::BitReader;
#[doc = "Field `SEQ_USR_WEND` reader - "]
pub type SEQ_USR_WEND_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:15"]
    #[inline(always)]
    pub fn seq_hd_index(&self) -> SEQ_HD_INDEX_R {
        SEQ_HD_INDEX_R::new(((self.bits >> 1) & 0x7fff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn seq_usr_trig(&self) -> SEQ_USR_TRIG_R {
        SEQ_USR_TRIG_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn lut_en(&self) -> LUT_EN_R {
        LUT_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn seq_usr_wend(&self) -> SEQ_USR_WEND_R {
        SEQ_USR_WEND_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NAND_FLASH_EN")
            .field("en", &self.en())
            .field("seq_hd_index", &self.seq_hd_index())
            .field("seq_usr_trig", &self.seq_usr_trig())
            .field("lut_en", &self.lut_en())
            .field("seq_usr_wend", &self.seq_usr_wend())
            .finish()
    }
}
#[doc = "NAND FLASH control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_en::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NAND_FLASH_EN_SPEC;
impl crate::RegisterSpec for NAND_FLASH_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nand_flash_en::R`](R) reader structure"]
impl crate::Readable for NAND_FLASH_EN_SPEC {}
#[doc = "`reset()` method sets NAND_FLASH_EN to value 0xfffe"]
impl crate::Resettable for NAND_FLASH_EN_SPEC {
    const RESET_VALUE: u32 = 0xfffe;
}
