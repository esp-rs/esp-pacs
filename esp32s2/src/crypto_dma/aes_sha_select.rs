#[doc = "Register `AES_SHA_SELECT` reader"]
pub type R = crate::R<AES_SHA_SELECT_SPEC>;
#[doc = "Register `AES_SHA_SELECT` writer"]
pub type W = crate::W<AES_SHA_SELECT_SPEC>;
#[doc = "Select one between AES and SHA to use DMA. 0: AES. 1: SHA.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SELECT {
    #[doc = "0: The AES peripheral uses the Crypto DMA"]
    Aes = 0,
    #[doc = "1: The SHA peripheral uses the Crypto DMA"]
    Sha = 1,
}
impl From<SELECT> for bool {
    #[inline(always)]
    fn from(variant: SELECT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELECT` reader - Select one between AES and SHA to use DMA. 0: AES. 1: SHA."]
pub type SELECT_R = crate::BitReader<SELECT>;
impl SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SELECT {
        match self.bits {
            false => SELECT::Aes,
            true => SELECT::Sha,
        }
    }
    #[doc = "The AES peripheral uses the Crypto DMA"]
    #[inline(always)]
    pub fn is_aes(&self) -> bool {
        *self == SELECT::Aes
    }
    #[doc = "The SHA peripheral uses the Crypto DMA"]
    #[inline(always)]
    pub fn is_sha(&self) -> bool {
        *self == SELECT::Sha
    }
}
#[doc = "Field `SELECT` writer - Select one between AES and SHA to use DMA. 0: AES. 1: SHA."]
pub type SELECT_W<'a, REG> = crate::BitWriter<'a, REG, SELECT>;
impl<'a, REG> SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The AES peripheral uses the Crypto DMA"]
    #[inline(always)]
    pub fn aes(self) -> &'a mut crate::W<REG> {
        self.variant(SELECT::Aes)
    }
    #[doc = "The SHA peripheral uses the Crypto DMA"]
    #[inline(always)]
    pub fn sha(self) -> &'a mut crate::W<REG> {
        self.variant(SELECT::Sha)
    }
}
impl R {
    #[doc = "Bit 0 - Select one between AES and SHA to use DMA. 0: AES. 1: SHA."]
    #[inline(always)]
    pub fn select(&self) -> SELECT_R {
        SELECT_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AES_SHA_SELECT")
            .field("select", &self.select())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Select one between AES and SHA to use DMA. 0: AES. 1: SHA."]
    #[inline(always)]
    pub fn select(&mut self) -> SELECT_W<AES_SHA_SELECT_SPEC> {
        SELECT_W::new(self, 0)
    }
}
#[doc = "AES/SHA select register\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_sha_select::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_sha_select::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AES_SHA_SELECT_SPEC;
impl crate::RegisterSpec for AES_SHA_SELECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_sha_select::R`](R) reader structure"]
impl crate::Readable for AES_SHA_SELECT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aes_sha_select::W`](W) writer structure"]
impl crate::Writable for AES_SHA_SELECT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AES_SHA_SELECT to value 0"]
impl crate::Resettable for AES_SHA_SELECT_SPEC {}
