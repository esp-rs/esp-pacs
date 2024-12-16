#[doc = "Register `RX_CH%sCONF0` reader"]
pub type R = crate::R<RX_CHCONF0_SPEC>;
#[doc = "Register `RX_CH%sCONF0` writer"]
pub type W = crate::W<RX_CHCONF0_SPEC>;
#[doc = "Field `DIV_CNT_CH4` reader - This register is used to configure the divider for clock of CHANNEL%s."]
pub type DIV_CNT_CH4_R = crate::FieldReader;
#[doc = "Field `DIV_CNT_CH4` writer - This register is used to configure the divider for clock of CHANNEL%s."]
pub type DIV_CNT_CH4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IDLE_THRES_CH4` reader - When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished."]
pub type IDLE_THRES_CH4_R = crate::FieldReader<u16>;
#[doc = "Field `IDLE_THRES_CH4` writer - When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished."]
pub type IDLE_THRES_CH4_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `MEM_SIZE_CH4` reader - This register is used to configure the maximum size of memory allocated to CHANNEL%s."]
pub type MEM_SIZE_CH4_R = crate::FieldReader;
#[doc = "Field `MEM_SIZE_CH4` writer - This register is used to configure the maximum size of memory allocated to CHANNEL%s."]
pub type MEM_SIZE_CH4_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CARRIER_EN_CH4` reader - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out."]
pub type CARRIER_EN_CH4_R = crate::BitReader;
#[doc = "Field `CARRIER_EN_CH4` writer - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out."]
pub type CARRIER_EN_CH4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARRIER_OUT_LV_CH4` reader - This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level."]
pub type CARRIER_OUT_LV_CH4_R = crate::BitReader;
#[doc = "Field `CARRIER_OUT_LV_CH4` writer - This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level."]
pub type CARRIER_OUT_LV_CH4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - This register is used to configure the divider for clock of CHANNEL%s."]
    #[inline(always)]
    pub fn div_cnt_ch4(&self) -> DIV_CNT_CH4_R {
        DIV_CNT_CH4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:22 - When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished."]
    #[inline(always)]
    pub fn idle_thres_ch4(&self) -> IDLE_THRES_CH4_R {
        IDLE_THRES_CH4_R::new(((self.bits >> 8) & 0x7fff) as u16)
    }
    #[doc = "Bits 24:27 - This register is used to configure the maximum size of memory allocated to CHANNEL%s."]
    #[inline(always)]
    pub fn mem_size_ch4(&self) -> MEM_SIZE_CH4_R {
        MEM_SIZE_CH4_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out."]
    #[inline(always)]
    pub fn carrier_en_ch4(&self) -> CARRIER_EN_CH4_R {
        CARRIER_EN_CH4_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level."]
    #[inline(always)]
    pub fn carrier_out_lv_ch4(&self) -> CARRIER_OUT_LV_CH4_R {
        CARRIER_OUT_LV_CH4_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CHCONF0")
            .field("div_cnt_ch4", &self.div_cnt_ch4())
            .field("idle_thres_ch4", &self.idle_thres_ch4())
            .field("mem_size_ch4", &self.mem_size_ch4())
            .field("carrier_en_ch4", &self.carrier_en_ch4())
            .field("carrier_out_lv_ch4", &self.carrier_out_lv_ch4())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - This register is used to configure the divider for clock of CHANNEL%s."]
    #[inline(always)]
    pub fn div_cnt_ch4(&mut self) -> DIV_CNT_CH4_W<RX_CHCONF0_SPEC> {
        DIV_CNT_CH4_W::new(self, 0)
    }
    #[doc = "Bits 8:22 - When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished."]
    #[inline(always)]
    pub fn idle_thres_ch4(&mut self) -> IDLE_THRES_CH4_W<RX_CHCONF0_SPEC> {
        IDLE_THRES_CH4_W::new(self, 8)
    }
    #[doc = "Bits 24:27 - This register is used to configure the maximum size of memory allocated to CHANNEL%s."]
    #[inline(always)]
    pub fn mem_size_ch4(&mut self) -> MEM_SIZE_CH4_W<RX_CHCONF0_SPEC> {
        MEM_SIZE_CH4_W::new(self, 24)
    }
    #[doc = "Bit 28 - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out."]
    #[inline(always)]
    pub fn carrier_en_ch4(&mut self) -> CARRIER_EN_CH4_W<RX_CHCONF0_SPEC> {
        CARRIER_EN_CH4_W::new(self, 28)
    }
    #[doc = "Bit 29 - This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level."]
    #[inline(always)]
    pub fn carrier_out_lv_ch4(&mut self) -> CARRIER_OUT_LV_CH4_W<RX_CHCONF0_SPEC> {
        CARRIER_OUT_LV_CH4_W::new(self, 29)
    }
}
#[doc = "Channel %s configure register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_chconf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_chconf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_CHCONF0_SPEC;
impl crate::RegisterSpec for RX_CHCONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_chconf0::R`](R) reader structure"]
impl crate::Readable for RX_CHCONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_chconf0::W`](W) writer structure"]
impl crate::Writable for RX_CHCONF0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX_CH%sCONF0 to value 0x317f_ff02"]
impl crate::Resettable for RX_CHCONF0_SPEC {
    const RESET_VALUE: u32 = 0x317f_ff02;
}
