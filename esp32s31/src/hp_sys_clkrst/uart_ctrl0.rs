#[doc = "Register `UART_CTRL0%s` reader"]
pub type R = crate::R<UART_CTRL0_SPEC>;
#[doc = "Register `UART_CTRL0%s` writer"]
pub type W = crate::W<UART_CTRL0_SPEC>;
#[doc = "Field `SYS_CLK_EN` reader - need_des"]
pub type SYS_CLK_EN_R = crate::BitReader;
#[doc = "Field `SYS_CLK_EN` writer - need_des"]
pub type SYS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_CLK_EN` reader - need_des"]
pub type APB_CLK_EN_R = crate::BitReader;
#[doc = "Field `APB_CLK_EN` writer - need_des"]
pub type APB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_RST_EN` reader - need_des"]
pub type CORE_RST_EN_R = crate::BitReader;
#[doc = "Field `CORE_RST_EN` writer - need_des"]
pub type CORE_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_RST_EN` reader - need_des"]
pub type APB_RST_EN_R = crate::BitReader;
#[doc = "Field `APB_RST_EN` writer - need_des"]
pub type APB_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_NORST` reader - need_des"]
pub type FORCE_NORST_R = crate::BitReader;
#[doc = "Field `FORCE_NORST` writer - need_des"]
pub type FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SRC_SEL` reader - need_des"]
pub type CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `CLK_SRC_SEL` writer - need_des"]
pub type CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLK_EN` reader - need_des"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - need_des"]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCLK_DIV_NUM` reader - need_des"]
pub type SCLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `SCLK_DIV_NUM` writer - need_des"]
pub type SCLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCLK_DIV_NUMERATOR` reader - need_des"]
pub type SCLK_DIV_NUMERATOR_R = crate::FieldReader;
#[doc = "Field `SCLK_DIV_NUMERATOR` writer - need_des"]
pub type SCLK_DIV_NUMERATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCLK_DIV_DENOMINATOR` reader - need_des"]
pub type SCLK_DIV_DENOMINATOR_R = crate::FieldReader;
#[doc = "Field `SCLK_DIV_DENOMINATOR` writer - need_des"]
pub type SCLK_DIV_DENOMINATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn sys_clk_en(&self) -> SYS_CLK_EN_R {
        SYS_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn apb_clk_en(&self) -> APB_CLK_EN_R {
        APB_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn core_rst_en(&self) -> CORE_RST_EN_R {
        CORE_RST_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn apb_rst_en(&self) -> APB_RST_EN_R {
        APB_RST_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn force_norst(&self) -> FORCE_NORST_R {
        FORCE_NORST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - need_des"]
    #[inline(always)]
    pub fn clk_src_sel(&self) -> CLK_SRC_SEL_R {
        CLK_SRC_SEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - need_des"]
    #[inline(always)]
    pub fn sclk_div_num(&self) -> SCLK_DIV_NUM_R {
        SCLK_DIV_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - need_des"]
    #[inline(always)]
    pub fn sclk_div_numerator(&self) -> SCLK_DIV_NUMERATOR_R {
        SCLK_DIV_NUMERATOR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - need_des"]
    #[inline(always)]
    pub fn sclk_div_denominator(&self) -> SCLK_DIV_DENOMINATOR_R {
        SCLK_DIV_DENOMINATOR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART_CTRL0")
            .field("sys_clk_en", &self.sys_clk_en())
            .field("apb_clk_en", &self.apb_clk_en())
            .field("core_rst_en", &self.core_rst_en())
            .field("apb_rst_en", &self.apb_rst_en())
            .field("force_norst", &self.force_norst())
            .field("clk_src_sel", &self.clk_src_sel())
            .field("clk_en", &self.clk_en())
            .field("sclk_div_num", &self.sclk_div_num())
            .field("sclk_div_numerator", &self.sclk_div_numerator())
            .field("sclk_div_denominator", &self.sclk_div_denominator())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn sys_clk_en(&mut self) -> SYS_CLK_EN_W<'_, UART_CTRL0_SPEC> {
        SYS_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn apb_clk_en(&mut self) -> APB_CLK_EN_W<'_, UART_CTRL0_SPEC> {
        APB_CLK_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn core_rst_en(&mut self) -> CORE_RST_EN_W<'_, UART_CTRL0_SPEC> {
        CORE_RST_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn apb_rst_en(&mut self) -> APB_RST_EN_W<'_, UART_CTRL0_SPEC> {
        APB_RST_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn force_norst(&mut self) -> FORCE_NORST_W<'_, UART_CTRL0_SPEC> {
        FORCE_NORST_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - need_des"]
    #[inline(always)]
    pub fn clk_src_sel(&mut self) -> CLK_SRC_SEL_W<'_, UART_CTRL0_SPEC> {
        CLK_SRC_SEL_W::new(self, 5)
    }
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<'_, UART_CTRL0_SPEC> {
        CLK_EN_W::new(self, 7)
    }
    #[doc = "Bits 8:15 - need_des"]
    #[inline(always)]
    pub fn sclk_div_num(&mut self) -> SCLK_DIV_NUM_W<'_, UART_CTRL0_SPEC> {
        SCLK_DIV_NUM_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - need_des"]
    #[inline(always)]
    pub fn sclk_div_numerator(&mut self) -> SCLK_DIV_NUMERATOR_W<'_, UART_CTRL0_SPEC> {
        SCLK_DIV_NUMERATOR_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - need_des"]
    #[inline(always)]
    pub fn sclk_div_denominator(&mut self) -> SCLK_DIV_DENOMINATOR_W<'_, UART_CTRL0_SPEC> {
        SCLK_DIV_DENOMINATOR_W::new(self, 24)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART_CTRL0_SPEC;
impl crate::RegisterSpec for UART_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_ctrl0::R`](R) reader structure"]
impl crate::Readable for UART_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uart_ctrl0::W`](W) writer structure"]
impl crate::Writable for UART_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_CTRL0%s to value 0x83"]
impl crate::Resettable for UART_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x83;
}
