#[doc = "Register `EMACMIIDATA` reader"]
pub type R = crate::R<EMACMIIDATA_SPEC>;
#[doc = "Register `EMACMIIDATA` writer"]
pub type W = crate::W<EMACMIIDATA_SPEC>;
#[doc = "Field `MII_DATA` reader - This field contains the 16-bit data value read from the PHY after a Management Read operation or the 16-bit data value to be written to the PHY before a Management Write operation."]
pub type MII_DATA_R = crate::FieldReader<u16>;
#[doc = "Field `MII_DATA` writer - This field contains the 16-bit data value read from the PHY after a Management Read operation or the 16-bit data value to be written to the PHY before a Management Write operation."]
pub type MII_DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
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
            .field("mii_data", &format_args!("{}", self.mii_data().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EMACMIIDATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field contains the 16-bit data value read from the PHY after a Management Read operation or the 16-bit data value to be written to the PHY before a Management Write operation."]
    #[inline(always)]
    #[must_use]
    pub fn mii_data(&mut self) -> MII_DATA_W<EMACMIIDATA_SPEC, 0> {
        MII_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EMACMIIDATA to value 0"]
impl crate::Resettable for EMACMIIDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
