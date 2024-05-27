#[doc = "Register `CH%sCONF0` reader"]
pub type R = crate::R<CHCONF0_SPEC>;
#[doc = "Register `CH%sCONF0` writer"]
pub type W = crate::W<CHCONF0_SPEC>;
#[doc = "Field `DIV_CNT` reader - This register is used to configure the frequency divider's factor in channel0."]
pub type DIV_CNT_R = crate::FieldReader;
#[doc = "Field `DIV_CNT` writer - This register is used to configure the frequency divider's factor in channel0."]
pub type DIV_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IDLE_THRES` reader - In receive mode when no edge is detected on the input signal for longer than reg_idle_thres_ch0 then the receive process is done."]
pub type IDLE_THRES_R = crate::FieldReader<u16>;
#[doc = "Field `IDLE_THRES` writer - In receive mode when no edge is detected on the input signal for longer than reg_idle_thres_ch0 then the receive process is done."]
pub type IDLE_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MEM_SIZE` reader - This register is used to configure the the amount of memory blocks allocated to channel0."]
pub type MEM_SIZE_R = crate::FieldReader;
#[doc = "Field `MEM_SIZE` writer - This register is used to configure the the amount of memory blocks allocated to channel0."]
pub type MEM_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CARRIER_EN` reader - This is the carrier modulation enable control bit for channel0."]
pub type CARRIER_EN_R = crate::BitReader;
#[doc = "Field `CARRIER_EN` writer - This is the carrier modulation enable control bit for channel0."]
pub type CARRIER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARRIER_OUT_LV` reader - This bit is used to configure the way carrier wave is modulated for channel0.1'b1:transmit on low output level 1'b0:transmit on high output level."]
pub type CARRIER_OUT_LV_R = crate::BitReader;
#[doc = "Field `CARRIER_OUT_LV` writer - This bit is used to configure the way carrier wave is modulated for channel0.1'b1:transmit on low output level 1'b0:transmit on high output level."]
pub type CARRIER_OUT_LV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_PD` reader - This bit is used to reduce power consumed by mem. 1:mem is in low power state."]
pub type MEM_PD_R = crate::BitReader;
#[doc = "Field `MEM_PD` writer - This bit is used to reduce power consumed by mem. 1:mem is in low power state."]
pub type MEM_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - This bit is used to control clock.when software config RMT internal registers it controls the register clock."]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - This bit is used to control clock.when software config RMT internal registers it controls the register clock."]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - This register is used to configure the frequency divider's factor in channel0."]
    #[inline(always)]
    pub fn div_cnt(&self) -> DIV_CNT_R {
        DIV_CNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:23 - In receive mode when no edge is detected on the input signal for longer than reg_idle_thres_ch0 then the receive process is done."]
    #[inline(always)]
    pub fn idle_thres(&self) -> IDLE_THRES_R {
        IDLE_THRES_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:27 - This register is used to configure the the amount of memory blocks allocated to channel0."]
    #[inline(always)]
    pub fn mem_size(&self) -> MEM_SIZE_R {
        MEM_SIZE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - This is the carrier modulation enable control bit for channel0."]
    #[inline(always)]
    pub fn carrier_en(&self) -> CARRIER_EN_R {
        CARRIER_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This bit is used to configure the way carrier wave is modulated for channel0.1'b1:transmit on low output level 1'b0:transmit on high output level."]
    #[inline(always)]
    pub fn carrier_out_lv(&self) -> CARRIER_OUT_LV_R {
        CARRIER_OUT_LV_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - This bit is used to reduce power consumed by mem. 1:mem is in low power state."]
    #[inline(always)]
    pub fn mem_pd(&self) -> MEM_PD_R {
        MEM_PD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This bit is used to control clock.when software config RMT internal registers it controls the register clock."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHCONF0")
            .field("div_cnt", &self.div_cnt())
            .field("idle_thres", &self.idle_thres())
            .field("mem_size", &self.mem_size())
            .field("carrier_en", &self.carrier_en())
            .field("carrier_out_lv", &self.carrier_out_lv())
            .field("mem_pd", &self.mem_pd())
            .field("clk_en", &self.clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - This register is used to configure the frequency divider's factor in channel0."]
    #[inline(always)]
    #[must_use]
    pub fn div_cnt(&mut self) -> DIV_CNT_W<CHCONF0_SPEC> {
        DIV_CNT_W::new(self, 0)
    }
    #[doc = "Bits 8:23 - In receive mode when no edge is detected on the input signal for longer than reg_idle_thres_ch0 then the receive process is done."]
    #[inline(always)]
    #[must_use]
    pub fn idle_thres(&mut self) -> IDLE_THRES_W<CHCONF0_SPEC> {
        IDLE_THRES_W::new(self, 8)
    }
    #[doc = "Bits 24:27 - This register is used to configure the the amount of memory blocks allocated to channel0."]
    #[inline(always)]
    #[must_use]
    pub fn mem_size(&mut self) -> MEM_SIZE_W<CHCONF0_SPEC> {
        MEM_SIZE_W::new(self, 24)
    }
    #[doc = "Bit 28 - This is the carrier modulation enable control bit for channel0."]
    #[inline(always)]
    #[must_use]
    pub fn carrier_en(&mut self) -> CARRIER_EN_W<CHCONF0_SPEC> {
        CARRIER_EN_W::new(self, 28)
    }
    #[doc = "Bit 29 - This bit is used to configure the way carrier wave is modulated for channel0.1'b1:transmit on low output level 1'b0:transmit on high output level."]
    #[inline(always)]
    #[must_use]
    pub fn carrier_out_lv(&mut self) -> CARRIER_OUT_LV_W<CHCONF0_SPEC> {
        CARRIER_OUT_LV_W::new(self, 29)
    }
    #[doc = "Bit 30 - This bit is used to reduce power consumed by mem. 1:mem is in low power state."]
    #[inline(always)]
    #[must_use]
    pub fn mem_pd(&mut self) -> MEM_PD_W<CHCONF0_SPEC> {
        MEM_PD_W::new(self, 30)
    }
    #[doc = "Bit 31 - This bit is used to control clock.when software config RMT internal registers it controls the register clock."]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<CHCONF0_SPEC> {
        CLK_EN_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chconf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chconf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CH%sCONF0 to value 0x3110_0002"]
impl crate::Resettable for CHCONF0_SPEC {
    const RESET_VALUE: u32 = 0x3110_0002;
}
