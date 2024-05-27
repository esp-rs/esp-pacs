#[doc = "Register `APB_CONF` reader"]
pub type R = crate::R<APB_CONF_SPEC>;
#[doc = "Register `APB_CONF` writer"]
pub type W = crate::W<APB_CONF_SPEC>;
#[doc = "Field `APB_FIFO_MASK` reader - Set this bit to disable apb fifo access"]
pub type APB_FIFO_MASK_R = crate::BitReader;
#[doc = "Field `APB_FIFO_MASK` writer - Set this bit to disable apb fifo access"]
pub type APB_FIFO_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_TX_WRAP_EN` reader - when datas need to be send is more than channel's mem can store then set this bit to enable reusage of mem this bit is used together with reg_rmt_tx_lim_chn."]
pub type MEM_TX_WRAP_EN_R = crate::BitReader;
#[doc = "Field `MEM_TX_WRAP_EN` writer - when datas need to be send is more than channel's mem can store then set this bit to enable reusage of mem this bit is used together with reg_rmt_tx_lim_chn."]
pub type MEM_TX_WRAP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to disable apb fifo access"]
    #[inline(always)]
    pub fn apb_fifo_mask(&self) -> APB_FIFO_MASK_R {
        APB_FIFO_MASK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - when datas need to be send is more than channel's mem can store then set this bit to enable reusage of mem this bit is used together with reg_rmt_tx_lim_chn."]
    #[inline(always)]
    pub fn mem_tx_wrap_en(&self) -> MEM_TX_WRAP_EN_R {
        MEM_TX_WRAP_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_CONF")
            .field("apb_fifo_mask", &self.apb_fifo_mask())
            .field("mem_tx_wrap_en", &self.mem_tx_wrap_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to disable apb fifo access"]
    #[inline(always)]
    #[must_use]
    pub fn apb_fifo_mask(&mut self) -> APB_FIFO_MASK_W<APB_CONF_SPEC> {
        APB_FIFO_MASK_W::new(self, 0)
    }
    #[doc = "Bit 1 - when datas need to be send is more than channel's mem can store then set this bit to enable reusage of mem this bit is used together with reg_rmt_tx_lim_chn."]
    #[inline(always)]
    #[must_use]
    pub fn mem_tx_wrap_en(&mut self) -> MEM_TX_WRAP_EN_W<APB_CONF_SPEC> {
        MEM_TX_WRAP_EN_W::new(self, 1)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB_CONF_SPEC;
impl crate::RegisterSpec for APB_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb_conf::R`](R) reader structure"]
impl crate::Readable for APB_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb_conf::W`](W) writer structure"]
impl crate::Writable for APB_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB_CONF to value 0"]
impl crate::Resettable for APB_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
