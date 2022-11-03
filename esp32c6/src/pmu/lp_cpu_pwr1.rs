#[doc = "Register `LP_CPU_PWR1` reader"]
pub struct R(crate::R<LP_CPU_PWR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LP_CPU_PWR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LP_CPU_PWR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LP_CPU_PWR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LP_CPU_PWR1` writer"]
pub struct W(crate::W<LP_CPU_PWR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LP_CPU_PWR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<LP_CPU_PWR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LP_CPU_PWR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LP_CPU_WAKEUP_EN` reader - need_des"]
pub type LP_CPU_WAKEUP_EN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LP_CPU_WAKEUP_EN` writer - need_des"]
pub type LP_CPU_WAKEUP_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LP_CPU_PWR1_SPEC, u16, u16, 16, O>;
#[doc = "Field `LP_CPU_SLEEP_REQ` writer - need_des"]
pub type LP_CPU_SLEEP_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, LP_CPU_PWR1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn lp_cpu_wakeup_en(&self) -> LP_CPU_WAKEUP_EN_R {
        LP_CPU_WAKEUP_EN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_cpu_wakeup_en(&mut self) -> LP_CPU_WAKEUP_EN_W<0> {
        LP_CPU_WAKEUP_EN_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_cpu_sleep_req(&mut self) -> LP_CPU_SLEEP_REQ_W<31> {
        LP_CPU_SLEEP_REQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lp_cpu_pwr1](index.html) module"]
pub struct LP_CPU_PWR1_SPEC;
impl crate::RegisterSpec for LP_CPU_PWR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lp_cpu_pwr1::R](R) reader structure"]
impl crate::Readable for LP_CPU_PWR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lp_cpu_pwr1::W](W) writer structure"]
impl crate::Writable for LP_CPU_PWR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LP_CPU_PWR1 to value 0"]
impl crate::Resettable for LP_CPU_PWR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
