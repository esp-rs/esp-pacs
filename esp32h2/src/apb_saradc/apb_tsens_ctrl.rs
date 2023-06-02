#[doc = "Register `APB_TSENS_CTRL` reader"]
pub struct R(crate::R<APB_TSENS_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_TSENS_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_TSENS_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_TSENS_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_TSENS_CTRL` writer"]
pub struct W(crate::W<APB_TSENS_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_TSENS_CTRL_SPEC>;
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
impl From<crate::W<APB_TSENS_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_TSENS_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSENS_OUT` reader - temperature sensor data out"]
pub type TSENS_OUT_R = crate::FieldReader;
#[doc = "Field `TSENS_IN_INV` reader - invert temperature sensor data"]
pub type TSENS_IN_INV_R = crate::BitReader;
#[doc = "Field `TSENS_IN_INV` writer - invert temperature sensor data"]
pub type TSENS_IN_INV_W<'a, const O: u8> = crate::BitWriter<'a, APB_TSENS_CTRL_SPEC, O>;
#[doc = "Field `TSENS_CLK_DIV` reader - temperature sensor clock divider"]
pub type TSENS_CLK_DIV_R = crate::FieldReader;
#[doc = "Field `TSENS_CLK_DIV` writer - temperature sensor clock divider"]
pub type TSENS_CLK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, APB_TSENS_CTRL_SPEC, 8, O>;
#[doc = "Field `TSENS_PU` reader - temperature sensor power up"]
pub type TSENS_PU_R = crate::BitReader;
#[doc = "Field `TSENS_PU` writer - temperature sensor power up"]
pub type TSENS_PU_W<'a, const O: u8> = crate::BitWriter<'a, APB_TSENS_CTRL_SPEC, O>;
impl R {
    #[doc = "Bits 0:7 - temperature sensor data out"]
    #[inline(always)]
    pub fn tsens_out(&self) -> TSENS_OUT_R {
        TSENS_OUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 13 - invert temperature sensor data"]
    #[inline(always)]
    pub fn tsens_in_inv(&self) -> TSENS_IN_INV_R {
        TSENS_IN_INV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:21 - temperature sensor clock divider"]
    #[inline(always)]
    pub fn tsens_clk_div(&self) -> TSENS_CLK_DIV_R {
        TSENS_CLK_DIV_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bit 22 - temperature sensor power up"]
    #[inline(always)]
    pub fn tsens_pu(&self) -> TSENS_PU_R {
        TSENS_PU_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_TSENS_CTRL")
            .field("tsens_out", &format_args!("{}", self.tsens_out().bits()))
            .field(
                "tsens_in_inv",
                &format_args!("{}", self.tsens_in_inv().bit()),
            )
            .field(
                "tsens_clk_div",
                &format_args!("{}", self.tsens_clk_div().bits()),
            )
            .field("tsens_pu", &format_args!("{}", self.tsens_pu().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APB_TSENS_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 13 - invert temperature sensor data"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_in_inv(&mut self) -> TSENS_IN_INV_W<13> {
        TSENS_IN_INV_W::new(self)
    }
    #[doc = "Bits 14:21 - temperature sensor clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_clk_div(&mut self) -> TSENS_CLK_DIV_W<14> {
        TSENS_CLK_DIV_W::new(self)
    }
    #[doc = "Bit 22 - temperature sensor power up"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_pu(&mut self) -> TSENS_PU_W<22> {
        TSENS_PU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "digital tsens configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_tsens_ctrl](index.html) module"]
pub struct APB_TSENS_CTRL_SPEC;
impl crate::RegisterSpec for APB_TSENS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_tsens_ctrl::R](R) reader structure"]
impl crate::Readable for APB_TSENS_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_tsens_ctrl::W](W) writer structure"]
impl crate::Writable for APB_TSENS_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB_TSENS_CTRL to value 0x0001_8080"]
impl crate::Resettable for APB_TSENS_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_8080;
}
