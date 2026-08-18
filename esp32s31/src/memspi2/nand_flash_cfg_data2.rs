#[doc = "Register `NAND_FLASH_CFG_DATA2` reader"]
pub type R = crate::R<NAND_FLASH_CFG_DATA2_SPEC>;
#[doc = "Field `CFG_DATA(4-5)` reader - "]
pub type CFG_DATA_R = crate::FieldReader<u16>;
impl R {
    #[doc = ""]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CFG_DATA4` field.</div>"]
    #[inline(always)]
    pub fn cfg_data(&self, n: u8) -> CFG_DATA_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CFG_DATA_R::new(((self.bits >> (n * 16)) & 0xffff) as u16)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn cfg_data_iter(&self) -> impl Iterator<Item = CFG_DATA_R> + '_ {
        (0..2).map(move |n| CFG_DATA_R::new(((self.bits >> (n * 16)) & 0xffff) as u16))
    }
    #[doc = "Bits 0:15 - CFG_DATA4"]
    #[inline(always)]
    pub fn cfg_data4(&self) -> CFG_DATA_R {
        CFG_DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - CFG_DATA5"]
    #[inline(always)]
    pub fn cfg_data5(&self) -> CFG_DATA_R {
        CFG_DATA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NAND_FLASH_CFG_DATA2")
            .field("cfg_data4", &self.cfg_data4())
            .field("cfg_data5", &self.cfg_data5())
            .finish()
    }
}
#[doc = "NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_cfg_data2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NAND_FLASH_CFG_DATA2_SPEC;
impl crate::RegisterSpec for NAND_FLASH_CFG_DATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nand_flash_cfg_data2::R`](R) reader structure"]
impl crate::Readable for NAND_FLASH_CFG_DATA2_SPEC {}
#[doc = "`reset()` method sets NAND_FLASH_CFG_DATA2 to value 0"]
impl crate::Resettable for NAND_FLASH_CFG_DATA2_SPEC {}
