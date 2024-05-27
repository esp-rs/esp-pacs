#[doc = "Register `CPU_FREQ_CONF` reader"]
pub type R = crate::R<CPU_FREQ_CONF_SPEC>;
#[doc = "Register `CPU_FREQ_CONF` writer"]
pub type W = crate::W<CPU_FREQ_CONF_SPEC>;
#[doc = "Field `CPU_DIV_NUM` reader - Set this field to generate clk_cpu drived by clk_hproot. The clk_cpu is div1(default)/div2/div4 of clk_hproot. This field is only avaliable for low-speed clock-source such as XTAL/FOSC, and should be used together with PCR_AHB_DIV_NUM."]
pub type CPU_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `CPU_DIV_NUM` writer - Set this field to generate clk_cpu drived by clk_hproot. The clk_cpu is div1(default)/div2/div4 of clk_hproot. This field is only avaliable for low-speed clock-source such as XTAL/FOSC, and should be used together with PCR_AHB_DIV_NUM."]
pub type CPU_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Set this field to generate clk_cpu drived by clk_hproot. The clk_cpu is div1(default)/div2/div4 of clk_hproot. This field is only avaliable for low-speed clock-source such as XTAL/FOSC, and should be used together with PCR_AHB_DIV_NUM."]
    #[inline(always)]
    pub fn cpu_div_num(&self) -> CPU_DIV_NUM_R {
        CPU_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_FREQ_CONF")
            .field("cpu_div_num", &self.cpu_div_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Set this field to generate clk_cpu drived by clk_hproot. The clk_cpu is div1(default)/div2/div4 of clk_hproot. This field is only avaliable for low-speed clock-source such as XTAL/FOSC, and should be used together with PCR_AHB_DIV_NUM."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_div_num(&mut self) -> CPU_DIV_NUM_W<CPU_FREQ_CONF_SPEC> {
        CPU_DIV_NUM_W::new(self, 0)
    }
}
#[doc = "CPU_FREQ configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_freq_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_freq_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_FREQ_CONF_SPEC;
impl crate::RegisterSpec for CPU_FREQ_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_freq_conf::R`](R) reader structure"]
impl crate::Readable for CPU_FREQ_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpu_freq_conf::W`](W) writer structure"]
impl crate::Writable for CPU_FREQ_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPU_FREQ_CONF to value 0"]
impl crate::Resettable for CPU_FREQ_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
