#[doc = "Register `BLEBDADDRU` reader"]
pub type R = crate::R<BLEBDADDRU_SPEC>;
#[doc = "Register `BLEBDADDRU` writer"]
pub type W = crate::W<BLEBDADDRU_SPEC>;
#[doc = "Field `BDADDRU` reader - "]
pub type BDADDRU_R = crate::FieldReader<u16>;
#[doc = "Field `BDADDRU` writer - "]
pub type BDADDRU_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PRIV_NPUB` reader - Address privacy indicator (0=public, 1=private)"]
pub type PRIV_NPUB_R = crate::BitReader;
#[doc = "Field `PRIV_NPUB` writer - Address privacy indicator (0=public, 1=private)"]
pub type PRIV_NPUB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn bdaddru(&self) -> BDADDRU_R {
        BDADDRU_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Address privacy indicator (0=public, 1=private)"]
    #[inline(always)]
    pub fn priv_npub(&self) -> PRIV_NPUB_R {
        PRIV_NPUB_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLEBDADDRU")
            .field("bdaddru", &self.bdaddru())
            .field("priv_npub", &self.priv_npub())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn bdaddru(&mut self) -> BDADDRU_W<'_, BLEBDADDRU_SPEC> {
        BDADDRU_W::new(self, 0)
    }
    #[doc = "Bit 16 - Address privacy indicator (0=public, 1=private)"]
    #[inline(always)]
    pub fn priv_npub(&mut self) -> PRIV_NPUB_W<'_, BLEBDADDRU_SPEC> {
        PRIV_NPUB_W::new(self, 16)
    }
}
#[doc = "BLE device address MSB register\n\nYou can [`read`](crate::Reg::read) this register and get [`blebdaddru::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blebdaddru::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLEBDADDRU_SPEC;
impl crate::RegisterSpec for BLEBDADDRU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blebdaddru::R`](R) reader structure"]
impl crate::Readable for BLEBDADDRU_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blebdaddru::W`](W) writer structure"]
impl crate::Writable for BLEBDADDRU_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLEBDADDRU to value 0"]
impl crate::Resettable for BLEBDADDRU_SPEC {}
