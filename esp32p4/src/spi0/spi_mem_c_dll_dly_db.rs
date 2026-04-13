#[doc = "Register `SPI_MEM_C_DLL_DLY_DB` reader"]
pub type R = crate::R<SPI_MEM_C_DLL_DLY_DB_SPEC>;
#[doc = "Register `SPI_MEM_C_DLL_DLY_DB` writer"]
pub type W = crate::W<SPI_MEM_C_DLL_DLY_DB_SPEC>;
#[doc = "Field `VAL` reader - "]
pub type VAL_R = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - "]
pub type VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_C_DLL_DLY_DB")
            .field("val", &self.val())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&mut self) -> VAL_W<'_, SPI_MEM_C_DLL_DLY_DB_SPEC> {
        VAL_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_mem_c_dll_dly_db::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_mem_c_dll_dly_db::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_C_DLL_DLY_DB_SPEC;
impl crate::RegisterSpec for SPI_MEM_C_DLL_DLY_DB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_c_dll_dly_db::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_C_DLL_DLY_DB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_c_dll_dly_db::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_C_DLL_DLY_DB_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_MEM_C_DLL_DLY_DB to value 0"]
impl crate::Resettable for SPI_MEM_C_DLL_DLY_DB_SPEC {}
