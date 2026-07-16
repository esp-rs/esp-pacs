#[doc = "Register `REGISTER5_GMIIDATAREGISTER` reader"]
pub type R = crate::R<REGISTER5_GMIIDATAREGISTER_SPEC>;
#[doc = "Register `REGISTER5_GMIIDATAREGISTER` writer"]
pub type W = crate::W<REGISTER5_GMIIDATAREGISTER_SPEC>;
#[doc = "Field `GD` reader - GMII Data This field contains the 16bit data value read from the PHY or RevMII after a Management Read operation or the 16bit data value to be written to the PHY or RevMII before a Management Write operation"]
pub type GD_R = crate::FieldReader<u16>;
#[doc = "Field `GD` writer - GMII Data This field contains the 16bit data value read from the PHY or RevMII after a Management Read operation or the 16bit data value to be written to the PHY or RevMII before a Management Write operation"]
pub type GD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - GMII Data This field contains the 16bit data value read from the PHY or RevMII after a Management Read operation or the 16bit data value to be written to the PHY or RevMII before a Management Write operation"]
    #[inline(always)]
    pub fn gd(&self) -> GD_R {
        GD_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER5_GMIIDATAREGISTER")
            .field("gd", &self.gd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - GMII Data This field contains the 16bit data value read from the PHY or RevMII after a Management Read operation or the 16bit data value to be written to the PHY or RevMII before a Management Write operation"]
    #[inline(always)]
    pub fn gd(&mut self) -> GD_W<'_, REGISTER5_GMIIDATAREGISTER_SPEC> {
        GD_W::new(self, 0)
    }
}
#[doc = "Contains the data to be written to or read from the PHY register This register is present only when you select the Station Management _MDIO_ feature in coreConsultant _See Table 726_\n\nYou can [`read`](crate::Reg::read) this register and get [`register5_gmiidataregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register5_gmiidataregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER5_GMIIDATAREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER5_GMIIDATAREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register5_gmiidataregister::R`](R) reader structure"]
impl crate::Readable for REGISTER5_GMIIDATAREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register5_gmiidataregister::W`](W) writer structure"]
impl crate::Writable for REGISTER5_GMIIDATAREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER5_GMIIDATAREGISTER to value 0"]
impl crate::Resettable for REGISTER5_GMIIDATAREGISTER_SPEC {}
