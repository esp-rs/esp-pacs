#[doc = "Register `AW_QOS_PEAK_RATE%s` reader"]
pub type R = crate::R<AW_QOS_PEAK_RATE_SPEC>;
#[doc = "Register `AW_QOS_PEAK_RATE%s` writer"]
pub type W = crate::W<AW_QOS_PEAK_RATE_SPEC>;
#[doc = "Field `REG_AW_QOS_PEAK_RATE_0` reader - "]
pub type REG_AW_QOS_PEAK_RATE_0_R = crate::FieldReader;
#[doc = "Field `REG_AW_QOS_PEAK_RATE_0` writer - "]
pub type REG_AW_QOS_PEAK_RATE_0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn reg_aw_qos_peak_rate_0(&self) -> REG_AW_QOS_PEAK_RATE_0_R {
        REG_AW_QOS_PEAK_RATE_0_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AW_QOS_PEAK_RATE")
            .field("reg_aw_qos_peak_rate_0", &self.reg_aw_qos_peak_rate_0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn reg_aw_qos_peak_rate_0(
        &mut self,
    ) -> REG_AW_QOS_PEAK_RATE_0_W<'_, AW_QOS_PEAK_RATE_SPEC> {
        REG_AW_QOS_PEAK_RATE_0_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`aw_qos_peak_rate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aw_qos_peak_rate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AW_QOS_PEAK_RATE_SPEC;
impl crate::RegisterSpec for AW_QOS_PEAK_RATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aw_qos_peak_rate::R`](R) reader structure"]
impl crate::Readable for AW_QOS_PEAK_RATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aw_qos_peak_rate::W`](W) writer structure"]
impl crate::Writable for AW_QOS_PEAK_RATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AW_QOS_PEAK_RATE%s to value 0"]
impl crate::Resettable for AW_QOS_PEAK_RATE_SPEC {}
