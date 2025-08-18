#[doc = "Register `SEC_CONF` reader"]
pub type R = crate::R<SEC_CONF_SPEC>;
#[doc = "Register `SEC_CONF` writer"]
pub type W = crate::W<SEC_CONF_SPEC>;
#[doc = "Field `SEC_CLK_SEL` reader - Configures the clock source for the External Memory Encryption and Decryption module.\\\\ 0(default): XTAL_CLK\\\\ 1 RC_FAST_CLK\\\\ 2: PLL_F480M_CLK\\\\"]
pub type SEC_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `SEC_CLK_SEL` writer - Configures the clock source for the External Memory Encryption and Decryption module.\\\\ 0(default): XTAL_CLK\\\\ 1 RC_FAST_CLK\\\\ 2: PLL_F480M_CLK\\\\"]
pub type SEC_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SEC_RST_EN` reader - Set 1 to reset sec module"]
pub type SEC_RST_EN_R = crate::BitReader;
#[doc = "Field `SEC_RST_EN` writer - Set 1 to reset sec module"]
pub type SEC_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Configures the clock source for the External Memory Encryption and Decryption module.\\\\ 0(default): XTAL_CLK\\\\ 1 RC_FAST_CLK\\\\ 2: PLL_F480M_CLK\\\\"]
    #[inline(always)]
    pub fn sec_clk_sel(&self) -> SEC_CLK_SEL_R {
        SEC_CLK_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Set 1 to reset sec module"]
    #[inline(always)]
    pub fn sec_rst_en(&self) -> SEC_RST_EN_R {
        SEC_RST_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CONF")
            .field("sec_clk_sel", &self.sec_clk_sel())
            .field("sec_rst_en", &self.sec_rst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Configures the clock source for the External Memory Encryption and Decryption module.\\\\ 0(default): XTAL_CLK\\\\ 1 RC_FAST_CLK\\\\ 2: PLL_F480M_CLK\\\\"]
    #[inline(always)]
    pub fn sec_clk_sel(&mut self) -> SEC_CLK_SEL_W<'_, SEC_CONF_SPEC> {
        SEC_CLK_SEL_W::new(self, 0)
    }
    #[doc = "Bit 2 - Set 1 to reset sec module"]
    #[inline(always)]
    pub fn sec_rst_en(&mut self) -> SEC_RST_EN_W<'_, SEC_CONF_SPEC> {
        SEC_RST_EN_W::new(self, 2)
    }
}
#[doc = "Clock source configuration register for External Memory Encryption and Decryption\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEC_CONF_SPEC;
impl crate::RegisterSpec for SEC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_conf::R`](R) reader structure"]
impl crate::Readable for SEC_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sec_conf::W`](W) writer structure"]
impl crate::Writable for SEC_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_CONF to value 0"]
impl crate::Resettable for SEC_CONF_SPEC {}
