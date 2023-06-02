#[doc = "Register `WR_TIM_CONF0` reader"]
pub struct R(crate::R<WR_TIM_CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WR_TIM_CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WR_TIM_CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WR_TIM_CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WR_TIM_CONF0` writer"]
pub struct W(crate::W<WR_TIM_CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WR_TIM_CONF0_SPEC>;
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
impl From<crate::W<WR_TIM_CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WR_TIM_CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THP_A` reader - Configures the hold time of programming operation."]
pub type THP_A_R = crate::FieldReader;
#[doc = "Field `THP_A` writer - Configures the hold time of programming operation."]
pub type THP_A_W<'a, const O: u8> = crate::FieldWriter<'a, WR_TIM_CONF0_SPEC, 8, O>;
#[doc = "Field `TPGM_INACTIVE` reader - Configures the length of pulse during programming 0 to eFuse."]
pub type TPGM_INACTIVE_R = crate::FieldReader;
#[doc = "Field `TPGM_INACTIVE` writer - Configures the length of pulse during programming 0 to eFuse."]
pub type TPGM_INACTIVE_W<'a, const O: u8> = crate::FieldWriter<'a, WR_TIM_CONF0_SPEC, 8, O>;
#[doc = "Field `TPGM` reader - Configures the length of pulse during programming 1 to eFuse."]
pub type TPGM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TPGM` writer - Configures the length of pulse during programming 1 to eFuse."]
pub type TPGM_W<'a, const O: u8> = crate::FieldWriter<'a, WR_TIM_CONF0_SPEC, 16, O, u16, u16>;
impl R {
    #[doc = "Bits 0:7 - Configures the hold time of programming operation."]
    #[inline(always)]
    pub fn thp_a(&self) -> THP_A_R {
        THP_A_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Configures the length of pulse during programming 0 to eFuse."]
    #[inline(always)]
    pub fn tpgm_inactive(&self) -> TPGM_INACTIVE_R {
        TPGM_INACTIVE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Configures the length of pulse during programming 1 to eFuse."]
    #[inline(always)]
    pub fn tpgm(&self) -> TPGM_R {
        TPGM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WR_TIM_CONF0")
            .field("thp_a", &format_args!("{}", self.thp_a().bits()))
            .field(
                "tpgm_inactive",
                &format_args!("{}", self.tpgm_inactive().bits()),
            )
            .field("tpgm", &format_args!("{}", self.tpgm().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WR_TIM_CONF0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures the hold time of programming operation."]
    #[inline(always)]
    #[must_use]
    pub fn thp_a(&mut self) -> THP_A_W<0> {
        THP_A_W::new(self)
    }
    #[doc = "Bits 8:15 - Configures the length of pulse during programming 0 to eFuse."]
    #[inline(always)]
    #[must_use]
    pub fn tpgm_inactive(&mut self) -> TPGM_INACTIVE_W<8> {
        TPGM_INACTIVE_W::new(self)
    }
    #[doc = "Bits 16:31 - Configures the length of pulse during programming 1 to eFuse."]
    #[inline(always)]
    #[must_use]
    pub fn tpgm(&mut self) -> TPGM_W<16> {
        TPGM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register 0 of eFuse programming timing parameters.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wr_tim_conf0](index.html) module"]
pub struct WR_TIM_CONF0_SPEC;
impl crate::RegisterSpec for WR_TIM_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wr_tim_conf0::R](R) reader structure"]
impl crate::Readable for WR_TIM_CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wr_tim_conf0::W](W) writer structure"]
impl crate::Writable for WR_TIM_CONF0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WR_TIM_CONF0 to value 0x00c8_0101"]
impl crate::Resettable for WR_TIM_CONF0_SPEC {
    const RESET_VALUE: Self::Ux = 0x00c8_0101;
}
