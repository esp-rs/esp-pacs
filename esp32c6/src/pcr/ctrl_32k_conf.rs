#[doc = "Register `CTRL_32K_CONF` reader"]
pub struct R(crate::R<CTRL_32K_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_32K_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_32K_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_32K_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL_32K_CONF` writer"]
pub struct W(crate::W<CTRL_32K_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_32K_CONF_SPEC>;
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
impl From<crate::W<CTRL_32K_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_32K_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `_32K_SEL` reader - This field indicates which one 32KHz clock will be used by MODEM_SYSTEM and timergroup. 0: OSC32K(default), 1: XTAL32K, 2/3: 32KHz from pad GPIO0."]
pub type _32K_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `_32K_SEL` writer - This field indicates which one 32KHz clock will be used by MODEM_SYSTEM and timergroup. 0: OSC32K(default), 1: XTAL32K, 2/3: 32KHz from pad GPIO0."]
pub type _32K_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL_32K_CONF_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - This field indicates which one 32KHz clock will be used by MODEM_SYSTEM and timergroup. 0: OSC32K(default), 1: XTAL32K, 2/3: 32KHz from pad GPIO0."]
    #[inline(always)]
    pub fn _32k_sel(&self) -> _32K_SEL_R {
        _32K_SEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - This field indicates which one 32KHz clock will be used by MODEM_SYSTEM and timergroup. 0: OSC32K(default), 1: XTAL32K, 2/3: 32KHz from pad GPIO0."]
    #[inline(always)]
    #[must_use]
    pub fn _32k_sel(&mut self) -> _32K_SEL_W<0> {
        _32K_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "32KHz clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_32k_conf](index.html) module"]
pub struct CTRL_32K_CONF_SPEC;
impl crate::RegisterSpec for CTRL_32K_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl_32k_conf::R](R) reader structure"]
impl crate::Readable for CTRL_32K_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl_32k_conf::W](W) writer structure"]
impl crate::Writable for CTRL_32K_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL_32K_CONF to value 0"]
impl crate::Resettable for CTRL_32K_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
