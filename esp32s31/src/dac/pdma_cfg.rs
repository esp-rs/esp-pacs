#[doc = "Register `PDMA_CFG` reader"]
pub type R = crate::R<PDMA_CFG_SPEC>;
#[doc = "Register `PDMA_CFG` writer"]
pub type W = crate::W<PDMA_CFG_SPEC>;
#[doc = "Field `PDMA_ALTER_MODE` reader - 1:make rotation between two pads when outputting data of ppdma input path requires reg_dac_1/0_data_sel = 1 0:no change"]
pub type PDMA_ALTER_MODE_R = crate::BitReader;
#[doc = "Field `PDMA_ALTER_MODE` writer - 1:make rotation between two pads when outputting data of ppdma input path requires reg_dac_1/0_data_sel = 1 0:no change"]
pub type PDMA_ALTER_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_RESET_FIFO` writer - 1:reset dac fifo 0:no change"]
pub type PDMA_RESET_FIFO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_TRANS` reader - 1:start transaction for pdma path output 0:stop transaction for pdma path output"]
pub type PDMA_TRANS_R = crate::BitReader;
#[doc = "Field `PDMA_TRANS` writer - 1:start transaction for pdma path output 0:stop transaction for pdma path output"]
pub type PDMA_TRANS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1:make rotation between two pads when outputting data of ppdma input path requires reg_dac_1/0_data_sel = 1 0:no change"]
    #[inline(always)]
    pub fn pdma_alter_mode(&self) -> PDMA_ALTER_MODE_R {
        PDMA_ALTER_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - 1:start transaction for pdma path output 0:stop transaction for pdma path output"]
    #[inline(always)]
    pub fn pdma_trans(&self) -> PDMA_TRANS_R {
        PDMA_TRANS_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDMA_CFG")
            .field("pdma_alter_mode", &self.pdma_alter_mode())
            .field("pdma_trans", &self.pdma_trans())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 1:make rotation between two pads when outputting data of ppdma input path requires reg_dac_1/0_data_sel = 1 0:no change"]
    #[inline(always)]
    pub fn pdma_alter_mode(&mut self) -> PDMA_ALTER_MODE_W<'_, PDMA_CFG_SPEC> {
        PDMA_ALTER_MODE_W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:reset dac fifo 0:no change"]
    #[inline(always)]
    pub fn pdma_reset_fifo(&mut self) -> PDMA_RESET_FIFO_W<'_, PDMA_CFG_SPEC> {
        PDMA_RESET_FIFO_W::new(self, 1)
    }
    #[doc = "Bit 2 - 1:start transaction for pdma path output 0:stop transaction for pdma path output"]
    #[inline(always)]
    pub fn pdma_trans(&mut self) -> PDMA_TRANS_W<'_, PDMA_CFG_SPEC> {
        PDMA_TRANS_W::new(self, 2)
    }
}
#[doc = "dac cfg register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdma_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdma_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDMA_CFG_SPEC;
impl crate::RegisterSpec for PDMA_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdma_cfg::R`](R) reader structure"]
impl crate::Readable for PDMA_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pdma_cfg::W`](W) writer structure"]
impl crate::Writable for PDMA_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDMA_CFG to value 0"]
impl crate::Resettable for PDMA_CFG_SPEC {}
