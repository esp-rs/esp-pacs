#[doc = "Register `EMACMIIDATA` reader"]
pub type R = crate::R<EMACMIIDATA_SPEC>;
#[doc = "Register `EMACMIIDATA` writer"]
pub type W = crate::W<EMACMIIDATA_SPEC>;
#[doc = "Field `MII_DATA` reader - This field contains the 16-bit data value read from the PHY after a Management Read operation or the 16-bit data value to be written to the PHY before a Management Write operation."]
pub type MII_DATA_R = crate::FieldReader<u16>;
#[doc = "Field `MII_DATA` writer - This field contains the 16-bit data value read from the PHY after a Management Read operation or the 16-bit data value to be written to the PHY before a Management Write operation."]
pub type MII_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This field contains the 16-bit data value read from the PHY after a Management Read operation or the 16-bit data value to be written to the PHY before a Management Write operation."]
    #[inline(always)]
    pub fn mii_data(&self) -> MII_DATA_R {
        MII_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMACMIIDATA")
            .field("mii_data", &self.mii_data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - This field contains the 16-bit data value read from the PHY after a Management Read operation or the 16-bit data value to be written to the PHY before a Management Write operation."]
    #[inline(always)]
    #[must_use]
    pub fn mii_data(&mut self) -> MII_DATA_W<EMACMIIDATA_SPEC> {
        MII_DATA_W::new(self, 0)
    }
}
#[doc = "PHY data read write\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacmiidata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacmiidata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMACMIIDATA_SPEC;
impl crate::RegisterSpec for EMACMIIDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emacmiidata::R`](R) reader structure"]
impl crate::Readable for EMACMIIDATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emacmiidata::W`](W) writer structure"]
impl crate::Writable for EMACMIIDATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMACMIIDATA to value 0"]
impl crate::Resettable for EMACMIIDATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
