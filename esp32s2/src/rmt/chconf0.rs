#[doc = "Register `CH%sCONF0` reader"]
pub type R = crate::R<CHCONF0_SPEC>;
#[doc = "Register `CH%sCONF0` writer"]
pub type W = crate::W<CHCONF0_SPEC>;
#[doc = "Field `DIV_CNT` reader - This register is used to configure the divider for clock of CHANNEL%s."]
pub type DIV_CNT_R = crate::FieldReader;
#[doc = "Field `DIV_CNT` writer - This register is used to configure the divider for clock of CHANNEL%s."]
pub type DIV_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IDLE_THRES` reader - When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished."]
pub type IDLE_THRES_R = crate::FieldReader<u16>;
#[doc = "Field `IDLE_THRES` writer - When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished."]
pub type IDLE_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MEM_SIZE` reader - This register is used to configure the maximum size of memory allocated to CHANNEL%s."]
pub type MEM_SIZE_R = crate::FieldReader;
#[doc = "Field `MEM_SIZE` writer - This register is used to configure the maximum size of memory allocated to CHANNEL%s."]
pub type MEM_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CARRIER_EFF_EN` reader - 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1."]
pub type CARRIER_EFF_EN_R = crate::BitReader;
#[doc = "Field `CARRIER_EFF_EN` writer - 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1."]
pub type CARRIER_EFF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARRIER_EN` reader - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out."]
pub type CARRIER_EN_R = crate::BitReader;
#[doc = "Field `CARRIER_EN` writer - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out."]
pub type CARRIER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARRIER_OUT_LV` reader - This bit is used to configure the position of carrier wave for CHANNEL%s. 1'h0: add carrier wave on low level. 1'h1: add carrier wave on high level."]
pub type CARRIER_OUT_LV_R = crate::BitReader;
#[doc = "Field `CARRIER_OUT_LV` writer - This bit is used to configure the position of carrier wave for CHANNEL%s. 1'h0: add carrier wave on low level. 1'h1: add carrier wave on high level."]
pub type CARRIER_OUT_LV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - This register is used to configure the divider for clock of CHANNEL%s."]
    #[inline(always)]
    pub fn div_cnt(&self) -> DIV_CNT_R {
        DIV_CNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:23 - When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished."]
    #[inline(always)]
    pub fn idle_thres(&self) -> IDLE_THRES_R {
        IDLE_THRES_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:26 - This register is used to configure the maximum size of memory allocated to CHANNEL%s."]
    #[inline(always)]
    pub fn mem_size(&self) -> MEM_SIZE_R {
        MEM_SIZE_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1."]
    #[inline(always)]
    pub fn carrier_eff_en(&self) -> CARRIER_EFF_EN_R {
        CARRIER_EFF_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out."]
    #[inline(always)]
    pub fn carrier_en(&self) -> CARRIER_EN_R {
        CARRIER_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This bit is used to configure the position of carrier wave for CHANNEL%s. 1'h0: add carrier wave on low level. 1'h1: add carrier wave on high level."]
    #[inline(always)]
    pub fn carrier_out_lv(&self) -> CARRIER_OUT_LV_R {
        CARRIER_OUT_LV_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHCONF0")
            .field("div_cnt", &self.div_cnt())
            .field("idle_thres", &self.idle_thres())
            .field("mem_size", &self.mem_size())
            .field("carrier_eff_en", &self.carrier_eff_en())
            .field("carrier_en", &self.carrier_en())
            .field("carrier_out_lv", &self.carrier_out_lv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - This register is used to configure the divider for clock of CHANNEL%s."]
    #[inline(always)]
    #[must_use]
    pub fn div_cnt(&mut self) -> DIV_CNT_W<CHCONF0_SPEC> {
        DIV_CNT_W::new(self, 0)
    }
    #[doc = "Bits 8:23 - When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished."]
    #[inline(always)]
    #[must_use]
    pub fn idle_thres(&mut self) -> IDLE_THRES_W<CHCONF0_SPEC> {
        IDLE_THRES_W::new(self, 8)
    }
    #[doc = "Bits 24:26 - This register is used to configure the maximum size of memory allocated to CHANNEL%s."]
    #[inline(always)]
    #[must_use]
    pub fn mem_size(&mut self) -> MEM_SIZE_W<CHCONF0_SPEC> {
        MEM_SIZE_W::new(self, 24)
    }
    #[doc = "Bit 27 - 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1."]
    #[inline(always)]
    #[must_use]
    pub fn carrier_eff_en(&mut self) -> CARRIER_EFF_EN_W<CHCONF0_SPEC> {
        CARRIER_EFF_EN_W::new(self, 27)
    }
    #[doc = "Bit 28 - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out."]
    #[inline(always)]
    #[must_use]
    pub fn carrier_en(&mut self) -> CARRIER_EN_W<CHCONF0_SPEC> {
        CARRIER_EN_W::new(self, 28)
    }
    #[doc = "Bit 29 - This bit is used to configure the position of carrier wave for CHANNEL%s. 1'h0: add carrier wave on low level. 1'h1: add carrier wave on high level."]
    #[inline(always)]
    #[must_use]
    pub fn carrier_out_lv(&mut self) -> CARRIER_OUT_LV_W<CHCONF0_SPEC> {
        CARRIER_OUT_LV_W::new(self, 29)
    }
}
#[doc = "Channel %s configure register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chconf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chconf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHCONF0_SPEC;
impl crate::RegisterSpec for CHCONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chconf0::R`](R) reader structure"]
impl crate::Readable for CHCONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chconf0::W`](W) writer structure"]
impl crate::Writable for CHCONF0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH%sCONF0 to value 0x3910_0002"]
impl crate::Resettable for CHCONF0_SPEC {
    const RESET_VALUE: u32 = 0x3910_0002;
}
