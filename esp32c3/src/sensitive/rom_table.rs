#[doc = "Register `ROM_TABLE` reader"]
pub type R = crate::R<ROM_TABLE_SPEC>;
#[doc = "Register `ROM_TABLE` writer"]
pub type W = crate::W<ROM_TABLE_SPEC>;
#[doc = "Field `ROM_TABLE` reader - rom_table"]
pub type ROM_TABLE_R = crate::FieldReader<u32>;
#[doc = "Field `ROM_TABLE` writer - rom_table"]
pub type ROM_TABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - rom_table"]
    #[inline(always)]
    pub fn rom_table(&self) -> ROM_TABLE_R {
        ROM_TABLE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROM_TABLE")
            .field("rom_table", &format_args!("{}", self.rom_table().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ROM_TABLE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - rom_table"]
    #[inline(always)]
    #[must_use]
    pub fn rom_table(&mut self) -> ROM_TABLE_W<ROM_TABLE_SPEC> {
        ROM_TABLE_W::new(self, 0)
    }
}
#[doc = "SENSITIVE_ROM_TABLE_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rom_table::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rom_table::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROM_TABLE_SPEC;
impl crate::RegisterSpec for ROM_TABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rom_table::R`](R) reader structure"]
impl crate::Readable for ROM_TABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rom_table::W`](W) writer structure"]
impl crate::Writable for ROM_TABLE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROM_TABLE to value 0"]
impl crate::Resettable for ROM_TABLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
