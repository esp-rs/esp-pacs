#[doc = "Register `FH2_CFG0` reader"]
pub struct R(crate::R<FH2_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FH2_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FH2_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FH2_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FH2_CFG0` writer"]
pub struct W(crate::W<FH2_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FH2_CFG0_SPEC>;
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
impl From<crate::W<FH2_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FH2_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FH2_SW_CBC` reader - "]
pub type FH2_SW_CBC_R = crate::BitReader<bool>;
#[doc = "Field `FH2_SW_CBC` writer - "]
pub type FH2_SW_CBC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FH2_CFG0_SPEC, bool, O>;
#[doc = "Field `FH2_F2_CBC` reader - "]
pub type FH2_F2_CBC_R = crate::BitReader<bool>;
#[doc = "Field `FH2_F2_CBC` writer - "]
pub type FH2_F2_CBC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FH2_CFG0_SPEC, bool, O>;
#[doc = "Field `FH2_F1_CBC` reader - "]
pub type FH2_F1_CBC_R = crate::BitReader<bool>;
#[doc = "Field `FH2_F1_CBC` writer - "]
pub type FH2_F1_CBC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FH2_CFG0_SPEC, bool, O>;
#[doc = "Field `FH2_F0_CBC` reader - "]
pub type FH2_F0_CBC_R = crate::BitReader<bool>;
#[doc = "Field `FH2_F0_CBC` writer - "]
pub type FH2_F0_CBC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FH2_CFG0_SPEC, bool, O>;
#[doc = "Field `FH2_SW_OST` reader - "]
pub type FH2_SW_OST_R = crate::BitReader<bool>;
#[doc = "Field `FH2_SW_OST` writer - "]
pub type FH2_SW_OST_W<'a, const O: u8> = crate::BitWriter<'a, u32, FH2_CFG0_SPEC, bool, O>;
#[doc = "Field `FH2_F2_OST` reader - "]
pub type FH2_F2_OST_R = crate::BitReader<bool>;
#[doc = "Field `FH2_F2_OST` writer - "]
pub type FH2_F2_OST_W<'a, const O: u8> = crate::BitWriter<'a, u32, FH2_CFG0_SPEC, bool, O>;
#[doc = "Field `FH2_F1_OST` reader - "]
pub type FH2_F1_OST_R = crate::BitReader<bool>;
#[doc = "Field `FH2_F1_OST` writer - "]
pub type FH2_F1_OST_W<'a, const O: u8> = crate::BitWriter<'a, u32, FH2_CFG0_SPEC, bool, O>;
#[doc = "Field `FH2_F0_OST` reader - "]
pub type FH2_F0_OST_R = crate::BitReader<bool>;
#[doc = "Field `FH2_F0_OST` writer - "]
pub type FH2_F0_OST_W<'a, const O: u8> = crate::BitWriter<'a, u32, FH2_CFG0_SPEC, bool, O>;
#[doc = "Field `FH2_A_CBC_D` reader - "]
pub type FH2_A_CBC_D_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FH2_A_CBC_D` writer - "]
pub type FH2_A_CBC_D_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FH2_CFG0_SPEC, u8, u8, 2, O>;
#[doc = "Field `FH2_A_CBC_U` reader - "]
pub type FH2_A_CBC_U_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FH2_A_CBC_U` writer - "]
pub type FH2_A_CBC_U_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FH2_CFG0_SPEC, u8, u8, 2, O>;
#[doc = "Field `FH2_A_OST_D` reader - "]
pub type FH2_A_OST_D_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FH2_A_OST_D` writer - "]
pub type FH2_A_OST_D_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FH2_CFG0_SPEC, u8, u8, 2, O>;
#[doc = "Field `FH2_A_OST_U` reader - "]
pub type FH2_A_OST_U_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FH2_A_OST_U` writer - "]
pub type FH2_A_OST_U_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FH2_CFG0_SPEC, u8, u8, 2, O>;
#[doc = "Field `FH2_B_CBC_D` reader - "]
pub type FH2_B_CBC_D_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FH2_B_CBC_D` writer - "]
pub type FH2_B_CBC_D_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FH2_CFG0_SPEC, u8, u8, 2, O>;
#[doc = "Field `FH2_B_CBC_U` reader - "]
pub type FH2_B_CBC_U_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FH2_B_CBC_U` writer - "]
pub type FH2_B_CBC_U_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FH2_CFG0_SPEC, u8, u8, 2, O>;
#[doc = "Field `FH2_B_OST_D` reader - "]
pub type FH2_B_OST_D_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FH2_B_OST_D` writer - "]
pub type FH2_B_OST_D_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FH2_CFG0_SPEC, u8, u8, 2, O>;
#[doc = "Field `FH2_B_OST_U` reader - "]
pub type FH2_B_OST_U_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FH2_B_OST_U` writer - "]
pub type FH2_B_OST_U_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FH2_CFG0_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fh2_sw_cbc(&self) -> FH2_SW_CBC_R {
        FH2_SW_CBC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn fh2_f2_cbc(&self) -> FH2_F2_CBC_R {
        FH2_F2_CBC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn fh2_f1_cbc(&self) -> FH2_F1_CBC_R {
        FH2_F1_CBC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fh2_f0_cbc(&self) -> FH2_F0_CBC_R {
        FH2_F0_CBC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fh2_sw_ost(&self) -> FH2_SW_OST_R {
        FH2_SW_OST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn fh2_f2_ost(&self) -> FH2_F2_OST_R {
        FH2_F2_OST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn fh2_f1_ost(&self) -> FH2_F1_OST_R {
        FH2_F1_OST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn fh2_f0_ost(&self) -> FH2_F0_OST_R {
        FH2_F0_OST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn fh2_a_cbc_d(&self) -> FH2_A_CBC_D_R {
        FH2_A_CBC_D_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn fh2_a_cbc_u(&self) -> FH2_A_CBC_U_R {
        FH2_A_CBC_U_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn fh2_a_ost_d(&self) -> FH2_A_OST_D_R {
        FH2_A_OST_D_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn fh2_a_ost_u(&self) -> FH2_A_OST_U_R {
        FH2_A_OST_U_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn fh2_b_cbc_d(&self) -> FH2_B_CBC_D_R {
        FH2_B_CBC_D_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn fh2_b_cbc_u(&self) -> FH2_B_CBC_U_R {
        FH2_B_CBC_U_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn fh2_b_ost_d(&self) -> FH2_B_OST_D_R {
        FH2_B_OST_D_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn fh2_b_ost_u(&self) -> FH2_B_OST_U_R {
        FH2_B_OST_U_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fh2_sw_cbc(&mut self) -> FH2_SW_CBC_W<0> {
        FH2_SW_CBC_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn fh2_f2_cbc(&mut self) -> FH2_F2_CBC_W<1> {
        FH2_F2_CBC_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn fh2_f1_cbc(&mut self) -> FH2_F1_CBC_W<2> {
        FH2_F1_CBC_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fh2_f0_cbc(&mut self) -> FH2_F0_CBC_W<3> {
        FH2_F0_CBC_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fh2_sw_ost(&mut self) -> FH2_SW_OST_W<4> {
        FH2_SW_OST_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn fh2_f2_ost(&mut self) -> FH2_F2_OST_W<5> {
        FH2_F2_OST_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn fh2_f1_ost(&mut self) -> FH2_F1_OST_W<6> {
        FH2_F1_OST_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn fh2_f0_ost(&mut self) -> FH2_F0_OST_W<7> {
        FH2_F0_OST_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn fh2_a_cbc_d(&mut self) -> FH2_A_CBC_D_W<8> {
        FH2_A_CBC_D_W::new(self)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn fh2_a_cbc_u(&mut self) -> FH2_A_CBC_U_W<10> {
        FH2_A_CBC_U_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn fh2_a_ost_d(&mut self) -> FH2_A_OST_D_W<12> {
        FH2_A_OST_D_W::new(self)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn fh2_a_ost_u(&mut self) -> FH2_A_OST_U_W<14> {
        FH2_A_OST_U_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn fh2_b_cbc_d(&mut self) -> FH2_B_CBC_D_W<16> {
        FH2_B_CBC_D_W::new(self)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn fh2_b_cbc_u(&mut self) -> FH2_B_CBC_U_W<18> {
        FH2_B_CBC_U_W::new(self)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn fh2_b_ost_d(&mut self) -> FH2_B_OST_D_W<20> {
        FH2_B_OST_D_W::new(self)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn fh2_b_ost_u(&mut self) -> FH2_B_OST_U_W<22> {
        FH2_B_OST_U_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fh2_cfg0](index.html) module"]
pub struct FH2_CFG0_SPEC;
impl crate::RegisterSpec for FH2_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fh2_cfg0::R](R) reader structure"]
impl crate::Readable for FH2_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fh2_cfg0::W](W) writer structure"]
impl crate::Writable for FH2_CFG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FH2_CFG0 to value 0"]
impl crate::Resettable for FH2_CFG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
