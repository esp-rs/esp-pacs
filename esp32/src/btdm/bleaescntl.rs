#[doc = "Register `BLEAESCNTL` reader"]
pub type R = crate::R<BLEAESCNTL_SPEC>;
#[doc = "Register `BLEAESCNTL` writer"]
pub type W = crate::W<BLEAESCNTL_SPEC>;
#[doc = "Field `AES_START` reader - Start AES encryption/decryption"]
pub type AES_START_R = crate::BitReader;
#[doc = "Field `AES_START` writer - Start AES encryption/decryption"]
pub type AES_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES_MODE` reader - AES mode (0=encrypt, 1=decrypt)"]
pub type AES_MODE_R = crate::BitReader;
#[doc = "Field `AES_MODE` writer - AES mode (0=encrypt, 1=decrypt)"]
pub type AES_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start AES encryption/decryption"]
    #[inline(always)]
    pub fn aes_start(&self) -> AES_START_R {
        AES_START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AES mode (0=encrypt, 1=decrypt)"]
    #[inline(always)]
    pub fn aes_mode(&self) -> AES_MODE_R {
        AES_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLEAESCNTL")
            .field("aes_start", &self.aes_start())
            .field("aes_mode", &self.aes_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Start AES encryption/decryption"]
    #[inline(always)]
    pub fn aes_start(&mut self) -> AES_START_W<'_, BLEAESCNTL_SPEC> {
        AES_START_W::new(self, 0)
    }
    #[doc = "Bit 1 - AES mode (0=encrypt, 1=decrypt)"]
    #[inline(always)]
    pub fn aes_mode(&mut self) -> AES_MODE_W<'_, BLEAESCNTL_SPEC> {
        AES_MODE_W::new(self, 1)
    }
}
#[doc = "AES encryption control register\n\nYou can [`read`](crate::Reg::read) this register and get [`bleaescntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleaescntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLEAESCNTL_SPEC;
impl crate::RegisterSpec for BLEAESCNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bleaescntl::R`](R) reader structure"]
impl crate::Readable for BLEAESCNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bleaescntl::W`](W) writer structure"]
impl crate::Writable for BLEAESCNTL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLEAESCNTL to value 0"]
impl crate::Resettable for BLEAESCNTL_SPEC {}
