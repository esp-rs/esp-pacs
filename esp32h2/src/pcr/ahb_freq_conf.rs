#[doc = "Register `AHB_FREQ_CONF` reader"]
pub type R = crate::R<AHB_FREQ_CONF_SPEC>;
#[doc = "Register `AHB_FREQ_CONF` writer"]
pub type W = crate::W<AHB_FREQ_CONF_SPEC>;
#[doc = "Field `AHB_DIV_NUM` reader - Set this field to generate clk_ahb drived by clk_hproot. The clk_ahb is div1(default)/div2/div4/div8 of clk_hproot. This field is only avaliable for low-speed clock-source such as XTAL/FOSC, and should be used together with PCR_CPU_DIV_NUM."]
pub type AHB_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `AHB_DIV_NUM` writer - Set this field to generate clk_ahb drived by clk_hproot. The clk_ahb is div1(default)/div2/div4/div8 of clk_hproot. This field is only avaliable for low-speed clock-source such as XTAL/FOSC, and should be used together with PCR_CPU_DIV_NUM."]
pub type AHB_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Set this field to generate clk_ahb drived by clk_hproot. The clk_ahb is div1(default)/div2/div4/div8 of clk_hproot. This field is only avaliable for low-speed clock-source such as XTAL/FOSC, and should be used together with PCR_CPU_DIV_NUM."]
    #[inline(always)]
    pub fn ahb_div_num(&self) -> AHB_DIV_NUM_R {
        AHB_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_FREQ_CONF")
            .field("ahb_div_num", &self.ahb_div_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Set this field to generate clk_ahb drived by clk_hproot. The clk_ahb is div1(default)/div2/div4/div8 of clk_hproot. This field is only avaliable for low-speed clock-source such as XTAL/FOSC, and should be used together with PCR_CPU_DIV_NUM."]
    #[inline(always)]
    pub fn ahb_div_num(&mut self) -> AHB_DIV_NUM_W<AHB_FREQ_CONF_SPEC> {
        AHB_DIV_NUM_W::new(self, 0)
    }
}
#[doc = "AHB_FREQ configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_freq_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_freq_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB_FREQ_CONF_SPEC;
impl crate::RegisterSpec for AHB_FREQ_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_freq_conf::R`](R) reader structure"]
impl crate::Readable for AHB_FREQ_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb_freq_conf::W`](W) writer structure"]
impl crate::Writable for AHB_FREQ_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB_FREQ_CONF to value 0"]
impl crate::Resettable for AHB_FREQ_CONF_SPEC {}
