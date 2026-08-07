#[doc = "Register `SEC_CONF` reader"]
pub type R = crate::R<SEC_CONF_SPEC>;
#[doc = "Register `SEC_CONF` writer"]
pub type W = crate::W<SEC_CONF_SPEC>;
#[doc = "Field `SEC_CLK_SEL` reader - Configures the clock source for the External Memory Encryption and Decryption module.\\\\ 0(default): XTAL_CLK\\\\ 1: RC_FAST_CLK\\\\ 2: PLL_F64M_CLK\\\\ 3: PLL_F96M_CLK\\\\"]
pub type SEC_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `SEC_CLK_SEL` writer - Configures the clock source for the External Memory Encryption and Decryption module.\\\\ 0(default): XTAL_CLK\\\\ 1: RC_FAST_CLK\\\\ 2: PLL_F64M_CLK\\\\ 3: PLL_F96M_CLK\\\\"]
pub type SEC_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SEC_RST_EN` reader - Set 1 to reset sec module"]
pub type SEC_RST_EN_R = crate::BitReader;
#[doc = "Field `SEC_RST_EN` writer - Set 1 to reset sec module"]
pub type SEC_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC_DIV_NUMINATOR` reader - The denominator of the frequency divider factor of the sec function clock."]
pub type SEC_DIV_NUMINATOR_R = crate::FieldReader;
#[doc = "Field `SEC_DIV_NUMINATOR` writer - The denominator of the frequency divider factor of the sec function clock."]
pub type SEC_DIV_NUMINATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEC_DIV_DENOMINATOR` reader - The numerator of the frequency divider factor of the sec function clock."]
pub type SEC_DIV_DENOMINATOR_R = crate::FieldReader;
#[doc = "Field `SEC_DIV_DENOMINATOR` writer - The numerator of the frequency divider factor of the sec function clock."]
pub type SEC_DIV_DENOMINATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEC_DIV_NUM` reader - The integral part of the frequency divider factor of the sec function clock."]
pub type SEC_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `SEC_DIV_NUM` writer - The integral part of the frequency divider factor of the sec function clock."]
pub type SEC_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Configures the clock source for the External Memory Encryption and Decryption module.\\\\ 0(default): XTAL_CLK\\\\ 1: RC_FAST_CLK\\\\ 2: PLL_F64M_CLK\\\\ 3: PLL_F96M_CLK\\\\"]
    #[inline(always)]
    pub fn sec_clk_sel(&self) -> SEC_CLK_SEL_R {
        SEC_CLK_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Set 1 to reset sec module"]
    #[inline(always)]
    pub fn sec_rst_en(&self) -> SEC_RST_EN_R {
        SEC_RST_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - The denominator of the frequency divider factor of the sec function clock."]
    #[inline(always)]
    pub fn sec_div_numinator(&self) -> SEC_DIV_NUMINATOR_R {
        SEC_DIV_NUMINATOR_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 7:10 - The numerator of the frequency divider factor of the sec function clock."]
    #[inline(always)]
    pub fn sec_div_denominator(&self) -> SEC_DIV_DENOMINATOR_R {
        SEC_DIV_DENOMINATOR_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 11:14 - The integral part of the frequency divider factor of the sec function clock."]
    #[inline(always)]
    pub fn sec_div_num(&self) -> SEC_DIV_NUM_R {
        SEC_DIV_NUM_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CONF")
            .field("sec_clk_sel", &self.sec_clk_sel())
            .field("sec_rst_en", &self.sec_rst_en())
            .field("sec_div_numinator", &self.sec_div_numinator())
            .field("sec_div_denominator", &self.sec_div_denominator())
            .field("sec_div_num", &self.sec_div_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Configures the clock source for the External Memory Encryption and Decryption module.\\\\ 0(default): XTAL_CLK\\\\ 1: RC_FAST_CLK\\\\ 2: PLL_F64M_CLK\\\\ 3: PLL_F96M_CLK\\\\"]
    #[inline(always)]
    pub fn sec_clk_sel(&mut self) -> SEC_CLK_SEL_W<'_, SEC_CONF_SPEC> {
        SEC_CLK_SEL_W::new(self, 0)
    }
    #[doc = "Bit 2 - Set 1 to reset sec module"]
    #[inline(always)]
    pub fn sec_rst_en(&mut self) -> SEC_RST_EN_W<'_, SEC_CONF_SPEC> {
        SEC_RST_EN_W::new(self, 2)
    }
    #[doc = "Bits 3:6 - The denominator of the frequency divider factor of the sec function clock."]
    #[inline(always)]
    pub fn sec_div_numinator(&mut self) -> SEC_DIV_NUMINATOR_W<'_, SEC_CONF_SPEC> {
        SEC_DIV_NUMINATOR_W::new(self, 3)
    }
    #[doc = "Bits 7:10 - The numerator of the frequency divider factor of the sec function clock."]
    #[inline(always)]
    pub fn sec_div_denominator(&mut self) -> SEC_DIV_DENOMINATOR_W<'_, SEC_CONF_SPEC> {
        SEC_DIV_DENOMINATOR_W::new(self, 7)
    }
    #[doc = "Bits 11:14 - The integral part of the frequency divider factor of the sec function clock."]
    #[inline(always)]
    pub fn sec_div_num(&mut self) -> SEC_DIV_NUM_W<'_, SEC_CONF_SPEC> {
        SEC_DIV_NUM_W::new(self, 11)
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
