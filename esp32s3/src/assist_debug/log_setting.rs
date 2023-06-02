#[doc = "Register `LOG_SETTING` reader"]
pub struct R(crate::R<LOG_SETTING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOG_SETTING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOG_SETTING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOG_SETTING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOG_SETTING` writer"]
pub struct W(crate::W<LOG_SETTING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOG_SETTING_SPEC>;
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
impl From<crate::W<LOG_SETTING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOG_SETTING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOG_ENA` reader - bus moniter enable: \\[0\\]Core1,\\[1\\]core1,\\[2\\]dma"]
pub type LOG_ENA_R = crate::FieldReader;
#[doc = "Field `LOG_ENA` writer - bus moniter enable: \\[0\\]Core1,\\[1\\]core1,\\[2\\]dma"]
pub type LOG_ENA_W<'a, const O: u8> = crate::FieldWriter<'a, LOG_SETTING_SPEC, 3, O>;
#[doc = "Field `LOG_MODE` reader - check_mode:0:write,1:word,2:halword,3:byte,4:doubleword,5:4word"]
pub type LOG_MODE_R = crate::FieldReader;
#[doc = "Field `LOG_MODE` writer - check_mode:0:write,1:word,2:halword,3:byte,4:doubleword,5:4word"]
pub type LOG_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, LOG_SETTING_SPEC, 3, O>;
#[doc = "Field `LOG_MEM_LOOP_ENABLE` reader - mem_loop enable,1 means that loop write"]
pub type LOG_MEM_LOOP_ENABLE_R = crate::BitReader;
#[doc = "Field `LOG_MEM_LOOP_ENABLE` writer - mem_loop enable,1 means that loop write"]
pub type LOG_MEM_LOOP_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, LOG_SETTING_SPEC, O>;
impl R {
    #[doc = "Bits 0:2 - bus moniter enable: \\[0\\]Core1,\\[1\\]core1,\\[2\\]dma"]
    #[inline(always)]
    pub fn log_ena(&self) -> LOG_ENA_R {
        LOG_ENA_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - check_mode:0:write,1:word,2:halword,3:byte,4:doubleword,5:4word"]
    #[inline(always)]
    pub fn log_mode(&self) -> LOG_MODE_R {
        LOG_MODE_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - mem_loop enable,1 means that loop write"]
    #[inline(always)]
    pub fn log_mem_loop_enable(&self) -> LOG_MEM_LOOP_ENABLE_R {
        LOG_MEM_LOOP_ENABLE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOG_SETTING")
            .field("log_ena", &format_args!("{}", self.log_ena().bits()))
            .field("log_mode", &format_args!("{}", self.log_mode().bits()))
            .field(
                "log_mem_loop_enable",
                &format_args!("{}", self.log_mem_loop_enable().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LOG_SETTING_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2 - bus moniter enable: \\[0\\]Core1,\\[1\\]core1,\\[2\\]dma"]
    #[inline(always)]
    #[must_use]
    pub fn log_ena(&mut self) -> LOG_ENA_W<0> {
        LOG_ENA_W::new(self)
    }
    #[doc = "Bits 3:5 - check_mode:0:write,1:word,2:halword,3:byte,4:doubleword,5:4word"]
    #[inline(always)]
    #[must_use]
    pub fn log_mode(&mut self) -> LOG_MODE_W<3> {
        LOG_MODE_W::new(self)
    }
    #[doc = "Bit 6 - mem_loop enable,1 means that loop write"]
    #[inline(always)]
    #[must_use]
    pub fn log_mem_loop_enable(&mut self) -> LOG_MEM_LOOP_ENABLE_W<6> {
        LOG_MEM_LOOP_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "log set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [log_setting](index.html) module"]
pub struct LOG_SETTING_SPEC;
impl crate::RegisterSpec for LOG_SETTING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [log_setting::R](R) reader structure"]
impl crate::Readable for LOG_SETTING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [log_setting::W](W) writer structure"]
impl crate::Writable for LOG_SETTING_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOG_SETTING to value 0x40"]
impl crate::Resettable for LOG_SETTING_SPEC {
    const RESET_VALUE: Self::Ux = 0x40;
}
