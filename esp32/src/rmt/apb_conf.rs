#[doc = "Register `APB_CONF` reader"]
pub struct R(crate::R<APB_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_CONF` writer"]
pub struct W(crate::W<APB_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<APB_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APB_FIFO_MASK` reader - Set this bit to disable apb fifo access"]
pub type APB_FIFO_MASK_R = crate::BitReader;
#[doc = "Field `APB_FIFO_MASK` writer - Set this bit to disable apb fifo access"]
pub type APB_FIFO_MASK_W<'a, const O: u8> = crate::BitWriter<'a, APB_CONF_SPEC, O>;
#[doc = "Field `MEM_TX_WRAP_EN` reader - when datas need to be send is more than channel's mem can store then set this bit to enable reusage of mem this bit is used together with reg_rmt_tx_lim_chn."]
pub type MEM_TX_WRAP_EN_R = crate::BitReader;
#[doc = "Field `MEM_TX_WRAP_EN` writer - when datas need to be send is more than channel's mem can store then set this bit to enable reusage of mem this bit is used together with reg_rmt_tx_lim_chn."]
pub type MEM_TX_WRAP_EN_W<'a, const O: u8> = crate::BitWriter<'a, APB_CONF_SPEC, O>;
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
            .field(
                "apb_fifo_mask",
                &format_args!("{}", self.apb_fifo_mask().bit()),
            )
            .field(
                "mem_tx_wrap_en",
                &format_args!("{}", self.mem_tx_wrap_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APB_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to disable apb fifo access"]
    #[inline(always)]
    #[must_use]
    pub fn apb_fifo_mask(&mut self) -> APB_FIFO_MASK_W<0> {
        APB_FIFO_MASK_W::new(self)
    }
    #[doc = "Bit 1 - when datas need to be send is more than channel's mem can store then set this bit to enable reusage of mem this bit is used together with reg_rmt_tx_lim_chn."]
    #[inline(always)]
    #[must_use]
    pub fn mem_tx_wrap_en(&mut self) -> MEM_TX_WRAP_EN_W<1> {
        MEM_TX_WRAP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_conf](index.html) module"]
pub struct APB_CONF_SPEC;
impl crate::RegisterSpec for APB_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_conf::R](R) reader structure"]
impl crate::Readable for APB_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_conf::W](W) writer structure"]
impl crate::Writable for APB_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB_CONF to value 0"]
impl crate::Resettable for APB_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
