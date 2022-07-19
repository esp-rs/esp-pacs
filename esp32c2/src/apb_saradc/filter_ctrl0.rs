#[doc = "Register `FILTER_CTRL0` reader"]
pub struct R(crate::R<FILTER_CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FILTER_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FILTER_CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FILTER_CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FILTER_CTRL0` writer"]
pub struct W(crate::W<FILTER_CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FILTER_CTRL0_SPEC>;
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
impl From<crate::W<FILTER_CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FILTER_CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FILTER_CHANNEL1` reader - Need add description"]
pub type FILTER_CHANNEL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FILTER_CHANNEL1` writer - Need add description"]
pub type FILTER_CHANNEL1_W<'a> = crate::FieldWriter<'a, u32, FILTER_CTRL0_SPEC, u8, u8, 4, 18>;
#[doc = "Field `FILTER_CHANNEL0` reader - apb_adc1_filter_factor"]
pub type FILTER_CHANNEL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FILTER_CHANNEL0` writer - apb_adc1_filter_factor"]
pub type FILTER_CHANNEL0_W<'a> = crate::FieldWriter<'a, u32, FILTER_CTRL0_SPEC, u8, u8, 4, 22>;
#[doc = "Field `FILTER_RESET` reader - enable apb_adc1_filter"]
pub type FILTER_RESET_R = crate::BitReader<bool>;
#[doc = "Field `FILTER_RESET` writer - enable apb_adc1_filter"]
pub type FILTER_RESET_W<'a> = crate::BitWriter<'a, u32, FILTER_CTRL0_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 18:21 - Need add description"]
    #[inline(always)]
    pub fn filter_channel1(&self) -> FILTER_CHANNEL1_R {
        FILTER_CHANNEL1_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:25 - apb_adc1_filter_factor"]
    #[inline(always)]
    pub fn filter_channel0(&self) -> FILTER_CHANNEL0_R {
        FILTER_CHANNEL0_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - enable apb_adc1_filter"]
    #[inline(always)]
    pub fn filter_reset(&self) -> FILTER_RESET_R {
        FILTER_RESET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 18:21 - Need add description"]
    #[inline(always)]
    pub fn filter_channel1(&mut self) -> FILTER_CHANNEL1_W {
        FILTER_CHANNEL1_W::new(self)
    }
    #[doc = "Bits 22:25 - apb_adc1_filter_factor"]
    #[inline(always)]
    pub fn filter_channel0(&mut self) -> FILTER_CHANNEL0_W {
        FILTER_CHANNEL0_W::new(self)
    }
    #[doc = "Bit 31 - enable apb_adc1_filter"]
    #[inline(always)]
    pub fn filter_reset(&mut self) -> FILTER_RESET_W {
        FILTER_RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filter_ctrl0](index.html) module"]
pub struct FILTER_CTRL0_SPEC;
impl crate::RegisterSpec for FILTER_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [filter_ctrl0::R](R) reader structure"]
impl crate::Readable for FILTER_CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [filter_ctrl0::W](W) writer structure"]
impl crate::Writable for FILTER_CTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FILTER_CTRL0 to value 0x0374_0000"]
impl crate::Resettable for FILTER_CTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0374_0000
    }
}
