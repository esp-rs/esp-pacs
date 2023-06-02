#[doc = "Register `HP_LP_CPU_COMM` writer"]
pub struct W(crate::W<HP_LP_CPU_COMM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HP_LP_CPU_COMM_SPEC>;
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
impl From<crate::W<HP_LP_CPU_COMM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HP_LP_CPU_COMM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LP_TRIGGER_HP` writer - need_des"]
pub type LP_TRIGGER_HP_W<'a, const O: u8> = crate::BitWriter<'a, HP_LP_CPU_COMM_SPEC, O>;
#[doc = "Field `HP_TRIGGER_LP` writer - need_des"]
pub type HP_TRIGGER_LP_W<'a, const O: u8> = crate::BitWriter<'a, HP_LP_CPU_COMM_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_LP_CPU_COMM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_trigger_hp(&mut self) -> LP_TRIGGER_HP_W<30> {
        LP_TRIGGER_HP_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_trigger_lp(&mut self) -> HP_TRIGGER_LP_W<31> {
        HP_TRIGGER_LP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hp_lp_cpu_comm](index.html) module"]
pub struct HP_LP_CPU_COMM_SPEC;
impl crate::RegisterSpec for HP_LP_CPU_COMM_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [hp_lp_cpu_comm::W](W) writer structure"]
impl crate::Writable for HP_LP_CPU_COMM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_LP_CPU_COMM to value 0"]
impl crate::Resettable for HP_LP_CPU_COMM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
