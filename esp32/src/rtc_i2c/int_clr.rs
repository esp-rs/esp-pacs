#[doc = "Register `INT_CLR` reader"]
pub type R = crate::R<INT_CLR_SPEC>;
#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `SLAVE_TRANS_COMPLETE` reader - "]
pub type SLAVE_TRANS_COMPLETE_R = crate::BitReader;
#[doc = "Field `SLAVE_TRANS_COMPLETE` writer - "]
pub type SLAVE_TRANS_COMPLETE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ARBITRATION_LOST` reader - "]
pub type ARBITRATION_LOST_R = crate::BitReader;
#[doc = "Field `ARBITRATION_LOST` writer - "]
pub type ARBITRATION_LOST_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MASTER_TRANS_COMPLETE` reader - "]
pub type MASTER_TRANS_COMPLETE_R = crate::BitReader;
#[doc = "Field `MASTER_TRANS_COMPLETE` writer - "]
pub type MASTER_TRANS_COMPLETE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TRANS_COMPLETE` reader - "]
pub type TRANS_COMPLETE_R = crate::BitReader;
#[doc = "Field `TRANS_COMPLETE` writer - "]
pub type TRANS_COMPLETE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TIME_OUT` writer - "]
pub type TIME_OUT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slave_trans_complete(&self) -> SLAVE_TRANS_COMPLETE_R {
        SLAVE_TRANS_COMPLETE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn arbitration_lost(&self) -> ARBITRATION_LOST_R {
        ARBITRATION_LOST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn master_trans_complete(&self) -> MASTER_TRANS_COMPLETE_R {
        MASTER_TRANS_COMPLETE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn trans_complete(&self) -> TRANS_COMPLETE_R {
        TRANS_COMPLETE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_CLR")
            .field("slave_trans_complete", &self.slave_trans_complete())
            .field("arbitration_lost", &self.arbitration_lost())
            .field("master_trans_complete", &self.master_trans_complete())
            .field("trans_complete", &self.trans_complete())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slave_trans_complete(&mut self) -> SLAVE_TRANS_COMPLETE_W<'_, INT_CLR_SPEC> {
        SLAVE_TRANS_COMPLETE_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn arbitration_lost(&mut self) -> ARBITRATION_LOST_W<'_, INT_CLR_SPEC> {
        ARBITRATION_LOST_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn master_trans_complete(&mut self) -> MASTER_TRANS_COMPLETE_W<'_, INT_CLR_SPEC> {
        MASTER_TRANS_COMPLETE_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn trans_complete(&mut self) -> TRANS_COMPLETE_W<'_, INT_CLR_SPEC> {
        TRANS_COMPLETE_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn time_out(&mut self) -> TIME_OUT_W<'_, INT_CLR_SPEC> {
        TIME_OUT_W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`int_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_clr::R`](R) reader structure"]
impl crate::Readable for INT_CLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01f0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {}
