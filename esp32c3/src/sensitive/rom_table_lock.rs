#[doc = "Register `ROM_TABLE_LOCK` reader"]
pub type R = crate::R<ROM_TABLE_LOCK_SPEC>;
#[doc = "Register `ROM_TABLE_LOCK` writer"]
pub type W = crate::W<ROM_TABLE_LOCK_SPEC>;
#[doc = "Field `ROM_TABLE_LOCK` reader - rom_table_lock"]
pub type ROM_TABLE_LOCK_R = crate::BitReader;
#[doc = "Field `ROM_TABLE_LOCK` writer - rom_table_lock"]
pub type ROM_TABLE_LOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - rom_table_lock"]
    #[inline(always)]
    pub fn rom_table_lock(&self) -> ROM_TABLE_LOCK_R {
        ROM_TABLE_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROM_TABLE_LOCK")
            .field(
                "rom_table_lock",
                &format_args!("{}", self.rom_table_lock().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ROM_TABLE_LOCK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - rom_table_lock"]
    #[inline(always)]
    #[must_use]
    pub fn rom_table_lock(&mut self) -> ROM_TABLE_LOCK_W<ROM_TABLE_LOCK_SPEC, 0> {
        ROM_TABLE_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SENSITIVE_ROM_TABLE_LOCK_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rom_table_lock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rom_table_lock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROM_TABLE_LOCK_SPEC;
impl crate::RegisterSpec for ROM_TABLE_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rom_table_lock::R`](R) reader structure"]
impl crate::Readable for ROM_TABLE_LOCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rom_table_lock::W`](W) writer structure"]
impl crate::Writable for ROM_TABLE_LOCK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ROM_TABLE_LOCK to value 0"]
impl crate::Resettable for ROM_TABLE_LOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
