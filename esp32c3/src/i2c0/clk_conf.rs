#[doc = "Register `CLK_CONF` reader"]
pub type R = crate::R<CLK_CONF_SPEC>;
#[doc = "Register `CLK_CONF` writer"]
pub type W = crate::W<CLK_CONF_SPEC>;
#[doc = "Field `SCLK_DIV_NUM` reader - reg_sclk_div_num"]
pub type SCLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `SCLK_DIV_NUM` writer - reg_sclk_div_num"]
pub type SCLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCLK_DIV_A` reader - reg_sclk_div_a"]
pub type SCLK_DIV_A_R = crate::FieldReader;
#[doc = "Field `SCLK_DIV_A` writer - reg_sclk_div_a"]
pub type SCLK_DIV_A_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SCLK_DIV_B` reader - reg_sclk_div_b"]
pub type SCLK_DIV_B_R = crate::FieldReader;
#[doc = "Field `SCLK_DIV_B` writer - reg_sclk_div_b"]
pub type SCLK_DIV_B_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SCLK_SEL` reader - reg_sclk_sel"]
pub type SCLK_SEL_R = crate::BitReader;
#[doc = "Field `SCLK_SEL` writer - reg_sclk_sel"]
pub type SCLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCLK_ACTIVE` reader - reg_sclk_active"]
pub type SCLK_ACTIVE_R = crate::BitReader;
#[doc = "Field `SCLK_ACTIVE` writer - reg_sclk_active"]
pub type SCLK_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - reg_sclk_div_num"]
    #[inline(always)]
    pub fn sclk_div_num(&self) -> SCLK_DIV_NUM_R {
        SCLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - reg_sclk_div_a"]
    #[inline(always)]
    pub fn sclk_div_a(&self) -> SCLK_DIV_A_R {
        SCLK_DIV_A_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:19 - reg_sclk_div_b"]
    #[inline(always)]
    pub fn sclk_div_b(&self) -> SCLK_DIV_B_R {
        SCLK_DIV_B_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bit 20 - reg_sclk_sel"]
    #[inline(always)]
    pub fn sclk_sel(&self) -> SCLK_SEL_R {
        SCLK_SEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - reg_sclk_active"]
    #[inline(always)]
    pub fn sclk_active(&self) -> SCLK_ACTIVE_R {
        SCLK_ACTIVE_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_CONF")
            .field(
                "sclk_div_num",
                &format_args!("{}", self.sclk_div_num().bits()),
            )
            .field("sclk_div_a", &format_args!("{}", self.sclk_div_a().bits()))
            .field("sclk_div_b", &format_args!("{}", self.sclk_div_b().bits()))
            .field("sclk_sel", &format_args!("{}", self.sclk_sel().bit()))
            .field("sclk_active", &format_args!("{}", self.sclk_active().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - reg_sclk_div_num"]
    #[inline(always)]
    #[must_use]
    pub fn sclk_div_num(&mut self) -> SCLK_DIV_NUM_W<CLK_CONF_SPEC> {
        SCLK_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - reg_sclk_div_a"]
    #[inline(always)]
    #[must_use]
    pub fn sclk_div_a(&mut self) -> SCLK_DIV_A_W<CLK_CONF_SPEC> {
        SCLK_DIV_A_W::new(self, 8)
    }
    #[doc = "Bits 14:19 - reg_sclk_div_b"]
    #[inline(always)]
    #[must_use]
    pub fn sclk_div_b(&mut self) -> SCLK_DIV_B_W<CLK_CONF_SPEC> {
        SCLK_DIV_B_W::new(self, 14)
    }
    #[doc = "Bit 20 - reg_sclk_sel"]
    #[inline(always)]
    #[must_use]
    pub fn sclk_sel(&mut self) -> SCLK_SEL_W<CLK_CONF_SPEC> {
        SCLK_SEL_W::new(self, 20)
    }
    #[doc = "Bit 21 - reg_sclk_active"]
    #[inline(always)]
    #[must_use]
    pub fn sclk_active(&mut self) -> SCLK_ACTIVE_W<CLK_CONF_SPEC> {
        SCLK_ACTIVE_W::new(self, 21)
    }
}
#[doc = "I2C_CLK_CONF_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_CONF_SPEC;
impl crate::RegisterSpec for CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_conf::R`](R) reader structure"]
impl crate::Readable for CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_conf::W`](W) writer structure"]
impl crate::Writable for CLK_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_CONF to value 0x0020_0000"]
impl crate::Resettable for CLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0020_0000;
}
