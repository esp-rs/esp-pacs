#[doc = "Register `AT_CMD_PRECNT` reader"]
pub struct R(crate::R<AT_CMD_PRECNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AT_CMD_PRECNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AT_CMD_PRECNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AT_CMD_PRECNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AT_CMD_PRECNT` writer"]
pub struct W(crate::W<AT_CMD_PRECNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AT_CMD_PRECNT_SPEC>;
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
impl From<crate::W<AT_CMD_PRECNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AT_CMD_PRECNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRE_IDLE_NUM` reader - This register is used to configure the idle duration time before the first at_cmd is received by receiver. when the the duration is less than this register value it will not take the next data received as at_cmd char."]
pub type PRE_IDLE_NUM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PRE_IDLE_NUM` writer - This register is used to configure the idle duration time before the first at_cmd is received by receiver. when the the duration is less than this register value it will not take the next data received as at_cmd char."]
pub type PRE_IDLE_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, AT_CMD_PRECNT_SPEC, 24, O, u32, u32>;
impl R {
    #[doc = "Bits 0:23 - This register is used to configure the idle duration time before the first at_cmd is received by receiver. when the the duration is less than this register value it will not take the next data received as at_cmd char."]
    #[inline(always)]
    pub fn pre_idle_num(&self) -> PRE_IDLE_NUM_R {
        PRE_IDLE_NUM_R::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AT_CMD_PRECNT")
            .field(
                "pre_idle_num",
                &format_args!("{}", self.pre_idle_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AT_CMD_PRECNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:23 - This register is used to configure the idle duration time before the first at_cmd is received by receiver. when the the duration is less than this register value it will not take the next data received as at_cmd char."]
    #[inline(always)]
    #[must_use]
    pub fn pre_idle_num(&mut self) -> PRE_IDLE_NUM_W<0> {
        PRE_IDLE_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [at_cmd_precnt](index.html) module"]
pub struct AT_CMD_PRECNT_SPEC;
impl crate::RegisterSpec for AT_CMD_PRECNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [at_cmd_precnt::R](R) reader structure"]
impl crate::Readable for AT_CMD_PRECNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [at_cmd_precnt::W](W) writer structure"]
impl crate::Writable for AT_CMD_PRECNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AT_CMD_PRECNT to value 0x0018_6a00"]
impl crate::Resettable for AT_CMD_PRECNT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0018_6a00;
}
