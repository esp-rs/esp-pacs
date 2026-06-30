#[doc = "Register `REF_80M_CTRL0` reader"]
pub type R = crate::R<REF_80M_CTRL0_SPEC>;
#[doc = "Register `REF_80M_CTRL0` writer"]
pub type W = crate::W<REF_80M_CTRL0_SPEC>;
#[doc = "Field `REG_REF_80M_CLK_DIV_NUM` reader - need_des"]
pub type REG_REF_80M_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `REG_REF_80M_CLK_DIV_NUM` writer - need_des"]
pub type REG_REF_80M_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_REF_80M_CLK_EN` reader - need_des"]
pub type REG_REF_80M_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_REF_80M_CLK_EN` writer - need_des"]
pub type REG_REF_80M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_REF_80M_SEL` reader - need_des"]
pub type REG_REF_80M_SEL_R = crate::BitReader;
#[doc = "Field `REG_REF_80M_SEL` writer - need_des"]
pub type REG_REF_80M_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_REF_80M_MUX_CLK_EN` reader - need_des"]
pub type REG_REF_80M_MUX_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_REF_80M_MUX_CLK_EN` writer - need_des"]
pub type REG_REF_80M_MUX_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn reg_ref_80m_clk_div_num(&self) -> REG_REF_80M_CLK_DIV_NUM_R {
        REG_REF_80M_CLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn reg_ref_80m_clk_en(&self) -> REG_REF_80M_CLK_EN_R {
        REG_REF_80M_CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - need_des"]
    #[inline(always)]
    pub fn reg_ref_80m_sel(&self) -> REG_REF_80M_SEL_R {
        REG_REF_80M_SEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    pub fn reg_ref_80m_mux_clk_en(&self) -> REG_REF_80M_MUX_CLK_EN_R {
        REG_REF_80M_MUX_CLK_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REF_80M_CTRL0")
            .field("reg_ref_80m_clk_div_num", &self.reg_ref_80m_clk_div_num())
            .field("reg_ref_80m_clk_en", &self.reg_ref_80m_clk_en())
            .field("reg_ref_80m_sel", &self.reg_ref_80m_sel())
            .field("reg_ref_80m_mux_clk_en", &self.reg_ref_80m_mux_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn reg_ref_80m_clk_div_num(&mut self) -> REG_REF_80M_CLK_DIV_NUM_W<'_, REF_80M_CTRL0_SPEC> {
        REG_REF_80M_CLK_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn reg_ref_80m_clk_en(&mut self) -> REG_REF_80M_CLK_EN_W<'_, REF_80M_CTRL0_SPEC> {
        REG_REF_80M_CLK_EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - need_des"]
    #[inline(always)]
    pub fn reg_ref_80m_sel(&mut self) -> REG_REF_80M_SEL_W<'_, REF_80M_CTRL0_SPEC> {
        REG_REF_80M_SEL_W::new(self, 9)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    pub fn reg_ref_80m_mux_clk_en(&mut self) -> REG_REF_80M_MUX_CLK_EN_W<'_, REF_80M_CTRL0_SPEC> {
        REG_REF_80M_MUX_CLK_EN_W::new(self, 10)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_80m_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ref_80m_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REF_80M_CTRL0_SPEC;
impl crate::RegisterSpec for REF_80M_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ref_80m_ctrl0::R`](R) reader structure"]
impl crate::Readable for REF_80M_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ref_80m_ctrl0::W`](W) writer structure"]
impl crate::Writable for REF_80M_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REF_80M_CTRL0 to value 0x0505"]
impl crate::Resettable for REF_80M_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x0505;
}
