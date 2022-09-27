#[doc = "Register `SYS_CONF` reader"]
pub struct R(crate::R<SYS_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_CONF` writer"]
pub struct W(crate::W<SYS_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_CONF_SPEC>;
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
impl From<crate::W<SYS_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APB_FIFO_MASK` reader - reg_apb_fifo_mask."]
pub type APB_FIFO_MASK_R = crate::BitReader<bool>;
#[doc = "Field `APB_FIFO_MASK` writer - reg_apb_fifo_mask."]
pub type APB_FIFO_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_CONF_SPEC, bool, O>;
#[doc = "Field `MEM_CLK_FORCE_ON` reader - reg_mem_clk_force_on."]
pub type MEM_CLK_FORCE_ON_R = crate::BitReader<bool>;
#[doc = "Field `MEM_CLK_FORCE_ON` writer - reg_mem_clk_force_on."]
pub type MEM_CLK_FORCE_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_CONF_SPEC, bool, O>;
#[doc = "Field `MEM_FORCE_PD` reader - reg_rmt_mem_force_pd."]
pub type MEM_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `MEM_FORCE_PD` writer - reg_rmt_mem_force_pd."]
pub type MEM_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_CONF_SPEC, bool, O>;
#[doc = "Field `MEM_FORCE_PU` reader - reg_rmt_mem_force_pu."]
pub type MEM_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `MEM_FORCE_PU` writer - reg_rmt_mem_force_pu."]
pub type MEM_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_CONF_SPEC, bool, O>;
#[doc = "Field `SCLK_DIV_NUM` reader - reg_rmt_sclk_div_num."]
pub type SCLK_DIV_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCLK_DIV_NUM` writer - reg_rmt_sclk_div_num."]
pub type SCLK_DIV_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYS_CONF_SPEC, u8, u8, 8, O>;
#[doc = "Field `SCLK_DIV_A` reader - reg_rmt_sclk_div_a."]
pub type SCLK_DIV_A_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCLK_DIV_A` writer - reg_rmt_sclk_div_a."]
pub type SCLK_DIV_A_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYS_CONF_SPEC, u8, u8, 6, O>;
#[doc = "Field `SCLK_DIV_B` reader - reg_rmt_sclk_div_b."]
pub type SCLK_DIV_B_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCLK_DIV_B` writer - reg_rmt_sclk_div_b."]
pub type SCLK_DIV_B_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYS_CONF_SPEC, u8, u8, 6, O>;
#[doc = "Field `SCLK_SEL` reader - reg_rmt_sclk_sel."]
pub type SCLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCLK_SEL` writer - reg_rmt_sclk_sel."]
pub type SCLK_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYS_CONF_SPEC, u8, u8, 2, O>;
#[doc = "Field `SCLK_ACTIVE` reader - reg_rmt_sclk_active."]
pub type SCLK_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `SCLK_ACTIVE` writer - reg_rmt_sclk_active."]
pub type SCLK_ACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_CONF_SPEC, bool, O>;
#[doc = "Field `CLK_EN` reader - reg_clk_en."]
pub type CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `CLK_EN` writer - reg_clk_en."]
pub type CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_CONF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - reg_apb_fifo_mask."]
    #[inline(always)]
    pub fn apb_fifo_mask(&self) -> APB_FIFO_MASK_R {
        APB_FIFO_MASK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reg_mem_clk_force_on."]
    #[inline(always)]
    pub fn mem_clk_force_on(&self) -> MEM_CLK_FORCE_ON_R {
        MEM_CLK_FORCE_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - reg_rmt_mem_force_pd."]
    #[inline(always)]
    pub fn mem_force_pd(&self) -> MEM_FORCE_PD_R {
        MEM_FORCE_PD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reg_rmt_mem_force_pu."]
    #[inline(always)]
    pub fn mem_force_pu(&self) -> MEM_FORCE_PU_R {
        MEM_FORCE_PU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:11 - reg_rmt_sclk_div_num."]
    #[inline(always)]
    pub fn sclk_div_num(&self) -> SCLK_DIV_NUM_R {
        SCLK_DIV_NUM_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:17 - reg_rmt_sclk_div_a."]
    #[inline(always)]
    pub fn sclk_div_a(&self) -> SCLK_DIV_A_R {
        SCLK_DIV_A_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - reg_rmt_sclk_div_b."]
    #[inline(always)]
    pub fn sclk_div_b(&self) -> SCLK_DIV_B_R {
        SCLK_DIV_B_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:25 - reg_rmt_sclk_sel."]
    #[inline(always)]
    pub fn sclk_sel(&self) -> SCLK_SEL_R {
        SCLK_SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - reg_rmt_sclk_active."]
    #[inline(always)]
    pub fn sclk_active(&self) -> SCLK_ACTIVE_R {
        SCLK_ACTIVE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 31 - reg_clk_en."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - reg_apb_fifo_mask."]
    #[inline(always)]
    pub fn apb_fifo_mask(&mut self) -> APB_FIFO_MASK_W<0> {
        APB_FIFO_MASK_W::new(self)
    }
    #[doc = "Bit 1 - reg_mem_clk_force_on."]
    #[inline(always)]
    pub fn mem_clk_force_on(&mut self) -> MEM_CLK_FORCE_ON_W<1> {
        MEM_CLK_FORCE_ON_W::new(self)
    }
    #[doc = "Bit 2 - reg_rmt_mem_force_pd."]
    #[inline(always)]
    pub fn mem_force_pd(&mut self) -> MEM_FORCE_PD_W<2> {
        MEM_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 3 - reg_rmt_mem_force_pu."]
    #[inline(always)]
    pub fn mem_force_pu(&mut self) -> MEM_FORCE_PU_W<3> {
        MEM_FORCE_PU_W::new(self)
    }
    #[doc = "Bits 4:11 - reg_rmt_sclk_div_num."]
    #[inline(always)]
    pub fn sclk_div_num(&mut self) -> SCLK_DIV_NUM_W<4> {
        SCLK_DIV_NUM_W::new(self)
    }
    #[doc = "Bits 12:17 - reg_rmt_sclk_div_a."]
    #[inline(always)]
    pub fn sclk_div_a(&mut self) -> SCLK_DIV_A_W<12> {
        SCLK_DIV_A_W::new(self)
    }
    #[doc = "Bits 18:23 - reg_rmt_sclk_div_b."]
    #[inline(always)]
    pub fn sclk_div_b(&mut self) -> SCLK_DIV_B_W<18> {
        SCLK_DIV_B_W::new(self)
    }
    #[doc = "Bits 24:25 - reg_rmt_sclk_sel."]
    #[inline(always)]
    pub fn sclk_sel(&mut self) -> SCLK_SEL_W<24> {
        SCLK_SEL_W::new(self)
    }
    #[doc = "Bit 26 - reg_rmt_sclk_active."]
    #[inline(always)]
    pub fn sclk_active(&mut self) -> SCLK_ACTIVE_W<26> {
        SCLK_ACTIVE_W::new(self)
    }
    #[doc = "Bit 31 - reg_clk_en."]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<31> {
        CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RMT_SYS_CONF_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_conf](index.html) module"]
pub struct SYS_CONF_SPEC;
impl crate::RegisterSpec for SYS_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_conf::R](R) reader structure"]
impl crate::Readable for SYS_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_conf::W](W) writer structure"]
impl crate::Writable for SYS_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_CONF to value 0x0500_0010"]
impl crate::Resettable for SYS_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0500_0010
    }
}
