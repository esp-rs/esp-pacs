///Register `REF_CNT_RST` writer
pub type W = crate::W<REF_CNT_RST_SPEC>;
///Field `CH0` writer - reg_ref_cnt_rst_ch0.
pub type CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1` writer - reg_ref_cnt_rst_ch1.
pub type CH1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH2` writer - reg_ref_cnt_rst_ch2.
pub type CH2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH3` writer - reg_ref_cnt_rst_ch3.
pub type CH3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REF_CNT_RST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - reg_ref_cnt_rst_ch0.
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH0_W<REF_CNT_RST_SPEC> {
        CH0_W::new(self, 0)
    }
    ///Bit 1 - reg_ref_cnt_rst_ch1.
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<REF_CNT_RST_SPEC> {
        CH1_W::new(self, 1)
    }
    ///Bit 2 - reg_ref_cnt_rst_ch2.
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH2_W<REF_CNT_RST_SPEC> {
        CH2_W::new(self, 2)
    }
    ///Bit 3 - reg_ref_cnt_rst_ch3.
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> CH3_W<REF_CNT_RST_SPEC> {
        CH3_W::new(self, 3)
    }
}
/**RMT_REF_CNT_RST_REG.

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ref_cnt_rst::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct REF_CNT_RST_SPEC;
impl crate::RegisterSpec for REF_CNT_RST_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ref_cnt_rst::W`](W) writer structure
impl crate::Writable for REF_CNT_RST_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets REF_CNT_RST to value 0
impl crate::Resettable for REF_CNT_RST_SPEC {
    const RESET_VALUE: u32 = 0;
}
