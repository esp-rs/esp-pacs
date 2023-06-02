#[doc = "Register `TICK_CONF` reader"]
pub struct R(crate::R<TICK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TICK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TICK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TICK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TICK_CONF` writer"]
pub struct W(crate::W<TICK_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TICK_CONF_SPEC>;
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
impl From<crate::W<TICK_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TICK_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XTAL_TICK_NUM` reader - "]
pub type XTAL_TICK_NUM_R = crate::FieldReader;
#[doc = "Field `XTAL_TICK_NUM` writer - "]
pub type XTAL_TICK_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, TICK_CONF_SPEC, 8, O>;
#[doc = "Field `CK8M_TICK_NUM` reader - "]
pub type CK8M_TICK_NUM_R = crate::FieldReader;
#[doc = "Field `CK8M_TICK_NUM` writer - "]
pub type CK8M_TICK_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, TICK_CONF_SPEC, 8, O>;
#[doc = "Field `TICK_ENABLE` reader - "]
pub type TICK_ENABLE_R = crate::BitReader;
#[doc = "Field `TICK_ENABLE` writer - "]
pub type TICK_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, TICK_CONF_SPEC, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn xtal_tick_num(&self) -> XTAL_TICK_NUM_R {
        XTAL_TICK_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn ck8m_tick_num(&self) -> CK8M_TICK_NUM_R {
        CK8M_TICK_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tick_enable(&self) -> TICK_ENABLE_R {
        TICK_ENABLE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TICK_CONF")
            .field(
                "xtal_tick_num",
                &format_args!("{}", self.xtal_tick_num().bits()),
            )
            .field(
                "ck8m_tick_num",
                &format_args!("{}", self.ck8m_tick_num().bits()),
            )
            .field("tick_enable", &format_args!("{}", self.tick_enable().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TICK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_tick_num(&mut self) -> XTAL_TICK_NUM_W<0> {
        XTAL_TICK_NUM_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn ck8m_tick_num(&mut self) -> CK8M_TICK_NUM_W<8> {
        CK8M_TICK_NUM_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn tick_enable(&mut self) -> TICK_ENABLE_W<16> {
        TICK_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tick_conf](index.html) module"]
pub struct TICK_CONF_SPEC;
impl crate::RegisterSpec for TICK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tick_conf::R](R) reader structure"]
impl crate::Readable for TICK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tick_conf::W](W) writer structure"]
impl crate::Writable for TICK_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TICK_CONF to value 0x0001_0727"]
impl crate::Resettable for TICK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0727;
}
