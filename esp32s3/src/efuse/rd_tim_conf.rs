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
#[doc = "Field `READ_INIT_NUM` reader - Configures the initial read time of eFuse."]
pub type READ_INIT_NUM_R = crate::FieldReader;
#[doc = "Field `READ_INIT_NUM` writer - Configures the initial read time of eFuse."]
pub type READ_INIT_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, RD_TIM_CONF_SPEC, 8, O>;
impl R {
    #[doc = "Bits 24:31 - Configures the initial read time of eFuse."]
    #[inline(always)]
    pub fn read_init_num(&self) -> READ_INIT_NUM_R {
        READ_INIT_NUM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_TIM_CONF")
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
    #[doc = "Bits 24:31 - Configures the initial read time of eFuse."]
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
#[doc = "`reset()` method sets RD_TIM_CONF to value 0x1200_0000"]
impl crate::Resettable for RD_TIM_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x1200_0000;
}
