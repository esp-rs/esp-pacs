#[doc = "Register `DCM_WAIT_DELAY` reader"]
pub type R = crate::R<DCM_WAIT_DELAY_SPEC>;
#[doc = "Register `DCM_WAIT_DELAY` writer"]
pub type W = crate::W<DCM_WAIT_DELAY_SPEC>;
#[doc = "Field `DCDC_PRE_DELAY` reader - DCDC pre-on/post off delay"]
pub type DCDC_PRE_DELAY_R = crate::FieldReader;
#[doc = "Field `DCDC_PRE_DELAY` writer - DCDC pre-on/post off delay"]
pub type DCDC_PRE_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DCDC_RES_OFF_DELAY` reader - DCDC fb res off delay"]
pub type DCDC_RES_OFF_DELAY_R = crate::FieldReader;
#[doc = "Field `DCDC_RES_OFF_DELAY` writer - DCDC fb res off delay"]
pub type DCDC_RES_OFF_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DCDC_STABLE_DELAY` reader - DCDC stable delay"]
pub type DCDC_STABLE_DELAY_R = crate::FieldReader<u16>;
#[doc = "Field `DCDC_STABLE_DELAY` writer - DCDC stable delay"]
pub type DCDC_STABLE_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:7 - DCDC pre-on/post off delay"]
    #[inline(always)]
    pub fn dcdc_pre_delay(&self) -> DCDC_PRE_DELAY_R {
        DCDC_PRE_DELAY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DCDC fb res off delay"]
    #[inline(always)]
    pub fn dcdc_res_off_delay(&self) -> DCDC_RES_OFF_DELAY_R {
        DCDC_RES_OFF_DELAY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:25 - DCDC stable delay"]
    #[inline(always)]
    pub fn dcdc_stable_delay(&self) -> DCDC_STABLE_DELAY_R {
        DCDC_STABLE_DELAY_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCM_WAIT_DELAY")
            .field(
                "dcdc_pre_delay",
                &format_args!("{}", self.dcdc_pre_delay().bits()),
            )
            .field(
                "dcdc_res_off_delay",
                &format_args!("{}", self.dcdc_res_off_delay().bits()),
            )
            .field(
                "dcdc_stable_delay",
                &format_args!("{}", self.dcdc_stable_delay().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DCM_WAIT_DELAY_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - DCDC pre-on/post off delay"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_pre_delay(&mut self) -> DCDC_PRE_DELAY_W<DCM_WAIT_DELAY_SPEC> {
        DCDC_PRE_DELAY_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DCDC fb res off delay"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_res_off_delay(&mut self) -> DCDC_RES_OFF_DELAY_W<DCM_WAIT_DELAY_SPEC> {
        DCDC_RES_OFF_DELAY_W::new(self, 8)
    }
    #[doc = "Bits 16:25 - DCDC stable delay"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_stable_delay(&mut self) -> DCDC_STABLE_DELAY_W<DCM_WAIT_DELAY_SPEC> {
        DCDC_STABLE_DELAY_W::new(self, 16)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcm_wait_delay::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcm_wait_delay::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCM_WAIT_DELAY_SPEC;
impl crate::RegisterSpec for DCM_WAIT_DELAY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcm_wait_delay::R`](R) reader structure"]
impl crate::Readable for DCM_WAIT_DELAY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcm_wait_delay::W`](W) writer structure"]
impl crate::Writable for DCM_WAIT_DELAY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCM_WAIT_DELAY to value 0x004b_0205"]
impl crate::Resettable for DCM_WAIT_DELAY_SPEC {
    const RESET_VALUE: Self::Ux = 0x004b_0205;
}