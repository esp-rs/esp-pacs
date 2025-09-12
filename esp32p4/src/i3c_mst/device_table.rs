#[doc = "Register `DEVICE_TABLE` reader"]
pub type R = crate::R<DEVICE_TABLE_SPEC>;
#[doc = "Register `DEVICE_TABLE` writer"]
pub type W = crate::W<DEVICE_TABLE_SPEC>;
#[doc = "Field `REG_DCT_DAA_INIT_INDEX` reader - Reserved"]
pub type REG_DCT_DAA_INIT_INDEX_R = crate::FieldReader;
#[doc = "Field `REG_DCT_DAA_INIT_INDEX` writer - Reserved"]
pub type REG_DCT_DAA_INIT_INDEX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REG_DAT_DAA_INIT_INDEX` reader - NA"]
pub type REG_DAT_DAA_INIT_INDEX_R = crate::FieldReader;
#[doc = "Field `REG_DAT_DAA_INIT_INDEX` writer - NA"]
pub type REG_DAT_DAA_INIT_INDEX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRESENT_DCT_INDEX` reader - NA"]
pub type PRESENT_DCT_INDEX_R = crate::FieldReader;
#[doc = "Field `PRESENT_DAT_INDEX` reader - NA"]
pub type PRESENT_DAT_INDEX_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Reserved"]
    #[inline(always)]
    pub fn reg_dct_daa_init_index(&self) -> REG_DCT_DAA_INIT_INDEX_R {
        REG_DCT_DAA_INIT_INDEX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - NA"]
    #[inline(always)]
    pub fn reg_dat_daa_init_index(&self) -> REG_DAT_DAA_INIT_INDEX_R {
        REG_DAT_DAA_INIT_INDEX_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - NA"]
    #[inline(always)]
    pub fn present_dct_index(&self) -> PRESENT_DCT_INDEX_R {
        PRESENT_DCT_INDEX_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - NA"]
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
    #[doc = "Bits 0:3 - Reserved"]
    #[inline(always)]
    pub fn reg_dct_daa_init_index(&mut self) -> REG_DCT_DAA_INIT_INDEX_W<'_, DEVICE_TABLE_SPEC> {
        REG_DCT_DAA_INIT_INDEX_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - NA"]
    #[inline(always)]
    pub fn reg_dat_daa_init_index(&mut self) -> REG_DAT_DAA_INIT_INDEX_W<'_, DEVICE_TABLE_SPEC> {
        REG_DAT_DAA_INIT_INDEX_W::new(self, 4)
    }
}
#[doc = "Pointer for Device Address Table\n\nYou can [`read`](crate::Reg::read) this register and get [`device_table::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`device_table::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVICE_TABLE_SPEC;
impl crate::RegisterSpec for DEVICE_TABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`device_table::R`](R) reader structure"]
impl crate::Readable for DEVICE_TABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`device_table::W`](W) writer structure"]
impl crate::Writable for DEVICE_TABLE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEVICE_TABLE to value 0"]
impl crate::Resettable for DEVICE_TABLE_SPEC {}
