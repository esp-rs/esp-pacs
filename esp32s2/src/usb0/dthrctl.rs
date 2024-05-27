///Register `DTHRCTL` reader
pub type R = crate::R<DTHRCTL_SPEC>;
///Register `DTHRCTL` writer
pub type W = crate::W<DTHRCTL_SPEC>;
///Field `NONISOTHREN` reader -
pub type NONISOTHREN_R = crate::BitReader;
///Field `NONISOTHREN` writer -
pub type NONISOTHREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ISOTHREN` reader -
pub type ISOTHREN_R = crate::BitReader;
///Field `ISOTHREN` writer -
pub type ISOTHREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXTHRLEN` reader -
pub type TXTHRLEN_R = crate::FieldReader<u16>;
///Field `TXTHRLEN` writer -
pub type TXTHRLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `AHBTHRRATIO` reader -
pub type AHBTHRRATIO_R = crate::FieldReader;
///Field `AHBTHRRATIO` writer -
pub type AHBTHRRATIO_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RXTHREN` reader -
pub type RXTHREN_R = crate::BitReader;
///Field `RXTHREN` writer -
pub type RXTHREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXTHRLEN` reader -
pub type RXTHRLEN_R = crate::FieldReader<u16>;
///Field `RXTHRLEN` writer -
pub type RXTHRLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `ARBPRKEN` reader -
pub type ARBPRKEN_R = crate::BitReader;
///Field `ARBPRKEN` writer -
pub type ARBPRKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn nonisothren(&self) -> NONISOTHREN_R {
        NONISOTHREN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn isothren(&self) -> ISOTHREN_R {
        ISOTHREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:10
    #[inline(always)]
    pub fn txthrlen(&self) -> TXTHRLEN_R {
        TXTHRLEN_R::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    ///Bits 11:12
    #[inline(always)]
    pub fn ahbthrratio(&self) -> AHBTHRRATIO_R {
        AHBTHRRATIO_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bit 16
    #[inline(always)]
    pub fn rxthren(&self) -> RXTHREN_R {
        RXTHREN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:25
    #[inline(always)]
    pub fn rxthrlen(&self) -> RXTHRLEN_R {
        RXTHRLEN_R::new(((self.bits >> 17) & 0x01ff) as u16)
    }
    ///Bit 27
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
    ///Bit 0
    #[inline(always)]
    #[must_use]
    pub fn nonisothren(&mut self) -> NONISOTHREN_W<DTHRCTL_SPEC> {
        NONISOTHREN_W::new(self, 0)
    }
    ///Bit 1
    #[inline(always)]
    #[must_use]
    pub fn isothren(&mut self) -> ISOTHREN_W<DTHRCTL_SPEC> {
        ISOTHREN_W::new(self, 1)
    }
    ///Bits 2:10
    #[inline(always)]
    #[must_use]
    pub fn txthrlen(&mut self) -> TXTHRLEN_W<DTHRCTL_SPEC> {
        TXTHRLEN_W::new(self, 2)
    }
    ///Bits 11:12
    #[inline(always)]
    #[must_use]
    pub fn ahbthrratio(&mut self) -> AHBTHRRATIO_W<DTHRCTL_SPEC> {
        AHBTHRRATIO_W::new(self, 11)
    }
    ///Bit 16
    #[inline(always)]
    #[must_use]
    pub fn rxthren(&mut self) -> RXTHREN_W<DTHRCTL_SPEC> {
        RXTHREN_W::new(self, 16)
    }
    ///Bits 17:25
    #[inline(always)]
    #[must_use]
    pub fn rxthrlen(&mut self) -> RXTHRLEN_W<DTHRCTL_SPEC> {
        RXTHRLEN_W::new(self, 17)
    }
    ///Bit 27
    #[inline(always)]
    #[must_use]
    pub fn arbprken(&mut self) -> ARBPRKEN_W<DTHRCTL_SPEC> {
        ARBPRKEN_W::new(self, 27)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`dthrctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dthrctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DTHRCTL_SPEC;
impl crate::RegisterSpec for DTHRCTL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dthrctl::R`](R) reader structure
impl crate::Readable for DTHRCTL_SPEC {}
///`write(|w| ..)` method takes [`dthrctl::W`](W) writer structure
impl crate::Writable for DTHRCTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DTHRCTL to value 0x0802_0020
impl crate::Resettable for DTHRCTL_SPEC {
    const RESET_VALUE: u32 = 0x0802_0020;
}
