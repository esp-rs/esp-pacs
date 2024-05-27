///Register `DEVICE_TABLE` reader
pub type R = crate::R<DEVICE_TABLE_SPEC>;
///Register `DEVICE_TABLE` writer
pub type W = crate::W<DEVICE_TABLE_SPEC>;
///Field `REG_DCT_DAA_INIT_INDEX` reader - Reserved
pub type REG_DCT_DAA_INIT_INDEX_R = crate::FieldReader;
///Field `REG_DCT_DAA_INIT_INDEX` writer - Reserved
pub type REG_DCT_DAA_INIT_INDEX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `REG_DAT_DAA_INIT_INDEX` reader - NA
pub type REG_DAT_DAA_INIT_INDEX_R = crate::FieldReader;
///Field `REG_DAT_DAA_INIT_INDEX` writer - NA
pub type REG_DAT_DAA_INIT_INDEX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PRESENT_DCT_INDEX` reader - NA
pub type PRESENT_DCT_INDEX_R = crate::FieldReader;
///Field `PRESENT_DAT_INDEX` reader - NA
pub type PRESENT_DAT_INDEX_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - Reserved
    #[inline(always)]
    pub fn reg_dct_daa_init_index(&self) -> REG_DCT_DAA_INIT_INDEX_R {
        REG_DCT_DAA_INIT_INDEX_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - NA
    #[inline(always)]
    pub fn reg_dat_daa_init_index(&self) -> REG_DAT_DAA_INIT_INDEX_R {
        REG_DAT_DAA_INIT_INDEX_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - NA
    #[inline(always)]
    pub fn present_dct_index(&self) -> PRESENT_DCT_INDEX_R {
        PRESENT_DCT_INDEX_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - NA
    #[inline(always)]
    pub fn present_dat_index(&self) -> PRESENT_DAT_INDEX_R {
        PRESENT_DAT_INDEX_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVICE_TABLE")
            .field("reg_dct_daa_init_index", &self.reg_dct_daa_init_index())
            .field("reg_dat_daa_init_index", &self.reg_dat_daa_init_index())
            .field("present_dct_index", &self.present_dct_index())
            .field("present_dat_index", &self.present_dat_index())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn reg_dct_daa_init_index(&mut self) -> REG_DCT_DAA_INIT_INDEX_W<DEVICE_TABLE_SPEC> {
        REG_DCT_DAA_INIT_INDEX_W::new(self, 0)
    }
    ///Bits 4:7 - NA
    #[inline(always)]
    #[must_use]
    pub fn reg_dat_daa_init_index(&mut self) -> REG_DAT_DAA_INIT_INDEX_W<DEVICE_TABLE_SPEC> {
        REG_DAT_DAA_INIT_INDEX_W::new(self, 4)
    }
}
/**Pointer for Device Address Table

You can [`read`](crate::generic::Reg::read) this register and get [`device_table::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`device_table::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DEVICE_TABLE_SPEC;
impl crate::RegisterSpec for DEVICE_TABLE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`device_table::R`](R) reader structure
impl crate::Readable for DEVICE_TABLE_SPEC {}
///`write(|w| ..)` method takes [`device_table::W`](W) writer structure
impl crate::Writable for DEVICE_TABLE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DEVICE_TABLE to value 0
impl crate::Resettable for DEVICE_TABLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
