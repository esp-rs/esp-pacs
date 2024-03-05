#[doc = "Register `CH%s_RX_CARRIER_RM` reader"]
pub type R = crate::R<CH_RX_CARRIER_RM_SPEC>;
#[doc = "Register `CH%s_RX_CARRIER_RM` writer"]
pub type W = crate::W<CH_RX_CARRIER_RM_SPEC>;
#[doc = "Field `CARRIER_LOW_THRES` reader - reg_carrier_low_thres_ch2."]
pub type CARRIER_LOW_THRES_R = crate::FieldReader<u16>;
#[doc = "Field `CARRIER_LOW_THRES` writer - reg_carrier_low_thres_ch2."]
pub type CARRIER_LOW_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CARRIER_HIGH_THRES` reader - reg_carrier_high_thres_ch2."]
pub type CARRIER_HIGH_THRES_R = crate::FieldReader<u16>;
#[doc = "Field `CARRIER_HIGH_THRES` writer - reg_carrier_high_thres_ch2."]
pub type CARRIER_HIGH_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - reg_carrier_low_thres_ch2."]
    #[inline(always)]
    pub fn carrier_low_thres(&self) -> CARRIER_LOW_THRES_R {
        CARRIER_LOW_THRES_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - reg_carrier_high_thres_ch2."]
    #[inline(always)]
    pub fn carrier_high_thres(&self) -> CARRIER_HIGH_THRES_R {
        CARRIER_HIGH_THRES_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_RX_CARRIER_RM")
            .field(
                "carrier_low_thres",
                &format_args!("{}", self.carrier_low_thres().bits()),
            )
            .field(
                "carrier_high_thres",
                &format_args!("{}", self.carrier_high_thres().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_RX_CARRIER_RM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - reg_carrier_low_thres_ch2."]
    #[inline(always)]
    #[must_use]
    pub fn carrier_low_thres(&mut self) -> CARRIER_LOW_THRES_W<CH_RX_CARRIER_RM_SPEC> {
        CARRIER_LOW_THRES_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - reg_carrier_high_thres_ch2."]
    #[inline(always)]
    #[must_use]
    pub fn carrier_high_thres(&mut self) -> CARRIER_HIGH_THRES_W<CH_RX_CARRIER_RM_SPEC> {
        CARRIER_HIGH_THRES_W::new(self, 16)
    }
}
#[doc = "RMT_CH2_RX_CARRIER_RM_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_rx_carrier_rm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_rx_carrier_rm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_RX_CARRIER_RM_SPEC;
impl crate::RegisterSpec for CH_RX_CARRIER_RM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_rx_carrier_rm::R`](R) reader structure"]
impl crate::Readable for CH_RX_CARRIER_RM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_rx_carrier_rm::W`](W) writer structure"]
impl crate::Writable for CH_RX_CARRIER_RM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH%s_RX_CARRIER_RM to value 0"]
impl crate::Resettable for CH_RX_CARRIER_RM_SPEC {
    const RESET_VALUE: u32 = 0;
}
