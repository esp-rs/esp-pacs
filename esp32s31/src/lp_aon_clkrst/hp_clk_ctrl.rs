#[doc = "Register `HP_CLK_CTRL` reader"]
pub type R = crate::R<HP_CLK_CTRL_SPEC>;
#[doc = "Field `CLK_XTAL_FREQ` reader - XTAL Clock Freq Indication. 7'd40: xtal_40m, 7'd48: xtal_48m"]
pub type CLK_XTAL_FREQ_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - XTAL Clock Freq Indication. 7'd40: xtal_40m, 7'd48: xtal_48m"]
    #[inline(always)]
    pub fn clk_xtal_freq(&self) -> CLK_XTAL_FREQ_R {
        CLK_XTAL_FREQ_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_CLK_CTRL")
            .field("clk_xtal_freq", &self.clk_xtal_freq())
            .finish()
    }
}
#[doc = "HP Clock Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_clk_ctrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_CLK_CTRL_SPEC;
impl crate::RegisterSpec for HP_CLK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_clk_ctrl::R`](R) reader structure"]
impl crate::Readable for HP_CLK_CTRL_SPEC {}
#[doc = "`reset()` method sets HP_CLK_CTRL to value 0x28"]
impl crate::Resettable for HP_CLK_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x28;
}
