#[doc = "Register `COEX_PTI` reader"]
pub type R = crate::R<COEX_PTI_SPEC>;
#[doc = "Register `COEX_PTI` writer"]
pub type W = crate::W<COEX_PTI_SPEC>;
#[doc = "Field `COEX_PTI` reader - "]
pub type COEX_PTI_R = crate::FieldReader;
#[doc = "Field `COEX_PTI` writer - "]
pub type COEX_PTI_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `COEX_ACK_PTI` reader - "]
pub type COEX_ACK_PTI_R = crate::FieldReader;
#[doc = "Field `COEX_ACK_PTI` writer - "]
pub type COEX_ACK_PTI_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CLOSE_RF_SEL` reader - "]
pub type CLOSE_RF_SEL_R = crate::BitReader;
#[doc = "Field `CLOSE_RF_SEL` writer - "]
pub type CLOSE_RF_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANT_MODE` reader - "]
pub type ANT_MODE_R = crate::FieldReader;
#[doc = "Field `ANT_MODE` writer - "]
pub type ANT_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HW_TX_ACK_ANT_MODE` reader - "]
pub type HW_TX_ACK_ANT_MODE_R = crate::FieldReader;
#[doc = "Field `HW_TX_ACK_ANT_MODE` writer - "]
pub type HW_TX_ACK_ANT_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `STBY_COEX_GRANT_MASK` reader - "]
pub type STBY_COEX_GRANT_MASK_R = crate::BitReader;
#[doc = "Field `STBY_COEX_GRANT_MASK` writer - "]
pub type STBY_COEX_GRANT_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn coex_pti(&self) -> COEX_PTI_R {
        COEX_PTI_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn coex_ack_pti(&self) -> COEX_ACK_PTI_R {
        COEX_ACK_PTI_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn close_rf_sel(&self) -> CLOSE_RF_SEL_R {
        CLOSE_RF_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn ant_mode(&self) -> ANT_MODE_R {
        ANT_MODE_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:16"]
    #[inline(always)]
    pub fn hw_tx_ack_ant_mode(&self) -> HW_TX_ACK_ANT_MODE_R {
        HW_TX_ACK_ANT_MODE_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn stby_coex_grant_mask(&self) -> STBY_COEX_GRANT_MASK_R {
        STBY_COEX_GRANT_MASK_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COEX_PTI")
            .field("coex_pti", &self.coex_pti())
            .field("coex_ack_pti", &self.coex_ack_pti())
            .field("close_rf_sel", &self.close_rf_sel())
            .field("ant_mode", &self.ant_mode())
            .field("hw_tx_ack_ant_mode", &self.hw_tx_ack_ant_mode())
            .field("stby_coex_grant_mask", &self.stby_coex_grant_mask())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn coex_pti(&mut self) -> COEX_PTI_W<'_, COEX_PTI_SPEC> {
        COEX_PTI_W::new(self, 0)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn coex_ack_pti(&mut self) -> COEX_ACK_PTI_W<'_, COEX_PTI_SPEC> {
        COEX_ACK_PTI_W::new(self, 5)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn close_rf_sel(&mut self) -> CLOSE_RF_SEL_W<'_, COEX_PTI_SPEC> {
        CLOSE_RF_SEL_W::new(self, 10)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn ant_mode(&mut self) -> ANT_MODE_W<'_, COEX_PTI_SPEC> {
        ANT_MODE_W::new(self, 11)
    }
    #[doc = "Bits 14:16"]
    #[inline(always)]
    pub fn hw_tx_ack_ant_mode(&mut self) -> HW_TX_ACK_ANT_MODE_W<'_, COEX_PTI_SPEC> {
        HW_TX_ACK_ANT_MODE_W::new(self, 14)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn stby_coex_grant_mask(&mut self) -> STBY_COEX_GRANT_MASK_W<'_, COEX_PTI_SPEC> {
        STBY_COEX_GRANT_MASK_W::new(self, 17)
    }
}
#[doc = "COEX_PTI\n\nYou can [`read`](crate::Reg::read) this register and get [`coex_pti::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`coex_pti::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COEX_PTI_SPEC;
impl crate::RegisterSpec for COEX_PTI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`coex_pti::R`](R) reader structure"]
impl crate::Readable for COEX_PTI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`coex_pti::W`](W) writer structure"]
impl crate::Writable for COEX_PTI_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COEX_PTI to value 0"]
impl crate::Resettable for COEX_PTI_SPEC {}
