#[doc = "Register `CH%s_RX_CONF0` reader"]
pub type R = crate::R<CH_RX_CONF0_SPEC>;
#[doc = "Register `CH%s_RX_CONF0` writer"]
pub type W = crate::W<CH_RX_CONF0_SPEC>;
#[doc = "Field `DIV_CNT` reader - reg_div_cnt_ch2."]
pub type DIV_CNT_R = crate::FieldReader;
#[doc = "Field `DIV_CNT` writer - reg_div_cnt_ch2."]
pub type DIV_CNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `IDLE_THRES` reader - reg_idle_thres_ch2."]
pub type IDLE_THRES_R = crate::FieldReader<u16>;
#[doc = "Field `IDLE_THRES` writer - reg_idle_thres_ch2."]
pub type IDLE_THRES_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 15, O, u16>;
#[doc = "Field `MEM_SIZE` reader - reg_mem_size_ch2."]
pub type MEM_SIZE_R = crate::FieldReader;
#[doc = "Field `MEM_SIZE` writer - reg_mem_size_ch2."]
pub type MEM_SIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CARRIER_EN` reader - reg_carrier_en_ch2."]
pub type CARRIER_EN_R = crate::BitReader;
#[doc = "Field `CARRIER_EN` writer - reg_carrier_en_ch2."]
pub type CARRIER_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CARRIER_OUT_LV` reader - reg_carrier_out_lv_ch2."]
pub type CARRIER_OUT_LV_R = crate::BitReader;
#[doc = "Field `CARRIER_OUT_LV` writer - reg_carrier_out_lv_ch2."]
pub type CARRIER_OUT_LV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:7 - reg_div_cnt_ch2."]
    #[inline(always)]
    pub fn div_cnt(&self) -> DIV_CNT_R {
        DIV_CNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:22 - reg_idle_thres_ch2."]
    #[inline(always)]
    pub fn idle_thres(&self) -> IDLE_THRES_R {
        IDLE_THRES_R::new(((self.bits >> 8) & 0x7fff) as u16)
    }
    #[doc = "Bits 23:25 - reg_mem_size_ch2."]
    #[inline(always)]
    pub fn mem_size(&self) -> MEM_SIZE_R {
        MEM_SIZE_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 28 - reg_carrier_en_ch2."]
    #[inline(always)]
    pub fn carrier_en(&self) -> CARRIER_EN_R {
        CARRIER_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - reg_carrier_out_lv_ch2."]
    #[inline(always)]
    pub fn carrier_out_lv(&self) -> CARRIER_OUT_LV_R {
        CARRIER_OUT_LV_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_RX_CONF0")
            .field("div_cnt", &format_args!("{}", self.div_cnt().bits()))
            .field("idle_thres", &format_args!("{}", self.idle_thres().bits()))
            .field("mem_size", &format_args!("{}", self.mem_size().bits()))
            .field("carrier_en", &format_args!("{}", self.carrier_en().bit()))
            .field(
                "carrier_out_lv",
                &format_args!("{}", self.carrier_out_lv().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_RX_CONF0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - reg_div_cnt_ch2."]
    #[inline(always)]
    #[must_use]
    pub fn div_cnt(&mut self) -> DIV_CNT_W<CH_RX_CONF0_SPEC, 0> {
        DIV_CNT_W::new(self)
    }
    #[doc = "Bits 8:22 - reg_idle_thres_ch2."]
    #[inline(always)]
    #[must_use]
    pub fn idle_thres(&mut self) -> IDLE_THRES_W<CH_RX_CONF0_SPEC, 8> {
        IDLE_THRES_W::new(self)
    }
    #[doc = "Bits 23:25 - reg_mem_size_ch2."]
    #[inline(always)]
    #[must_use]
    pub fn mem_size(&mut self) -> MEM_SIZE_W<CH_RX_CONF0_SPEC, 23> {
        MEM_SIZE_W::new(self)
    }
    #[doc = "Bit 28 - reg_carrier_en_ch2."]
    #[inline(always)]
    #[must_use]
    pub fn carrier_en(&mut self) -> CARRIER_EN_W<CH_RX_CONF0_SPEC, 28> {
        CARRIER_EN_W::new(self)
    }
    #[doc = "Bit 29 - reg_carrier_out_lv_ch2."]
    #[inline(always)]
    #[must_use]
    pub fn carrier_out_lv(&mut self) -> CARRIER_OUT_LV_W<CH_RX_CONF0_SPEC, 29> {
        CARRIER_OUT_LV_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RMT_CH2CONF0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_rx_conf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_rx_conf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_RX_CONF0_SPEC;
impl crate::RegisterSpec for CH_RX_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_rx_conf0::R`](R) reader structure"]
impl crate::Readable for CH_RX_CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_rx_conf0::W`](W) writer structure"]
impl crate::Writable for CH_RX_CONF0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH%s_RX_CONF0 to value 0x30ff_ff02"]
impl crate::Resettable for CH_RX_CONF0_SPEC {
    const RESET_VALUE: Self::Ux = 0x30ff_ff02;
}
