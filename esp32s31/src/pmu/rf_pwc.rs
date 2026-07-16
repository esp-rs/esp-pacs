#[doc = "Register `RF_PWC` reader"]
pub type R = crate::R<RF_PWC_SPEC>;
#[doc = "Register `RF_PWC` writer"]
pub type W = crate::W<RF_PWC_SPEC>;
#[doc = "Field `XPD_RF_CIRCUIT` reader - need_des"]
pub type XPD_RF_CIRCUIT_R = crate::FieldReader<u16>;
#[doc = "Field `XPD_RF_CIRCUIT` writer - need_des"]
pub type XPD_RF_CIRCUIT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - need_des"]
    #[inline(always)]
    pub fn xpd_rf_circuit(&self) -> XPD_RF_CIRCUIT_R {
        XPD_RF_CIRCUIT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RF_PWC")
            .field("xpd_rf_circuit", &self.xpd_rf_circuit())
            .finish()
    }
}
impl W {
    #[doc = "Bits 16:31 - need_des"]
    #[inline(always)]
    pub fn xpd_rf_circuit(&mut self) -> XPD_RF_CIRCUIT_W<'_, RF_PWC_SPEC> {
        XPD_RF_CIRCUIT_W::new(self, 16)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_pwc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_pwc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RF_PWC_SPEC;
impl crate::RegisterSpec for RF_PWC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_pwc::R`](R) reader structure"]
impl crate::Readable for RF_PWC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rf_pwc::W`](W) writer structure"]
impl crate::Writable for RF_PWC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RF_PWC to value 0"]
impl crate::Resettable for RF_PWC_SPEC {}
