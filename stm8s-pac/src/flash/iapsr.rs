#[doc = "Register `IAPSR` reader"]
pub type R = crate::R<IAPSR_SPEC>;
#[doc = "Register `IAPSR` writer"]
pub type W = crate::W<IAPSR_SPEC>;
#[doc = "Field `WR_PG_DIS` reader - "]
pub type WR_PG_DIS_R = crate::BitReader;
#[doc = "Field `WR_PG_DIS` writer - "]
pub type WR_PG_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUL` reader - "]
pub type PUL_R = crate::BitReader;
#[doc = "Field `PUL` writer - "]
pub type PUL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOP` reader - "]
pub type EOP_R = crate::BitReader;
#[doc = "Field `EOP` writer - "]
pub type EOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUL` reader - "]
pub type DUL_R = crate::BitReader;
#[doc = "Field `DUL` writer - "]
pub type DUL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HVOFF` reader - "]
pub type HVOFF_R = crate::BitReader;
#[doc = "Field `HVOFF` writer - "]
pub type HVOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wr_pg_dis(&self) -> WR_PG_DIS_R {
        WR_PG_DIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pul(&self) -> PUL_R {
        PUL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dul(&self) -> DUL_R {
        DUL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn hvoff(&self) -> HVOFF_R {
        HVOFF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn wr_pg_dis(&mut self) -> WR_PG_DIS_W<IAPSR_SPEC> {
        WR_PG_DIS_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pul(&mut self) -> PUL_W<IAPSR_SPEC> {
        PUL_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn eop(&mut self) -> EOP_W<IAPSR_SPEC> {
        EOP_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn dul(&mut self) -> DUL_W<IAPSR_SPEC> {
        DUL_W::new(self, 3)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn hvoff(&mut self) -> HVOFF_W<IAPSR_SPEC> {
        HVOFF_W::new(self, 6)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Flash in-application programming status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iapsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iapsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IAPSR_SPEC;
impl crate::RegisterSpec for IAPSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`iapsr::R`](R) reader structure"]
impl crate::Readable for IAPSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iapsr::W`](W) writer structure"]
impl crate::Writable for IAPSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IAPSR to value 0"]
impl crate::Resettable for IAPSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
