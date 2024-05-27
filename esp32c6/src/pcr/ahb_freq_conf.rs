///Register `AHB_FREQ_CONF` reader
pub type R = crate::R<AHB_FREQ_CONF_SPEC>;
///Register `AHB_FREQ_CONF` writer
pub type W = crate::W<AHB_FREQ_CONF_SPEC>;
///Field `AHB_LS_DIV_NUM` reader - Set as one within (0,1,3,7) to generate clk_ahb drived by clk_hproot. The clk_ahb is div1(default)/div2/div4/div8 of clk_hproot. This field is only avaliable for low-speed clock-source such as XTAL/FOSC, and should be used together with PCR_CPU_LS_DIV_NUM.
pub type AHB_LS_DIV_NUM_R = crate::FieldReader;
///Field `AHB_LS_DIV_NUM` writer - Set as one within (0,1,3,7) to generate clk_ahb drived by clk_hproot. The clk_ahb is div1(default)/div2/div4/div8 of clk_hproot. This field is only avaliable for low-speed clock-source such as XTAL/FOSC, and should be used together with PCR_CPU_LS_DIV_NUM.
pub type AHB_LS_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `AHB_HS_DIV_NUM` reader - Set as one within (3,7,15) to generate clk_ahb drived by clk_hproot. The clk_ahb is div4(default)/div8/div16 of clk_hproot. This field is only avaliable for high-speed clock-source such as SPLL, and should be used together with PCR_CPU_HS_DIV_NUM.
pub type AHB_HS_DIV_NUM_R = crate::FieldReader;
///Field `AHB_HS_DIV_NUM` writer - Set as one within (3,7,15) to generate clk_ahb drived by clk_hproot. The clk_ahb is div4(default)/div8/div16 of clk_hproot. This field is only avaliable for high-speed clock-source such as SPLL, and should be used together with PCR_CPU_HS_DIV_NUM.
pub type AHB_HS_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Set as one within (0,1,3,7) to generate clk_ahb drived by clk_hproot. The clk_ahb is div1(default)/div2/div4/div8 of clk_hproot. This field is only avaliable for low-speed clock-source such as XTAL/FOSC, and should be used together with PCR_CPU_LS_DIV_NUM.
    #[inline(always)]
    pub fn ahb_ls_div_num(&self) -> AHB_LS_DIV_NUM_R {
        AHB_LS_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Set as one within (3,7,15) to generate clk_ahb drived by clk_hproot. The clk_ahb is div4(default)/div8/div16 of clk_hproot. This field is only avaliable for high-speed clock-source such as SPLL, and should be used together with PCR_CPU_HS_DIV_NUM.
    #[inline(always)]
    pub fn ahb_hs_div_num(&self) -> AHB_HS_DIV_NUM_R {
        AHB_HS_DIV_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_FREQ_CONF")
            .field("ahb_ls_div_num", &self.ahb_ls_div_num())
            .field("ahb_hs_div_num", &self.ahb_hs_div_num())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Set as one within (0,1,3,7) to generate clk_ahb drived by clk_hproot. The clk_ahb is div1(default)/div2/div4/div8 of clk_hproot. This field is only avaliable for low-speed clock-source such as XTAL/FOSC, and should be used together with PCR_CPU_LS_DIV_NUM.
    #[inline(always)]
    #[must_use]
    pub fn ahb_ls_div_num(&mut self) -> AHB_LS_DIV_NUM_W<AHB_FREQ_CONF_SPEC> {
        AHB_LS_DIV_NUM_W::new(self, 0)
    }
    ///Bits 8:15 - Set as one within (3,7,15) to generate clk_ahb drived by clk_hproot. The clk_ahb is div4(default)/div8/div16 of clk_hproot. This field is only avaliable for high-speed clock-source such as SPLL, and should be used together with PCR_CPU_HS_DIV_NUM.
    #[inline(always)]
    #[must_use]
    pub fn ahb_hs_div_num(&mut self) -> AHB_HS_DIV_NUM_W<AHB_FREQ_CONF_SPEC> {
        AHB_HS_DIV_NUM_W::new(self, 8)
    }
}
/**AHB_FREQ configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`ahb_freq_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_freq_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AHB_FREQ_CONF_SPEC;
impl crate::RegisterSpec for AHB_FREQ_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ahb_freq_conf::R`](R) reader structure
impl crate::Readable for AHB_FREQ_CONF_SPEC {}
///`write(|w| ..)` method takes [`ahb_freq_conf::W`](W) writer structure
impl crate::Writable for AHB_FREQ_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AHB_FREQ_CONF to value 0x0300
impl crate::Resettable for AHB_FREQ_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0300;
}
