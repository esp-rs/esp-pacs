#[doc = "Register `SYS_CONF` reader"]
pub type R = crate::R<SYS_CONF_SPEC>;
#[doc = "Register `SYS_CONF` writer"]
pub type W = crate::W<SYS_CONF_SPEC>;
#[doc = "Field `APB_FIFO_MASK` reader - reg_apb_fifo_mask."]
pub type APB_FIFO_MASK_R = crate::BitReader;
#[doc = "Field `APB_FIFO_MASK` writer - reg_apb_fifo_mask."]
pub type APB_FIFO_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_CLK_FORCE_ON` reader - reg_mem_clk_force_on."]
pub type MEM_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `MEM_CLK_FORCE_ON` writer - reg_mem_clk_force_on."]
pub type MEM_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_FORCE_PD` reader - reg_rmt_mem_force_pd."]
pub type MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `MEM_FORCE_PD` writer - reg_rmt_mem_force_pd."]
pub type MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_FORCE_PU` reader - reg_rmt_mem_force_pu."]
pub type MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `MEM_FORCE_PU` writer - reg_rmt_mem_force_pu."]
pub type MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCLK_DIV_NUM` reader - reg_rmt_sclk_div_num."]
pub type SCLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `SCLK_DIV_NUM` writer - reg_rmt_sclk_div_num."]
pub type SCLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCLK_DIV_A` reader - reg_rmt_sclk_div_a."]
pub type SCLK_DIV_A_R = crate::FieldReader;
#[doc = "Field `SCLK_DIV_A` writer - reg_rmt_sclk_div_a."]
pub type SCLK_DIV_A_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SCLK_DIV_B` reader - reg_rmt_sclk_div_b."]
pub type SCLK_DIV_B_R = crate::FieldReader;
#[doc = "Field `SCLK_DIV_B` writer - reg_rmt_sclk_div_b."]
pub type SCLK_DIV_B_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SCLK_SEL` reader - reg_rmt_sclk_sel."]
pub type SCLK_SEL_R = crate::FieldReader;
#[doc = "Field `SCLK_SEL` writer - reg_rmt_sclk_sel."]
pub type SCLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCLK_ACTIVE` reader - reg_rmt_sclk_active."]
pub type SCLK_ACTIVE_R = crate::BitReader;
#[doc = "Field `SCLK_ACTIVE` writer - reg_rmt_sclk_active."]
pub type SCLK_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - reg_clk_en."]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - reg_clk_en."]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYS_CONF")
            .field(
                "apb_fifo_mask",
                &format_args!("{}", self.apb_fifo_mask().bit()),
            )
            .field(
                "mem_clk_force_on",
                &format_args!("{}", self.mem_clk_force_on().bit()),
            )
            .field(
                "mem_force_pd",
                &format_args!("{}", self.mem_force_pd().bit()),
            )
            .field(
                "mem_force_pu",
                &format_args!("{}", self.mem_force_pu().bit()),
            )
            .field(
                "sclk_div_num",
                &format_args!("{}", self.sclk_div_num().bits()),
            )
            .field("sclk_div_a", &format_args!("{}", self.sclk_div_a().bits()))
            .field("sclk_div_b", &format_args!("{}", self.sclk_div_b().bits()))
            .field("sclk_sel", &format_args!("{}", self.sclk_sel().bits()))
            .field("sclk_active", &format_args!("{}", self.sclk_active().bit()))
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SYS_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - reg_apb_fifo_mask."]
    #[inline(always)]
    #[must_use]
    pub fn apb_fifo_mask(&mut self) -> APB_FIFO_MASK_W<SYS_CONF_SPEC> {
        APB_FIFO_MASK_W::new(self, 0)
    }
    #[doc = "Bit 1 - reg_mem_clk_force_on."]
    #[inline(always)]
    #[must_use]
    pub fn mem_clk_force_on(&mut self) -> MEM_CLK_FORCE_ON_W<SYS_CONF_SPEC> {
        MEM_CLK_FORCE_ON_W::new(self, 1)
    }
    #[doc = "Bit 2 - reg_rmt_mem_force_pd."]
    #[inline(always)]
    #[must_use]
    pub fn mem_force_pd(&mut self) -> MEM_FORCE_PD_W<SYS_CONF_SPEC> {
        MEM_FORCE_PD_W::new(self, 2)
    }
    #[doc = "Bit 3 - reg_rmt_mem_force_pu."]
    #[inline(always)]
    #[must_use]
    pub fn mem_force_pu(&mut self) -> MEM_FORCE_PU_W<SYS_CONF_SPEC> {
        MEM_FORCE_PU_W::new(self, 3)
    }
    #[doc = "Bits 4:11 - reg_rmt_sclk_div_num."]
    #[inline(always)]
    #[must_use]
    pub fn sclk_div_num(&mut self) -> SCLK_DIV_NUM_W<SYS_CONF_SPEC> {
        SCLK_DIV_NUM_W::new(self, 4)
    }
    #[doc = "Bits 12:17 - reg_rmt_sclk_div_a."]
    #[inline(always)]
    #[must_use]
    pub fn sclk_div_a(&mut self) -> SCLK_DIV_A_W<SYS_CONF_SPEC> {
        SCLK_DIV_A_W::new(self, 12)
    }
    #[doc = "Bits 18:23 - reg_rmt_sclk_div_b."]
    #[inline(always)]
    #[must_use]
    pub fn sclk_div_b(&mut self) -> SCLK_DIV_B_W<SYS_CONF_SPEC> {
        SCLK_DIV_B_W::new(self, 18)
    }
    #[doc = "Bits 24:25 - reg_rmt_sclk_sel."]
    #[inline(always)]
    #[must_use]
    pub fn sclk_sel(&mut self) -> SCLK_SEL_W<SYS_CONF_SPEC> {
        SCLK_SEL_W::new(self, 24)
    }
    #[doc = "Bit 26 - reg_rmt_sclk_active."]
    #[inline(always)]
    #[must_use]
    pub fn sclk_active(&mut self) -> SCLK_ACTIVE_W<SYS_CONF_SPEC> {
        SCLK_ACTIVE_W::new(self, 26)
    }
    #[doc = "Bit 31 - reg_clk_en."]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<SYS_CONF_SPEC> {
        CLK_EN_W::new(self, 31)
    }
}
#[doc = "RMT_SYS_CONF_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_CONF_SPEC;
impl crate::RegisterSpec for SYS_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_conf::R`](R) reader structure"]
impl crate::Readable for SYS_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_conf::W`](W) writer structure"]
impl crate::Writable for SYS_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYS_CONF to value 0x0500_0010"]
impl crate::Resettable for SYS_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0500_0010;
}
