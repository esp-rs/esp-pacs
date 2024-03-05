#[doc = "Register `CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON` reader"]
pub type R = crate::R<CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON_SPEC>;
#[doc = "Register `CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON` writer"]
pub type W = crate::W<CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON_SPEC>;
#[doc = "Field `CLK_FORCE_ON_MANUAL_CRYPT` reader - The bit is used to close clock gating of manual crypt clock. 1: close gating, 0: open clock gating."]
pub type CLK_FORCE_ON_MANUAL_CRYPT_R = crate::BitReader;
#[doc = "Field `CLK_FORCE_ON_MANUAL_CRYPT` writer - The bit is used to close clock gating of manual crypt clock. 1: close gating, 0: open clock gating."]
pub type CLK_FORCE_ON_MANUAL_CRYPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_FORCE_ON_AUTO_CRYPT` reader - The bit is used to close clock gating of automatic crypt clock. 1: close gating, 0: open clock gating."]
pub type CLK_FORCE_ON_AUTO_CRYPT_R = crate::BitReader;
#[doc = "Field `CLK_FORCE_ON_AUTO_CRYPT` writer - The bit is used to close clock gating of automatic crypt clock. 1: close gating, 0: open clock gating."]
pub type CLK_FORCE_ON_AUTO_CRYPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_FORCE_ON_CRYPT` reader - The bit is used to close clock gating of external memory encrypt and decrypt clock. 1: close gating, 0: open clock gating."]
pub type CLK_FORCE_ON_CRYPT_R = crate::BitReader;
#[doc = "Field `CLK_FORCE_ON_CRYPT` writer - The bit is used to close clock gating of external memory encrypt and decrypt clock. 1: close gating, 0: open clock gating."]
pub type CLK_FORCE_ON_CRYPT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bit is used to close clock gating of manual crypt clock. 1: close gating, 0: open clock gating."]
    #[inline(always)]
    pub fn clk_force_on_manual_crypt(&self) -> CLK_FORCE_ON_MANUAL_CRYPT_R {
        CLK_FORCE_ON_MANUAL_CRYPT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to close clock gating of automatic crypt clock. 1: close gating, 0: open clock gating."]
    #[inline(always)]
    pub fn clk_force_on_auto_crypt(&self) -> CLK_FORCE_ON_AUTO_CRYPT_R {
        CLK_FORCE_ON_AUTO_CRYPT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to close clock gating of external memory encrypt and decrypt clock. 1: close gating, 0: open clock gating."]
    #[inline(always)]
    pub fn clk_force_on_crypt(&self) -> CLK_FORCE_ON_CRYPT_R {
        CLK_FORCE_ON_CRYPT_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON")
            .field(
                "clk_force_on_manual_crypt",
                &format_args!("{}", self.clk_force_on_manual_crypt().bit()),
            )
            .field(
                "clk_force_on_auto_crypt",
                &format_args!("{}", self.clk_force_on_auto_crypt().bit()),
            )
            .field(
                "clk_force_on_crypt",
                &format_args!("{}", self.clk_force_on_crypt().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to close clock gating of manual crypt clock. 1: close gating, 0: open clock gating."]
    #[inline(always)]
    #[must_use]
    pub fn clk_force_on_manual_crypt(
        &mut self,
    ) -> CLK_FORCE_ON_MANUAL_CRYPT_W<CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON_SPEC> {
        CLK_FORCE_ON_MANUAL_CRYPT_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to close clock gating of automatic crypt clock. 1: close gating, 0: open clock gating."]
    #[inline(always)]
    #[must_use]
    pub fn clk_force_on_auto_crypt(
        &mut self,
    ) -> CLK_FORCE_ON_AUTO_CRYPT_W<CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON_SPEC> {
        CLK_FORCE_ON_AUTO_CRYPT_W::new(self, 1)
    }
    #[doc = "Bit 2 - The bit is used to close clock gating of external memory encrypt and decrypt clock. 1: close gating, 0: open clock gating."]
    #[inline(always)]
    #[must_use]
    pub fn clk_force_on_crypt(
        &mut self,
    ) -> CLK_FORCE_ON_CRYPT_W<CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON_SPEC> {
        CLK_FORCE_ON_CRYPT_W::new(self, 2)
    }
}
#[doc = "This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_encrypt_decrypt_clk_force_on::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_encrypt_decrypt_clk_force_on::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON_SPEC;
impl crate::RegisterSpec for CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_encrypt_decrypt_clk_force_on::R`](R) reader structure"]
impl crate::Readable for CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_encrypt_decrypt_clk_force_on::W`](W) writer structure"]
impl crate::Writable for CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON to value 0x07"]
impl crate::Resettable for CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON_SPEC {
    const RESET_VALUE: u32 = 0x07;
}
