#[doc = "Register `DTHRCTL` reader"]
pub type R = crate::R<DTHRCTL_SPEC>;
#[doc = "Register `DTHRCTL` writer"]
pub type W = crate::W<DTHRCTL_SPEC>;
#[doc = "Field `NONISOTHREN` reader - "]
pub type NONISOTHREN_R = crate::BitReader;
#[doc = "Field `NONISOTHREN` writer - "]
pub type NONISOTHREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOTHREN` reader - "]
pub type ISOTHREN_R = crate::BitReader;
#[doc = "Field `ISOTHREN` writer - "]
pub type ISOTHREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTHRLEN` reader - "]
pub type TXTHRLEN_R = crate::FieldReader<u16>;
#[doc = "Field `TXTHRLEN` writer - "]
pub type TXTHRLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `AHBTHRRATIO` reader - "]
pub type AHBTHRRATIO_R = crate::FieldReader;
#[doc = "Field `AHBTHRRATIO` writer - "]
pub type AHBTHRRATIO_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RXTHREN` reader - "]
pub type RXTHREN_R = crate::BitReader;
#[doc = "Field `RXTHREN` writer - "]
pub type RXTHREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTHRLEN` reader - "]
pub type RXTHRLEN_R = crate::FieldReader<u16>;
#[doc = "Field `RXTHRLEN` writer - "]
pub type RXTHRLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `ARBPRKEN` reader - "]
pub type ARBPRKEN_R = crate::BitReader;
#[doc = "Field `ARBPRKEN` writer - "]
pub type ARBPRKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn nonisothren(&self) -> NONISOTHREN_R {
        NONISOTHREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn isothren(&self) -> ISOTHREN_R {
        ISOTHREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:10"]
    #[inline(always)]
    pub fn txthrlen(&self) -> TXTHRLEN_R {
        TXTHRLEN_R::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn ahbthrratio(&self) -> AHBTHRRATIO_R {
        AHBTHRRATIO_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rxthren(&self) -> RXTHREN_R {
        RXTHREN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:25"]
    #[inline(always)]
    pub fn rxthrlen(&self) -> RXTHRLEN_R {
        RXTHRLEN_R::new(((self.bits >> 17) & 0x01ff) as u16)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn arbprken(&self) -> ARBPRKEN_R {
        ARBPRKEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTHRCTL")
            .field("nonisothren", &self.nonisothren())
            .field("isothren", &self.isothren())
            .field("txthrlen", &self.txthrlen())
            .field("ahbthrratio", &self.ahbthrratio())
            .field("rxthren", &self.rxthren())
            .field("rxthrlen", &self.rxthrlen())
            .field("arbprken", &self.arbprken())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn nonisothren(&mut self) -> NONISOTHREN_W<DTHRCTL_SPEC> {
        NONISOTHREN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn isothren(&mut self) -> ISOTHREN_W<DTHRCTL_SPEC> {
        ISOTHREN_W::new(self, 1)
    }
    #[doc = "Bits 2:10"]
    #[inline(always)]
    pub fn txthrlen(&mut self) -> TXTHRLEN_W<DTHRCTL_SPEC> {
        TXTHRLEN_W::new(self, 2)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn ahbthrratio(&mut self) -> AHBTHRRATIO_W<DTHRCTL_SPEC> {
        AHBTHRRATIO_W::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rxthren(&mut self) -> RXTHREN_W<DTHRCTL_SPEC> {
        RXTHREN_W::new(self, 16)
    }
    #[doc = "Bits 17:25"]
    #[inline(always)]
    pub fn rxthrlen(&mut self) -> RXTHRLEN_W<DTHRCTL_SPEC> {
        RXTHRLEN_W::new(self, 17)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn arbprken(&mut self) -> ARBPRKEN_W<DTHRCTL_SPEC> {
        ARBPRKEN_W::new(self, 27)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dthrctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dthrctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTHRCTL_SPEC;
impl crate::RegisterSpec for DTHRCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dthrctl::R`](R) reader structure"]
impl crate::Readable for DTHRCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dthrctl::W`](W) writer structure"]
impl crate::Writable for DTHRCTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DTHRCTL to value 0x0802_0020"]
impl crate::Resettable for DTHRCTL_SPEC {
    const RESET_VALUE: u32 = 0x0802_0020;
}
