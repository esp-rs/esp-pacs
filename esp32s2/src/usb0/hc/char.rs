#[doc = "Register `CHAR` reader"]
pub type R = crate::R<CHAR_SPEC>;
#[doc = "Register `CHAR` writer"]
pub type W = crate::W<CHAR_SPEC>;
#[doc = "Field `MPS` reader - "]
pub type MPS_R = crate::FieldReader<u16>;
#[doc = "Field `MPS` writer - "]
pub type MPS_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `EPNUM` reader - "]
pub type EPNUM_R = crate::FieldReader;
#[doc = "Field `EPNUM` writer - "]
pub type EPNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EPDIR` reader - "]
pub type EPDIR_R = crate::BitReader;
#[doc = "Field `EPDIR` writer - "]
pub type EPDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSPDDEV` reader - "]
pub type LSPDDEV_R = crate::BitReader;
#[doc = "Field `LSPDDEV` writer - "]
pub type LSPDDEV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPTYPE` reader - "]
pub type EPTYPE_R = crate::FieldReader;
#[doc = "Field `EPTYPE` writer - "]
pub type EPTYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EC` reader - "]
pub type EC_R = crate::BitReader;
#[doc = "Field `EC` writer - "]
pub type EC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVADDR` reader - "]
pub type DEVADDR_R = crate::FieldReader;
#[doc = "Field `DEVADDR` writer - "]
pub type DEVADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ODDFRM` reader - "]
pub type ODDFRM_R = crate::BitReader;
#[doc = "Field `ODDFRM` writer - "]
pub type ODDFRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHDIS` reader - "]
pub type CHDIS_R = crate::BitReader;
#[doc = "Field `CHDIS` writer - "]
pub type CHDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHENA` reader - "]
pub type CHENA_R = crate::BitReader;
#[doc = "Field `CHENA` writer - "]
pub type CHENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn mps(&self) -> MPS_R {
        MPS_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn epdir(&self) -> EPDIR_R {
        EPDIR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn lspddev(&self) -> LSPDDEV_R {
        LSPDDEV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ec(&self) -> EC_R {
        EC_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:28"]
    #[inline(always)]
    pub fn devaddr(&self) -> DEVADDR_R {
        DEVADDR_R::new(((self.bits >> 22) & 0x7f) as u8)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn oddfrm(&self) -> ODDFRM_R {
        ODDFRM_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn chdis(&self) -> CHDIS_R {
        CHDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn chena(&self) -> CHENA_R {
        CHENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHAR")
            .field("mps", &self.mps())
            .field("epnum", &self.epnum())
            .field("epdir", &self.epdir())
            .field("lspddev", &self.lspddev())
            .field("eptype", &self.eptype())
            .field("ec", &self.ec())
            .field("devaddr", &self.devaddr())
            .field("oddfrm", &self.oddfrm())
            .field("chdis", &self.chdis())
            .field("chena", &self.chena())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    #[must_use]
    pub fn mps(&mut self) -> MPS_W<CHAR_SPEC> {
        MPS_W::new(self, 0)
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    #[must_use]
    pub fn epnum(&mut self) -> EPNUM_W<CHAR_SPEC> {
        EPNUM_W::new(self, 11)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn epdir(&mut self) -> EPDIR_W<CHAR_SPEC> {
        EPDIR_W::new(self, 15)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn lspddev(&mut self) -> LSPDDEV_W<CHAR_SPEC> {
        LSPDDEV_W::new(self, 17)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn eptype(&mut self) -> EPTYPE_W<CHAR_SPEC> {
        EPTYPE_W::new(self, 18)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn ec(&mut self) -> EC_W<CHAR_SPEC> {
        EC_W::new(self, 21)
    }
    #[doc = "Bits 22:28"]
    #[inline(always)]
    #[must_use]
    pub fn devaddr(&mut self) -> DEVADDR_W<CHAR_SPEC> {
        DEVADDR_W::new(self, 22)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn oddfrm(&mut self) -> ODDFRM_W<CHAR_SPEC> {
        ODDFRM_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn chdis(&mut self) -> CHDIS_W<CHAR_SPEC> {
        CHDIS_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn chena(&mut self) -> CHENA_W<CHAR_SPEC> {
        CHENA_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`char::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`char::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHAR_SPEC;
impl crate::RegisterSpec for CHAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`char::R`](R) reader structure"]
impl crate::Readable for CHAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`char::W`](W) writer structure"]
impl crate::Writable for CHAR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHAR to value 0"]
impl crate::Resettable for CHAR_SPEC {
    const RESET_VALUE: u32 = 0;
}
