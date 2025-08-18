#[doc = "Register `INTERFACE_CRYPTO_CONTROL%s` reader"]
pub type R = crate::R<INTERFACE_CRYPTO_CONTROL_SPEC>;
#[doc = "Register `INTERFACE_CRYPTO_CONTROL%s` writer"]
pub type W = crate::W<INTERFACE_CRYPTO_CONTROL_SPEC>;
#[doc = "Field `SPP_ENABLE` reader - Enable Signaling and Payload Protection (SPP) for A-MSDUs"]
pub type SPP_ENABLE_R = crate::BitReader;
#[doc = "Field `SPP_ENABLE` writer - Enable Signaling and Payload Protection (SPP) for A-MSDUs"]
pub type SPP_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMF_DISABLE` reader - Disable Protected Management Frames (PMF)"]
pub type PMF_DISABLE_R = crate::BitReader;
#[doc = "Field `PMF_DISABLE` writer - Disable Protected Management Frames (PMF)"]
pub type PMF_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AEAD_CIPHER` reader - Does the cipher employ Authenticated Encryption with Associated Data (AEAD)"]
pub type AEAD_CIPHER_R = crate::BitReader;
#[doc = "Field `AEAD_CIPHER` writer - Does the cipher employ Authenticated Encryption with Associated Data (AEAD)"]
pub type AEAD_CIPHER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMS4` reader - Cipher is SMS4"]
pub type SMS4_R = crate::BitReader;
#[doc = "Field `SMS4` writer - Cipher is SMS4"]
pub type SMS4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 9 - Enable Signaling and Payload Protection (SPP) for A-MSDUs"]
    #[inline(always)]
    pub fn spp_enable(&self) -> SPP_ENABLE_R {
        SPP_ENABLE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 17 - Disable Protected Management Frames (PMF)"]
    #[inline(always)]
    pub fn pmf_disable(&self) -> PMF_DISABLE_R {
        PMF_DISABLE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 28 - Does the cipher employ Authenticated Encryption with Associated Data (AEAD)"]
    #[inline(always)]
    pub fn aead_cipher(&self) -> AEAD_CIPHER_R {
        AEAD_CIPHER_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - Cipher is SMS4"]
    #[inline(always)]
    pub fn sms4(&self) -> SMS4_R {
        SMS4_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTERFACE_CRYPTO_CONTROL")
            .field("spp_enable", &self.spp_enable())
            .field("pmf_disable", &self.pmf_disable())
            .field("aead_cipher", &self.aead_cipher())
            .field("sms4", &self.sms4())
            .finish()
    }
}
impl W {
    #[doc = "Bit 9 - Enable Signaling and Payload Protection (SPP) for A-MSDUs"]
    #[inline(always)]
    pub fn spp_enable(&mut self) -> SPP_ENABLE_W<'_, INTERFACE_CRYPTO_CONTROL_SPEC> {
        SPP_ENABLE_W::new(self, 9)
    }
    #[doc = "Bit 17 - Disable Protected Management Frames (PMF)"]
    #[inline(always)]
    pub fn pmf_disable(&mut self) -> PMF_DISABLE_W<'_, INTERFACE_CRYPTO_CONTROL_SPEC> {
        PMF_DISABLE_W::new(self, 17)
    }
    #[doc = "Bit 28 - Does the cipher employ Authenticated Encryption with Associated Data (AEAD)"]
    #[inline(always)]
    pub fn aead_cipher(&mut self) -> AEAD_CIPHER_W<'_, INTERFACE_CRYPTO_CONTROL_SPEC> {
        AEAD_CIPHER_W::new(self, 28)
    }
    #[doc = "Bit 31 - Cipher is SMS4"]
    #[inline(always)]
    pub fn sms4(&mut self) -> SMS4_W<'_, INTERFACE_CRYPTO_CONTROL_SPEC> {
        SMS4_W::new(self, 31)
    }
}
#[doc = "Control over cryptographic parameters for a specific interface\n\nYou can [`read`](crate::Reg::read) this register and get [`interface_crypto_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interface_crypto_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERFACE_CRYPTO_CONTROL_SPEC;
impl crate::RegisterSpec for INTERFACE_CRYPTO_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interface_crypto_control::R`](R) reader structure"]
impl crate::Readable for INTERFACE_CRYPTO_CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`interface_crypto_control::W`](W) writer structure"]
impl crate::Writable for INTERFACE_CRYPTO_CONTROL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTERFACE_CRYPTO_CONTROL%s to value 0"]
impl crate::Resettable for INTERFACE_CRYPTO_CONTROL_SPEC {}
