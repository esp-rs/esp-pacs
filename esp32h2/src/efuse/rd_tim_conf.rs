#[doc = "Register `RD_TIM_CONF` reader"]
pub struct R(crate::R<RD_TIM_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_TIM_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_TIM_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_TIM_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RD_TIM_CONF` writer"]
pub struct W(crate::W<RD_TIM_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RD_TIM_CONF_SPEC>;
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
impl From<crate::W<RD_TIM_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RD_TIM_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THR_A` reader - Configures the read hold time."]
pub type THR_A_R = crate::FieldReader;
#[doc = "Field `THR_A` writer - Configures the read hold time."]
pub type THR_A_W<'a, const O: u8> = crate::FieldWriter<'a, RD_TIM_CONF_SPEC, 8, O>;
#[doc = "Field `TRD` reader - Configures the read time."]
pub type TRD_R = crate::FieldReader;
#[doc = "Field `TRD` writer - Configures the read time."]
pub type TRD_W<'a, const O: u8> = crate::FieldWriter<'a, RD_TIM_CONF_SPEC, 8, O>;
#[doc = "Field `TSUR_A` reader - Configures the read setup time."]
pub type TSUR_A_R = crate::FieldReader;
#[doc = "Field `TSUR_A` writer - Configures the read setup time."]
pub type TSUR_A_W<'a, const O: u8> = crate::FieldWriter<'a, RD_TIM_CONF_SPEC, 8, O>;
#[doc = "Field `READ_INIT_NUM` reader - Configures the waiting time of reading eFuse memory."]
pub type READ_INIT_NUM_R = crate::FieldReader;
#[doc = "Field `READ_INIT_NUM` writer - Configures the waiting time of reading eFuse memory."]
pub type READ_INIT_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, RD_TIM_CONF_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Configures the read hold time."]
    #[inline(always)]
    pub fn thr_a(&self) -> THR_A_R {
        THR_A_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Configures the read time."]
    #[inline(always)]
    pub fn trd(&self) -> TRD_R {
        TRD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Configures the read setup time."]
    #[inline(always)]
    pub fn tsur_a(&self) -> TSUR_A_R {
        TSUR_A_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Configures the waiting time of reading eFuse memory."]
    #[inline(always)]
    pub fn read_init_num(&self) -> READ_INIT_NUM_R {
        READ_INIT_NUM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_TIM_CONF")
            .field("thr_a", &format_args!("{}", self.thr_a().bits()))
            .field("trd", &format_args!("{}", self.trd().bits()))
            .field("tsur_a", &format_args!("{}", self.tsur_a().bits()))
            .field(
                "read_init_num",
                &format_args!("{}", self.read_init_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_TIM_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures the read hold time."]
    #[inline(always)]
    #[must_use]
    pub fn thr_a(&mut self) -> THR_A_W<0> {
        THR_A_W::new(self)
    }
    #[doc = "Bits 8:15 - Configures the read time."]
    #[inline(always)]
    #[must_use]
    pub fn trd(&mut self) -> TRD_W<8> {
        TRD_W::new(self)
    }
    #[doc = "Bits 16:23 - Configures the read setup time."]
    #[inline(always)]
    #[must_use]
    pub fn tsur_a(&mut self) -> TSUR_A_W<16> {
        TSUR_A_W::new(self)
    }
    #[doc = "Bits 24:31 - Configures the waiting time of reading eFuse memory."]
    #[inline(always)]
    #[must_use]
    pub fn read_init_num(&mut self) -> READ_INIT_NUM_W<24> {
        READ_INIT_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configures read timing parameters.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_tim_conf](index.html) module"]
pub struct RD_TIM_CONF_SPEC;
impl crate::RegisterSpec for RD_TIM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_tim_conf::R](R) reader structure"]
impl crate::Readable for RD_TIM_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rd_tim_conf::W](W) writer structure"]
impl crate::Writable for RD_TIM_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RD_TIM_CONF to value 0x0f01_0201"]
impl crate::Resettable for RD_TIM_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f01_0201;
}
