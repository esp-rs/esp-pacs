#[doc = "Register `UART3_CTRL0` reader"]
pub type R = crate::R<UART3_CTRL0_SPEC>;
#[doc = "Register `UART3_CTRL0` writer"]
pub type W = crate::W<UART3_CTRL0_SPEC>;
#[doc = "Field `UART3_SYS_CLK_EN` reader - need_des"]
pub type UART3_SYS_CLK_EN_R = crate::BitReader;
#[doc = "Field `UART3_SYS_CLK_EN` writer - need_des"]
pub type UART3_SYS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART3_APB_CLK_EN` reader - need_des"]
pub type UART3_APB_CLK_EN_R = crate::BitReader;
#[doc = "Field `UART3_APB_CLK_EN` writer - need_des"]
pub type UART3_APB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART3_CORE_RST_EN` reader - need_des"]
pub type UART3_CORE_RST_EN_R = crate::BitReader;
#[doc = "Field `UART3_CORE_RST_EN` writer - need_des"]
pub type UART3_CORE_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART3_APB_RST_EN` reader - need_des"]
pub type UART3_APB_RST_EN_R = crate::BitReader;
#[doc = "Field `UART3_APB_RST_EN` writer - need_des"]
pub type UART3_APB_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART3_FORCE_NORST` reader - need_des"]
pub type UART3_FORCE_NORST_R = crate::BitReader;
#[doc = "Field `UART3_FORCE_NORST` writer - need_des"]
pub type UART3_FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART3_CLK_SRC_SEL` reader - need_des"]
pub type UART3_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `UART3_CLK_SRC_SEL` writer - need_des"]
pub type UART3_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UART3_CLK_EN` reader - need_des"]
pub type UART3_CLK_EN_R = crate::BitReader;
#[doc = "Field `UART3_CLK_EN` writer - need_des"]
pub type UART3_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART3_SCLK_DIV_NUM` reader - need_des"]
pub type UART3_SCLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `UART3_SCLK_DIV_NUM` writer - need_des"]
pub type UART3_SCLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `UART3_SCLK_DIV_NUMERATOR` reader - need_des"]
pub type UART3_SCLK_DIV_NUMERATOR_R = crate::FieldReader;
#[doc = "Field `UART3_SCLK_DIV_NUMERATOR` writer - need_des"]
pub type UART3_SCLK_DIV_NUMERATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `UART3_SCLK_DIV_DENOMINATOR` reader - need_des"]
pub type UART3_SCLK_DIV_DENOMINATOR_R = crate::FieldReader;
#[doc = "Field `UART3_SCLK_DIV_DENOMINATOR` writer - need_des"]
pub type UART3_SCLK_DIV_DENOMINATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn uart3_sys_clk_en(&self) -> UART3_SYS_CLK_EN_R {
        UART3_SYS_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn uart3_apb_clk_en(&self) -> UART3_APB_CLK_EN_R {
        UART3_APB_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn uart3_core_rst_en(&self) -> UART3_CORE_RST_EN_R {
        UART3_CORE_RST_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn uart3_apb_rst_en(&self) -> UART3_APB_RST_EN_R {
        UART3_APB_RST_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn uart3_force_norst(&self) -> UART3_FORCE_NORST_R {
        UART3_FORCE_NORST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - need_des"]
    #[inline(always)]
    pub fn uart3_clk_src_sel(&self) -> UART3_CLK_SRC_SEL_R {
        UART3_CLK_SRC_SEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    pub fn uart3_clk_en(&self) -> UART3_CLK_EN_R {
        UART3_CLK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - need_des"]
    #[inline(always)]
    pub fn uart3_sclk_div_num(&self) -> UART3_SCLK_DIV_NUM_R {
        UART3_SCLK_DIV_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - need_des"]
    #[inline(always)]
    pub fn uart3_sclk_div_numerator(&self) -> UART3_SCLK_DIV_NUMERATOR_R {
        UART3_SCLK_DIV_NUMERATOR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - need_des"]
    #[inline(always)]
    pub fn uart3_sclk_div_denominator(&self) -> UART3_SCLK_DIV_DENOMINATOR_R {
        UART3_SCLK_DIV_DENOMINATOR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART3_CTRL0")
            .field("uart3_sys_clk_en", &self.uart3_sys_clk_en())
            .field("uart3_apb_clk_en", &self.uart3_apb_clk_en())
            .field("uart3_core_rst_en", &self.uart3_core_rst_en())
            .field("uart3_apb_rst_en", &self.uart3_apb_rst_en())
            .field("uart3_force_norst", &self.uart3_force_norst())
            .field("uart3_clk_src_sel", &self.uart3_clk_src_sel())
            .field("uart3_clk_en", &self.uart3_clk_en())
            .field("uart3_sclk_div_num", &self.uart3_sclk_div_num())
            .field("uart3_sclk_div_numerator", &self.uart3_sclk_div_numerator())
            .field(
                "uart3_sclk_div_denominator",
                &self.uart3_sclk_div_denominator(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn uart3_sys_clk_en(&mut self) -> UART3_SYS_CLK_EN_W<'_, UART3_CTRL0_SPEC> {
        UART3_SYS_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn uart3_apb_clk_en(&mut self) -> UART3_APB_CLK_EN_W<'_, UART3_CTRL0_SPEC> {
        UART3_APB_CLK_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn uart3_core_rst_en(&mut self) -> UART3_CORE_RST_EN_W<'_, UART3_CTRL0_SPEC> {
        UART3_CORE_RST_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn uart3_apb_rst_en(&mut self) -> UART3_APB_RST_EN_W<'_, UART3_CTRL0_SPEC> {
        UART3_APB_RST_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn uart3_force_norst(&mut self) -> UART3_FORCE_NORST_W<'_, UART3_CTRL0_SPEC> {
        UART3_FORCE_NORST_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - need_des"]
    #[inline(always)]
    pub fn uart3_clk_src_sel(&mut self) -> UART3_CLK_SRC_SEL_W<'_, UART3_CTRL0_SPEC> {
        UART3_CLK_SRC_SEL_W::new(self, 5)
    }
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    pub fn uart3_clk_en(&mut self) -> UART3_CLK_EN_W<'_, UART3_CTRL0_SPEC> {
        UART3_CLK_EN_W::new(self, 7)
    }
    #[doc = "Bits 8:15 - need_des"]
    #[inline(always)]
    pub fn uart3_sclk_div_num(&mut self) -> UART3_SCLK_DIV_NUM_W<'_, UART3_CTRL0_SPEC> {
        UART3_SCLK_DIV_NUM_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - need_des"]
    #[inline(always)]
    pub fn uart3_sclk_div_numerator(&mut self) -> UART3_SCLK_DIV_NUMERATOR_W<'_, UART3_CTRL0_SPEC> {
        UART3_SCLK_DIV_NUMERATOR_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - need_des"]
    #[inline(always)]
    pub fn uart3_sclk_div_denominator(
        &mut self,
    ) -> UART3_SCLK_DIV_DENOMINATOR_W<'_, UART3_CTRL0_SPEC> {
        UART3_SCLK_DIV_DENOMINATOR_W::new(self, 24)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`uart3_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart3_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART3_CTRL0_SPEC;
impl crate::RegisterSpec for UART3_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart3_ctrl0::R`](R) reader structure"]
impl crate::Readable for UART3_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uart3_ctrl0::W`](W) writer structure"]
impl crate::Writable for UART3_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART3_CTRL0 to value 0x83"]
impl crate::Resettable for UART3_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x83;
}
