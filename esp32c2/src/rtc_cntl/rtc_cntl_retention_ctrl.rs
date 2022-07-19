#[doc = "Register `RTC_CNTL_RETENTION_CTRL` reader"]
pub struct R(crate::R<RTC_CNTL_RETENTION_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_CNTL_RETENTION_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_CNTL_RETENTION_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_CNTL_RETENTION_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_CNTL_RETENTION_CTRL` writer"]
pub struct W(crate::W<RTC_CNTL_RETENTION_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_CNTL_RETENTION_CTRL_SPEC>;
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
impl From<crate::W<RTC_CNTL_RETENTION_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_CNTL_RETENTION_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RETENTION_CLK_SEL` reader - Need add desc"]
pub type RETENTION_CLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `RETENTION_CLK_SEL` writer - Need add desc"]
pub type RETENTION_CLK_SEL_W<'a> =
    crate::BitWriter<'a, u32, RTC_CNTL_RETENTION_CTRL_SPEC, bool, 18>;
#[doc = "Field `RETENTION_DONE_WAIT` reader - Need add desc"]
pub type RETENTION_DONE_WAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RETENTION_DONE_WAIT` writer - Need add desc"]
pub type RETENTION_DONE_WAIT_W<'a> =
    crate::FieldWriter<'a, u32, RTC_CNTL_RETENTION_CTRL_SPEC, u8, u8, 3, 19>;
#[doc = "Field `RETENTION_CLKOFF_WAIT` reader - Need add desc"]
pub type RETENTION_CLKOFF_WAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RETENTION_CLKOFF_WAIT` writer - Need add desc"]
pub type RETENTION_CLKOFF_WAIT_W<'a> =
    crate::FieldWriter<'a, u32, RTC_CNTL_RETENTION_CTRL_SPEC, u8, u8, 4, 22>;
#[doc = "Field `RETENTION_EN` reader - Need add desc"]
pub type RETENTION_EN_R = crate::BitReader<bool>;
#[doc = "Field `RETENTION_EN` writer - Need add desc"]
pub type RETENTION_EN_W<'a> = crate::BitWriter<'a, u32, RTC_CNTL_RETENTION_CTRL_SPEC, bool, 26>;
#[doc = "Field `RETENTION_WAIT` reader - wait cycles for rention operation"]
pub type RETENTION_WAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RETENTION_WAIT` writer - wait cycles for rention operation"]
pub type RETENTION_WAIT_W<'a> =
    crate::FieldWriter<'a, u32, RTC_CNTL_RETENTION_CTRL_SPEC, u8, u8, 5, 27>;
impl R {
    #[doc = "Bit 18 - Need add desc"]
    #[inline(always)]
    pub fn retention_clk_sel(&self) -> RETENTION_CLK_SEL_R {
        RETENTION_CLK_SEL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21 - Need add desc"]
    #[inline(always)]
    pub fn retention_done_wait(&self) -> RETENTION_DONE_WAIT_R {
        RETENTION_DONE_WAIT_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:25 - Need add desc"]
    #[inline(always)]
    pub fn retention_clkoff_wait(&self) -> RETENTION_CLKOFF_WAIT_R {
        RETENTION_CLKOFF_WAIT_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 26 - Need add desc"]
    #[inline(always)]
    pub fn retention_en(&self) -> RETENTION_EN_R {
        RETENTION_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31 - wait cycles for rention operation"]
    #[inline(always)]
    pub fn retention_wait(&self) -> RETENTION_WAIT_R {
        RETENTION_WAIT_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 18 - Need add desc"]
    #[inline(always)]
    pub fn retention_clk_sel(&mut self) -> RETENTION_CLK_SEL_W {
        RETENTION_CLK_SEL_W::new(self)
    }
    #[doc = "Bits 19:21 - Need add desc"]
    #[inline(always)]
    pub fn retention_done_wait(&mut self) -> RETENTION_DONE_WAIT_W {
        RETENTION_DONE_WAIT_W::new(self)
    }
    #[doc = "Bits 22:25 - Need add desc"]
    #[inline(always)]
    pub fn retention_clkoff_wait(&mut self) -> RETENTION_CLKOFF_WAIT_W {
        RETENTION_CLKOFF_WAIT_W::new(self)
    }
    #[doc = "Bit 26 - Need add desc"]
    #[inline(always)]
    pub fn retention_en(&mut self) -> RETENTION_EN_W {
        RETENTION_EN_W::new(self)
    }
    #[doc = "Bits 27:31 - wait cycles for rention operation"]
    #[inline(always)]
    pub fn retention_wait(&mut self) -> RETENTION_WAIT_W {
        RETENTION_WAIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_retention_ctrl](index.html) module"]
pub struct RTC_CNTL_RETENTION_CTRL_SPEC;
impl crate::RegisterSpec for RTC_CNTL_RETENTION_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_cntl_retention_ctrl::R](R) reader structure"]
impl crate::Readable for RTC_CNTL_RETENTION_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_retention_ctrl::W](W) writer structure"]
impl crate::Writable for RTC_CNTL_RETENTION_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_CNTL_RETENTION_CTRL to value 0xa0d0_0000"]
impl crate::Resettable for RTC_CNTL_RETENTION_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa0d0_0000
    }
}
