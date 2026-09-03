#[doc = "Register `MIIDATA` reader"]
pub type R = crate::R<MIIDATA_SPEC>;
#[doc = "Register `MIIDATA` writer"]
pub type W = crate::W<MIIDATA_SPEC>;
#[doc = "Field `MII_DATA` reader - GMII Data This field contains the 16bit data value read from the PHY or RevMII after a Management Read operation or the 16bit data value to be written to the PHY or RevMII before a Management Write operation"]
pub type MII_DATA_R = crate::FieldReader<u16>;
#[doc = "Field `MII_DATA` writer - GMII Data This field contains the 16bit data value read from the PHY or RevMII after a Management Read operation or the 16bit data value to be written to the PHY or RevMII before a Management Write operation"]
pub type MII_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - GMII Data This field contains the 16bit data value read from the PHY or RevMII after a Management Read operation or the 16bit data value to be written to the PHY or RevMII before a Management Write operation"]
    #[inline(always)]
    pub fn mii_data(&self) -> MII_DATA_R {
        MII_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MIIDATA")
            .field("mii_data", &self.mii_data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - GMII Data This field contains the 16bit data value read from the PHY or RevMII after a Management Read operation or the 16bit data value to be written to the PHY or RevMII before a Management Write operation"]
    #[inline(always)]
    pub fn mii_data(&mut self) -> MII_DATA_W<'_, MIIDATA_SPEC> {
        MII_DATA_W::new(self, 0)
    }
}
#[doc = "PHY data read write\n\nYou can [`read`](crate::Reg::read) this register and get [`miidata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miidata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MIIDATA_SPEC;
impl crate::RegisterSpec for MIIDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`miidata::R`](R) reader structure"]
impl crate::Readable for MIIDATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`miidata::W`](W) writer structure"]
impl crate::Writable for MIIDATA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MIIDATA to value 0"]
impl crate::Resettable for MIIDATA_SPEC {}
