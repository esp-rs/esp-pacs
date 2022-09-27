#[doc = "Register `LOG_MEM_FULL_FLAG` reader"]
pub struct R(crate::R<LOG_MEM_FULL_FLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOG_MEM_FULL_FLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOG_MEM_FULL_FLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOG_MEM_FULL_FLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOG_MEM_FULL_FLAG` writer"]
pub struct W(crate::W<LOG_MEM_FULL_FLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOG_MEM_FULL_FLAG_SPEC>;
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
impl From<crate::W<LOG_MEM_FULL_FLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOG_MEM_FULL_FLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOG_MEM_FULL_FLAG` reader - reg_log_mem_full_flag"]
pub type LOG_MEM_FULL_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `CLR_LOG_MEM_FULL_FLAG` reader - reg_clr_log_mem_full_flag"]
pub type CLR_LOG_MEM_FULL_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `CLR_LOG_MEM_FULL_FLAG` writer - reg_clr_log_mem_full_flag"]
pub type CLR_LOG_MEM_FULL_FLAG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LOG_MEM_FULL_FLAG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - reg_log_mem_full_flag"]
    #[inline(always)]
    pub fn log_mem_full_flag(&self) -> LOG_MEM_FULL_FLAG_R {
        LOG_MEM_FULL_FLAG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reg_clr_log_mem_full_flag"]
    #[inline(always)]
    pub fn clr_log_mem_full_flag(&self) -> CLR_LOG_MEM_FULL_FLAG_R {
        CLR_LOG_MEM_FULL_FLAG_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - reg_clr_log_mem_full_flag"]
    #[inline(always)]
    pub fn clr_log_mem_full_flag(&mut self) -> CLR_LOG_MEM_FULL_FLAG_W<1> {
        CLR_LOG_MEM_FULL_FLAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ASSIST_DEBUG_LOG_MEM_FULL_FLAG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [log_mem_full_flag](index.html) module"]
pub struct LOG_MEM_FULL_FLAG_SPEC;
impl crate::RegisterSpec for LOG_MEM_FULL_FLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [log_mem_full_flag::R](R) reader structure"]
impl crate::Readable for LOG_MEM_FULL_FLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [log_mem_full_flag::W](W) writer structure"]
impl crate::Writable for LOG_MEM_FULL_FLAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LOG_MEM_FULL_FLAG to value 0"]
impl crate::Resettable for LOG_MEM_FULL_FLAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
