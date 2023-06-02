#[doc = "Register `RETENTION_CTRL` reader"]
pub struct R(crate::R<RETENTION_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RETENTION_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RETENTION_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RETENTION_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RETENTION_CTRL` writer"]
pub struct W(crate::W<RETENTION_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RETENTION_CTRL_SPEC>;
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
impl From<crate::W<RETENTION_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RETENTION_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RETENTION_TAG_MODE` reader - No public"]
pub type RETENTION_TAG_MODE_R = crate::FieldReader;
#[doc = "Field `RETENTION_TAG_MODE` writer - No public"]
pub type RETENTION_TAG_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, RETENTION_CTRL_SPEC, 4, O>;
#[doc = "Field `RETENTION_TARGET` reader - congfigure retention target cpu and/or tag"]
pub type RETENTION_TARGET_R = crate::FieldReader;
#[doc = "Field `RETENTION_TARGET` writer - congfigure retention target cpu and/or tag"]
pub type RETENTION_TARGET_W<'a, const O: u8> = crate::FieldWriter<'a, RETENTION_CTRL_SPEC, 2, O>;
#[doc = "Field `RETENTION_CLK_SEL` reader - No public"]
pub type RETENTION_CLK_SEL_R = crate::BitReader;
#[doc = "Field `RETENTION_CLK_SEL` writer - No public"]
pub type RETENTION_CLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, RETENTION_CTRL_SPEC, O>;
#[doc = "Field `RETENTION_DONE_WAIT` reader - wait retention done cycle"]
pub type RETENTION_DONE_WAIT_R = crate::FieldReader;
#[doc = "Field `RETENTION_DONE_WAIT` writer - wait retention done cycle"]
pub type RETENTION_DONE_WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, RETENTION_CTRL_SPEC, 3, O>;
#[doc = "Field `RETENTION_CLKOFF_WAIT` reader - wait clk off cycle"]
pub type RETENTION_CLKOFF_WAIT_R = crate::FieldReader;
#[doc = "Field `RETENTION_CLKOFF_WAIT` writer - wait clk off cycle"]
pub type RETENTION_CLKOFF_WAIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, RETENTION_CTRL_SPEC, 4, O>;
#[doc = "Field `RETENTION_EN` reader - enable retention"]
pub type RETENTION_EN_R = crate::BitReader;
#[doc = "Field `RETENTION_EN` writer - enable retention"]
pub type RETENTION_EN_W<'a, const O: u8> = crate::BitWriter<'a, RETENTION_CTRL_SPEC, O>;
#[doc = "Field `RETENTION_WAIT` reader - wait cycles for rention operation"]
pub type RETENTION_WAIT_R = crate::FieldReader;
#[doc = "Field `RETENTION_WAIT` writer - wait cycles for rention operation"]
pub type RETENTION_WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, RETENTION_CTRL_SPEC, 7, O>;
impl R {
    #[doc = "Bits 10:13 - No public"]
    #[inline(always)]
    pub fn retention_tag_mode(&self) -> RETENTION_TAG_MODE_R {
        RETENTION_TAG_MODE_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:15 - congfigure retention target cpu and/or tag"]
    #[inline(always)]
    pub fn retention_target(&self) -> RETENTION_TARGET_R {
        RETENTION_TARGET_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - No public"]
    #[inline(always)]
    pub fn retention_clk_sel(&self) -> RETENTION_CLK_SEL_R {
        RETENTION_CLK_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - wait retention done cycle"]
    #[inline(always)]
    pub fn retention_done_wait(&self) -> RETENTION_DONE_WAIT_R {
        RETENTION_DONE_WAIT_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:23 - wait clk off cycle"]
    #[inline(always)]
    pub fn retention_clkoff_wait(&self) -> RETENTION_CLKOFF_WAIT_R {
        RETENTION_CLKOFF_WAIT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - enable retention"]
    #[inline(always)]
    pub fn retention_en(&self) -> RETENTION_EN_R {
        RETENTION_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - wait cycles for rention operation"]
    #[inline(always)]
    pub fn retention_wait(&self) -> RETENTION_WAIT_R {
        RETENTION_WAIT_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RETENTION_CTRL")
            .field(
                "retention_tag_mode",
                &format_args!("{}", self.retention_tag_mode().bits()),
            )
            .field(
                "retention_target",
                &format_args!("{}", self.retention_target().bits()),
            )
            .field(
                "retention_clk_sel",
                &format_args!("{}", self.retention_clk_sel().bit()),
            )
            .field(
                "retention_done_wait",
                &format_args!("{}", self.retention_done_wait().bits()),
            )
            .field(
                "retention_clkoff_wait",
                &format_args!("{}", self.retention_clkoff_wait().bits()),
            )
            .field(
                "retention_en",
                &format_args!("{}", self.retention_en().bit()),
            )
            .field(
                "retention_wait",
                &format_args!("{}", self.retention_wait().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RETENTION_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 10:13 - No public"]
    #[inline(always)]
    #[must_use]
    pub fn retention_tag_mode(&mut self) -> RETENTION_TAG_MODE_W<10> {
        RETENTION_TAG_MODE_W::new(self)
    }
    #[doc = "Bits 14:15 - congfigure retention target cpu and/or tag"]
    #[inline(always)]
    #[must_use]
    pub fn retention_target(&mut self) -> RETENTION_TARGET_W<14> {
        RETENTION_TARGET_W::new(self)
    }
    #[doc = "Bit 16 - No public"]
    #[inline(always)]
    #[must_use]
    pub fn retention_clk_sel(&mut self) -> RETENTION_CLK_SEL_W<16> {
        RETENTION_CLK_SEL_W::new(self)
    }
    #[doc = "Bits 17:19 - wait retention done cycle"]
    #[inline(always)]
    #[must_use]
    pub fn retention_done_wait(&mut self) -> RETENTION_DONE_WAIT_W<17> {
        RETENTION_DONE_WAIT_W::new(self)
    }
    #[doc = "Bits 20:23 - wait clk off cycle"]
    #[inline(always)]
    #[must_use]
    pub fn retention_clkoff_wait(&mut self) -> RETENTION_CLKOFF_WAIT_W<20> {
        RETENTION_CLKOFF_WAIT_W::new(self)
    }
    #[doc = "Bit 24 - enable retention"]
    #[inline(always)]
    #[must_use]
    pub fn retention_en(&mut self) -> RETENTION_EN_W<24> {
        RETENTION_EN_W::new(self)
    }
    #[doc = "Bits 25:31 - wait cycles for rention operation"]
    #[inline(always)]
    #[must_use]
    pub fn retention_wait(&mut self) -> RETENTION_WAIT_W<25> {
        RETENTION_WAIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure retention\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [retention_ctrl](index.html) module"]
pub struct RETENTION_CTRL_SPEC;
impl crate::RegisterSpec for RETENTION_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [retention_ctrl::R](R) reader structure"]
impl crate::Readable for RETENTION_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [retention_ctrl::W](W) writer structure"]
impl crate::Writable for RETENTION_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RETENTION_CTRL to value 0x2834_0000"]
impl crate::Resettable for RETENTION_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x2834_0000;
}
