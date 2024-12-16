#[doc = "Register `FORCE_WPD_SAR` reader"]
pub type R = crate::R<FORCE_WPD_SAR_SPEC>;
#[doc = "Register `FORCE_WPD_SAR` writer"]
pub type W = crate::W<FORCE_WPD_SAR_SPEC>;
#[doc = "Field `FORCE_XPD_SAR1` reader - 2'b11:software control, force on. 2'b10:software control, force off. 2'b0x:hardware control."]
pub type FORCE_XPD_SAR1_R = crate::FieldReader;
#[doc = "Field `FORCE_XPD_SAR1` writer - 2'b11:software control, force on. 2'b10:software control, force off. 2'b0x:hardware control."]
pub type FORCE_XPD_SAR1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FORCE_XPD_SAR2` reader - 2'b11:software control, force on. 2'b10:software control, force off. 2'b0x:hardware control."]
pub type FORCE_XPD_SAR2_R = crate::FieldReader;
#[doc = "Field `FORCE_XPD_SAR2` writer - 2'b11:software control, force on. 2'b10:software control, force off. 2'b0x:hardware control."]
pub type FORCE_XPD_SAR2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 2'b11:software control, force on. 2'b10:software control, force off. 2'b0x:hardware control."]
    #[inline(always)]
    pub fn force_xpd_sar1(&self) -> FORCE_XPD_SAR1_R {
        FORCE_XPD_SAR1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 2'b11:software control, force on. 2'b10:software control, force off. 2'b0x:hardware control."]
    #[inline(always)]
    pub fn force_xpd_sar2(&self) -> FORCE_XPD_SAR2_R {
        FORCE_XPD_SAR2_R::new(((self.bits >> 2) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FORCE_WPD_SAR")
            .field("force_xpd_sar1", &self.force_xpd_sar1())
            .field("force_xpd_sar2", &self.force_xpd_sar2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - 2'b11:software control, force on. 2'b10:software control, force off. 2'b0x:hardware control."]
    #[inline(always)]
    pub fn force_xpd_sar1(&mut self) -> FORCE_XPD_SAR1_W<FORCE_WPD_SAR_SPEC> {
        FORCE_XPD_SAR1_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - 2'b11:software control, force on. 2'b10:software control, force off. 2'b0x:hardware control."]
    #[inline(always)]
    pub fn force_xpd_sar2(&mut self) -> FORCE_XPD_SAR2_W<FORCE_WPD_SAR_SPEC> {
        FORCE_XPD_SAR2_W::new(self, 2)
    }
}
#[doc = "In sleep, force to use rtc to control ADC\n\nYou can [`read`](crate::Reg::read) this register and get [`force_wpd_sar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`force_wpd_sar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FORCE_WPD_SAR_SPEC;
impl crate::RegisterSpec for FORCE_WPD_SAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`force_wpd_sar::R`](R) reader structure"]
impl crate::Readable for FORCE_WPD_SAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`force_wpd_sar::W`](W) writer structure"]
impl crate::Writable for FORCE_WPD_SAR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FORCE_WPD_SAR to value 0"]
impl crate::Resettable for FORCE_WPD_SAR_SPEC {
    const RESET_VALUE: u32 = 0;
}
