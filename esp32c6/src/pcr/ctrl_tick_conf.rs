#[doc = "Register `CTRL_TICK_CONF` reader"]
pub struct R(crate::R<CTRL_TICK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_TICK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_TICK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_TICK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL_TICK_CONF` writer"]
pub struct W(crate::W<CTRL_TICK_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_TICK_CONF_SPEC>;
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
impl From<crate::W<CTRL_TICK_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_TICK_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XTAL_TICK_NUM` reader - ******* Description ***********"]
pub type XTAL_TICK_NUM_R = crate::FieldReader;
#[doc = "Field `XTAL_TICK_NUM` writer - ******* Description ***********"]
pub type XTAL_TICK_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, CTRL_TICK_CONF_SPEC, 8, O>;
#[doc = "Field `FOSC_TICK_NUM` reader - ******* Description ***********"]
pub type FOSC_TICK_NUM_R = crate::FieldReader;
#[doc = "Field `FOSC_TICK_NUM` writer - ******* Description ***********"]
pub type FOSC_TICK_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, CTRL_TICK_CONF_SPEC, 8, O>;
#[doc = "Field `TICK_ENABLE` reader - ******* Description ***********"]
pub type TICK_ENABLE_R = crate::BitReader;
#[doc = "Field `TICK_ENABLE` writer - ******* Description ***********"]
pub type TICK_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_TICK_CONF_SPEC, O>;
#[doc = "Field `RST_TICK_CNT` reader - ******* Description ***********"]
pub type RST_TICK_CNT_R = crate::BitReader;
#[doc = "Field `RST_TICK_CNT` writer - ******* Description ***********"]
pub type RST_TICK_CNT_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_TICK_CONF_SPEC, O>;
impl R {
    #[doc = "Bits 0:7 - ******* Description ***********"]
    #[inline(always)]
    pub fn xtal_tick_num(&self) -> XTAL_TICK_NUM_R {
        XTAL_TICK_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ******* Description ***********"]
    #[inline(always)]
    pub fn fosc_tick_num(&self) -> FOSC_TICK_NUM_R {
        FOSC_TICK_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - ******* Description ***********"]
    #[inline(always)]
    pub fn tick_enable(&self) -> TICK_ENABLE_R {
        TICK_ENABLE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ******* Description ***********"]
    #[inline(always)]
    pub fn rst_tick_cnt(&self) -> RST_TICK_CNT_R {
        RST_TICK_CNT_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL_TICK_CONF")
            .field(
                "xtal_tick_num",
                &format_args!("{}", self.xtal_tick_num().bits()),
            )
            .field(
                "fosc_tick_num",
                &format_args!("{}", self.fosc_tick_num().bits()),
            )
            .field("tick_enable", &format_args!("{}", self.tick_enable().bit()))
            .field(
                "rst_tick_cnt",
                &format_args!("{}", self.rst_tick_cnt().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CTRL_TICK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - ******* Description ***********"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_tick_num(&mut self) -> XTAL_TICK_NUM_W<0> {
        XTAL_TICK_NUM_W::new(self)
    }
    #[doc = "Bits 8:15 - ******* Description ***********"]
    #[inline(always)]
    #[must_use]
    pub fn fosc_tick_num(&mut self) -> FOSC_TICK_NUM_W<8> {
        FOSC_TICK_NUM_W::new(self)
    }
    #[doc = "Bit 16 - ******* Description ***********"]
    #[inline(always)]
    #[must_use]
    pub fn tick_enable(&mut self) -> TICK_ENABLE_W<16> {
        TICK_ENABLE_W::new(self)
    }
    #[doc = "Bit 17 - ******* Description ***********"]
    #[inline(always)]
    #[must_use]
    pub fn rst_tick_cnt(&mut self) -> RST_TICK_CNT_W<17> {
        RST_TICK_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TICK configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_tick_conf](index.html) module"]
pub struct CTRL_TICK_CONF_SPEC;
impl crate::RegisterSpec for CTRL_TICK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl_tick_conf::R](R) reader structure"]
impl crate::Readable for CTRL_TICK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl_tick_conf::W](W) writer structure"]
impl crate::Writable for CTRL_TICK_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL_TICK_CONF to value 0x0001_0727"]
impl crate::Resettable for CTRL_TICK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0727;
}
