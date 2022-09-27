#[doc = "Register `DT1_CFG` reader"]
pub struct R(crate::R<DT1_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DT1_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DT1_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DT1_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DT1_CFG` writer"]
pub struct W(crate::W<DT1_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DT1_CFG_SPEC>;
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
impl From<crate::W<DT1_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DT1_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DT1_FED_UPMETHOD` reader - "]
pub type DT1_FED_UPMETHOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DT1_FED_UPMETHOD` writer - "]
pub type DT1_FED_UPMETHOD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DT1_CFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `DT1_RED_UPMETHOD` reader - "]
pub type DT1_RED_UPMETHOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DT1_RED_UPMETHOD` writer - "]
pub type DT1_RED_UPMETHOD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DT1_CFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `DT1_DEB_MODE` reader - "]
pub type DT1_DEB_MODE_R = crate::BitReader<bool>;
#[doc = "Field `DT1_DEB_MODE` writer - "]
pub type DT1_DEB_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DT1_CFG_SPEC, bool, O>;
#[doc = "Field `DT1_A_OUTSWAP` reader - "]
pub type DT1_A_OUTSWAP_R = crate::BitReader<bool>;
#[doc = "Field `DT1_A_OUTSWAP` writer - "]
pub type DT1_A_OUTSWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DT1_CFG_SPEC, bool, O>;
#[doc = "Field `DT1_B_OUTSWAP` reader - "]
pub type DT1_B_OUTSWAP_R = crate::BitReader<bool>;
#[doc = "Field `DT1_B_OUTSWAP` writer - "]
pub type DT1_B_OUTSWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DT1_CFG_SPEC, bool, O>;
#[doc = "Field `DT1_RED_INSEL` reader - "]
pub type DT1_RED_INSEL_R = crate::BitReader<bool>;
#[doc = "Field `DT1_RED_INSEL` writer - "]
pub type DT1_RED_INSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DT1_CFG_SPEC, bool, O>;
#[doc = "Field `DT1_FED_INSEL` reader - "]
pub type DT1_FED_INSEL_R = crate::BitReader<bool>;
#[doc = "Field `DT1_FED_INSEL` writer - "]
pub type DT1_FED_INSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DT1_CFG_SPEC, bool, O>;
#[doc = "Field `DT1_RED_OUTINVERT` reader - "]
pub type DT1_RED_OUTINVERT_R = crate::BitReader<bool>;
#[doc = "Field `DT1_RED_OUTINVERT` writer - "]
pub type DT1_RED_OUTINVERT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DT1_CFG_SPEC, bool, O>;
#[doc = "Field `DT1_FED_OUTINVERT` reader - "]
pub type DT1_FED_OUTINVERT_R = crate::BitReader<bool>;
#[doc = "Field `DT1_FED_OUTINVERT` writer - "]
pub type DT1_FED_OUTINVERT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DT1_CFG_SPEC, bool, O>;
#[doc = "Field `DT1_A_OUTBYPASS` reader - "]
pub type DT1_A_OUTBYPASS_R = crate::BitReader<bool>;
#[doc = "Field `DT1_A_OUTBYPASS` writer - "]
pub type DT1_A_OUTBYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DT1_CFG_SPEC, bool, O>;
#[doc = "Field `DT1_B_OUTBYPASS` reader - "]
pub type DT1_B_OUTBYPASS_R = crate::BitReader<bool>;
#[doc = "Field `DT1_B_OUTBYPASS` writer - "]
pub type DT1_B_OUTBYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DT1_CFG_SPEC, bool, O>;
#[doc = "Field `DT1_CLK_SEL` reader - "]
pub type DT1_CLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `DT1_CLK_SEL` writer - "]
pub type DT1_CLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DT1_CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn dt1_fed_upmethod(&self) -> DT1_FED_UPMETHOD_R {
        DT1_FED_UPMETHOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn dt1_red_upmethod(&self) -> DT1_RED_UPMETHOD_R {
        DT1_RED_UPMETHOD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dt1_deb_mode(&self) -> DT1_DEB_MODE_R {
        DT1_DEB_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dt1_a_outswap(&self) -> DT1_A_OUTSWAP_R {
        DT1_A_OUTSWAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dt1_b_outswap(&self) -> DT1_B_OUTSWAP_R {
        DT1_B_OUTSWAP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dt1_red_insel(&self) -> DT1_RED_INSEL_R {
        DT1_RED_INSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dt1_fed_insel(&self) -> DT1_FED_INSEL_R {
        DT1_FED_INSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dt1_red_outinvert(&self) -> DT1_RED_OUTINVERT_R {
        DT1_RED_OUTINVERT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dt1_fed_outinvert(&self) -> DT1_FED_OUTINVERT_R {
        DT1_FED_OUTINVERT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dt1_a_outbypass(&self) -> DT1_A_OUTBYPASS_R {
        DT1_A_OUTBYPASS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn dt1_b_outbypass(&self) -> DT1_B_OUTBYPASS_R {
        DT1_B_OUTBYPASS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn dt1_clk_sel(&self) -> DT1_CLK_SEL_R {
        DT1_CLK_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn dt1_fed_upmethod(&mut self) -> DT1_FED_UPMETHOD_W<0> {
        DT1_FED_UPMETHOD_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn dt1_red_upmethod(&mut self) -> DT1_RED_UPMETHOD_W<4> {
        DT1_RED_UPMETHOD_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dt1_deb_mode(&mut self) -> DT1_DEB_MODE_W<8> {
        DT1_DEB_MODE_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dt1_a_outswap(&mut self) -> DT1_A_OUTSWAP_W<9> {
        DT1_A_OUTSWAP_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dt1_b_outswap(&mut self) -> DT1_B_OUTSWAP_W<10> {
        DT1_B_OUTSWAP_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dt1_red_insel(&mut self) -> DT1_RED_INSEL_W<11> {
        DT1_RED_INSEL_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dt1_fed_insel(&mut self) -> DT1_FED_INSEL_W<12> {
        DT1_FED_INSEL_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dt1_red_outinvert(&mut self) -> DT1_RED_OUTINVERT_W<13> {
        DT1_RED_OUTINVERT_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dt1_fed_outinvert(&mut self) -> DT1_FED_OUTINVERT_W<14> {
        DT1_FED_OUTINVERT_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dt1_a_outbypass(&mut self) -> DT1_A_OUTBYPASS_W<15> {
        DT1_A_OUTBYPASS_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn dt1_b_outbypass(&mut self) -> DT1_B_OUTBYPASS_W<16> {
        DT1_B_OUTBYPASS_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn dt1_clk_sel(&mut self) -> DT1_CLK_SEL_W<17> {
        DT1_CLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dt1_cfg](index.html) module"]
pub struct DT1_CFG_SPEC;
impl crate::RegisterSpec for DT1_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dt1_cfg::R](R) reader structure"]
impl crate::Readable for DT1_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dt1_cfg::W](W) writer structure"]
impl crate::Writable for DT1_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DT1_CFG to value 0x0001_8000"]
impl crate::Resettable for DT1_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_8000
    }
}
