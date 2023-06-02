#[doc = "Register `CLK_OUT_EN` reader"]
pub struct R(crate::R<CLK_OUT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_OUT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_OUT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_OUT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_OUT_EN` writer"]
pub struct W(crate::W<CLK_OUT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_OUT_EN_SPEC>;
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
impl From<crate::W<CLK_OUT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_OUT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK20_OEN` reader - "]
pub type CLK20_OEN_R = crate::BitReader;
#[doc = "Field `CLK20_OEN` writer - "]
pub type CLK20_OEN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_OUT_EN_SPEC, O>;
#[doc = "Field `CLK22_OEN` reader - "]
pub type CLK22_OEN_R = crate::BitReader;
#[doc = "Field `CLK22_OEN` writer - "]
pub type CLK22_OEN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_OUT_EN_SPEC, O>;
#[doc = "Field `CLK44_OEN` reader - "]
pub type CLK44_OEN_R = crate::BitReader;
#[doc = "Field `CLK44_OEN` writer - "]
pub type CLK44_OEN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_OUT_EN_SPEC, O>;
#[doc = "Field `CLK_BB_OEN` reader - "]
pub type CLK_BB_OEN_R = crate::BitReader;
#[doc = "Field `CLK_BB_OEN` writer - "]
pub type CLK_BB_OEN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_OUT_EN_SPEC, O>;
#[doc = "Field `CLK80_OEN` reader - "]
pub type CLK80_OEN_R = crate::BitReader;
#[doc = "Field `CLK80_OEN` writer - "]
pub type CLK80_OEN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_OUT_EN_SPEC, O>;
#[doc = "Field `CLK160_OEN` reader - "]
pub type CLK160_OEN_R = crate::BitReader;
#[doc = "Field `CLK160_OEN` writer - "]
pub type CLK160_OEN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_OUT_EN_SPEC, O>;
#[doc = "Field `CLK_320M_OEN` reader - "]
pub type CLK_320M_OEN_R = crate::BitReader;
#[doc = "Field `CLK_320M_OEN` writer - "]
pub type CLK_320M_OEN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_OUT_EN_SPEC, O>;
#[doc = "Field `CLK_ADC_INF_OEN` reader - "]
pub type CLK_ADC_INF_OEN_R = crate::BitReader;
#[doc = "Field `CLK_ADC_INF_OEN` writer - "]
pub type CLK_ADC_INF_OEN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_OUT_EN_SPEC, O>;
#[doc = "Field `CLK_DAC_CPU_OEN` reader - "]
pub type CLK_DAC_CPU_OEN_R = crate::BitReader;
#[doc = "Field `CLK_DAC_CPU_OEN` writer - "]
pub type CLK_DAC_CPU_OEN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_OUT_EN_SPEC, O>;
#[doc = "Field `CLK40X_BB_OEN` reader - "]
pub type CLK40X_BB_OEN_R = crate::BitReader;
#[doc = "Field `CLK40X_BB_OEN` writer - "]
pub type CLK40X_BB_OEN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_OUT_EN_SPEC, O>;
#[doc = "Field `CLK_XTAL_OEN` reader - "]
pub type CLK_XTAL_OEN_R = crate::BitReader;
#[doc = "Field `CLK_XTAL_OEN` writer - "]
pub type CLK_XTAL_OEN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_OUT_EN_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clk20_oen(&self) -> CLK20_OEN_R {
        CLK20_OEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clk22_oen(&self) -> CLK22_OEN_R {
        CLK22_OEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clk44_oen(&self) -> CLK44_OEN_R {
        CLK44_OEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clk_bb_oen(&self) -> CLK_BB_OEN_R {
        CLK_BB_OEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn clk80_oen(&self) -> CLK80_OEN_R {
        CLK80_OEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn clk160_oen(&self) -> CLK160_OEN_R {
        CLK160_OEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn clk_320m_oen(&self) -> CLK_320M_OEN_R {
        CLK_320M_OEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clk_adc_inf_oen(&self) -> CLK_ADC_INF_OEN_R {
        CLK_ADC_INF_OEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clk_dac_cpu_oen(&self) -> CLK_DAC_CPU_OEN_R {
        CLK_DAC_CPU_OEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clk40x_bb_oen(&self) -> CLK40X_BB_OEN_R {
        CLK40X_BB_OEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn clk_xtal_oen(&self) -> CLK_XTAL_OEN_R {
        CLK_XTAL_OEN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_OUT_EN")
            .field("clk20_oen", &format_args!("{}", self.clk20_oen().bit()))
            .field("clk22_oen", &format_args!("{}", self.clk22_oen().bit()))
            .field("clk44_oen", &format_args!("{}", self.clk44_oen().bit()))
            .field("clk_bb_oen", &format_args!("{}", self.clk_bb_oen().bit()))
            .field("clk80_oen", &format_args!("{}", self.clk80_oen().bit()))
            .field("clk160_oen", &format_args!("{}", self.clk160_oen().bit()))
            .field(
                "clk_320m_oen",
                &format_args!("{}", self.clk_320m_oen().bit()),
            )
            .field(
                "clk_adc_inf_oen",
                &format_args!("{}", self.clk_adc_inf_oen().bit()),
            )
            .field(
                "clk_dac_cpu_oen",
                &format_args!("{}", self.clk_dac_cpu_oen().bit()),
            )
            .field(
                "clk40x_bb_oen",
                &format_args!("{}", self.clk40x_bb_oen().bit()),
            )
            .field(
                "clk_xtal_oen",
                &format_args!("{}", self.clk_xtal_oen().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLK_OUT_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn clk20_oen(&mut self) -> CLK20_OEN_W<0> {
        CLK20_OEN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn clk22_oen(&mut self) -> CLK22_OEN_W<1> {
        CLK22_OEN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn clk44_oen(&mut self) -> CLK44_OEN_W<2> {
        CLK44_OEN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn clk_bb_oen(&mut self) -> CLK_BB_OEN_W<3> {
        CLK_BB_OEN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn clk80_oen(&mut self) -> CLK80_OEN_W<4> {
        CLK80_OEN_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn clk160_oen(&mut self) -> CLK160_OEN_W<5> {
        CLK160_OEN_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn clk_320m_oen(&mut self) -> CLK_320M_OEN_W<6> {
        CLK_320M_OEN_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn clk_adc_inf_oen(&mut self) -> CLK_ADC_INF_OEN_W<7> {
        CLK_ADC_INF_OEN_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn clk_dac_cpu_oen(&mut self) -> CLK_DAC_CPU_OEN_W<8> {
        CLK_DAC_CPU_OEN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn clk40x_bb_oen(&mut self) -> CLK40X_BB_OEN_W<9> {
        CLK40X_BB_OEN_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn clk_xtal_oen(&mut self) -> CLK_XTAL_OEN_W<10> {
        CLK_XTAL_OEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_out_en](index.html) module"]
pub struct CLK_OUT_EN_SPEC;
impl crate::RegisterSpec for CLK_OUT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_out_en::R](R) reader structure"]
impl crate::Readable for CLK_OUT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_out_en::W](W) writer structure"]
impl crate::Writable for CLK_OUT_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_OUT_EN to value 0x07ff"]
impl crate::Resettable for CLK_OUT_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0x07ff;
}
