///Register `MSPI_CLK_CONF` reader
pub type R = crate::R<MSPI_CLK_CONF_SPEC>;
///Register `MSPI_CLK_CONF` writer
pub type W = crate::W<MSPI_CLK_CONF_SPEC>;
///Field `MSPI_FAST_DIV_NUM` reader - Set as one within (0,1,2) to generate div1(default)/div2/div4 of low-speed clock-source to drive clk_mspi_fast. Only avaiable whe the clck-source is a low-speed clock-source such as XTAL/FOSC.
pub type MSPI_FAST_DIV_NUM_R = crate::FieldReader;
///Field `MSPI_FAST_DIV_NUM` writer - Set as one within (0,1,2) to generate div1(default)/div2/div4 of low-speed clock-source to drive clk_mspi_fast. Only avaiable whe the clck-source is a low-speed clock-source such as XTAL/FOSC.
pub type MSPI_FAST_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Set as one within (0,1,2) to generate div1(default)/div2/div4 of low-speed clock-source to drive clk_mspi_fast. Only avaiable whe the clck-source is a low-speed clock-source such as XTAL/FOSC.
    #[inline(always)]
    pub fn mspi_fast_div_num(&self) -> MSPI_FAST_DIV_NUM_R {
        MSPI_FAST_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSPI_CLK_CONF")
            .field("mspi_fast_div_num", &self.mspi_fast_div_num())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Set as one within (0,1,2) to generate div1(default)/div2/div4 of low-speed clock-source to drive clk_mspi_fast. Only avaiable whe the clck-source is a low-speed clock-source such as XTAL/FOSC.
    #[inline(always)]
    #[must_use]
    pub fn mspi_fast_div_num(&mut self) -> MSPI_FAST_DIV_NUM_W<MSPI_CLK_CONF_SPEC> {
        MSPI_FAST_DIV_NUM_W::new(self, 0)
    }
}
/**MSPI_CLK configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`mspi_clk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mspi_clk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MSPI_CLK_CONF_SPEC;
impl crate::RegisterSpec for MSPI_CLK_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mspi_clk_conf::R`](R) reader structure
impl crate::Readable for MSPI_CLK_CONF_SPEC {}
///`write(|w| ..)` method takes [`mspi_clk_conf::W`](W) writer structure
impl crate::Writable for MSPI_CLK_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MSPI_CLK_CONF to value 0
impl crate::Resettable for MSPI_CLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
