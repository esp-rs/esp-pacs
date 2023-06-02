#[doc = "Register `CLK_CONF1_FORCE_ON` reader"]
pub struct R(crate::R<CLK_CONF1_FORCE_ON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CONF1_FORCE_ON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CONF1_FORCE_ON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CONF1_FORCE_ON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CONF1_FORCE_ON` writer"]
pub struct W(crate::W<CLK_CONF1_FORCE_ON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CONF1_FORCE_ON_SPEC>;
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
impl From<crate::W<CLK_CONF1_FORCE_ON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CONF1_FORCE_ON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_FE_FO` reader - ."]
pub type CLK_FE_FO_R = crate::BitReader;
#[doc = "Field `CLK_FE_FO` writer - ."]
pub type CLK_FE_FO_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF1_FORCE_ON_SPEC, O>;
#[doc = "Field `CLK_BT_FO` reader - ."]
pub type CLK_BT_FO_R = crate::BitReader;
#[doc = "Field `CLK_BT_FO` writer - ."]
pub type CLK_BT_FO_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF1_FORCE_ON_SPEC, O>;
impl R {
    #[doc = "Bit 16 - ."]
    #[inline(always)]
    pub fn clk_fe_fo(&self) -> CLK_FE_FO_R {
        CLK_FE_FO_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - ."]
    #[inline(always)]
    pub fn clk_bt_fo(&self) -> CLK_BT_FO_R {
        CLK_BT_FO_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_CONF1_FORCE_ON")
            .field("clk_fe_fo", &format_args!("{}", self.clk_fe_fo().bit()))
            .field("clk_bt_fo", &format_args!("{}", self.clk_bt_fo().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLK_CONF1_FORCE_ON_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 16 - ."]
    #[inline(always)]
    #[must_use]
    pub fn clk_fe_fo(&mut self) -> CLK_FE_FO_W<16> {
        CLK_FE_FO_W::new(self)
    }
    #[doc = "Bit 18 - ."]
    #[inline(always)]
    #[must_use]
    pub fn clk_bt_fo(&mut self) -> CLK_BT_FO_W<18> {
        CLK_BT_FO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_conf1_force_on](index.html) module"]
pub struct CLK_CONF1_FORCE_ON_SPEC;
impl crate::RegisterSpec for CLK_CONF1_FORCE_ON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_conf1_force_on::R](R) reader structure"]
impl crate::Readable for CLK_CONF1_FORCE_ON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_conf1_force_on::W](W) writer structure"]
impl crate::Writable for CLK_CONF1_FORCE_ON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_CONF1_FORCE_ON to value 0"]
impl crate::Resettable for CLK_CONF1_FORCE_ON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
