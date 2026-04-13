#[doc = "Register `SYSTEM_USB_OTGHS_PHY_ST` reader"]
pub type R = crate::R<SYSTEM_USB_OTGHS_PHY_ST_SPEC>;
#[doc = "Register `SYSTEM_USB_OTGHS_PHY_ST` writer"]
pub type W = crate::W<SYSTEM_USB_OTGHS_PHY_ST_SPEC>;
#[doc = "Field `VAL` reader - "]
pub type VAL_R = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - "]
pub type VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSTEM_USB_OTGHS_PHY_ST")
            .field("val", &self.val())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&mut self) -> VAL_W<'_, SYSTEM_USB_OTGHS_PHY_ST_SPEC> {
        VAL_W::new(self, 0)
    }
}
#[doc = "USB OTG HS PHY status\n\nYou can [`read`](crate::Reg::read) this register and get [`system_usb_otghs_phy_st::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`system_usb_otghs_phy_st::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSTEM_USB_OTGHS_PHY_ST_SPEC;
impl crate::RegisterSpec for SYSTEM_USB_OTGHS_PHY_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`system_usb_otghs_phy_st::R`](R) reader structure"]
impl crate::Readable for SYSTEM_USB_OTGHS_PHY_ST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`system_usb_otghs_phy_st::W`](W) writer structure"]
impl crate::Writable for SYSTEM_USB_OTGHS_PHY_ST_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSTEM_USB_OTGHS_PHY_ST to value 0"]
impl crate::Resettable for SYSTEM_USB_OTGHS_PHY_ST_SPEC {}
