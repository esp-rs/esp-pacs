#[doc = "Register `HP_USB_OTGHS_PHY_CTRL` reader"]
pub type R = crate::R<HP_USB_OTGHS_PHY_CTRL_SPEC>;
#[doc = "Register `HP_USB_OTGHS_PHY_CTRL` writer"]
pub type W = crate::W<HP_USB_OTGHS_PHY_CTRL_SPEC>;
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
        f.debug_struct("HP_USB_OTGHS_PHY_CTRL")
            .field("val", &self.val())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&mut self) -> VAL_W<'_, HP_USB_OTGHS_PHY_CTRL_SPEC> {
        VAL_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_usb_otghs_phy_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_usb_otghs_phy_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_USB_OTGHS_PHY_CTRL_SPEC;
impl crate::RegisterSpec for HP_USB_OTGHS_PHY_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_usb_otghs_phy_ctrl::R`](R) reader structure"]
impl crate::Readable for HP_USB_OTGHS_PHY_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_usb_otghs_phy_ctrl::W`](W) writer structure"]
impl crate::Writable for HP_USB_OTGHS_PHY_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_USB_OTGHS_PHY_CTRL to value 0"]
impl crate::Resettable for HP_USB_OTGHS_PHY_CTRL_SPEC {}
