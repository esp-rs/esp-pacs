#[doc = "Register `LP_AONCLKRST_HP_CLK_CTRL` reader"]
pub type R = crate::R<LP_AONCLKRST_HP_CLK_CTRL_SPEC>;
#[doc = "Field `LP_AONCLKRST_CLK_XTAL_FREQ` reader - XTAL Clock Freq Indication. 7'd40: xtal_40m, 7'd48: xtal_48m"]
pub type LP_AONCLKRST_CLK_XTAL_FREQ_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - XTAL Clock Freq Indication. 7'd40: xtal_40m, 7'd48: xtal_48m"]
    #[inline(always)]
    pub fn lp_aonclkrst_clk_xtal_freq(&self) -> LP_AONCLKRST_CLK_XTAL_FREQ_R {
        LP_AONCLKRST_CLK_XTAL_FREQ_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_AONCLKRST_HP_CLK_CTRL")
            .field(
                "lp_aonclkrst_clk_xtal_freq",
                &self.lp_aonclkrst_clk_xtal_freq(),
            )
            .finish()
    }
}
#[doc = "HP Clock Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_hp_clk_ctrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_AONCLKRST_HP_CLK_CTRL_SPEC;
impl crate::RegisterSpec for LP_AONCLKRST_HP_CLK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_hp_clk_ctrl::R`](R) reader structure"]
impl crate::Readable for LP_AONCLKRST_HP_CLK_CTRL_SPEC {}
#[doc = "`reset()` method sets LP_AONCLKRST_HP_CLK_CTRL to value 0x28"]
impl crate::Resettable for LP_AONCLKRST_HP_CLK_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x28;
}
